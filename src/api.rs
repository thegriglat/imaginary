use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{image::Converter, request};

#[derive(Deserialize, Serialize, Clone)]
pub struct QueryParams {
    url: String,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub blur: Option<f32>,
}

#[get("/")]
pub async fn handle_image(query: web::Query<QueryParams>) -> impl Responder {
    let image = request::request(&query.url).await;
    let query_params = query.0;

    let image_bytes = match image {
        Ok(bytes) => bytes,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found"),
    };

    let mut converter = match Converter::new(&image_bytes, query_params) {
        Ok(converter) => converter,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot read image"),
    };

    let converted_image = match converter.result() {
        Ok(image) => image,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot convert image"),
    };

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(converted_image)
}
