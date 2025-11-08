use anchor_lang::prelude::*;
use crate::state::Player;
use crate::state::Card;

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub owner: Pubkey,
    pub entry_stake: u64,
    pub game_vault: Pubkey,
    pub stake_mint: Pubkey,
    pub no_players: u8,
    pub player_turn: u8,
    #[max_len(5)]
    pub players: Vec<Player>,
    pub winner: Option<Pubkey>,
    pub call_card: Option<Card>,
    #[max_len(54)]
    pub draw_pile: Option<Vec<Card>>,
    pub wait_time: i64,
    pub seed: u64,
    pub random_seed: Option<u64>,
    pub delegated: bool,
    pub started: bool,
    pub ended: bool,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub bump: u8,
}