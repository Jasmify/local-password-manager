use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::str::FromStr;

use anyhow::Result;

//Create SQLite Connection Pool
pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

//Database Migration
async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}

//Database Setup
pub fn setup_database() -> Result<SqlitePool> {
    // main関数はasync fnではないので、asyncな関数を呼ぶのにblock_on関数を使う
    use tauri::async_runtime::block_on;

    const DATABASE_DIR: &str = "DB";
    const DATABASE_FILE: &str = "db.sqlite";

    let home_dir = std::env::current_dir().expect("Cannot access the current directory");
    let database_dir = home_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    let db_dir_exist = std::fs::metadata(&database_dir).is_ok();
    let db_exist = std::fs::metadata(&database_file).is_ok();

    if !db_dir_exist {
        std::fs::create_dir(&database_dir)?;
    }

    let database_dir_string = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_string, DATABASE_FILE);

    let sqlite_pool = block_on(create_pool(&database_url))?;

    if !db_exist {
        block_on(migrate(&sqlite_pool))?;
    }

    Ok(sqlite_pool)
}
