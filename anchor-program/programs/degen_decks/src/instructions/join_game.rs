use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenInterface, TokenAccount }
};
use crate::{constants::{
    CONFIG_SEED, GAME_SEED, PROFILE_SEED
}, instruction::ConsumeRandomness};
use crate::errors::GameErrors;
use crate::state::{Game, Config, Player, Profile};
use crate::utils::spl_transfer;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;


#[vrf]
#[derive(Accounts)]
pub struct JoinGame<'info> {
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
    #[account(
        seeds = [
            &CONFIG_SEED.as_bytes()
        ],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinGame<'info> {
    pub fn join_game(&mut self) -> Result<()> {
        require!(
            self.game.players.len() < self.game.no_players as usize,
            GameErrors::PlayersAlreadyComplete
        );
        require!(
            self.user_ata.amount >= self.game.entry_stake,
            GameErrors::InsufficientFunds
        );
        require!(
            self.config.allowed_mints.contains(&self.stake_mint.key()), 
            GameErrors::InvalidMint
        );

        // user's player account
        let player_account = Player {
            owner: self.signer.key(),
            username: self.profile.username.to_owned(),
            hand: None,
            player_index: None,
            card_count: None
        };
        self.game.players.push(player_account);
        Ok(())
    }

    pub fn deposit_stake(&mut self) -> Result<()> {
        spl_transfer(
            self.user_ata.to_account_info(), 
            self.game_vault.to_account_info(), 
            self.signer.to_account_info(), 
            self.token_program.to_account_info(), 
            self.game.entry_stake, 
            None
        )
    }

    pub fn request_randomness(&mut self) -> Result<()> {
        // request randomness
        let ix = create_request_randomness_ix(
            RequestRandomnessParams {
                payer: self.signer.key(),
                oracle_queue: self.oracle_queue.key(),
                callback_program_id: crate::ID,
                callback_discriminator: ConsumeRandomness::DISCRIMINATOR.to_vec(),
                caller_seed: [5; 32],
                accounts_metas: Some(
                    vec![
                        SerializableAccountMeta {
                            pubkey: self.game.key(),
                            is_signer: false,
                            is_writable: true
                        }
                    ]
                ),
                ..Default::default()
            }
        );
        self.invoke_signed_vrf(&self.signer.to_account_info(), &ix)?;
        Ok(())
    }
}
