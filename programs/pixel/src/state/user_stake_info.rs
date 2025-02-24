use crate::*;
pub const USER_STAKE_INFO_SEED: &[u8] = b"user_stake_info";

#[account]
#[derive(InitSpace)]
pub struct UserStakeInfo {
  pub initialized: bool,
  pub stake_amount: u64,
  pub pending_reward: u64,
  pub last_epoch: u16,
  pub bump: u8,
}
