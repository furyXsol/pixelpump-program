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
    pub fee_base_points: u64, // 100 is 1%
    pub bump: u8,
}