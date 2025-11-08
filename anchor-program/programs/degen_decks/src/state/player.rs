use anchor_lang::prelude::*;
use crate::state::Card;

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Player {
    pub owner: Pubkey,
    #[max_len(32)]
    pub username: String,
    #[max_len(54)]
    pub hand: Option<Vec<Card>>,
    pub player_index: Option<u8>,
}