//! `https://chat.openai.com`
//! and `https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs`
//! and `https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/chat/src/main.rs`

use std::net::SocketAddr;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};
use axum_extra::headers;
use axum_extra::TypedHeader;
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
//allows to split the websocket stream into separate TX and RX branches
use axum::body::Body;
use axum::extract::Query;
use futures::SinkExt;
use futures::StreamExt;
use jsonwebtoken::{decode, Validation};
use serde::Deserialize;

use crate::{
    api_authorize_jwt::{Claims, KEYS},
    errors_and_responses::AppError,
    state::SharedState,
};

#[derive(Debug, Deserialize)]
pub(crate) struct QueryToken {
    token: String,
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[axum::debug_handler]
pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<SharedState>,
    // _claims: Claims, // NO! no easy way to add custom headers in yewhook's websocket to we use a query param "token" instead
    query_token: Query<QueryToken>,
) -> Result<Response, AppError> {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::debug!("ws_handler: `{user_agent}` at {addr} connected. [token = {query_token:?}]");

    // cf "impl<S> FromRequestParts<S> for Claims"
    // Decode the token here, similar to how you would in your FromRequestParts implementation
    // This is just a placeholder, replace it with your actual decoding logic
    let token_data = decode::<Claims>(&query_token.token, &KEYS.decoding, &Validation::default())
        .map_err(|_jwt_err| AppError::LoginError)?;

    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    Ok(ws
        .protocols(["chat", "geolocation"])
        .on_failed_upgrade(|error| {
            tracing::error!("ws_handler on_failed_upgrade: error: {error}");
        })
        .on_upgrade(move |socket| {
            let fut = handle_socket(socket, addr, state.clone(), token_data.claims.sub);
            async move {
                if let Err(e) = fut.await {
                    tracing::error!("Error in handle_socket: {:?}", e);
                }
            }
        }))
}

async fn handle_socket(
    socket: WebSocket,
    addr: SocketAddr,
    state: SharedState,
    claims_sub: String,
) -> Result<Response, AppError> {
    tracing::debug!("handle_socket: protocol: {:?}", socket.protocol());

    let protocol = socket.protocol().and_then(|value| value.to_str().ok());

    if let Some("chat") = protocol {
        tracing::info!("handle_socket: chat");
        Ok(handle_socket_chat(socket, addr, state, claims_sub).await?)
    } else if let Some("geolocation") = protocol {
        tracing::info!("handle_socket: geolocation");
        Ok(handle_socket_geolocation(socket, addr, state, claims_sub).await?)
    } else {
        tracing::warn!("handle_socket: unsupported protocol: {:?}", protocol);
        // todo!("handle_socket: unsupported protocol")
        Err(AppError::BadRequest)
    }
}

/// `https://github.com/tokio-rs/axum/blob/9ebd105d0410dcb8a4133374c32415b5a6950371/examples/chat/src/main.rs#L72C44-L72C59`
/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket_chat(
    socket: WebSocket,
    who: SocketAddr,
    state: SharedState,
    claims_sub: String,
) -> Result<Response, AppError> {
    tracing::debug!(
        "handle_socket_chat: protocol: {:?}, who: {who:?}",
        socket.protocol()
    );

    // "By splitting, we can send and receive at the same time."
    let (mut sender, mut receiver) = socket.split();

    // Username is extracted from Auth header(or query param token in this case)
    let username = claims_sub;

    // "We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client."
    let mut rx = state
        .write()
        .map_err(|err| {
            tracing::error!("handle_socket_chat: state write lock error: {:?}", err,);
            AppError::InternalError
        })?
        .chat_broadcast_sender
        .subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state
        .write()
        .map_err(|err| {
            tracing::error!("handle_socket_chat: state write lock error: {:?}", err,);
            AppError::InternalError
        })?
        .chat_broadcast_sender
        .send(msg);

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
    let chat_broadcast_sender = state
        .write()
        .map_err(|err| {
            tracing::error!("handle_socket_chat: state write lock error: {:?}", err,);
            AppError::InternalError
        })?
        .chat_broadcast_sender
        .clone();

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
    let _ = state
        .write()
        .map_err(|err| {
            tracing::error!("handle_socket_chat: state write lock error: {:?}", err,);
            AppError::InternalError
        })?
        .chat_broadcast_sender
        .send(msg);

    Ok(Response::new(Body::empty()))
}

///
async fn handle_socket_geolocation(
    socket: WebSocket,
    who: SocketAddr,
    state: SharedState,
    claims_sub: String,
) -> Result<Response, AppError> {
    tracing::debug!(
        "handle_socket_geolocation: protocol: {:?}, who: {who:?}",
        socket.protocol()
    );

    // "By splitting, we can send and receive at the same time."
    let (mut sender, mut receiver) = socket.split();

    // Username is extracted from Auth header(or query param token in this case)
    let username = claims_sub;

    // "We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client."
    let mut rx = state
        .write()
        .map_err(|err| {
            tracing::error!(
                "handle_socket_geolocation: state write lock error 0: {:?}",
                err,
            );
            AppError::InternalError
        })?
        .location_broadcast_sender
        .subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state
        .write()
        .map_err(|err| {
            tracing::error!(
                "handle_socket_geolocation: state write lock error: {:?}",
                err,
            );
            AppError::InternalError
        })?
        .location_broadcast_sender
        .send(msg);

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
    let location_broadcast_sender = state
        .write()
        .map_err(|err| {
            tracing::error!(
                "handle_socket_geolocation: state write lock error 1: {:?}",
                err,
            );
            AppError::InternalError
        })?
        .location_broadcast_sender
        .clone();

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
    let _ = state
        .write()
        .map_err(|err| {
            tracing::error!(
                "handle_socket_geolocation: state write lock error 2: {:?}",
                err,
            );
            AppError::InternalError
        })?
        .location_broadcast_sender
        .send(msg);

    Ok(Response::new(Body::empty()))
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

/// cf https://github.com/tokio-rs/axum/blob/main/examples/testing-websockets/src/main.rs
#[cfg(test)]
mod tests {
    use crate::{db::setup_db, new_app};

    use super::*;

    use axum_test::http::Request;
    use base64::Engine;
    use rand::Rng;
    use std::{
        future::IntoFuture,
        net::{Ipv4Addr, SocketAddr},
    };
    use tokio_tungstenite::tungstenite::{self};

    async fn setup(websocket_protocol: &str, username: &str) -> Request<()> {
        let listener = tokio::net::TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
            .await
            .unwrap();
        let addr = listener.local_addr().unwrap();
        let db_pool = setup_db("sqlite::memory:", None, None).await.unwrap();
        tokio::spawn(
            axum::serve(
                listener,
                new_app(db_pool)
                    .unwrap()
                    .into_make_service_with_connect_info::<SocketAddr>(),
            )
            .into_future(),
        );

        // "Generate a random 16-byte nonce"
        let nonce: [u8; 16] = rand::thread_rng().gen();
        let sec_websocket_key = base64::engine::general_purpose::STANDARD.encode(&nonce);

        let token = crate::api_authorize_jwt::tests::generate_token(username);

        let request = Request::builder()
            .uri(format!("ws://{addr}/ws?token={token}"))
            .header("Host", "127.0.0.1")
            .header("Upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header(
                tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL,
                websocket_protocol,
            )
            .header(
                tungstenite::http::header::SEC_WEBSOCKET_KEY,
                sec_websocket_key,
            )
            // `Sec-WebSocket-Version` header did not include '13'
            // "The most recent final version of the WebSocket protocol is version 13."
            // cf https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism#version
            .header(tungstenite::http::header::SEC_WEBSOCKET_VERSION, "13")
            .body(())
            .unwrap();

        request
    }

    #[tokio::test]
    async fn test_handle_socket_chat() {
        let username = "aaa";

        let request = setup("chat", username).await;
        let (mut socket, _response) = tokio_tungstenite::connect_async(request).await.unwrap();

        socket
            .send(tungstenite::Message::Text("hello world".to_string()))
            .await
            .expect("Failed to write WebSocket request");

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, format!("{username} joined."));

        socket
            .send(tungstenite::Message::Text("does not matter".to_string()))
            .await
            .expect("Failed to write WebSocket request");

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, format!("{username}: hello world"));
    }

    // NOTE: for now the logic is exactly the same as `handle_socket_chat`
    #[tokio::test]
    async fn test_handle_socket_geolocation() {
        let username = "aaa";

        let request = setup("geolocation", username).await;
        let (mut socket, _response) = tokio_tungstenite::connect_async(request).await.unwrap();

        socket
            .send(tungstenite::Message::Text("hello world".to_string()))
            .await
            .expect("Failed to write WebSocket request");

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, format!("{username} joined."));

        socket
            .send(tungstenite::Message::Text("does not matter".to_string()))
            .await
            .expect("Failed to write WebSocket request");

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, format!("{username}: hello world"));
    }
}
