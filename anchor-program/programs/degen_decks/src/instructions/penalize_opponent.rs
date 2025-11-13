use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{
    anchor::{commit, delegate}, 
    cpi::DelegateConfig, 
    ephem::commit_and_undelegate_accounts
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

#[commit]
#[delegate]
#[derive(Accounts)]
pub struct PenalizeOpponent<'info> {
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

impl<'info> PenalizeOpponent<'info> {
    pub fn penalize_opponent(&mut self) -> Result<()> {
        let player = self.game.players.iter().find(|p| p.owner == self.signer.key()).ok_or(GameErrors::PlayerNotFound)?;
        
        require!(player.player_index != Some(self.game.player_turn), GameErrors::CannotPenalizeYourself);
        require!(self.game.started == true, GameErrors::GameNotStarted);
        require!(self.game.ended == false, GameErrors::GameEnded);

        self.game.last_move_time = Some(Clock::get()?.unix_timestamp);
        self.game.handle_penalize_opponent()?;
        self.game.check_winner()?;

        if self.game.ended && self.game.delegated {
            self.game.delegated = false;
            self.game.exit(&crate::ID)?;

            // commit and undelegate
            commit_and_undelegate_accounts(
                &self.signer,
                vec![&self.game.to_account_info()],
                &self.magic_context,
                &self.magic_program,
            )?;
        }

        else if !self.game.delegated && !self.game.ended {
            self.game.delegated = true;
            self.game.exit(&crate::ID)?;

            self.delegate_game(
                &self.signer,
                &[
                    &GAME_SEED.as_bytes(), 
                    self.game.seed.to_le_bytes().as_ref(), 
                    self.game.owner.as_ref(),
                    &[self.game.bump]
                ],
                DelegateConfig {
                    ..Default::default()
                }
            )?;
        }

        Ok(())
    }
}