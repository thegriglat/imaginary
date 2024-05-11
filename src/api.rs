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
    app_state::AppState,
    image::{guess_mime_type, Converter},
    query::{Format, QueryParams},
    request,
};

pub async fn handle_image(
    query: Query<QueryParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // check cache
    let image = get_image_bytes(&query.url, &mut state.clone()).await;
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

fn get_image_from_redis(key: &str, redis_client: &mut Client) -> Option<Bytes> {
    let cached_image: redis::RedisResult<Bytes> = redis_client.get(key);
    match cached_image {
        Ok(image) => Some(image),
        Err(_) => None,
    }
}

fn set_image_in_redis(key: &str, image: &Bytes, redis_client: &mut Client, ttl: usize) {
    let expiration_in_seconds = redis::SetExpiry::EX(ttl);
    let options = SetOptions::default().with_expiration(expiration_in_seconds);
    let _: RedisResult<Bytes> = redis_client.set_options(key, image.as_bytes(), options);
}

async fn download_image(url: &str) -> Result<Bytes> {
    request::request(url)
        .await
        .map_err(|error| anyhow::anyhow!("Failed to download image: {}", error))
}

async fn get_image_bytes(url: &str, state: &mut AppState) -> Result<Bytes> {
    let key = redis_key(url, state.config.redis_prefix.as_str());
    let cached_image = get_image_from_redis(&key, &mut state.redis_client);
    match cached_image {
        Some(image) => Ok(image),
        None => {
            let image = download_image(url).await?;
            let ttl = state.config.redis_ttl;
            set_image_in_redis(&key, &image, &mut state.redis_client, ttl);
            Ok(image)
        }
    }
}

fn redis_key(url: &str, prefix: &str) -> String {
    format!("{}:cache:{}", prefix, url)
}
