use std::ops::DerefMut;
use crate::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  metadata::{
      create_metadata_accounts_v3,
      mpl_token_metadata::{accounts::Metadata as MetadataAccount, types::DataV2},
      CreateMetadataAccountsV3, Metadata,
  },
  token::Token,
  token_interface::{Mint, MintTo, TokenAccount, mint_to },
};

#[derive(Accounts)]
pub struct CreateToken<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    init,
    payer = payer,
    mint::decimals = 6,
    mint::authority = bonding_curve,
    mint::token_program = token_program,
  )]
  pub token_mint: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK
  #[account(
    init,
    seeds = [
      BONDING_CURVE_SEED,
      token_mint.key().as_ref()
    ],
    payer = payer,
    space = 8 + BondingCurve::INIT_SPACE,
    bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    init,
    associated_token::mint = token_mint,
    associated_token::authority = bonding_curve,
    payer = payer,
    token::token_program = token_program,
  )]
  pub associted_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    seeds = [
      CONFIG_SEED,
    ],
    bump = config.bump
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
    mut,
    address = MetadataAccount::find_pda(&token_mint.key()).0
  )]
  pub metadata: UncheckedAccount<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub token_metadata_program: Program<'info, Metadata>,
  pub rent: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>
}

impl CreateToken<'_> {
  pub fn apply(
    ctx: &mut Context<CreateToken>,
    params: &CreateTokenParams,
  ) -> Result<()> {

    require!(ctx.accounts.config.initialized, PixelError::NotInitialized);

    // create metadata account
    let seeds = &[BONDING_CURVE_SEED, &ctx.accounts.token_mint.key().to_bytes(), &[ctx.bumps.bonding_curve]];
    let signer_seeds = [&seeds[..]];
    let cpi_context = CpiContext::new_with_signer(
      ctx.accounts.token_metadata_program.to_account_info(),
      CreateMetadataAccountsV3 {
          metadata: ctx.accounts.metadata.to_account_info(),
          mint: ctx.accounts.token_mint.to_account_info(),
          mint_authority: ctx.accounts.bonding_curve.to_account_info(),
          update_authority: ctx.accounts.bonding_curve.to_account_info(),
          payer: ctx.accounts.payer.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          rent: ctx.accounts.rent.to_account_info(),
      },
      &signer_seeds,
    );
    let data_v2 = DataV2 {
      name: String::from_utf8(params.name.clone()).unwrap(),
      symbol: String::from_utf8(params.symbol.clone()).unwrap(),
      uri: String::from_utf8(params.uri.clone()).unwrap(),
      seller_fee_basis_points: 0,
      creators: None,
      collection: None,
      uses: None,
    };
    create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

    let cpi_accounts = MintTo {
      mint: ctx.accounts.token_mint.to_account_info(),
      to: ctx.accounts.associted_bonding_curve.to_account_info(),
      authority: ctx.accounts.bonding_curve.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(
      cpi_context.with_signer(&signer_seeds),
      ctx.accounts.config.token_total_supply
    )?;

    let bonding_curve = ctx.accounts.bonding_curve.deref_mut();
    bonding_curve.bump = ctx.bumps.bonding_curve;
    bonding_curve.complete = false;
    bonding_curve.token_total_supply = ctx.accounts.config.token_total_supply;
    bonding_curve.virtual_sol_reserves = ctx.accounts.config.initial_virtual_sol_reserves;
    bonding_curve.virtual_token_reserves = ctx.accounts.config.initial_virtual_token_reserves;
    bonding_curve.real_sol_reserves = 0;
    bonding_curve.real_token_reserves = ctx.accounts.config.initial_real_token_reserves;

    emit!(CreateTokenEvent {
      creator: ctx.accounts.payer.key(),
      token_name: String::from_utf8(params.name.to_vec()).unwrap(),
      token_symbol: String::from_utf8(params.symbol.to_vec()).unwrap(),
      token_uri: String::from_utf8(params.uri.to_vec()).unwrap(),
      mint: ctx.accounts.token_mint.key()
    });

    Ok(())
  }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateTokenParams {
  pub name: Vec<u8>,
  pub symbol: Vec<u8>,
  pub uri: Vec<u8>,
}
