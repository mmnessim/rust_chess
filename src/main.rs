use axum::{Json, Router, extract::State, routing::get};
use reqwest::{StatusCode, header};

use crate::{
    chess::{
        api::{random_game, seed_db},
        game::Game,
    },
    data::{db::init_db, player::Player},
    state::AppState,
};

mod chess;
mod data;
mod state;

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
        http: client.clone(),
        pool: pool,
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/game", get(game))
        .route("/dbtest", get(db_test))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

async fn root() -> &'static str {
    tracing::info!("/ GET");
    "Hello world"
}

async fn game(State(state): State<AppState>) -> Result<Json<Game>, StatusCode> {
    tracing::info!("/game GET");
    const MAX_ATTEMPTS: u8 = 3;
    let mut last_err = None;

    for attempt in 1..=MAX_ATTEMPTS {
        match random_game("tenderllama", state.http.clone()).await {
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
