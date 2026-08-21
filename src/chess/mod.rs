//! Chess domain logic: fetching games from the chess.com public API and
//! the data types that model them.

/// Fetching games/players from the chess.com API and seeding the database.
pub mod api;
/// Data types modeling a chess.com game and its metadata.
pub mod game;

pub use api::_stats;
