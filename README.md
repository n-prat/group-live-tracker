# Group Live tracker

Server-less (ie a static html) site displaying the live location of all users connected to a room.
FAIL: not really possible using WebRTC. It is only meant for P2P, and can not scale to "chat room" level with 50-100 peers.

No database, no history, no storing, etc

Credits: https://chat.openai.com
Based on https://github.com/codec-abc/Yew-WebRTC-Chat
Based on https://github.com/rksm/axum-yew-setup/

## TODO

- https://yew.rs/docs/next/concepts/function-components/communication
  - When receiving a message that is lat/lon, it should update the map
- the map SHOULD display all the locations for all users
- leaflet: ask for user's location
- https://github.com/codec-abc/Yew-WebRTC-Chat : make joining automatic; AVOID copy-pasting codes around
- see https://github.com/nag763/tchatchers for templates/styles/etc
- see https://github.com/tokio-rs/axum/tree/main/examples/chat
- Websocket: MAYBE TRY https://github.com/najamelan/ws_stream_wasm
- AND/OR https://crates.io/crates/yew-websocket ? (but possibly not compatible with latest yew?)
- related ???
  - https://github.com/snapview/tokio-tungstenite/issues/278
  - https://github.com/tokio-rs/axum/issues/1961

## DEV/local test

- `openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365`
  - NOTE: https is needed b/c modern browser DO NOT allow location access in HTTP
- `./dev.sh`
  - NOTE: both the frontend and backend are started separately so CHECK the logs/terminal for compilation errors!

see also https://github.com/trunk-rs/trunk?tab=readme-ov-file#getting-started
