use axum::{routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod config;
use config::Config;
mod api;
mod app_state;
mod image;
mod query;
mod request;

#[tokio::main]
async fn main() {
    let config = Config::read();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let redis = redis::Client::open(config.redis_url.as_str()).unwrap();

    let app_port = config.port;
    let app_state = app_state::AppState {
        redis_client: redis,
        config,
    };

    let router = Router::new()
        .route("/", get(api::handle_image))
        .with_state(app_state);

    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_port));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
