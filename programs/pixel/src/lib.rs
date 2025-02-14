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

declare_id!("2K3pM9K2D3JYGWKVRFBGSDWLez299NFQXSWVDJ6YpsdD");

#[program]
pub mod pixel {
    use super::*;

    pub fn create_config(mut ctx: Context<CreateConfig>, params: CreateConfigParams) -> Result<()> {
        CreateConfig::apply(&mut ctx, &params)
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

    //withdraw sol from bonding_curve
    pub fn withdraw(mut ctx: Context<Withdraw>) -> Result<()>{
        Withdraw::apply(&mut ctx)
    }

}

