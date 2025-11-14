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
    PlayersAlreadyComplete,
    #[msg("Not your turn")]
    NotYourTurn,
    #[msg("Cannot play this card")]
    CannotPlayCard,
    #[msg("No call card to validate")]
    NoCallCard,
    #[msg("No draw pile")]
    NoDrawPile,
    #[msg("Player not found")]
    PlayerNotFound,
    #[msg("Game not started")]
    GameNotStarted,
    #[msg("Game ended")]
    GameEnded,
    #[msg("Game not ended")]
    GameNotEnded,
    #[msg("You are not the winner")]
    YouAreNotWinner,
    #[msg("Cannot penalize yourself")]
    CannotPenalizeYourself,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Invalid authority")]
    InvalidAuthority
}