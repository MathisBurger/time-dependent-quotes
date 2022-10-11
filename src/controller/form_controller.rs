use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn upload_form() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/api/quote/add?title=Hallo" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[get("/validate")]
pub async fn validate_form() -> HttpResponse {
    let html = r#"<html>
        <head><title>Validate test</title></head>
        <body>
            <form action="/api/quote/validate?admin_key=B6yNDwbgMsq1IPvkTSklRFTS3ZOHXs3JaXRkmuPOYpxukPfMndeclYIYgF7svnsUWx1VexxO2PTp5csmTnRB635z5U4n78YYm6JTZghfKCqLOKKwwmAGMabSYUfgMk1UXXH7jCgDzBVlA1pHHpismPyVJXQoEmQ9d1LCodi4ydCNsfDyEsg9KVt17L3ESwf4howBl0BUDK50GtCrwPunl1dnYzcziAUaFq4Ah3a6faDJaRnFra2vFSXc9rGPmhd" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}