use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;

declare_id!("3gDpsb4yGizkRo7hpj2otYdrrGd5cLXtc3NC9iEF6riF");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        initialize_escrow::handler(ctx, amount)
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        deposit::handler(ctx)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        cancel_escrow::handler(ctx)
    }

    pub fn complete(ctx: Context<Complete>) -> Result<()> {
        complete::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
