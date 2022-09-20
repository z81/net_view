use actix_web::{get, web, HttpResponse, Responder, Result};
use ffmpeg_next::{frame, Packet, Stream};
use serde::Deserialize;
use std::io::Cursor;

use ffmpeg_next::codec::context::Context as FFCodecContext;
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct ViewReq {
    path: String,
    w_max: u32,
}
#[get("/screen")]
async fn screen(args: web::Query<ViewReq>) -> Result<impl Responder> {
    let mut ictx = ffmpeg_next::format::input(&args.path).expect("err");

    let input = ictx.streams().best(Type::Video).expect("No streams found in video");
    let video_stream_index = input.index();

    let context_decoder = FFCodecContext::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let packet: Packet = ictx
        .packets()
        .find(|(s, _)| s.index() == video_stream_index).map(|(_, p)| p).unwrap();

    decoder.send_packet(&packet.to_owned()).unwrap();


    let ratio = (decoder.width() as f64 / args.w_max as f64);
    let w = 300;
    let h = (decoder.height() as f64 * ratio) as u32;


    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )
    .expect("ctx err");

    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame).unwrap();

        let rgb_image = image::RgbImage::from_vec(
            decoder.width(),
            decoder.height(),
            rgb_frame.data(0).to_vec(),
        )
        .unwrap();

        let mut preview = Cursor::new(Vec::new());
        let dyn_image = DynamicImage::from(rgb_image);

        dyn_image
            .write_to(&mut preview, ImageOutputFormat::Jpeg(60))
            .unwrap();

        // image::image(rgb_image, 300, 300, 1);



        return Ok(HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(preview.into_inner()));
    }

    return Ok(HttpResponse::Ok().body(""));
}
