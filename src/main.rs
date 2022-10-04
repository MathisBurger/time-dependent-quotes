use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv;

mod database;
mod controller;

struct AppState {
    db: Pool<Postgres>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    let conn = database::connect().await;

    sqlx::migrate!("./migrations/")
        .run(&conn)
        .await.expect("Cannot migrate");

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                db: conn.clone()
            })
            .service(controller::quote_controller::add_quote)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
