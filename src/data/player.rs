use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub active: bool,
}
