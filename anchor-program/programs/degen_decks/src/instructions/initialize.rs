use crate::constants::{CONFIG_SEED, MAX_MINTS_LENGTH};
use crate::errors::ConfigErrors;
use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [&CONFIG_SEED.as_bytes()],
        bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    // Making sure only the program update authority can add creators to the array
    #[account(constraint = program_data.upgrade_authority_address == Some(admin.key()) @ConfigErrors::NotAuthorized)]
    pub program_data: Account<'info, ProgramData>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, platform_fee: u16, allowed_mints: Vec<Pubkey>, bump: u8) -> Result<()> {
        require!(
            platform_fee > 0 && platform_fee <= 10_000,
            ConfigErrors::InvalidFeeRange
        );
        require!(
            allowed_mints.len() <= MAX_MINTS_LENGTH as usize
            && allowed_mints.len() > 0,
            ConfigErrors::InvalidMintsRange
        );
        self.config.set_inner(
            Config {
                allowed_mints,
                platform_fee,
                bump
            }
        );
        Ok(())
    }
}
