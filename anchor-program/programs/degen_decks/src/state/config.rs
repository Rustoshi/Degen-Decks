use anchor_lang::prelude::*;

use crate::constants::MAX_MINTS_LENGTH;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub platform_fee: u16,
    #[max_len(MAX_MINTS_LENGTH)]
    pub allowed_mints: Vec<Pubkey>,
    pub bump: u8
}