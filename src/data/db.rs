use sqlx::{
    Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
};

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let opts = SqliteConnectOptions::new()
        .filename("chess.db")
        .create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(opts).await?;
    let _ = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS players (
                 id INTEGER PRIMARY KEY,
                 username TEXT NOT NULL,
                 active INTEGER NOT NULL
             )
        ",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub async fn insert(username: &str, pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    let x = sqlx::query("INSERT INTO players (username, active) VALUES (?, ?)")
        .bind(username)
        .bind(true)
        .execute(pool)
        .await?;
    Ok(x)
}
