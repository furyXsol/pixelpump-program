use std::ops::DerefMut;
use crate::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    /// Admin address
  #[account(
    mut,
    address = config.authority
  )]
  pub authority: Signer<'info>,

  #[account(
    mut,
    seeds=[
      CONFIG_SEED,
    ],
    bump = config.bump
  )]
  pub config: Account<'info, Config>,
}

impl UpdateConfig<'_> {
  pub fn apply(ctx: &mut Context<UpdateConfig>, params: &UpdateConfigParams) -> Result<()> {
    let config = ctx.accounts.config.deref_mut();

    if params.authority.is_some() {
      config.authority = params.authority.unwrap();
    }
    if params.fee_recipient.is_some() {
      config.fee_recipient = params.fee_recipient.unwrap();
    }
    if params.initial_virtual_token_reserves.is_some() {
      config.initial_virtual_token_reserves = params.initial_virtual_token_reserves.unwrap();
    }
    if params.initial_virtual_sol_reserves.is_some() {
      config.initial_virtual_sol_reserves = params.initial_virtual_sol_reserves.unwrap();
    }
    if params.initial_real_token_reserves.is_some() {
      config.initial_real_token_reserves = params.initial_real_token_reserves.unwrap();
    }
    if params.fee_base_points.is_some() {
      require!(params.fee_base_points.unwrap() < 5000, PixelError::InvalidParam);
      config.fee_base_points = params.fee_base_points.unwrap();
    }
    if params.token_total_supply.is_some() {
      config.token_total_supply = params.token_total_supply.unwrap();
    }

    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
  pub authority: Option<Pubkey>,
  pub fee_recipient: Option<Pubkey>,
  pub initial_virtual_token_reserves: Option<u64>,
  pub initial_virtual_sol_reserves: Option<u64>,
  pub initial_real_token_reserves: Option<u64>,
  pub token_total_supply: Option<u64>,
  pub fee_base_points: Option<u64>, // 100 is 1%
}
