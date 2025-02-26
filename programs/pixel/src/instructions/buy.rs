use std::cmp::min;

use crate::*;

use anchor_spl::{
  associated_token::AssociatedToken,
  token::Token,
  token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Buy<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    mint::token_program = token_program,
  )]
  pub token_mint: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    seeds = [ CONFIG_SEED ],
    bump = config.bump,
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
    mut,
    address = config.fee_recipient,
  )]
  pub fee_recipient: UncheckedAccount<'info>,

  /// CHECK
  #[account(
    mut,
    seeds = [
      STAKE_HOLDER_SEED,
      config.stake_token.key().as_ref(),
    ],
    bump = stake_holder.bump,
  )]
  pub stake_holder: Box<Account<'info, StakeHolder>>,

  #[account(
    mut,
    seeds = [
      BONDING_CURVE_SEED,
      token_mint.key().as_ref(),
    ],
    bump = bonding_curve.bump
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = bonding_curve,
    token::token_program = token_program,
  )]
  pub associted_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = user,
    token::token_program = token_program,
  )]
  pub associted_user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl Buy<'_> {
  pub fn apply(ctx: &mut Context<Buy>, params: &BuyParams) -> Result<()> {

    require!(!ctx.accounts.bonding_curve.complete, PixelError::SoldAllToken);

    let decimals = ctx.accounts.token_mint.decimals;
    let amount = params.amount;
    require!(amount > 0, PixelError::InvalidParam);
    let real_token_reserves = ctx.accounts.bonding_curve.real_token_reserves;

    let min_amount = min(amount, real_token_reserves);
    let virtual_sol_reserves = ctx.accounts.bonding_curve.virtual_sol_reserves;
    let virtual_token_reserves = ctx.accounts.bonding_curve.virtual_token_reserves;
    require!(virtual_token_reserves > min_amount, PixelError::NotEnoughAmount);
    let sol_amount = ((min_amount as u128) * (virtual_sol_reserves as u128) / ((virtual_token_reserves as u128)- (min_amount as u128)) + 1_u128) as u64;
    let fee_sol_amount = sol_amount * (ctx.accounts.config.fee_base_points as u64) / 10000;

    //
    let fee_staker_amount = fee_sol_amount * (ctx.accounts.config.fee_stakeholders as u64) /10000;
    let fee_recipient_amount = fee_sol_amount - fee_staker_amount;

    require!(sol_amount <= params.max_sol_cost, PixelError::SlippageExceed);

    //transfer sol to fee_receipient
    if fee_recipient_amount > 0 {
      transfer_sol(
          ctx.accounts.user.to_account_info(),
          ctx.accounts.fee_recipient.to_account_info(),
          fee_recipient_amount,
      )?;
    }
    // transfer sol to stake_holder
    if fee_staker_amount > 0 {
      transfer_sol(
        ctx.accounts.user.to_account_info(),
        ctx.accounts.stake_holder.to_account_info(),
        fee_staker_amount,
      )?;
      //update stake_holder status
      let current_time = Clock::get()?.unix_timestamp as u32;
      let first_epoch_start_time = ctx.accounts.stake_holder.first_epoch_start_time;
      let epoch_duration = ctx.accounts.config.epoch_duration;
      let current_epoch = ((current_time - first_epoch_start_time) / epoch_duration) as u16;
      require!(current_epoch < MAX_EPOCH, PixelError::EpochExceed);
      if ctx.accounts.stake_holder.rewards.contains_key(&current_epoch) {
          if let Some(x) = ctx.accounts.stake_holder.rewards.get_mut(&current_epoch) {
            *x += fee_staker_amount;
          }
      } else {
        ctx.accounts.stake_holder.rewards.insert(
          current_epoch,
          fee_staker_amount,
        );
      }
    }

    //transfer sol to bonding_curve
    transfer_sol(
      ctx.accounts.user.to_account_info(),
      ctx.accounts.bonding_curve.to_account_info(),
      sol_amount,
    )?;

    //transfer token from bonding_curve to user
    let token_mint = ctx.accounts.token_mint.key();
    let vault_seeds = &[
        BONDING_CURVE_SEED,
        token_mint.as_ref(),
        &[ctx.accounts.bonding_curve.bump],
    ];

    let vault_signer_seeds = &[&vault_seeds[..]];

    transfer_token_from_vault_to_user(
        ctx.accounts.bonding_curve.to_account_info(),
        ctx.accounts.associted_bonding_curve.to_account_info(),
        ctx.accounts.associted_user_token_account.to_account_info(),
        ctx.accounts.token_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        min_amount,
        decimals,
        vault_signer_seeds,
    )?;

    //update bonding_curve
    ctx.accounts.bonding_curve.real_sol_reserves += sol_amount;
    ctx.accounts.bonding_curve.real_token_reserves -= min_amount;
    ctx.accounts.bonding_curve.virtual_sol_reserves += sol_amount;
    ctx.accounts.bonding_curve.virtual_token_reserves -= min_amount;
    let mut is_completed = false;
    if ctx.accounts.bonding_curve.real_token_reserves == 0 {
      ctx.accounts.bonding_curve.complete = true;
      is_completed = true;
    }

    emit!(BuyEvent {
        mint: ctx.accounts.token_mint.key(),
        token_output: min_amount,
        sol_input: sol_amount,
        buyer: ctx.accounts.user.key(),
        is_completed,
    });
    Ok(())
  }
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BuyParams {
  pub amount: u64,
  pub max_sol_cost: u64,
}
