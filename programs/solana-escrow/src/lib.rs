use anchor_lang::prelude::*;

declare_id!("3gDpsb4yGizkRo7hpj2otYdrrGd5cLXtc3NC9iEF6riF");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
