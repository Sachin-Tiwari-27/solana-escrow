use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, InitializeAccount, TokenAccount as SPLTokenAccount};

use crate::{state::Escrow, error::EscrowError};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    /// Buyer (initializer) signs and pays for account creation
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// The token account from which buyer will deposit tokens
    #[account()]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,

    /// The token account where buyer expects tokens back if cancelled
    #[account()]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,

    /// The escrow account PDA, created and initialized
    #[account(
        init,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + Escrow::LEN
    )]
    pub escrow_account: Account<'info, Escrow>,

    /// The mint for the token being escrowed
    pub mint: Account<'info, Mint>,

    /// The vault token account (also PDA), will be created here
    #[account(
        init,
        seeds = [b"vault", escrow_account.key().as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = escrow_account
    )]
    pub vault: Account<'info, TokenAccount>,

    /// Rent, Token Program, System Program required by Anchor
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;

    escrow.initializer = ctx.accounts.initializer.key();
    escrow.initializer_deposit_token_account = ctx.accounts.initializer_deposit_token_account.key();
    escrow.initializer_receive_token_account = ctx.accounts.initializer_receive_token_account.key();
    escrow.taker = Pubkey::default(); // not known yet
    escrow.taker_receive_token_account = Pubkey::default(); // not known yet

    escrow.amount = amount;
    escrow.deposited = false;
    escrow.completed = false;

    escrow.bump = ctx.bumps.escrow_account;
    escrow.vault_bump = ctx.bumps.vault;

    Ok(())
}