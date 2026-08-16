use rand::random_range;

use crate::chess::game::{Archives, Game, ManyGames};

pub async fn stats(username: &str, client: reqwest::Client) {
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
