use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Transfer, Mint, Token, TokenAccount},
    },
    solana_program::system_instructions,
};

use crate::{
    constants::AUTHORITY_SEED,
    state::Crowdsale,
};

pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u32) -> Result<()> {
    // calculate how much SOL needed in order to buy x amount of tokens
    // amount of tokens * cost of 1 token
    let amount_of_lamports = (amount * ctx.accounts.crowdsale.cost) as u64;

    // transfer SOL
    let from = &ctx.accounts.buyer;
    let to = &ctx.accounts.crowdsale;

    let transfer_instruction = system_instruction::transfer(
        &from.key(),
        &to.key(),
        amount_of_lamports
    );

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from.to_account_info(),
            to.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // transfer tokens
    let authority_bump = ctx.bumps.crowdsale_authority;
    let authority_seeds = &[
        &ctx.accounts.crowdsale.id.to_bytes(),
        AUTHORITY_SEED,
        &[authority_bump],
    ];
    let signer_seeds = &[&authority_seeds[..]];

    transfer(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.crowdsale_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.crowdsale_authority.to_account_info(),
        },
        signer_seeds,
    ),
    amount as u64, // transfer amount
    )?;

    msg!("Token transferred!");
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>, //who is buying the tokens

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint_account,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, //buyer's token account

    #[account(
        mut,
        seed = [
            crowdsale.id.as_ref(),
        ],
        bump,
    )]
    pub crowdsale: Account<'info, Crowdsale>, // crowdsale state

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = crowdsale_authority,
    )]
    pub crowdsale_token_account: Account<'info, TokenAccounts>, // crowdsale's token account

    // Check: Read only authority
    // this will allow our crowdsale to transfer tokens
    #[account(
        seeds = [
            crowdsale.id.as_ref(),
            AUTHORITY_SEED
        ],
        bump,
    )]
    pub crowdsale_authority: AccountInfo<'info>, // crowdsale authority

    pub mint_account: Account<'info, Mint>, // token mint account

    pub token_program: Program<'info, Token>, // token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, // system program
}
