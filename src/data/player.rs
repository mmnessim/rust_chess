use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: i64,
    pub username: String,
    pub active: bool,
}
