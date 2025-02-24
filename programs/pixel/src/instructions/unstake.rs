use crate::*;

use anchor_spl::{
  associated_token::AssociatedToken,
  token::Token,
  token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    address = config.stake_token
  )]
  pub stake_token_mint: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    seeds = [ CONFIG_SEED ],
    bump = config.bump,
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
    mut,
    seeds = [
      STAKE_HOLDER_SEED,
      stake_token_mint.key().as_ref(),
    ],
    bump = stake_holder.bump,
  )]
  pub stake_holder: Box<Account<'info, StakeHolder>>,

  #[account(
    mut,
    seeds = [
      USER_STAKE_INFO_SEED,
      stake_token_mint.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = user_stake_info.bump,
  )]
  pub user_stake_info: Box<Account<'info, UserStakeInfo>>,

  #[account(
    mut,
    associated_token::mint = stake_token_mint,
    associated_token::authority = user,
    token::token_program = token_program,
  )]
  pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    mut,
    associated_token::mint = stake_token_mint,
    associated_token::authority = stake_holder,
    token::token_program = token_program,
  )]
  pub stake_holder_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl Unstake<'_> {
  pub fn apply(ctx: &mut Context<Unstake>, params: &UnstakeParams) -> Result<()> {
    let unstake_amount = params.amount;
    let current_total_stake_amount = ctx.accounts.stake_holder.curent_total_stake;
    require!(unstake_amount < current_total_stake_amount, PixelError::InvalidParam);

    let decimals = ctx.accounts.stake_token_mint.decimals;
    // transfer stake token to user ata
    let token_mint = ctx.accounts.stake_token_mint.key();
    let seeds: &[&[u8]; 3] = &[
      STAKE_HOLDER_SEED,
      token_mint.as_ref(),
      &[ctx.accounts.stake_holder.bump],
    ];

    let signer_seeds = &[&seeds[..]];
    transfer_token_from_vault_to_user(
      ctx.accounts.stake_holder.to_account_info(),
      ctx.accounts.stake_holder_ata.to_account_info(),
      ctx.accounts.user_ata.to_account_info(),
      ctx.accounts.stake_token_mint.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      unstake_amount,
      decimals,
      signer_seeds,
    )?;

    //update stake-holder's status
    let current_time = Clock::get()?.unix_timestamp as u32;
    let first_epoch_start_time = ctx.accounts.stake_holder.first_epoch_start_time;
    let epoch_duration = ctx.accounts.config.epoch_duration;
    let current_epoch = (current_time - first_epoch_start_time / epoch_duration) as u16;

    // update total_stake_amount for next epoch.
    if ctx.accounts.stake_holder.total_stakes.contains_key(&(current_epoch + 1)) {
      if let Some(x) = ctx.accounts.stake_holder.total_stakes.get_mut(&(current_epoch + 1)) {
        *x -= unstake_amount;
      }
    } else {
      ctx.accounts.stake_holder.total_stakes.insert(
        current_epoch + 1,
        current_total_stake_amount - unstake_amount,
      );
    }
    // calculate pending_reward
    let last_epoch = ctx.accounts.user_stake_info.last_epoch;
    let user_stake_amount = ctx.accounts.user_stake_info.stake_amount;
    if user_stake_amount > 0 {
      if last_epoch + 1 < current_epoch {
        let mut i = last_epoch + 1;
        let mut pending_rewards: u64 = 0;
        let mut prev_total_stake_amount = 0;
        loop {
          if i >= current_epoch {
            break;
          }
          if ctx.accounts.stake_holder.rewards.contains_key(&i) {
            let epoch_reward = *ctx.accounts.stake_holder.rewards.get(&i).unwrap();
            if ctx.accounts.stake_holder.total_stakes.contains_key(&i) {
              let epoch_total_stakes = *ctx.accounts.stake_holder.total_stakes.get(&i).unwrap();
              prev_total_stake_amount = epoch_total_stakes;
              if epoch_total_stakes > 0 {
                pending_rewards += user_stake_amount * epoch_reward / (epoch_total_stakes as u64);
              }
            } else {
              let epoch_total_stakes = prev_total_stake_amount;
              if epoch_total_stakes > 0 {
                pending_rewards += user_stake_amount * epoch_reward / (epoch_total_stakes as u64);
              }
            }
          }
          i += 1;
        }
        ctx.accounts.user_stake_info.pending_reward += pending_rewards;
      }
    }
    ctx.accounts.user_stake_info.stake_amount -= unstake_amount;
    ctx.accounts.user_stake_info.last_epoch = current_epoch;

    ctx.accounts.stake_holder.curent_total_stake -= unstake_amount;
    emit!(UnstakeEvent {
      staker: ctx.accounts.user.key(),
      amount: unstake_amount,
    });
    Ok(())
  }
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UnstakeParams {
  pub amount: u64,
}
