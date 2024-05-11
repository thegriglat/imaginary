use anyhow::Result;
use axum::body::Bytes;

pub async fn request(uri: &str) -> Result<Bytes> {
    let bytes = reqwest::get(uri).await?.bytes().await?;
    Ok(bytes)
}
