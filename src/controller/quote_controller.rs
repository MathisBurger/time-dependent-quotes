use actix_web::{HttpResponse, Responder, web, get};
use crate::AppState;
use serde::Deserialize;
use crate::database::quote::Quote;

#[derive(Deserialize)]
struct AddQuoteRequest {
    title: String
}

#[get("/api/quote/add")]
pub(crate) async fn add_quote(data: web::Data<AppState>, req: web::Json<AddQuoteRequest>) -> HttpResponse {
    let quote = Quote::insert_quote(&data.db, &req.title).await;
    HttpResponse::Ok()
        .json(quote)
}