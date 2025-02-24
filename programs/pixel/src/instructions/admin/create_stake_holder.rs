use std::ops::DerefMut;
use crate::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::Token,
  token_interface::{ Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateStakeHolder<'info> {
  #[account(
    mut,
  )]
  pub payer: Signer<'info>,

  #[account(
    seeds = [ CONFIG_SEED ],
    bump = config.bump,
  )]
  pub config: Box<Account<'info, Config>>,

  #[account(
    mint::token_program = token_program,
    address = config.stake_token,
  )]
  pub stake_token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    init,
    seeds=[
      STAKE_HOLDER_SEED,
      stake_token.key().as_ref(),
    ],
    bump,
    payer = payer,
    space = 8 + 10*300,
  )]
  pub stake_holder: Account<'info, StakeHolder>,
  #[account(
    init,
    associated_token::mint = stake_token,
    associated_token::authority = stake_holder,
    token::token_program = token_program,
    payer = payer,
  )]
  pub stake_holder_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl CreateStakeHolder<'_> {
  pub fn apply(ctx: &mut Context<CreateStakeHolder>) -> Result<()> {
    let stake_holder = ctx.accounts.stake_holder.deref_mut();
    require!(!stake_holder.initialized, PixelError::AlreadyInitialized);
    stake_holder.initialized = true;
    stake_holder.bump = ctx.bumps.stake_holder;
    stake_holder.first_epoch_start_time = Clock::get()?.unix_timestamp as u32; //current time
    stake_holder.rewards.insert(0, 0);
    stake_holder.total_stakes.insert(0, 0);
    Ok(())
  }
}
