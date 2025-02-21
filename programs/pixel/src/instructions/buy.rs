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
    let real_token_reserves = ctx.accounts.bonding_curve.real_token_reserves;

    let min_amount = min(amount, real_token_reserves);
    let virtual_sol_reserves = ctx.accounts.bonding_curve.virtual_sol_reserves;
    let virtual_token_reserves = ctx.accounts.bonding_curve.virtual_token_reserves;
    let sol_amount = ((min_amount as u128) * (virtual_sol_reserves as u128) / ((virtual_token_reserves as u128)- (min_amount as u128)) + 1_u128) as u64;
    let fee_sol_amount = sol_amount * ctx.accounts.config.fee_base_points / 10000;

    require!(sol_amount <= params.max_sol_cost, PixelError::SlippageExceed);

    //transfer sol to fee_receipient
    transfer_sol(
        ctx.accounts.user.to_account_info(),
        ctx.accounts.fee_recipient.to_account_info(),
        fee_sol_amount,
    )?;

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
