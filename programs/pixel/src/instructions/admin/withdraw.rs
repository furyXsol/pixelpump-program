use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
  #[account(
      mut,
      address = config.authority
  )]
  pub authority: Signer<'info>,

  #[account(
    mut,
    mint::token_program = token_program,
  )]
  pub token_mint: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK
  #[account(
    mut,
    seeds=[
      CONFIG_SEED
    ],
    bump = config.bump
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
      mut,
      seeds = [
      BONDING_CURVE_SEED,
      token_mint.key().as_ref()
      ],
      bump = bonding_curve.bump,
  )]
  pub bonding_curve: Account<'info, BondingCurve>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = bonding_curve,
    token::token_program = token_program,
  )]
  pub associted_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    associated_token::mint = token_mint,
    associated_token::authority = authority,
    token::token_program = token_program,
  )]
  pub associted_admin_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
impl Withdraw<'_> {
  pub fn apply(ctx: &mut Context<Withdraw>) -> Result<()> {

    let sol_amount = ctx.accounts.bonding_curve.to_account_info().lamports();
    let token_amount = ctx.accounts.associted_bonding_curve.amount;
    //transfer token from vault to user
    let token_mint = ctx.accounts.token_mint.key();
    let vault_seeds = &[
        BONDING_CURVE_SEED,
        token_mint.as_ref(),
        &[ctx.accounts.bonding_curve.bump],
    ];
    let vault_signer_seeds = &[&vault_seeds[..]];
    let decimals = ctx.accounts.token_mint.decimals;
    transfer_token_from_vault_to_user(
        ctx.accounts.bonding_curve.to_account_info(),
        ctx.accounts.associted_bonding_curve.to_account_info(),
        ctx.accounts.associted_admin_token_account.to_account_info(),
        ctx.accounts.token_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        token_amount,
        decimals,
        vault_signer_seeds,
    )?;

    //transfer sol from vault to admin
    transfer_sol_from_vault_to_user(
        ctx.accounts.bonding_curve.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        sol_amount,
    )?;

    emit!(WithdrawEvent {
        mint: ctx.accounts.token_mint.key(),
        withdrawer: ctx.accounts.authority.key(),
        sol_output: sol_amount,
        token_output: token_amount,
    });

    Ok(())
  }
}