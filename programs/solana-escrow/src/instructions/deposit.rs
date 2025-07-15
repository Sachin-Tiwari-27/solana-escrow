use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    TokenAccount, TokenInterface, transfer_checked, TransferChecked
};

use crate::{state::Escrow, error::EscrowError};

#[derive(Accounts)]
pub struct DepositEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub initializer_deposit_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds = [b"escrow", initializer.key().as_ref()], bump = escrow_account.bump)]
    pub escrow_account: Account<'info, Escrow>,

    #[account(mut, seeds = [b"vault", escrow_account.key().as_ref()], bump = escrow_account.vault_bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<DepositEscrow>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;

    // ❌ If already deposited, don't allow re-deposit
    require!(!escrow.deposited, EscrowError::AlreadyDeposited);

    // Use transfer_checked for interface compatibility
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.initializer_deposit_token_account.to_account_info(),
        mint: ctx.accounts.initializer_deposit_token_account.mint.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    
    // Get decimals from the mint
    let decimals = ctx.accounts.initializer_deposit_token_account.mint.decimals;
    transfer_checked(cpi_ctx, escrow.amount, decimals)?;

    // Set deposited = true
    escrow.deposited = true;

    Ok(())
}