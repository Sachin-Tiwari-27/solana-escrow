use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, CloseAccount, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    
    #[account(
        mut,
        close = initializer,
        has_one = initializer,
    )]
    pub escrow: Account<'info, EscrowAccount>,
    
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    // Add the mint account explicitly
    #[account(
        address = vault.mint
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CancelEscrow>) -> Result<()> {
    let escrow_key = ctx.accounts.escrow.key();
    let seeds = &[
        b"vault".as_ref(),
        escrow_key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer_seeds = &[&seeds[..]];
    
    // Transfer tokens back to initializer
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.initializer_deposit_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token::transfer(cpi_ctx, ctx.accounts.vault.amount)?;
    
    // Burn any remaining tokens in vault if needed
    if ctx.accounts.vault.amount > 0 {
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(), // Now using mint account
                from: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer_seeds,
        );
        anchor_spl::token::burn(burn_ctx, ctx.accounts.vault.amount)?;
    }
    
    // Close the vault account
    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.initializer.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token::close_account(close_ctx)?;
    
    // Access decimals from mint account
    let decimals = ctx.accounts.mint.decimals;
    msg!("Mint decimals: {}", decimals);
    
    Ok(())
}

// Assuming you have this struct defined somewhere
#[account]
pub struct EscrowAccount {
    pub initializer: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub taker_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}