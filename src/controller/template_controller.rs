use actix_web::{HttpResponse, web, get, HttpRequest};
use chrono::{DateTime, NaiveDateTime, Utc};
use tera::Context;
use serde::Serialize;
use crate::AppState;
use crate::database::quote::Quote;
use std::str::FromStr;

/// Default web response
#[get("/")]
pub(crate) async fn index_page(data: web::Data<AppState>) -> HttpResponse {
    let ctx = Context::new();
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

/// Web page for uploading new quotes
#[get("/upload")]
pub(crate) async fn upload_page(data: web::Data<AppState>) -> HttpResponse {
    let ctx = Context::new();
    let rendered = data.tmpl.render("upload.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

/// Web page for validating quotes
#[get("/validate/{admin_key}")]
pub(crate) async fn validate_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let admin_key = req.match_info().get("admin_key").unwrap();
    let quote = Quote::get_by_admin_key(&data.db, &admin_key.to_string()).await;
    if quote.is_none() {
        return HttpResponse::BadRequest().body("admin key does not exist");
    }
    let mut ctx = Context::new();
    ctx.insert("title", &quote.as_ref().unwrap().title);
    let rendered = data.tmpl.render("validate.html", &ctx).unwrap();
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

/// Webpage for fetching all quotes
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

/// Webpage for specific details of a quote
#[get("/view/{id}")]
pub(crate) async fn view_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let admin_key = req.match_info().get("id").unwrap();
    let id: i32 = FromStr::from_str(admin_key).unwrap();
    let quote = Quote::get_by_id(&data.db, id).await;
    if quote.is_none() {
        return HttpResponse::BadRequest().body("Quote does not exist");
    }
    let mut ctx = Context::new();
    let naive = NaiveDateTime::from_timestamp(quote.as_ref().unwrap().uploaded_at.unwrap(), 0);
    let utc_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let raw_quote = quote.unwrap();
    let response_quote = ResponseQuote {
        id: raw_quote.id,
        title: raw_quote.title,
        hash: raw_quote.hash,
        uploaded_at: utc_time.to_rfc2822(),
        admin_key: raw_quote.admin_key,
        filename: raw_quote.filename
    };
    ctx.insert("quote", &response_quote);
    let rendered = data.tmpl.render("view.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}