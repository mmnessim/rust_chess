//! HTTP server that serves random chess.com games and exposes a small
//! player database backed by SQLite.

use axum::{Json, Router, extract::State, routing::get};
use reqwest::{StatusCode, header};

use crate::{
    chess::{
        fetch::{fetch_random_game_chesscom, seed_db},
        game::Game,
    },
    data::{db::init_db, player::Player},
    state::AppState,
};

mod chess;
mod data;
mod handlers;
mod state;

/// Initializes the DB, seeds it, and starts the Axum server on
/// `127.0.0.1:3000`.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = init_client();

    let pool = match init_db().await {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    if let Err(e) = seed_db(&pool, client.clone()).await {
        tracing::error!("seed_db failed: {e}");
    }

    let state = AppState {
        client: client.clone(),
        pool: pool,
    };

    let app = Router::new()
        .route("/game", get(handlers::api::serve_random_game))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

/// Handler for `GET /`: a basic health-check/greeting endpoint.
async fn root() -> &'static str {
    tracing::info!("/ GET");
    "Hello world"
}

/// Handler for `GET /game`: fetches a random game for a hardcoded
/// player from the chess.com API, retrying up to `MAX_ATTEMPTS` times
/// on failure.
async fn game(State(state): State<AppState>) -> Result<Json<Game>, StatusCode> {
    tracing::info!("/game GET");
    const MAX_ATTEMPTS: u8 = 3;
    let mut last_err = None;

    for attempt in 1..=MAX_ATTEMPTS {
        match fetch_random_game_chesscom("tenderllama", state.client.clone()).await {
            Ok(game) => return Ok(Json(game)),
            Err(err) => {
                tracing::warn!("attempt {attempt}/{MAX_ATTEMPTS} failed: {err}");
                last_err = Some(err);
            }
        }
    }

    tracing::error!("all {MAX_ATTEMPTS} attempts failed: {}", last_err.unwrap());
    Err(StatusCode::BAD_GATEWAY)
}

/// Handler for `GET /dbtest`: sanity-check endpoint that returns a
/// single row from the `players` table.
async fn db_test(State(state): State<AppState>) -> Result<Json<Vec<Player>>, StatusCode> {
    tracing::info!("/dbtest GET");
    match sqlx::query_as::<_, Player>("SELECT * FROM players LIMIT 1")
        .fetch_all(&state.pool)
        .await
    {
        Ok(p) => return Ok(Json(p)),
        Err(e) => tracing::error!("Error: {e}"),
    };

    Err(StatusCode::BAD_GATEWAY)
}

/// Builds the shared `reqwest::Client` used for chess.com API calls,
/// with a custom `User-Agent` header set.
fn init_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        header::HeaderValue::from_static("Rust Chess / 1.0"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    client
}
