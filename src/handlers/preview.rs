use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, get, web, HttpResponse, Responder, ResponseError, Result};
use serde::Deserialize;
use std::io::Cursor;
use std::sync::Arc;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{ImageDecoder, ImageError, ImageOutputFormat};

use derive_more::{Display, Error};

#[derive(Deserialize)]
struct ViewReq {
    path: String,
    w_max: u32,
}

// impl error::ResponseError for ImageError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match *self {
//             UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
// #[derive(Clone)]
// pub struct MarketSpecificData(Arc<dyn image::ImageDecoder<'static, Reader = ImageReader<Any>>>);

// impl From<MarketSpecificData> for actix_web::Error {
//     fn from(err: image::error::DecodingError) -> actix_web::Error {
//         actix_web::Error {
//             cause: Box::new(err),
//         }
//     }
// }

// impl<T> From<T> for actix_web::Error {
//     fn from(err: T) -> actix_web::Error {
//         actix_web::Error {
//             cause: Box::new(err),
//         }
//     }
// }

#[get("/preview")]
async fn preview(args: web::Query<ViewReq>) -> Result<impl Responder> {
    let buff = ImageReader::open(args.path.to_owned())?;

    let img = buff.decode().map_err(|err| error::ErrorBadRequest(err))?;

    let mut preview = Cursor::new(Vec::new());
    let ration = img.width() as f64 / img.height() as f64;
    let dyn_image = img.resize(
        args.w_max,
        (img.height() as f64 * ration) as u32,
        FilterType::Lanczos3,
    );

    dyn_image
        .write_to(&mut preview, ImageOutputFormat::Jpeg(60))
        .map_err(|err| {
            error::ErrorBadRequest(err);
        })
        .err();

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(preview.into_inner()))
}

// pub fn resize<'a>(
//     buf: &[u8],
//     width: NonZeroU32,
//     height: NonZeroU32,
// ) -> Option<fr::Image<'a>> {
//     let processor = Processor::new();
//     let decoded = processor.process_8bit(buf).ok()?;

//     let src_image = fr::Image::from_vec_u8(
//         NonZeroU32::try_from(decoded.width()).expect("zero width"),
//         NonZeroU32::try_from(decoded.height()).expect("zero height"),
//         decoded.deref().to_vec(),
//         fr::PixelType::U8x3,
//     )
//     .ok()?;

//     let mut dst_image = fr::Image::new(width, height, src_image.pixel_type());

//     // Get mutable view of destination image data
//     let mut dst_view = dst_image.view_mut();

//     // Create Resizer instance and resize source image
//     // into buffer of destination image
//     let mut resizer = fs::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::CatmullRom));
//     resizer.resize(&src_image.view(), &mut dst_view).ok()?;
//     Some(dst_image)
// }
