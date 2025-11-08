use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    #[max_len(32)]
    pub username: String,
    pub total_won: u64,
    pub total_lost: u64,
    pub created_at: i64,
    pub bump: u8
}
