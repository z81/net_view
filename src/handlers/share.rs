use actix_web::{get, web};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize)]
struct File {
    path: String,
}

#[derive(Serialize)]
struct ShareRes {
    files: Vec<File>,
}

#[derive(Deserialize)]
struct ShareReq {
    path: String,
}

#[get("/share")]
async fn share(args: web::Query<ShareReq>) -> actix_web::Result<web::Json<ShareRes>> {
    let dir = fs::read_dir(args.path.to_owned())?;

    let files = dir
        .flatten()
        .map(|entry| File {
            path: entry.path().to_str().unwrap().to_owned(),
        })
        .collect();

    Ok(web::Json(ShareRes { files }))
}
