use chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Postgres, Row};
use crate::random::get_random_string;

pub(crate) struct Quote {
    id: i32,
    title: Option<String>,
    hash: Option<String>,
    uploaded_at: Option<DateTime<Utc>>,
    admin_key: Option<String>
}

impl Quote {

    pub async fn insert_quote(conn: &Pool<Postgres>, title: &String) -> Quote {
        let date = Utc::now();
        let admin_key = get_random_string(255);
        conn.execute(
            "INSERT INTO quotes (title, uploaded_at, admin_key) VALUES ($1, $2 $3)",
            &[title, &date, &admin_key]
        ).await.unwrap();
        return Quote::get_by_admin_key(conn, admin_key).await.unwrap();
    }

    pub async fn get_by_admin_key(conn: &Pool<Postgres>, admin_key: String) -> Option<Quote> {
        let res = conn
            .fetch_one("SELECT * FROM quotes WHERE admin_key=$1", &[&admin_key])
            .await;
        match res {
            Err(e) => None,
            Ok(row) => Quote {
                id: row.get(0),
                title: row.get(1),
                hash: row.get(2),
                uploaded_at: row.get(3),
                admin_key: row.get(4)
            }
        }

    }
}