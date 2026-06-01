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

    //Modified the schema to fit the gloal test.db_init
    //file --- name of the file
    //filepath --- path of that file (used to open the file)
    //dir --- to store the directory the
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS wfly_path (

        path TEXT PRIMARY KEY

        )"#,
    )
    .execute(&pool)
    .await?;

    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn add_path(pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR REPLACE INTO wfly_path (path) VALUES (?)")
        .bind(path)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_path(pool: &SqlitePool) -> Result<String, sqlx::Error> {
    let path = sqlx::query_scalar::<_, String>("SELECT path FROM wfly_path LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(path)
}
