use actix_web::{
    get,
    web::{self, Bytes},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{image::Converter, request};

#[derive(Deserialize, Serialize, Clone)]
pub struct QueryParams {
    url: String,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
}

#[get("/")]
pub async fn handle_image(query: web::Query<QueryParams>) -> impl Responder {
    let image = request::request(&query.url).await;
    let query_params = query.0;

    let image_bytes = match image {
        Ok(bytes) => bytes,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found"),
    };

    let converted_image = match convert_image(&image_bytes, query_params) {
        Ok(converted_image) => converted_image,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot convert image"),
    };

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(converted_image)
}

fn convert_image(bytes: &Bytes, params: QueryParams) -> Result<Bytes, String> {
    Converter::new(bytes, params).and_then(|mut converter| converter.result())
}
