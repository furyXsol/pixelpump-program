use crate::*;
pub const BONDING_CURVE_SEED: &[u8] = b"bonding_curve";

#[account]
#[derive(InitSpace)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub bump: u8,
}