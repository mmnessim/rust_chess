use sqlx::{
    Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
};

/// Opens (creating if necessary) the `chess.db` SQLite database and
/// ensures the `players` table exists.
///
/// # Errors
///
/// Returns an error if the connection cannot be established or the
/// `CREATE TABLE` statement fails.
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

/// Inserts a new player row with the given `username`, marked active.
///
/// # Errors
///
/// Returns an error if the insert statement fails to execute.
pub async fn insert(username: &str, pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    let x = sqlx::query("INSERT INTO players (username, active) VALUES (?, ?)")
        .bind(username)
        .bind(true)
        .execute(pool)
        .await?;
    Ok(x)
}
