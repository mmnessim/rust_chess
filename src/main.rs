use axum::{Router, routing::get};
use reqwest::header;

use crate::chess::{api::random_game, stats};

mod chess;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = init_client();

    println!("Hello, world!");
    stats("tenderllama", client.clone()).await;
    random_game("tenderllama", client.clone()).await;

    let app = Router::new().route("/", get(root));

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
