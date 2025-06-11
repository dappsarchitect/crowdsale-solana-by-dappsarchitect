use anchor_lang::prelude::*;

mod state;
mod constants;

declare_id!("HciPz9qoNEBBWga6KWomnDovANbQWnTAT5iFSNW7Ji3K");

#[program]
pub mod crowdsale {
    use super::*;

    // Acts like a constructor
    pub fn initialize(ctx: Context<CreateCrowdsale>, id: Pubkey, cost: u32) -> Result<()> {
            //msg!("Greetings from: {:?}", ctx.program_id);
            //Ok(())
    }

    // For user to buy tokens

    // For user to withdraw SOL 
}

#[derive(Accounts)]
pub struct Initialize {}
