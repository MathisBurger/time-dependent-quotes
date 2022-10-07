use chrono::{DateTime, Utc};
use sqlx::{Error, Executor, FromRow, Pool, Postgres, query, query_as, Row};
use crate::random::get_random_string;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct Quote {
    id: i32,
    title: Option<String>,
    hash: Option<String>,
    uploaded_at: Option<i64>,
    admin_key: Option<String>
}

impl Quote {

    pub async fn insert_quote(conn: &Pool<Postgres>, title: &String) -> Quote {
        let date = Utc::now().timestamp_millis();
        let admin_key = get_random_string(255);
        let quote = query_as::<_, Quote>(
            "INSERT INTO quotes (title, uploaded_at, admin_key) VALUES ($1, $2, $3) RETURNING *;"
        )
            .bind(title)
            .bind(date)
            .bind(admin_key)
            .fetch_one(conn).await.unwrap();
        return quote;
    }

    pub async fn get_by_admin_key(conn: &Pool<Postgres>, admin_key: String) -> Option<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE admin_key=$1")
            .bind(admin_key)
            .fetch_one(conn)
            .await;
        match res {
            Err(e) => None,
            Ok(row) => Some(row)
        }

    }
}