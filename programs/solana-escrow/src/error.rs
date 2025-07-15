use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError{
    #[msg("Tokens already deposited.")]
    AlreadyDeposited,

    #[msg("Tokens not yet deposited.")]
    NotFunded,

    #[msg("Escrow already completed or cancelled.")]
    AlreadyCompleted,
}