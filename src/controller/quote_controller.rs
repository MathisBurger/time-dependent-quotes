use actix_web::{Responder, web};
use crate::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    title: String
}

pub(crate) async fn add_quote(data: web::Data<AppState>, req: web::Json<Request>) -> std::io::Result<impl Responder> {

}