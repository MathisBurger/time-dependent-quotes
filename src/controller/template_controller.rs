use actix_web::{HttpResponse, web, get};
use actix_web::http::header::q;
use chrono::{DateTime, NaiveDateTime, Utc};
use tera::Context;
use serde::Serialize;
use crate::AppState;
use crate::database::quote::Quote;

#[get("/")]
pub(crate) async fn index_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = Context::new();
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Serialize)]
pub(crate) struct ResponseQuote {
    pub(crate) id: i32,
    pub(crate) title: Option<String>,
    pub(crate) hash: Option<String>,
    pub(crate) uploaded_at: String,
    pub(crate) admin_key: Option<String>,
    pub(crate) filename: Option<String>
}

#[get("/all-sources")]
pub(crate) async fn all_sources_page(data: web::Data<AppState>) -> HttpResponse {
    let quotes = Quote::get_all(&data.db).await;
    let mut ctx = Context::new();
    let mut response_quotes = vec![];
    for quote in quotes {
        let naive = NaiveDateTime::from_timestamp(quote.uploaded_at.unwrap(), 0);
        let utc_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        response_quotes.push(ResponseQuote {
            id: quote.id,
            title: quote.title,
            hash: quote.hash,
            uploaded_at: utc_time.to_rfc2822(),
            admin_key: quote.admin_key,
            filename: quote.filename
        });
    }
    ctx.insert("quotes", &response_quotes);
    let rendered = data.tmpl.render("all-sources.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}