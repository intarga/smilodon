use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337);

    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
