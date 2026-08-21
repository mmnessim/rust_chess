use std::error::Error;

use rand::random_range;

use serde::Deserialize;

use crate::{
    chess::game::{Archives, Game, ManyGames},
    data::db::insert,
};

/// Response body of a chess.com "list of players" endpoint (e.g. by
/// country or title), containing just usernames.
#[derive(Debug, Deserialize)]
struct PlayerList {
    players: Vec<String>,
}

/// Fetches and prints a player's stats from the chess.com API.
///
/// This is a debugging helper: it swallows request errors and simply
/// prints the raw response body.
pub async fn _stats(username: &str, client: reqwest::Client) {
    let url = format!("https://api.chess.com/pub/player/{username}/stats");
    if let Ok(response) = client.get(url).send().await {
        println!("{:#?}", response.text().await.unwrap());
    }
}

/// Errors that can occur while fetching a random game for a player.
#[derive(Debug, thiserror::Error)]
pub enum GameFetchError {
    /// The underlying HTTP request failed.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    /// The response body could not be deserialized.
    #[error("failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
    /// The player has no monthly archives at all.
    #[error("no archives found for user")]
    NoArchives,
    /// A randomly chosen archive contained no games.
    #[error("archive contained no games")]
    NoGames,
}

/// Picks a random monthly archive for `username`, then a random game
/// from within it, and returns that game.
///
/// # Errors
///
/// Returns [`GameFetchError`] if the archive list or game list can't be
/// fetched/parsed, if the player has no archives, or if the chosen
/// archive turns out to be empty.
pub async fn fetch_random_game_chesscom(
    username: &str,
    client: reqwest::Client,
) -> Result<Game, GameFetchError> {
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

/// Populates the `players` table from a handful of chess.com player
/// listing endpoints (a few countries plus titled players), unless it
/// already has rows.
///
/// Per-endpoint fetch failures and per-username insert failures are
/// logged as warnings and otherwise ignored, so a single bad endpoint
/// doesn't prevent seeding the rest.
///
/// # Errors
///
/// Returns an error only if the initial row-count check fails.
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

/// Fetches the list of usernames from a chess.com player-listing
/// `endpoint` (e.g. `"country/US/players"` or `"titled/GM"`).
///
/// # Errors
///
/// Returns an error if the request fails or the response can't be
/// parsed as a [`PlayerList`].
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
