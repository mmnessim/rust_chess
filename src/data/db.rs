use rusqlite::{Connection, Error};

pub fn init_db() -> Result<(), Error> {
    let conn = Connection::open("chess.db")?;

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                active INTEGER NOT NULL
            )",
        (),
    ) {
        Ok(_) => println!("Created table (if not exists)"),
        Err(e) => println!("Could not create table: {e}"),
    };
    Ok(())
}
