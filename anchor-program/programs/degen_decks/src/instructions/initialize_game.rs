use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenInterface, TokenAccount }
};
use crate::constants::{
    CONFIG_SEED, GAME_SEED, MAX_NO_PLAYERS, MAX_WAIT_TIME, MIN_NO_PLAYERS, MIN_WAIT_TIME, PROFILE_SEED
};
use crate::errors::GameErrors;
use crate::state::{Game, Config, Player, Profile};
use crate::utils::spl_transfer;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeGame<'info> {
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
        init,
        payer = signer,
        space = Game::DISCRIMINATOR.len() + Game::INIT_SPACE,
        seeds = [
            &GAME_SEED.as_bytes(), 
            seed.to_le_bytes().as_ref(), 
            signer.key().as_ref()
            ],
        bump
    )]
    pub game: Account<'info, Game>,
        #[account(
        init,
        payer = signer,
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
    #[account(
        seeds = [
            &CONFIG_SEED.as_bytes()
        ],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn initialize_game(&mut self, 
    seed: u64,
    entry_stake: u64,
    no_players: u8,
    wait_time: i64,
    bump: u8
    ) -> Result<()> {
        let clock = Clock::get()?;
        // check if user has enough balance for stake
        require!(self.user_ata.amount >= entry_stake, 
            GameErrors::InsufficientFunds);
        // make sure wait time is not below 30s and above 2 minutes
        require!(wait_time >= MIN_WAIT_TIME && wait_time <= MAX_WAIT_TIME, 
            GameErrors::InvalidWaitTimeRange);
        // make sure no players are in range
        require!(no_players >= MIN_NO_PLAYERS && no_players <= MAX_NO_PLAYERS,
            GameErrors::InvalidNoPlayers
        );
        // make sure mint is in allowed list
        require!(self.config.allowed_mints.contains(&self.stake_mint.key()), 
        GameErrors::InvalidMint);

        // user's player account
        let player_account = Player {
            owner: self.signer.key(),
            username: self.profile.username.to_owned(),
            hand: None,
            player_index: None
        };

        // update game state
        self.game.set_inner(
            Game {
                owner: self.signer.key(),
                entry_stake: entry_stake,
                game_vault: self.game_vault.key(),
                stake_mint: self.stake_mint.key(),
                no_players: no_players,
                player_turn: 0,
                players: vec![player_account],
                winner: None,
                call_card: None,
                draw_pile: None,
                wait_time: wait_time, 
                seed: seed,
                random_seed: None,
                delegated: false,
                started: false,
                ended: false,
                created_at: clock.unix_timestamp,
                started_at: None,
                ended_at: None,
                bump
            }
        );
        Ok(())
    }

    pub fn deposit_stake(&mut self, amount: u64) -> Result<()> {
        spl_transfer(
            self.user_ata.to_account_info(), 
            self.game_vault.to_account_info(), 
            self.signer.to_account_info(), 
            self.token_program.to_account_info(), 
            amount, 
            None
        )
    }
}
