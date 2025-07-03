use anchor_lang::prelude::*;
use crate::state::{Escrow, EscrowStatus};
use crate::error::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = initializer)]
    pub escrow: Account<'info, Escrow>,

    pub initializer: Signer<'info>,
}

pub fn handler(ctx: Context<Deposit>) -> Result<()> {
    //Deposit instructions will come here

    Ok(())
}