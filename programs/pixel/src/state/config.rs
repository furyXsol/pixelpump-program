use crate::*;
pub const CONFIG_SEED: &[u8] = b"config";

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub initialized: bool,
    pub authority: Pubkey, // authority to modify config
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_base_points: u16, // 100 is 1%
    pub fee_stakeholders: u16, // 100 is 1%
    pub epoch_duration: u32, // ONE_WEEK
    pub bump: u8,
    pub stake_token: Pubkey,
}