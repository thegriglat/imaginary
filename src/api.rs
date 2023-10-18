use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    image::{guess_mime_type, Converter},
    query::{Format, QueryParams},
    request,
};

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
        Err(_) => return HttpResponse::BadRequest().body("400 Cannot read image"),
    };

    let content_type = match query_params.format {
        Some(Format::JPEG(_)) => "image/jpeg",
        Some(Format::PNG) => "image/png",
        None => match guess_mime_type(&image_bytes) {
            Ok(value) => value,
            Err(_) => return HttpResponse::BadRequest().body("400 Cannot guess format"),
        },
    };

    let converted_image = match converter.result() {
        Ok(image) => image,
        Err(_) => return HttpResponse::InternalServerError().body("500 Cannot convert image"),
    };

    println!("type: {}", content_type);
    HttpResponse::Ok()
        .content_type(content_type)
        .body(converted_image)
}
