use anchor_lang::prelude::*;

#[error_code]
pub enum ConfigErrors{
    #[msg("Invalid Fee Range")]
    InvalidFeeRange,
    #[msg("You Are Not Unauthorized")]
    NotAuthorized,
    #[msg("Invalid mints range")]
    InvalidMintsRange
}