use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{
    anchor::{delegate}, 
    cpi::DelegateConfig
};
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

#[delegate]
#[derive(Accounts)]
pub struct DrawFromPileDelegate<'info> {
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
        del,
        seeds = [
            &GAME_SEED.as_bytes(), 
            game.seed.to_le_bytes().as_ref(), 
            game.owner.as_ref()
            ],
        bump = game.bump
    )]
    game: Account<'info, Game>
}

impl<'info> DrawFromPileDelegate<'info> {
    pub fn draw_from_pile_delegate(&mut self) -> Result<()> {
        let player = self.game.players.iter().find(|p| p.owner == self.signer.key()).ok_or(GameErrors::PlayerNotFound)?;
        
        require!(player.player_index == Some(self.game.player_turn), GameErrors::NotYourTurn);
        require!(self.game.started == true, GameErrors::GameNotStarted);
        require!(self.game.ended == false, GameErrors::GameEnded);

        self.game.last_move_time = Some(Clock::get()?.unix_timestamp);
        self.game.handle_draw_from_pile()?;


        if !self.game.delegated && !self.game.ended {
            self.game.delegated = true;
            // delegate to ER
            self.game.exit(&crate::ID)?;
            self.delegate_game(
                &self.signer,
                &[
                    &GAME_SEED.as_bytes(), 
                    self.game.seed.to_le_bytes().as_ref(), 
                    self.game.owner.as_ref()
                ],
                DelegateConfig {
                    ..Default::default()
                }
            )?;
        }

        Ok(())
    }
}