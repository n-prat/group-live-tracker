//! `https://chat.openai.com`
//! and `https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs`
//! and `https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/chat/src/main.rs`

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::headers;
use axum_extra::TypedHeader;
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
//allows to split the websocket stream into separate TX and RX branches
use axum::extract::State;
use futures::SinkExt;
use futures::StreamExt;

use crate::AppState;

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::debug!("ws_handler: `{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.protocols(["chat", "geolocation"])
        .on_failed_upgrade(|error| {
            tracing::error!("ws_handler on_failed_upgrade: error: {error}");
        })
        .on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr, state: Arc<AppState>) {
    tracing::debug!("handle_socket: protocol: {:?}", socket.protocol());

    let protocol = socket.protocol().and_then(|value| value.to_str().ok());

    if let Some("chat") = protocol {
        tracing::info!("handle_socket: chat");
        handle_socket_chat(socket, addr, state).await;
    } else if let Some("geolocation") = protocol {
        tracing::info!("handle_socket: geolocation");
        handle_socket_geolocation(socket, addr, state).await;
    } else {
        tracing::warn!("handle_socket: unsupported protocol: {:?}", protocol);
        // todo!("handle_socket: unsupported protocol")
    }
}

/// `https://github.com/tokio-rs/axum/blob/9ebd105d0410dcb8a4133374c32415b5a6950371/examples/chat/src/main.rs#L72C44-L72C59`
/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket_chat(socket: WebSocket, _who: SocketAddr, state: Arc<AppState>) {
    tracing::debug!("handle_socket: protocol: {:?}", socket.protocol());

    // "By splitting, we can send and receive at the same time."
    let (mut sender, mut receiver) = socket.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name, true);

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    // "We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client."
    let mut rx = state.chat_broadcast_sender.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.chat_broadcast_sender.send(msg);

    // "Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client."
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // "Clone things we want to pass (move) to the receiving task."
    let chat_broadcast_sender = state.chat_broadcast_sender.clone();

    // "Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers."
    let username_copy = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = chat_broadcast_sender.send(format!("{username_copy}: {text}"));
        }
    });

    // "If any one of the tasks run to completion, we abort the other."
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // "Send "user left" message (similar to "joined" above)."
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.chat_broadcast_sender.send(msg);

    remove_user(&state, &username, true);
}

///
async fn handle_socket_geolocation(socket: WebSocket, _who: SocketAddr, state: Arc<AppState>) {
    tracing::debug!("handle_socket: protocol: {:?}", socket.protocol());

    // "By splitting, we can send and receive at the same time."
    let (mut sender, mut receiver) = socket.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name, false);

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    // "We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client."
    let mut rx = state.location_broadcast_sender.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.location_broadcast_sender.send(msg);

    // "Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client."
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // "Clone things we want to pass (move) to the receiving task."
    let location_broadcast_sender = state.location_broadcast_sender.clone();

    // "Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers."
    let username_copy = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = location_broadcast_sender.send(format!("{username_copy}: {text}"));
        }
    });

    // "If any one of the tasks run to completion, we abort the other."
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // "Send "user left" message (similar to "joined" above)."
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.location_broadcast_sender.send(msg);

    remove_user(&state, &username, false);
}

/// Use a AppState to get a new unique username
/// ALSO increment `state.nb_users`
fn check_username(
    state: &AppState,
    result_username: &mut String,
    username_to_check: &str,
    is_chat: bool,
) {
    let mut users_set = if is_chat {
        state.chat_users_set.lock().unwrap()
    } else {
        state.location_users_set.lock().unwrap()
    };

    if !users_set.contains(username_to_check) {
        users_set.insert(username_to_check.to_owned());

        result_username.push_str(username_to_check);
    }
}

/// decrement `state.nb_users`
fn remove_user(state: &AppState, username: &str, is_chat: bool) {
    let mut users_set = if is_chat {
        state.chat_users_set.lock().unwrap()
    } else {
        state.location_users_set.lock().unwrap()
    };
    users_set.remove(username);
}

// /// helper to print contents of messages to stdout. Has special treatment for Close.
// fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
//     match msg {
//         Message::Text(t) => {
//             println!(">>> {who} sent str: {t:?}");
//         }
//         Message::Binary(d) => {
//             println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
//         }
//         Message::Close(c) => {
//             if let Some(cf) = c {
//                 println!(
//                     ">>> {} sent close with code {} and reason `{}`",
//                     who, cf.code, cf.reason
//                 );
//             } else {
//                 println!(">>> {who} somehow sent close message without CloseFrame");
//             }
//             return ControlFlow::Break(());
//         }

//         Message::Pong(v) => {
//             println!(">>> {who} sent pong with {v:?}");
//         }
//         // You should never need to manually handle Message::Ping, as axum's websocket library
//         // will do so for you automagically by replying with Pong and copying the v according to
//         // spec. But if you need the contents of the pings you can see them here.
//         Message::Ping(v) => {
//             println!(">>> {who} sent ping with {v:?}");
//         }
//     }
//     ControlFlow::Continue(())
// }
