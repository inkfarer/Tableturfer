use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;
use crate::socket::socket_handler;
use crate::socket::room_store::SocketRoomStore;

mod app_config;
mod socket;

pub struct AppState {
    room_store: Mutex<SocketRoomStore>,
}

async fn hello() -> &'static str {
    "Hello!"
}

#[tokio::main]
async fn main() {
    let config = app_config::load_config();

    env_logger::Builder::new()
        .parse_filters(&config.logger.filters)
        .parse_write_style(&config.logger.write_style)
        .init();

    // log::info!("Connecting to Redis at {}:{}", config.redis.host, config.redis.port);
    // let redis = redis::Client::open(format!("redis://{}:{}", config.redis.host, config.redis.port)).unwrap();

    let room_store = Mutex::new(SocketRoomStore::default());
    let app_state = Arc::new(AppState { room_store });

    let router = Router::with_state(app_state)
        .route("/", get(hello))
        .route("/ws", get(socket_handler));

    let addr = SocketAddr::from((IpAddr::from_str(&config.app.host).unwrap(), config.app.port));
    log::info!("Starting server at {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
