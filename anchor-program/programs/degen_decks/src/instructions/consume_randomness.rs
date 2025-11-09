use anchor_lang::prelude::*;
use ephemeral_vrf_sdk;

use crate::constants::{GAME_SEED};
use crate::state::Game;

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(
        mut,
        seeds = [
            &GAME_SEED.as_bytes(), 
            game.seed.to_le_bytes().as_ref(), 
            game.owner.as_ref()
            ],
        bump = game.bump
    )]
    pub game: Account<'info, Game>
}

impl<'info> ConsumeRandomness<'info> {
    pub fn consume_randomness(&mut self, randomness: [u8; 32]) -> Result<()> {
        let rnd_u64 = ephemeral_vrf_sdk::rnd::random_u64(&randomness);
        self.game.random_seed = Some(rnd_u64);
        Ok(())
    }
}
