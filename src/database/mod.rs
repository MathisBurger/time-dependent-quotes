use std::{env, process};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

/// Connects to the postgres database
/// and quits the application if there is no database
/// url provided via .env
pub(crate) async fn connect() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL");
    if (&db_url).is_err() {
        process::exit(0x0100);
    }
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*db_url.unwrap()).await.unwrap();
    return pool;
}