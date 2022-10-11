use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, web, post};
use crate::AppState;
use serde::Deserialize;
use futures_util::TryStreamExt as _;
use std::io::Write;
use uuid::Uuid;
use crate::database::quote::Quote;

#[derive(Deserialize)]
pub(crate) struct ValidateQuoteRequest {
    admin_key: String
}

/// Validates the incoming file
#[post("/api/quote/validate")]
pub(crate) async fn validate_quote(
    mut payload: Multipart,
    query: web::Query<ValidateQuoteRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse, Error> {
    let mut final_file_path = String::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{}", filename.clone());
        final_file_path = filepath.clone();

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }
    let hash = sha256::digest_file(std::path::Path::new(&final_file_path)).unwrap();
    let comparison_quote = Quote::get_by_admin_key(&data.db, &query.admin_key)
        .await;
    if comparison_quote.is_none() {
        return Ok(HttpResponse::BadRequest().into());
    }
    std::fs::remove_file(final_file_path)?;
    if comparison_quote.unwrap().hash.unwrap() == hash {
        return Ok(HttpResponse::Ok().into());
    }
    Ok(HttpResponse::BadRequest().into())
}