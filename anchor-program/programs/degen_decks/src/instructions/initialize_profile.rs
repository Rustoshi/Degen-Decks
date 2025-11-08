use crate::constants::PROFILE_SEED;
use crate::errors::ProfileErrors;
use crate::state::Profile;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Profile::DISCRIMINATOR.len() + Profile::INIT_SPACE,
        seeds = [&PROFILE_SEED.as_bytes(), signer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProfile<'info> {
    pub fn initialize_profile(&mut self, username: String, bump: u8) -> Result<()> {
        require!(
            username.len() > 0 && username.len() <= 32,
            ProfileErrors::InvalidUsername
        );

        let clock = Clock::get()?;
        self.profile.set_inner(Profile {
            username,
            total_won: 0,
            total_lost: 0,
            created_at: clock.unix_timestamp,
            bump
        });
        Ok(())
    }
}
