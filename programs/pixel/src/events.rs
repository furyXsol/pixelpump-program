use crate::*;

#[event]
pub struct CreateTokenEvent {
  pub creator: Pubkey,
  pub token_name: String,
  pub token_symbol: String,
  pub token_uri: String,
  pub mint: Pubkey,
}

#[event]
pub struct BuyEvent {
  pub mint: Pubkey,
  pub buyer: Pubkey,
  pub sol_input: u64,
  pub token_output: u64,
}

#[event]
pub struct SellEvent {
  pub mint: Pubkey,
  pub seller: Pubkey,
  pub sol_output: u64,
  pub token_input: u64,
}

#[event]
pub struct WithdrawEvent {
  pub mint: Pubkey,
  pub withdrawer: Pubkey,
  pub sol_output: u64,
  pub token_output: u64,
}