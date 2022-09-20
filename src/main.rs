use actix_web::middleware::Logger;

use actix_cors::Cors;
use actix_web::{
    cookie::time::Duration,
    dev::{self, HttpServiceFactory, ServiceResponse},
    http::{self, header, KeepAlive, StatusCode},
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpServer,
};
use random_string::generate;
use serde::Serialize;
mod handlers;
use actix_web::dev::Service as _;
// use futures_util::future::FutureExt;

// pub fn not_found_error_handler<B>(
//     mut res: dev::ServiceResponse<B>,
// ) -> actix_web::Result<middleware::ErrorHandlerResponse<B>> {
//     let (req, res) = res.into_parts();
//     let res = res.set_body(r#"{"json", "in the body"}"#.to_owned());

//     let res = ServiceResponse::new(req, res)
//         .map_into_boxed_body()
//         .map_into_right_body();

//     Ok(ErrorHandlerResponse::Response(res))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ffmpeg_next::init().unwrap();

    let charset = "qwertyuopasdfghkzxcvbnmQWERTYUPASDFGHJKLZXCVBNM1234567890";
    let passwd = generate(12, charset);
    println!("PASS: {}", passwd);

    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(handlers::share)
            .service(handlers::preview)
            .service(handlers::read)
            .service(handlers::screen)
            .service(actix_files::Files::new("/", ".").use_last_modified(true))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 15950))?
    .bind(("0.0.0.0", 15951))?
    .bind(("0.0.0.0", 15952))?
    .workers(16)
    .run()
    .await
}
