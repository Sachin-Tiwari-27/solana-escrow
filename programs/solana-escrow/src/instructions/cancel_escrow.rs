use anchor_lang::prelude::*;
use crate::state::{Escrow, EscrowStatus};
use crate::error::*;

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account (mut, has_one = initializer, close = initializer )]
    pub escrow: Account<'info, Escrow>,

    pub initializer: Signer<'info>,
}

pub fn handler(ctx: Context<Cancel>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    require!(escrow.status == EscrowStatus::Pending, EscrowError::InvalidState);
    escrow.status = EscrowStatus::Cancelled;

    Ok(())
}