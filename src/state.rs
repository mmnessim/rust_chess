#[derive(Clone)]
pub struct AppState {
    pub http: reqwest::Client,
    pub pool: sqlx::SqlitePool,
}
