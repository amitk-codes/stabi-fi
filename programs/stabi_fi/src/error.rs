use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Price")]
    InvalidPrice,

    #[msg("Account is below minimum health factor")]
    BelowHealthFactor,
}
