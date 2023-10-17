use actix_web::{
    get,
    web::{self, Bytes},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::request;

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    url: String,
    flip_x: Option<bool>,
}

#[get("/")]
pub async fn handle_image(query: web::Query<QueryParams>) -> impl Responder {
    let image = request::request(&query.url).await;

    match image {
        Ok(_) => match image {
            Ok(image_bytes) => HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(convert_image(&image_bytes)),
            Err(_) => HttpResponse::NotFound().body("404 Not Found"),
        },
        Err(_) => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn convert_image(bytes: &Bytes) -> Bytes {
    bytes.clone()
}
