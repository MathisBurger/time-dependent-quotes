use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
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

#[derive(Deserialize)]
pub(crate) struct SearchForQuoteRequest {
    search_string: String
}

#[derive(Deserialize)]
pub(crate) struct DownloadQuoteRequest {
    id: i32
}


#[post("/api/quote/add")]
pub(crate) async fn add_quote(
    mut payload: Multipart,
    query: web::Query<AddQuoteRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = format!("{}.{}", Uuid::new_v4().to_string(), content_disposition.get_filename().unwrap());
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

#[get("/api/quote/search")]
pub(crate) async fn search_for_quote(
    query: web::Query<SearchForQuoteRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse, Error> {

    let result = Quote::search_by_title(&data.db, &query.search_string)
        .await;
    if result.is_some() {
        return Ok(HttpResponse::Ok().json(result.unwrap()));
    }
    return Ok(HttpResponse::Ok().into());
}

#[get("/api/quote/download")]
pub(crate) async fn download_quote(
    data: web::Data<AppState>,
    query: web::Query<DownloadQuoteRequest>
) -> HttpResponse {
    let quote = Quote::get_by_id(&data.db, query.id).await;
    if quote.is_none() {
        return HttpResponse::BadRequest().body("Quote does not exist");
    }
    let unwrapped_quote = quote.unwrap();
    let filepath = format!("./data/{}", unwrapped_quote.filename.unwrap());
    let file_bytes = get_file_as_byte_vec(&filepath);
    HttpResponse::Ok().body(file_bytes)
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}