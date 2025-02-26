use std::collections::BTreeMap;

use crate::*;
pub const STAKE_HOLDER_SEED: &[u8] = b"stake_holder";
pub const MAX_EPOCH:u16 = 300;

#[account]
#[derive(Debug)]
pub struct StakeHolder {
  pub initialized: bool,
  pub rewards: BTreeMap<u16, u64>, //epoch
  pub total_stakes: BTreeMap<u16, u64>, //epoch
  pub curent_total_stake: u64,
  pub first_epoch_start_time: u32,
  pub bump: u8,
}
