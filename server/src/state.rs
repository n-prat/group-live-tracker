use std::sync::Arc;
use std::sync::RwLock;

use tokio::sync::broadcast;

/// `https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/chat/src/main.rs#L26C1-L32C2`
/// Our shared state
pub(crate) struct AppState {
    /// Channel used to send messages to all connected clients.
    pub(crate) chat_broadcast_sender: broadcast::Sender<String>,
    /// Channel used to send locations to all connected clients.
    pub(crate) location_broadcast_sender: broadcast::Sender<String>,
    /// GeoJSON result of https://github.com/georust/geozero/blob/52a4d2d3c11f02e734274fcb6ee4b88b94b5b53d/geozero/src/geojson/mod.rs#L34
    /// so this is a String
    pub(crate) geojson: Option<String>,
}

/// For now just an Arc; but if needed we can add a `RwLock`
/// cf https://github.com/tokio-rs/axum/blob/4d65ba0215b57797193ec49245d32d4dd79bb701/examples/key-value-store/src/main.rs#L83
pub(crate) type SharedState = Arc<RwLock<AppState>>;

pub(crate) fn new_state() -> SharedState {
    // Set up application state for use with with_state().
    let (chat_tx, _rx) = broadcast::channel(100);
    let (location_tx, _rx) = broadcast::channel(100);

    let app_state = AppState {
        chat_broadcast_sender: chat_tx,
        location_broadcast_sender: location_tx,
        geojson: None,
    };

    Arc::new(RwLock::new(app_state))
}
