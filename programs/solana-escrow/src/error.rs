use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError{
    #[msg("Unauthorized operation.")]
    Unauthorized,

    #[msg("Token transfer failed.")]
    TransferFailed,

    #[msg("Escrow already completed or cancelled.")]
    InvalidState,
}