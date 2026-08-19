use sqlx::{Pool, Sqlite};

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = sqlx::SqlitePool::connect("chess.db").await?;
    let q = sqlx::query(
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
