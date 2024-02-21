// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
#![warn(clippy::expect_used)]
#![warn(clippy::panic)]
#![warn(clippy::unwrap_used)]

use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use auth_jwt::Claims;
use axum::routing::post;
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use tokio::sync::broadcast;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod api_auth;
mod auth_jwt;
mod errors_and_responses;
mod user;
mod ws_handler;

use crate::ws_handler::ws_handler;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

/// `https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/chat/src/main.rs#L26C1-L32C2`
/// Our shared state
struct AppState {
    /// We require unique usernames. This tracks which usernames have been taken.
    users_set: Mutex<HashSet<String>>,
    /// Channel used to send messages to all connected clients.
    chat_broadcast_sender: broadcast::Sender<String>,
    /// Channel used to send locations to all connected clients.
    location_broadcast_sender: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    // https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/static-file-server/src/main.rs#L44
    // `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    // https://github.com/tokio-rs/axum/blob/9ebd105d0410dcb8a4133374c32415b5a6950371/examples/websockets/src/main.rs#L54
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    // Set up application state for use with with_state().
    let users_set = Mutex::new(HashSet::new());
    let (chat_tx, _rx) = broadcast::channel(100);
    let (location_tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        users_set,
        chat_broadcast_sender: chat_tx,
        location_broadcast_sender: location_tx,
    });

    // let origins = [
    //     "https://localhost:8080".parse().unwrap(),
    //     "http://localhost:8081".parse().unwrap(),
    // ];
    // let cors_layer = CorsLayer::new()
    //     .allow_origin(origins)
    //     .allow_credentials(true);
    let cors_layer = CorsLayer::very_permissive();

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/ws", get(ws_handler))
        // .route(
        //     "/api/auth/login",
        //     // cf https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/axum/src/lib.rs#L266
        //     post({
        //         let app_state = Arc::clone(&app_state);
        //         move |body| api_auth::api_auth_login(body, app_state)
        //     }),
        // )
        .route(
            "/authorize",
            // cf https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/axum/src/lib.rs#L266
            post({
                let app_state = Arc::clone(&app_state);
                move |body| auth_jwt::authorize(body, app_state)
            }),
        )
        .fallback_service(static_files_service)
        .layer(cors_layer)
        .with_state(app_state);

    // let sock_addr = SocketAddr::from((
    //     IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
    //     opt.port,
    // ));

    // tracing::info!("listening on http://{}", sock_addr);

    // axum::Server::bind(&sock_addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Unable to start server");

    // https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/websockets/src/main.rs#L66C5-L76C15
    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

async fn hello(_claims: Claims) -> impl IntoResponse {
    "hello from server!"
}
