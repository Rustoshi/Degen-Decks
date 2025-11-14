use anchor_lang::prelude::*;
use crate::{
    constants::{
        GAME_SEED, 
        PROFILE_SEED
    }, 
    errors::GameErrors, 
    state::{
        Game, 
        Profile
    }
};

#[derive(Accounts)]
pub struct DrawFromPile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [
            &PROFILE_SEED.as_bytes(), 
            signer.key().as_ref()
            ],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [
            &GAME_SEED.as_bytes(), 
            game.seed.to_le_bytes().as_ref(), 
            game.owner.as_ref()
            ],
        bump = game.bump
    )]
    game: Account<'info, Game>
}

impl<'info> DrawFromPile<'info> {
    pub fn draw_from_pile(&mut self) -> Result<()> {
        let player = self.game.players.iter().find(|p| p.owner == self.signer.key()).ok_or(GameErrors::PlayerNotFound)?;
        
        require!(player.player_index == Some(self.game.player_turn), GameErrors::NotYourTurn);
        require!(self.game.started == true, GameErrors::GameNotStarted);
        require!(self.game.ended == false, GameErrors::GameEnded);

        self.game.last_move_time = Some(Clock::get()?.unix_timestamp);
        self.game.handle_draw_from_pile()?;

        Ok(())
    }
}