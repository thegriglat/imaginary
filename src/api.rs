use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{image::Converter, request};
use regex::Regex;

#[derive(Serialize, Clone, Debug)]
pub enum Format {
    JPEG(u8),
    PNG,
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Format, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let str = s.as_str();
        if str == "png" {
            return Ok(Format::PNG);
        }

        if str == "jpeg" {
            return Ok(Format::JPEG(95));
        }

        let regex = Regex::new(r"jpeg:(\d+)").unwrap();
        match regex.captures(str) {
            Some(caps) => {
                let quality: u8 = caps.get(1).unwrap().as_str().parse().unwrap_or(95).min(100);
                return Ok(Format::JPEG(quality));
            }
            None => {}
        }

        return Err(serde::de::Error::custom(
            "expected png or jpeg:<quality> as format",
        ));
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryParams {
    url: String,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub grayscale: Option<bool>,
    pub blur: Option<f32>,
    pub crop: Option<String>,
    pub rotate: Option<u32>,
    pub format: Option<Format>,
}

#[get("/")]
pub async fn handle_image(query: web::Query<QueryParams>) -> impl Responder {
    let image = request::request(&query.url).await;
    let query_params = query.0;
    let image_bytes = match image {
        Ok(bytes) => bytes,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found"),
    };

    let mut converter = match Converter::new(&image_bytes, query_params.clone()) {
        Ok(converter) => converter,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot read image"),
    };

    let content_type = match query_params.format {
        Some(Format::JPEG(_)) => "image/jpeg",
        Some(Format::PNG) => "image/png",
        None => Converter::guess_format(&image_bytes),
    };

    let converted_image = match converter.result() {
        Ok(image) => image,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot convert image"),
    };

    HttpResponse::Ok()
        .content_type(content_type)
        .body(converted_image)
}
