use axum::body::Bytes;

pub async fn request(uri: &str) -> reqwest::Result<Bytes> {
    let bytes = reqwest::get(uri).await?.bytes().await?;
    Ok(bytes)
}
