use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use sqlx::{Pool, Postgres};
use dotenv;
use tera::{Tera, Context};

mod database;
mod controller;
mod random;

struct AppState {
    db: Pool<Postgres>,
    tmpl: Tera
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let conn = database::connect().await;

    sqlx::migrate!("./migrations/")
        .run(&conn)
        .await.expect("Cannot migrate");

    HttpServer::new(move || {
        let tera =
            Tera::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
            ).unwrap();

        App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .app_data(Data::new(AppState {
                db: conn.clone(),
                tmpl: tera
            }))
            .service(controller::quote_controller::add_quote)
            .service(controller::quote_controller::search_for_quote)
            .service(controller::validate_controller::validate_quote)
            .service(controller::template_controller::index_page)
            .service(controller::template_controller::all_sources_page)
            .service(controller::template_controller::upload_page)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
