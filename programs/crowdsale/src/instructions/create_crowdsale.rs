use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
};

use crate::{
    constants::AUTHORITY_SEED,
    state::{Crowdsale, CrowdsaleStatus},
};

pub fn create_crowdsale(ctx: Context<CreateCrowdsale>, id: Pubkey, cost: u32) -> Result<()> {
    let crowdsale = &mut ctx.accounts.crowdsale;
    crowdsale.id = id;
    crowdsale.cost = cost;
    crowdsale.mint_account = ctx.accounts.mint_account.key();
    crowdsale.token_account = ctx.accounts.token_account.key();
    crowdsale.status = CrowdsaleStatus::Open;
    crowdsale.owner = ctx.accounts.owner.key();

    msg!("Crowdsale created!");
    Ok(())
}
