use anchor_lang::prelude::*;

use crate::state::Crowdsale;

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    // get the total balance
    let balance = ctx.accounts.crowdsale.get_lamports();

    // get the minimum rent
    let rent = RENT::get()?.minimum_balance(Crowdsale::MAXIMUM_SIZE + 8);

    // calculate the current balance - rent
    let amount = balance - rent;

    // update lamports
    ctx.accounts.crowdsale.sub_lamports(amount)?;
    ctx.accounts.owner.add_lamports(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, constraint = &owner.key() == &crowdsale.owner)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            crowdsale.id.as_ref(),
        ],
        bump,
    )]
    pub crowdsale: Account<'info, Crowdsale>,

    pub system_program: Program<'info, System>,
}
