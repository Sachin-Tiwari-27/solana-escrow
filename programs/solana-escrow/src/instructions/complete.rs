use anchor_lang::prelude::*;
use crate::state::{Escrow, EscrowStatus};
use crate::error::*;

#[derive(Accounts)]
pub struct Complete<'info> {
    #[account(mut, has_one = receiver)]
    pub escrow: Account<'info, Escrow>,

    pub receiver: Signer<'info>,
}

pub fn handler(ctx: Context<Complete>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    require!(escrow.status == EscrowStatus::Pending, EscrowError::InvalidState);
    escrow.status = EscrowStatus::Completed;

    Ok(())
}