use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ReadReq {
    path: String,
}

#[get("/read")]
async fn read(args: web::Query<ReadReq>) -> actix_web::Result<impl Responder> {
    let image_content = fs::read(args.path.to_owned())?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(image_content))
}
