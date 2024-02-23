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

use std::net::IpAddr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use auth_jwt::Claims;
use axum::routing::post;
use axum::Extension;
use axum::{response::IntoResponse, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod auth_jwt;
mod db;
mod errors_and_responses;
mod route_gpx;
mod state;
mod user;
mod ws_handler;

use crate::state::new_state;
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

    /// eg "../key.pem"
    #[clap(long, requires("tls_cert_path"))]
    tls_key_path: Option<PathBuf>,

    /// eg "../cert.pem"
    #[clap(long, requires("tls_key_path"))]
    tls_cert_path: Option<PathBuf>,
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

    let app_state = new_state();

    let db_pool = db::setup_db("sqlite://file:db.sqlite?mode=rwc").await?;

    #[allow(unused_mut)]
    let mut cors_layer = CorsLayer::very_permissive();
    #[cfg(not(debug_assertions))]
    let origins = ["https://n-prat.github.io/".parse().unwrap()];
    #[cfg(not(debug_assertions))]
    let cors_layer = cors_layer.clone().allow_origin(origins);

    let tls_config = match (opt.tls_cert_path, opt.tls_key_path) {
        (Some(tls_cert_path), Some(tls_key_path)) => {
            // get the absolute path to the certificate and private key files
            tracing::info!(
                "current dir: {}, tls_cert_path: {}, tls_key_path: {}",
                std::env::current_dir().unwrap().display(),
                tls_cert_path.display(),
                tls_key_path.display(),
            );
            let tls_cert_path = tls_cert_path.canonicalize().map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("tls_cert_path not found: {:?}", err),
                )
            })?;
            let tls_key_path = tls_key_path.canonicalize().map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("tls_key_path not found: {:?}", err),
                )
            })?;
            tracing::info!(
                "will use TLS {} {}",
                tls_cert_path.display(),
                tls_key_path.display()
            );
            // configure certificate and private key used by https
            let config = RustlsConfig::from_pem_file(tls_cert_path, tls_key_path)
                .await
                .unwrap();

            Some(config)
        }
        // NOTE: the clap args "tls_cert_path" and "tls_key_path" so we SHOULD get neither OR both
        _ => {
            tracing::info!("will NOT use TLS");
            None
        }
    };

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route(
            "/api/gpx",
            // cf https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/axum/src/lib.rs#L266
            // post({
            //     let app_state = Arc::clone(&app_state);
            //     move |body, claims| route_gpx::handle_gpx_upload(app_state, claims, body)
            // }),
            post(route_gpx::handle_gpx_upload),
        )
        .route("/ws", get(ws_handler))
        .route("/authorize", post(auth_jwt::authorize))
        .fallback_service(static_files_service)
        .layer(cors_layer)
        .layer(Extension(app_state.clone()))
        .with_state(app_state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    tracing::info!("listening on http://{}", sock_addr);

    match tls_config {
        Some(tls_config) => {
            axum_server::bind_rustls(sock_addr, tls_config)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
        }
        None => {
            // https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/websockets/src/main.rs#L66C5-L76C15
            // run it with hyper
            let listener = tokio::net::TcpListener::bind(&sock_addr).await?;
            tracing::debug!("listening on {}", listener.local_addr()?);

            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
        }
    }
}

async fn hello(_claims: Claims) -> impl IntoResponse {
    "hello from server!"
}
