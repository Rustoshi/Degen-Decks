use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{ephemeral};

pub mod state;
pub mod instructions;
pub mod constants;
pub mod errors;
pub mod events;
pub mod utils;

pub use instructions::*;

declare_id!("HjEFHo24qf3iiP2n8DEVJyKBkoBG5AUvxDPVzRiyPL5T");

#[ephemeral]
#[program]
pub mod degen_decks {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, platform_fee: u16, allow_mints: Vec<Pubkey>) -> Result<()> {
        ctx.accounts.initialize(platform_fee, allow_mints, ctx.bumps.config)
    }

    pub fn initialize_profile(ctx: Context<InitializeProfile>, username: String) -> Result<()> {
        ctx.accounts.initialize_profile(username, ctx.bumps.profile)
    }

    pub fn initialize_game(ctx: Context<InitializeGame>, 
        seed: u64,
        entry_stake: u64,
        no_players: u8,
        wait_time: i64
    ) -> Result<()> {
        ctx.accounts.initialize_game(seed, entry_stake, no_players, wait_time, ctx.bumps.game)?;
        ctx.accounts.deposit_stake(entry_stake)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        ctx.accounts.join_game()?;
        ctx.accounts.deposit_stake()
    }
}
