use crate::*;

use anchor_spl::{
  associated_token::AssociatedToken,
  token::Token,
  token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Sell<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

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

impl Sell<'_> {
  pub fn apply(ctx: &mut Context<Sell>, params: &SellParams) -> Result<()> {

    require!(!ctx.accounts.bonding_curve.complete, PixelError::SoldAllToken);

    let decimals = ctx.accounts.token_mint.decimals;
    let amount = params.amount;
    let virtual_sol_reserves = ctx.accounts.bonding_curve.virtual_sol_reserves;
    let virtual_token_reserves = ctx.accounts.bonding_curve.virtual_token_reserves;
    let real_sol_reserves = ctx.accounts.bonding_curve.real_sol_reserves;
    let sol_amount = ((amount as u128) * (virtual_sol_reserves as u128) / ((virtual_token_reserves as u128) + (amount as u128))) as u64;

    require!(sol_amount <= real_sol_reserves, PixelError::NotEnoughToken);
    let fee_sol_amount = sol_amount * ctx.accounts.config.fee_base_points / 10000;
    let user_sol_amount = sol_amount - fee_sol_amount;

    require!(params.min_sol_output <= user_sol_amount, PixelError::SlippageExceed);

    //transfer token from user to bonding_vurve
    transfer_token_from_user_to_vault(
      ctx.accounts.user.to_account_info(),
      ctx.accounts.associted_user_token_account.to_account_info(),
      ctx.accounts.associted_bonding_curve.to_account_info(),
      ctx.accounts.token_mint.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      amount,
      decimals
    )?;

    //transfer sol from vault to fee_receipient
    transfer_sol_from_vault_to_user(
        ctx.accounts.bonding_curve.to_account_info(),
        ctx.accounts.fee_recipient.to_account_info(),
        fee_sol_amount,
    )?;

    //transfer sol from bonding_curve to user
    transfer_sol_from_vault_to_user(
      ctx.accounts.bonding_curve.to_account_info(),
      ctx.accounts.user.to_account_info(),
      user_sol_amount,
    )?;

    //update bonding_curve
    ctx.accounts.bonding_curve.real_sol_reserves -= sol_amount;
    ctx.accounts.bonding_curve.real_token_reserves += amount;
    ctx.accounts.bonding_curve.virtual_sol_reserves -= sol_amount;
    ctx.accounts.bonding_curve.virtual_token_reserves += amount;

    emit!(SellEvent {
        mint: ctx.accounts.token_mint.key(),
        token_input: amount,
        sol_output: sol_amount,
        seller: ctx.accounts.user.key()
    });
    Ok(())
  }
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SellParams {
  pub amount: u64,
  pub min_sol_output: u64,
}
