use std::error::Error;

use rand::random_range;

use serde::Deserialize;

use crate::{
    chess::game::{Archives, Game, ManyGames},
    data::db::insert,
};

#[derive(Debug, Deserialize)]
struct PlayerList {
    players: Vec<String>,
}

pub async fn _stats(username: &str, client: reqwest::Client) {
    let url = format!("https://api.chess.com/pub/player/{username}/stats");
    if let Ok(response) = client.get(url).send().await {
        println!("{:#?}", response.text().await.unwrap());
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameFetchError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("no archives found for user")]
    NoArchives,
    #[error("archive contained no games")]
    NoGames,
}

pub async fn random_game(username: &str, client: reqwest::Client) -> Result<Game, GameFetchError> {
    let url = format!("https://api.chess.com/pub/player/{username}/games/archives");
    let archives = client
        .get(url)
        .send()
        .await?
        .json::<Archives>()
        .await?
        .archives;

    if archives.is_empty() {
        return Err(GameFetchError::NoArchives);
    }
    let arch_num = random_range(0..archives.len());
    let games = client
        .get(&archives[arch_num])
        .send()
        .await?
        .json::<ManyGames>()
        .await?
        .games;
    if games.is_empty() {
        return Err(GameFetchError::NoGames);
    }
    let game_num = rand::random_range(0..games.len());
    Ok(games.into_iter().nth(game_num).unwrap())
}

pub async fn seed_db(
    pool: &sqlx::SqlitePool,
    client: reqwest::Client,
) -> Result<(), Box<dyn Error>> {
    let rows: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM players")
        .fetch_one(pool)
        .await?;
    if rows > 0 {
        tracing::info!("Database already seeded");
        return Ok(());
    }

    let endpoints = vec![
        "country/US/players",
        "country/CA/players",
        "country/IT/players",
        "titled/GM",
        "titled/IM",
        "titled/FM",
    ];
    for e in endpoints {
        match get_players(e, client.clone()).await {
            Ok(usernames) => {
                for username in usernames {
                    if let Err(err) = insert(&username, pool).await {
                        tracing::warn!("failed to insert {username}: {err}");
                    }
                }
            }
            Err(err) => tracing::warn!("failed to fetch players for {e}: {err}"),
        }
    }
    Ok(())
}

async fn get_players(
    endpoint: &str,
    client: reqwest::Client,
) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("https://api.chess.com/pub/{endpoint}");
    let players = client
        .get(url)
        .send()
        .await?
        .json::<PlayerList>()
        .await?
        .players;
    Ok(players)
}
