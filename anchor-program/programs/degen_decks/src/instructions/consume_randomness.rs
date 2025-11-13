use anchor_lang::prelude::*;
use ephemeral_vrf_sdk;

use crate::constants::{GAME_SEED};
use crate::state::{Game};
use crate::utils::shuffle_cards;

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
        //derive a u64 seed from the randomness
        let rnd_u64 = ephemeral_vrf_sdk::rnd::random_u64(&randomness);

        //get a shuffled full deck
        let shuffled_cards = shuffle_cards(rnd_u64);

        let num_players = self.game.no_players as usize;
        let cards_per_player = 5;
        let total_to_share = num_players * cards_per_player;

        //slice out the part of the deck for sharing
        let cards_to_share = &shuffled_cards[0..total_to_share];
        let call_card = shuffled_cards[total_to_share].clone();
        let draw_pile = shuffled_cards[total_to_share + 1..].to_vec();

        //distribute cards to each player
        for (i, player) in self.game.players.iter_mut().enumerate() {
            let start = i * cards_per_player;
            let end = start + cards_per_player;
            let hand_cards = cards_to_share[start..end].to_vec();

            player.hand = Some(hand_cards);
            player.player_index = Some((i + 1) as u8);
        }

        //assign call card and draw pile
        self.game.call_card = Some(call_card);
        self.game.draw_pile = Some(draw_pile);
        

        //update game state
        self.game.random_seed = Some(rnd_u64);
        self.game.started = true;
        self.game.started_at = Some(Clock::get()?.unix_timestamp);
        self.game.player_turn = 1;
        self.game.last_move_time = Some(Clock::get()?.unix_timestamp);

        Ok(())
    }

}
