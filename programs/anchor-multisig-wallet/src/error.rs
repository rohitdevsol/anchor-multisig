use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid threshold value")]
    InvalidThresholdValue,

    #[msg("Invalid Signer")]
    InvalidSigner,

    #[msg("Invalid Amount")]
    InvalidAmount,
}
