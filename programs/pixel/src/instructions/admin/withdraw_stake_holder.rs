use crate::*;

#[derive(Accounts)]
pub struct WithdrawStakeHolder<'info> {
  #[account(
      mut,
      address = config.authority
  )]
  pub authority: Signer<'info>,

  /// CHECK
  #[account(
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
      STAKE_HOLDER_SEED,
      config.stake_token.key().as_ref(),
    ],
    bump = stake_holder.bump,
  )]
  pub stake_holder: Box<Account<'info, StakeHolder>>,
  pub system_program: Program<'info, System>,
}
impl WithdrawStakeHolder<'_> {
  pub fn apply(ctx: &mut Context<WithdrawStakeHolder>) -> Result<()> {

    let sol_amount = ctx.accounts.stake_holder.to_account_info().lamports();
    //transfer sol from vault to admin
    transfer_sol_from_vault_to_user(
        ctx.accounts.stake_holder.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        sol_amount,
    )?;
    Ok(())
  }
}
