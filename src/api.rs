use actix_web::{get, web, HttpResponse, Responder};
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
        Ok(_) => {
            let image_bytes = image.unwrap();
            println!("bytes: {}", image_bytes.len());
            HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(image_bytes)
        }
        Err(_) => HttpResponse::NotFound().body("404 Not Found"),
    }
}
