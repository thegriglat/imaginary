use axum::{routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod config;
use config::Config;
mod api;
mod image;
mod query;
mod request;

#[tokio::main]
async fn main() {
    let config = Config::read();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let router = Router::new().route("/", get(api::handle_image));

    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
