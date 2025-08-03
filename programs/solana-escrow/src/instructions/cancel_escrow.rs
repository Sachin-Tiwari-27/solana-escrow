use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    TokenAccount, TokenInterface, transfer_checked, TransferChecked, Mint, CloseAccount, close_account
};

use crate::{state::Escrow, error::EscrowError};

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    
    #[account(
        mut,
        close = initializer,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump = escrow_account.bump,
        has_one = initializer,
        has_one = initializer_receive_token_account
    )]
    pub escrow_account: Account<'info, Escrow>,
    
    #[account(
        mut,
        seeds = [b"vault", escrow_account.key().as_ref()],
        bump = escrow_account.vault_bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    /// The mint for the token being escrowed
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub initializer_receive_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn cancel_escrow_handler(ctx: Context<CancelEscrow>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_account;

    // ❌ If already completed, cannot cancel
    require!(!escrow.completed, EscrowError::AlreadyCompleted);

    // ❌ If not deposited yet, nothing to cancel
    require!(escrow.deposited, EscrowError::NotFunded);

    let escrow_key = ctx.accounts.escrow_account.key();
    let seeds = &[
        b"vault".as_ref(),
        escrow_key.as_ref(),
        &[escrow.vault_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    // Transfer tokens back to initializer
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.initializer_receive_token_account.to_account_info(),
        authority: ctx.accounts.escrow_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    // Get decimals from the mint and transfer the escrow amount
    let decimals = ctx.accounts.mint.decimals;
    transfer_checked(cpi_ctx, escrow.amount, decimals)?;
    
    // Close the vault account
    let close_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.initializer.to_account_info(),
        authority: ctx.accounts.escrow_account.to_account_info(),
    };

    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_accounts,
        signer_seeds,
    );
    close_account(close_ctx)?;
    
    Ok(())
}