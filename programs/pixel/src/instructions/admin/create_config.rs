use std::ops::DerefMut;
use crate::*;

#[derive(Accounts)]
pub struct CreateConfig<'info> {
  #[account(
    mut,
  )]
  pub payer: Signer<'info>,

  #[account(
    init,
    seeds=[
      CONFIG_SEED,
    ],
    bump,
    payer = payer,
    space = 8 + Config::INIT_SPACE
  )]
  pub config: Account<'info, Config>,
  pub system_program: Program<'info, System>,
}

impl CreateConfig<'_> {
  pub fn apply(ctx: &mut Context<CreateConfig>, params: &CreateConfigParams) -> Result<()> {
    let config = ctx.accounts.config.deref_mut();
    require!(!config.initialized, PixelError::AlreadyInitialized);
    require!(params.fee_base_points <= 1000, PixelError::InvalidParam); //less than 10%
    require!(params.fee_stakeholders <= 8000, PixelError::InvalidParam); // less than 80%
    require!(
      params.epoch_duration >= 259200 &&
      params.epoch_duration <= 604800,
    PixelError::InvalidParam);  // from 3 days and 7 days
    require!(
      params.initial_virtual_token_reserves > params.initial_real_token_reserves,
      PixelError::InvalidParam
    );

    config.authority = params.authority;
    config.fee_recipient = params.fee_recipient;
    config.initial_virtual_token_reserves = params.initial_virtual_token_reserves;
    config.initial_virtual_sol_reserves = params.initial_virtual_sol_reserves;
    config.initial_real_token_reserves = params.initial_real_token_reserves;
    config.token_total_supply = params.token_total_supply;
    config.fee_base_points = params.fee_base_points; //trade_fee
    config.fee_stakeholders = params.fee_stakeholders; // 30% fee go to stake holders
    config.bump = ctx.bumps.config;
    config.stake_token = params.stake_token;
    config.epoch_duration = params.epoch_duration;
    config.initialized = true;
    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateConfigParams {
  pub authority: Pubkey,
  pub fee_recipient: Pubkey,
  pub initial_virtual_token_reserves: u64,
  pub initial_virtual_sol_reserves: u64,
  pub initial_real_token_reserves: u64,
  pub token_total_supply: u64,
  pub fee_base_points: u16, // 100 is 1%
  pub fee_stakeholders: u16, // 100 is 1%,  30%
  pub stake_token: Pubkey,
  pub epoch_duration: u32,
}
