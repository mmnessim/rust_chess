use axum::{Json, Router, extract::State, routing::get};
use reqwest::{StatusCode, header};

use crate::{
    chess::{api::random_game, game::Game, stats},
    state::AppState,
};

mod chess;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = init_client();
    let state = AppState {
        http: client.clone(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/game", get(game))
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
        match random_game("tenderllam", state.http.clone()).await {
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
