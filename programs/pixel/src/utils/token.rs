use anchor_lang::{
  prelude::*,
  solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token_2022;

pub fn transfer_sol<'info>(
  sender: AccountInfo<'info>,
  to: AccountInfo<'info>,
  transfer_lamports: u64,
) -> Result<()> {
  let transfer_ins = system_instruction::transfer(&sender.key, &to.key, transfer_lamports);
  invoke(
      &transfer_ins,
      &[sender.to_account_info(), to.to_account_info()],
  )?;
  Ok(())
}

pub fn transfer_sol_from_vault_to_user<'info>(
  sender: AccountInfo<'info>,
  to: AccountInfo<'info>,
  transfer_lamports: u64,
) -> Result<()> {
  **to.try_borrow_mut_lamports()? = to
      .try_lamports()?
      .checked_add(transfer_lamports)
      .ok_or(ProgramError::InsufficientFunds)?;

  let source_balance = sender.try_lamports()?;
  **sender.try_borrow_mut_lamports()? = source_balance
      .checked_sub(transfer_lamports)
      .ok_or(ProgramError::InsufficientFunds)?;
  Ok(())
}

pub fn transfer_token_from_user_to_vault<'info>(
  authority: AccountInfo<'info>,
  sender: AccountInfo<'info>,
  to: AccountInfo<'info>,
  mint: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  amount: u64,
  decimals: u8,
) -> Result <()> {
  token_2022::transfer_checked(CpiContext::new(
    token_program.to_account_info(),
    token_2022::TransferChecked {
      from: sender,
      mint: mint,
      to: to,
      authority: authority,
    }
  ), amount, decimals)
}

pub fn transfer_token_from_vault_to_user<'info>(
  authority: AccountInfo<'info>,
  sender: AccountInfo<'info>,
  to: AccountInfo<'info>,
  mint: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  amount: u64,
  decimals: u8,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  token_2022::transfer_checked(
      CpiContext::new(
          token_program.to_account_info(),
          token_2022::TransferChecked {
              from: sender,
              mint,
              to,
              authority,
          },
      )
      .with_signer(signer_seeds),
      amount,
      decimals,
  )
}
