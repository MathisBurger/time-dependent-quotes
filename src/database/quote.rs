use chrono::Utc;
use sqlx::{Pool, Postgres, query_as};
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

    /// Creates a new quote from the given data
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

    /// Gets a specific quote by the admin key
    pub async fn get_by_admin_key(conn: &Pool<Postgres>, admin_key: &String) -> Option<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE admin_key=$1;")
            .bind(admin_key)
            .fetch_one(conn)
            .await;
        match res {
            Err(_) => None,
            Ok(row) => Some(row)
        }
    }

    /// Gets a specific quote by the ID
    pub async fn get_by_id(conn: &Pool<Postgres>, id: i32) -> Option<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE id=$1;")
            .bind(id)
            .fetch_one(conn)
            .await;
        match res {
            Err(_) => None,
            Ok(row) => Some(row)
        }
    }

    /// Searches for multiple quotes with a title
    pub async fn search_by_title(conn: &Pool<Postgres>, search_string: &String) -> Option<Vec<Quote>> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes WHERE title LIKE $1;")
            .bind("%".to_owned() + search_string + "%")
            .fetch_all(conn)
            .await;
        match res {
            Err(_) => None,
            Ok(row) => Some(row)
        }
    }

    /// Gets all quotes
    pub async fn get_all(conn: &Pool<Postgres>) -> Vec<Quote> {
        let res = query_as::<_, Quote>("SELECT * FROM quotes;")
            .fetch_all(conn).await;
        return res.unwrap();
    }
}