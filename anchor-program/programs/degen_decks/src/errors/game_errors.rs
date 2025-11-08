use anchor_lang::prelude::*;

#[error_code]
pub enum GameErrors{
    #[msg("Invalid entry stake")]
    InvalidEntryStake,
    #[msg("Mint is not yet supported")]
    InvalidMint,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Wait Time Range")]
    InvalidWaitTimeRange,
    #[msg("Players must be between 2 - 5")]
    InvalidNoPlayers,
    #[msg("Players are already complete")]
    PlayersAlreadyComplete
}