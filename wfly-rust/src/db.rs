use sqlx::{Result, SqlitePool};
use std::env;
use std::path::PathBuf;

pub async fn db_init() -> Result<SqlitePool> {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE")) // Windows fallback
        .expect("Could not determine home directory");

    let db_path = PathBuf::from(home_dir).join("wfly_path.db");
    let connection_string = format!("sqlite://{}?mode=rwc", db_path.display());

    let pool = SqlitePool::connect(&connection_string).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS wfly_path (

        key TEXT PRIMARY KEY,
        path TEXT 

        )"#,
    )
    .execute(&pool)
    .await?;

    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn add_path(pool: &SqlitePool, path: &str, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR REPLACE INTO wfly_path (key, path) VALUES (?, ?)")
        .bind(key)
        .bind(path)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_path(pool: &SqlitePool, key: &str) -> Result<String, sqlx::Error> {
    let path = sqlx::query_scalar::<_, String>("SELECT path FROM wfly_path WHERE key = ?")
        .bind(key)
        .fetch_one(pool)
        .await?;

    Ok(path)
}

pub async fn get_keys(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let keys = sqlx::query_scalar("SELECT key FROM wfly_path")
        .fetch_all(pool)
        .await?;

    Ok(keys)
}
