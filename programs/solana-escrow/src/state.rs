use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Pending,
    Completed,
    Cancelled,
}

#[account]
pub struct Escrow {
    pub initializer: Pubkey, //Account that deposit the funds
    pub vault: Pubkey, //Escorw
    pub receiver: Pubkey, //Account that receive the funds
    pub amount: u64, //Funds
    pub status: EscrowStatus, //Status of the Escrow
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 1 + 1;
}