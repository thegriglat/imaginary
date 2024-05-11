use std::env;

use anyhow::Result;
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use image::EncodableLayout;
use redis::{Client, Commands, RedisResult, SetOptions};
use reqwest::StatusCode;

use crate::{
    image::{guess_mime_type, Converter},
    query::{Format, QueryParams},
    request,
};

pub async fn handle_image(
    query: Query<QueryParams>,
    State(redis_client): State<Client>,
) -> impl IntoResponse {
    // check cache
    let image = download_image(&query.url, &mut redis_client.clone()).await;
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
        Some(Format::WebP) => "image/webp",
        Some(Format::Avif) => "image/avif",
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

async fn download_image(url: &str, redis_client: &mut Client) -> Result<axum::body::Bytes> {
    let key = redis_key(url);
    let cached_image: redis::RedisResult<axum::body::Bytes> = redis_client.get(key.clone());
    let image: Result<axum::body::Bytes> = match cached_image {
        Ok(image) => Ok(image),
        Err(err) => {
            println!("{}", err);
            let image = request::request(url).await;
            match image {
                Ok(img) => {
                    let expiration_in_seconds = redis::SetExpiry::EX(60); // 60 seconds
                    let options = SetOptions::default().with_expiration(expiration_in_seconds);
                    let _res: RedisResult<Bytes> =
                        redis_client.set_options(key, img.as_bytes(), options);

                    Ok(img)
                }
                Err(err) => {
                    println!("Failed to download image: {}", err);
                    Err(anyhow::Error::from(err))
                }
            }
        }
    };

    image.map_err(|error| anyhow::anyhow!("Failed to download image: {}", error))
}

fn redis_key(url: &str) -> String {
    let prefix = env::var("REDIS_PREFIX").unwrap_or_else(|_| "imaginary".to_string());
    format!("{}:cache:{}", prefix, url)
}
