use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init, 
        seeds = [b"escrow", initializer.key().as_ref()], 
        bump, 
        payer = initializer, 
        space = 8 + Escrow::LEN,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    /// CHECK: This is just a placeholder address reference
    pub vault: AccountInfo<'info>,

    /// CHECK: same as above
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.initializer = ctx.accounts.initializer.key();
    escrow.vault = ctx.accounts.vault.key();
    escrow.receiver = ctx.accounts.receiver.key();
    escrow.amount = amount;
    escrow.status = EscrowStatus::Pending;
    escrow.bump = ctx.bumps.escrow;

    Ok(())
}