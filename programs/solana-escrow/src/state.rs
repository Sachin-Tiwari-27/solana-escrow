use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub initializer: Pubkey,                        //Account that deposit the funds
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,

    pub taker: Pubkey,                             // Will be set on complete
    pub taker_receive_token_account: Pubkey,       // Seller's token account
    
    
    pub amount: u64,                              // Tokens locked in escrow
    pub deposited: bool,                          // Has the buyer deposited?
    pub completed: bool,                          // Has the seller claimed?
    pub bump: u8,                                 // PDA bump for escrow account
    pub vault_bump: u8,                           // PDA bump for vault token account
}

impl Escrow {
    pub const LEN: usize = 32 * 5 + 8 + 1 + 1 + 1 + 1;
}