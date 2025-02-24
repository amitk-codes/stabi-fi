use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Price")]
    InvalidPrice,

    #[msg("Account is below minimum health factor")]
    BelowHealthFactor,

    #[msg("Account is above minimum health factor, so account can't be liquidated")]
    AboveHealthFactor
}
