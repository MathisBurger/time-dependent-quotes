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
    mut payload: Multipart
) -> Result<HttpResponse, Error> {
    /*while let Some(mut field) = multipart.try_next().await? {
        let disposition = field.content_disposition();
        let quote = Quote::insert_quote(&data.db, &query.title).await;
        let filepath = format!("./data/{}.{}", quote.admin_key.as_ref().unwrap(), disposition.get_filename_ext().unwrap());
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
        return Ok(HttpResponse::Ok().json(quote));
    }


    Err(error::ErrorBadRequest("No file given"))*/

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./data/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}