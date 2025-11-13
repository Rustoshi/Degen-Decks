// Seeds
pub const CONFIG_SEED: &str = "CONFIG";
pub const PROFILE_SEED: &str = "PROFILE";
pub const GAME_SEED: &str = "GAME";

// Config Constants
pub const MAX_MINTS_LENGTH: u8 = 10;
// Game Constants
pub const MIN_WAIT_TIME: i64 = 30; // 30 seconds
pub const MAX_WAIT_TIME: i64 = 120; // 2 minutes
pub const MIN_NO_PLAYERS: u8 = 2;
pub const MAX_NO_PLAYERS: u8 = 5;

// CARDS
pub const CIRCLE: [u8; 12] = [1, 2, 3, 4, 5, 7, 8, 10, 11, 12, 13, 14];
pub const TRIANGLE: [u8; 12] = [1, 2, 3, 4, 5, 7, 8, 10, 11, 12, 13, 14];
pub const CROSS: [u8; 9] = [1, 2, 3, 5, 7, 10, 11, 13, 14];
pub const SQUARE: [u8; 9] = [1, 2, 3, 5, 7, 10, 11, 13, 14];
pub const STAR: [u8; 7] = [1, 2, 3, 4, 5, 7, 8];
pub const WHOT: [u8; 5] = [20, 20, 20, 20, 20];
