use actix_web::{HttpResponse, Responder, web, get, post};
use crate::AppState;
use serde::Deserialize;
use crate::database::quote::Quote;

#[derive(Deserialize)]
pub(crate) struct AddQuoteRequest {
    title: String
}

#[post("/api/quote/add")]
pub(crate) async fn add_quote(data: web::Data<AppState>, req: web::Json<AddQuoteRequest>) -> HttpResponse {
    let quote = Quote::insert_quote(&data.db, &req.title).await;
    HttpResponse::Ok()
        .json(quote)
}