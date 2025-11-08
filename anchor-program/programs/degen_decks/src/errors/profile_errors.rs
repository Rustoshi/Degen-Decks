use anchor_lang::prelude::*;

#[error_code]
pub enum ProfileErrors{
    #[msg("Username should be between 3 and 32 characters")]
    InvalidUsername
}