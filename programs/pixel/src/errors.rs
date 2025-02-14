use anchor_lang::prelude::error_code;

#[error_code]
pub enum PixelError {
  #[msg("Account Not Initialized")]
  NotInitialized,
  #[msg("Account Already Initialized")]
  AlreadyInitialized,
  #[msg("Invalid Parameters")]
  InvalidParam,
  #[msg("Slippage Exceed")]
  SlippageExceed,
  #[msg("Sold All Tokens")]
  SoldAllToken,
  #[msg("Not Enough Tokens")]
  NotEnoughToken
}