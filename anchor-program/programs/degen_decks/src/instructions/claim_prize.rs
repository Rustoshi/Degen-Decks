use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenInterface, TokenAccount }
};
use crate::{
    constants::{
        CONFIG_SEED, GAME_SEED, PROFILE_SEED
    }, 
    errors::GameErrors, 
    state::{
        Config, Game, Profile
    }, utils::spl_transfer
};

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
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
    game: Account<'info, Game>,
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
    #[account(
        mut,
        associated_token::mint = stake_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut
    )]
    pub fee_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            &CONFIG_SEED.as_bytes()
        ],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    pub fn claim_prize(&mut self) -> Result<()> {
        require!(self.game.ended == true, GameErrors::GameNotEnded);
        // check if player has claimed
        let player_index = self.game.players.iter().position(|p| p.owner == self.signer.key()).ok_or(GameErrors::PlayerNotFound)?;
        require!(self.game.players[player_index].claimed == false, GameErrors::AlreadyClaimed);
        require!(self.fee_ata.owner == self.config.fee_wallet, GameErrors::InvalidAuthority);


        let game_seed =  self.game.seed.to_le_bytes();
        let signer_seeds: &[&[&[u8]]]  = &[&[   
            &GAME_SEED.as_bytes(), 
            game_seed.as_ref(), 
            self.game.owner.as_ref(),
            &[self.game.bump]
        ]];
        // send out prize to winner(s)
        let winner = self.game.winner;
        if winner.is_some(){
            require!(self.signer.key() == winner.unwrap(), GameErrors::YouAreNotWinner);
            // calculate fee
            let fee_amount = self.game_vault.amount * self.config.platform_fee as u64 / 10_000;
            let amount = self.game_vault.amount - fee_amount;
            // transfer prize - fee
            spl_transfer(
                self.game_vault.to_account_info(), 
                self.user_ata.to_account_info(), 
                self.game.to_account_info(), 
                self.token_program.to_account_info(), 
                amount, 
                Some(signer_seeds)
            )?;
            // transfer fee
            spl_transfer(
                self.game_vault.to_account_info(), 
                self.fee_ata.to_account_info(), 
                self.game.to_account_info(), 
                self.token_program.to_account_info(), 
                fee_amount, 
                Some(signer_seeds)
            )?;
            // mark player as claimed
            self.game.players[player_index].claimed = true;
        }
        else if winner.is_none() {
            let amount = self.game.entry_stake;
            spl_transfer(
                self.game_vault.to_account_info(), 
                self.user_ata.to_account_info(), 
                self.game.to_account_info(), 
                self.token_program.to_account_info(), 
                amount, 
                Some(signer_seeds)
            )?;

            // mark player as claimed
            self.game.players[player_index].claimed = true;
        }
        Ok(())
    }
}