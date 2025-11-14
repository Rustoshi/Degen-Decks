use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{
    anchor::{commit}, 
    ephem::commit_and_undelegate_accounts
};
use crate::{
    constants::{
        GAME_SEED
    }, 
    state::{
        Game
    },
};

#[commit]
#[derive(Accounts)]
pub struct CommitGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            &GAME_SEED.as_bytes(), 
            game.seed.to_le_bytes().as_ref(), 
            game.owner.as_ref()
            ],
        bump = game.bump
    )]
    game: Account<'info, Game>,

}

impl<'info> CommitGame<'info> {
    pub fn commit_game(&mut self) -> Result<()> {

        if self.game.ended && self.game.delegated {
            self.game.delegated = false;
            // commit and undelegate
            self.game.exit(&crate::ID)?;
            commit_and_undelegate_accounts(
                &self.signer,
                vec![&self.game.to_account_info()],
                &self.magic_context,
                &self.magic_program,
            )?;
        }

        Ok(())
    }
}