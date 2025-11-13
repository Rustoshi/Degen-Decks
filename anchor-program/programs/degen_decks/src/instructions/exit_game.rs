use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenInterface, TokenAccount }
};
use crate::constants::{GAME_SEED, PROFILE_SEED};
use crate::errors::GameErrors;
use crate::state::{Game, Profile};
use crate::utils::spl_transfer;


#[derive(Accounts)]
pub struct ExitGame<'info> {
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
    pub game: Account<'info, Game>,
    #[account(
        mut,
        associated_token::mint = stake_mint,
        associated_token::authority = game,
        associated_token::token_program = token_program
    )]
    pub game_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mint::token_program = token_program
    )]
    pub stake_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = stake_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ExitGame<'info> {
    pub fn exit_game(&mut self) -> Result<()> {
        // check if player is in the list
        require!(
            self.game.players.iter().any(|player| player.owner == self.signer.key()),
            GameErrors::PlayerNotFound
        );
        require!(
            self.game_vault.amount >= self.game.entry_stake,
            GameErrors::InsufficientFunds
        );

        if self.game.owner == self.signer.key() {
            self.game.ended = true;
            self.game.winner = None;            
        }

        if let Some(index) = self.game.players.iter().position(|p| p.owner == self.signer.key()) {
            self.game.players.remove(index);
        }
        Ok(())
    }

    pub fn withdraw_stake(&mut self) -> Result<()> {
        let game_seed =  self.game.seed.to_le_bytes();
        let signer_seeds: &[&[&[u8]]]  = &[&[   
            &GAME_SEED.as_bytes(), 
            game_seed.as_ref(), 
            self.game.owner.as_ref(),
            &[self.game.bump]
        ]];

        spl_transfer(
            self.game_vault.to_account_info(), 
            self.user_ata.to_account_info(), 
            self.game.to_account_info(), 
            self.token_program.to_account_info(), 
            self.game.entry_stake, 
            Some(signer_seeds)
        )
    }
}
