use anchor_lang::prelude::*;
mod instructions;
pub mod state;
mod events;
mod errors;
pub mod utils;

use instructions::*;
use state::*;
use events::*;
use errors::*;
use utils::*;

declare_id!("2A4pYsctJgE2KNSspqipdZbv2sF3Gmyk8Ad2iZs2PtyJ");

#[program]
pub mod pixel {
    use super::*;

    pub fn create_config(mut ctx: Context<CreateConfig>, params: CreateConfigParams) -> Result<()> {
        CreateConfig::apply(&mut ctx, &params)
    }

    pub fn create_stake_holder(mut ctx: Context<CreateStakeHolder>) -> Result<()> {
        CreateStakeHolder::apply(&mut ctx)
    }


    pub fn update_config(mut ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        UpdateConfig::apply(&mut ctx, &params)
    }

    // create meme token
    pub fn create_token(mut ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        CreateToken::apply(&mut ctx, &params)
    }

    // buy meme token
    pub fn buy(mut ctx: Context<Buy>, params: BuyParams) -> Result<()> {
        Buy::apply(&mut ctx, &params)
    }
    // sell meme token
    pub fn sell(mut ctx: Context<Sell>, params: SellParams) -> Result<()> {
        Sell::apply(&mut ctx, &params)
    }

    //withdraw sol and spl-token from bonding_curve
    pub fn withdraw(mut ctx: Context<Withdraw>) -> Result<()>{
        Withdraw::apply(&mut ctx)
    }

    // stake STAKING_TOKEN by staker
    pub fn stake(mut ctx: Context<Stake>, params: StakeParams) -> Result<()> {
        Stake::apply(&mut ctx, &params)
    }

    // unstake STAKING_TOKEN by staker
    pub fn unstake(mut ctx: Context<Unstake>, params: UnstakeParams) -> Result<()> {
        Unstake::apply(&mut ctx, &params)
    }

    // claim rewards(SOL) by staker
    pub fn claim(mut ctx: Context<Claim>) -> Result<()> {
        Claim::apply(&mut ctx)
    }

    // withdraw SOL from stake_holder PDA
    pub fn withdraw_stake_holder(mut ctx: Context<WithdrawStakeHolder>) -> Result<()> {
        WithdrawStakeHolder::apply(&mut ctx)
    }
}
