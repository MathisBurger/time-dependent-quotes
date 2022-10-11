use chrono::{DateTime, Utc};
use sqlx::{Error, Executor, FromRow, Pool, Postgres, query, query_as, Row};
use crate::random::get_random_string;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct Quote {
    pub(crate) id: i32,
    pub(crate) title: Option<String>,
    pub(crate) hash: Option<String>,
    pub(crate) uploaded_at: Option<i64>,
    pub(crate) admin_key: Option<String>,
    pub(crate) filename: Option<String>
}

impl Quote {

    pub async fn insert_quote(conn: &Pool<Postgres>, title: &String, filename: &String, hash: &String) -> Quote {
        let date = Utc::now().timestamp_millis();
        let admin_key = get_random_string(255);
        let quote = query_as::<_, Quote>(
            "INSERT INTO quotes (title, uploaded_at, admin_key, filename, hash) VALUES ($1, $2, $3, $4, $5) RETURNING *;"
        )
            .bind(title)
            .bind(date)
            .bind(admin_key)
            .bind(filename)
            .bind(hash)
            .fetch_one(conn).await.unwrap();
        return quote;
    }

    pub async fn get_by_admin_key(conn: &Pool<Postgres>, admin_key: String) -> Option<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE admin_key=$1;")
            .bind(admin_key)
            .fetch_one(conn)
            .await;
        match res {
            Err(e) => None,
            Ok(row) => Some(row)
        }
    }

    pub async fn search_by_title(conn: &Pool<Postgres>, search_string: &String) -> Option<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE title LIKE $1;")
            .bind("%".to_owned() + search_string + "%")
            .fetch_one(conn)
            .await;
        match res {
            Err(e) => None,
            Ok(row) => Some(row)
        }
    }

    pub async fn update_hash(conn: &Pool<Postgres>, quote_id: i32, hash: String) -> Option<Quote> {
        let res = query_as::<_, Quote>("UPDATE quotes SET hash=$1 WHERE id=$2 RETURNING *;")
            .bind(hash)
            .bind(quote_id)
            .fetch_one(conn)
            .await;
        match res {
            Err(e) => None,
            Ok(row) => Some(row)
        }
    }
}