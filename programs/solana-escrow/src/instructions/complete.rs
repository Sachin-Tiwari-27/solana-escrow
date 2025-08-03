use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    TokenAccount, TokenInterface, transfer_checked, TransferChecked, Mint
};

use crate::{state::Escrow, error::EscrowError};

#[derive(Accounts)]
pub struct CompleteEscrow<'info> {
    /// Taker (seller) claims tokens
    #[account(mut)]
    pub taker: Signer<'info>,

    /// The receiving token account of the taker
    #[account(mut)]
    pub taker_receive_token_account: InterfaceAccount<'info, TokenAccount>,

    /// The escrow PDA account storing state
    #[account(
        mut,
        seeds = [b"escrow", escrow_account.initializer.as_ref()],
        bump = escrow_account.bump,
        has_one = initializer_receive_token_account
    )]
    pub escrow_account: Account<'info, Escrow>,

    /// Vault owned by escrow PDA — holding tokens temporarily
    #[account(
        mut,
        seeds = [b"vault", escrow_account.key().as_ref()],
        bump = escrow_account.vault_bump
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// Token account that will receive tokens if escrow is cancelled
    /// (also validates ownership)
    #[account()]
    pub initializer_receive_token_account: InterfaceAccount<'info, TokenAccount>,

    /// The mint for the token being escrowed
    pub mint: InterfaceAccount<'info, Mint>,

    /// Token program
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn complete_handler(ctx: Context<CompleteEscrow>) -> Result<()> {
    let escrow_info = ctx.accounts.escrow_account.to_account_info();
    let escrow = &mut ctx.accounts.escrow_account;

    // ❌ If already completed
    require!(!escrow.completed, EscrowError::AlreadyCompleted);

    // ❌ If not deposited yet
    require!(escrow.deposited, EscrowError::NotFunded);

    // Transfer from vault (PDA) to taker (seller)
    let escrow_key = escrow.key();
    let seeds = &[
        b"vault",
        escrow_key.as_ref(),
        &[escrow.vault_bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.taker_receive_token_account.to_account_info(),
        authority: escrow_info,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    // Get decimals from the mint
    let decimals = ctx.accounts.mint.decimals;
    transfer_checked(cpi_ctx, escrow.amount, decimals)?;

    // Update escrow state
    escrow.taker = ctx.accounts.taker.key();
    escrow.taker_receive_token_account = ctx.accounts.taker_receive_token_account.key();
    escrow.completed = true;

    Ok(())
}