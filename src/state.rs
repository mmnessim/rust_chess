/// Shared application state handed to every Axum route handler.
///
/// Cloning is cheap: `reqwest::Client` and `SqlitePool` are both
/// internally reference-counted, so clones share the same underlying
/// connection pool and HTTP client.
#[derive(Clone)]
pub struct AppState {
    /// HTTP client used to call the chess.com public API.
    pub http: reqwest::Client,
    /// Connection pool for the SQLite database.
    pub pool: sqlx::SqlitePool,
}
