use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use instructions::*;

declare_id!("3gDpsb4yGizkRo7hpj2otYdrrGd5cLXtc3NC9iEF6riF");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        initialize_escrow::handler(ctx, amount)
    }

    pub fn deposit(ctx: Context<DepositEscrow>) -> Result<()> {
        deposit::handler(ctx)
    }

    pub fn complete(ctx: Context<CompleteEscrow>) -> Result<()> {
        complete::handler(ctx)
    }

    pub fn cancel(ctx: Context<CancelEscrow>) -> Result<()> {
        cancel_escrow::handler(ctx)
    }
}


