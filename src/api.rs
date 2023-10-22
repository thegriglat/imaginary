use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use image::EncodableLayout;
use reqwest::StatusCode;

use crate::{
    image::{guess_mime_type, Converter},
    query::{Format, QueryParams},
    request,
};

pub async fn handle_image(query: Query<QueryParams>) -> impl IntoResponse {
    let image = request::request(&query.url).await;
    let query_params = query.0;
    let image_bytes = match image {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::NOT_FOUND, "404 Not found").into_response(),
    };

    let mut converter = match Converter::new(image_bytes.as_bytes(), query_params.clone()) {
        Ok(converter) => converter,
        Err(err) => {
            println!("Failed to convert image: {}", err);
            return (StatusCode::BAD_REQUEST, "400 Cannot read image").into_response();
        }
    };

    let content_type = match query_params.format {
        Some(Format::Jpeg(_)) => "image/jpeg",
        Some(Format::Png) => "image/png",
        None => match guess_mime_type(&image_bytes) {
            Ok(value) => value,
            Err(_) => return (StatusCode::BAD_REQUEST, "400 Cannot guess format").into_response(),
        },
    };

    let converted_image = match converter.result() {
        Ok(image) => image,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "500 Cannot convert image",
            )
                .into_response()
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", content_type.parse().unwrap());

    (headers, converted_image).into_response()
}
