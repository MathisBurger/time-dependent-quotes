use actix_web::{HttpResponse, web, get};
use tera::Context;
use crate::AppState;

#[get("/")]
pub(crate) async fn index_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("name", "Mathis");
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}