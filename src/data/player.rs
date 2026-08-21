use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// A player row from the `players` table.
///
/// `username` corresponds to a chess.com username, and `active` marks
/// whether the player should still be considered for operations like
/// random game selection.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Player {
    /// Primary key in the `players` table.
    pub id: i64,
    /// The player's chess.com username.
    pub username: String,
    /// Whether the player is currently active.
    pub active: bool,
}
