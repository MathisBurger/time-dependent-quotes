use std::io::Write;
use actix_web::{HttpResponse, Responder, web, get, post, Error, HttpRequest, error, middleware};
use crate::AppState;
use serde::Deserialize;
use crate::controller::responses::ErrorResponse;
use crate::database::quote::Quote;
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct AddQuoteRequest {
    title: String
}


#[post("/api/quote/add")]
pub(crate) async fn add_quote(
    mut payload: Multipart,
    query: web::Query<AddQuoteRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./data/{}", filename.clone());
        let cloned_path = filepath.clone();

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
        let hash = sha256::digest_file(std::path::Path::new(&cloned_path)).unwrap();
        Quote::insert_quote(&data.db, &query.title, &filename, &hash).await;
    }

    Ok(HttpResponse::Ok().into())
}