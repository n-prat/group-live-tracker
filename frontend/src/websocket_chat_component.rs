//! `https://chat.openai.com`
use web_sys::console::{self};
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;

use crate::websockets_common;

// TODO maybe switch to tungstenite cf https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs
// b/c the whole "initial delay" sucks...
// https://github.com/snapview/tokio-tungstenite/issues/278 related ?

pub struct WebSocketChatComponent {
    // link: ComponentLink<Self>,
    ws: WebSocket,
    messages: Vec<String>,
    ws_is_ready: bool,
}

pub enum Msg {
    WebSocketMessage(String),
    SendMessage,
    WebSocketReady,
    WebSocketClosed,
    WebSocketErrored,
}

impl Component for WebSocketChatComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let on_message_callback = ctx.link().callback(|event: MessageEvent| {
            let msg = event.data().as_string().unwrap(); // Handle potential errors here
            Msg::WebSocketMessage(msg)
        });
        let on_open_callback = ctx
            .link()
            .callback(|_event: MessageEvent| Msg::WebSocketReady);
        let on_close_callback = ctx
            .link()
            .callback(|_event: MessageEvent| Msg::WebSocketClosed);
        let on_error_callback = ctx
            .link()
            .callback(|_event: MessageEvent| Msg::WebSocketErrored);
        let ws = websockets_common::new_websocket(
            "chat",
            on_message_callback,
            on_open_callback,
            on_close_callback,
            on_error_callback,
        );

        console::log_1(&"WebSocketChatComponent create: done!".into());

        Self {
            // link,
            ws,
            messages: Vec::new(),
            ws_is_ready: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WebSocketMessage(message) => {
                self.messages.push(message);
                true // Re-render
            }
            Msg::SendMessage => {
                if self.ws_is_ready {
                    self.ws
                        .clone()
                        .send_with_str("Hello from Yew update")
                        .expect("update send_with_str failed");
                } else {
                    console::log_1(&"WebSocketChatComponent update: websocket not ready".into());
                }

                false // No re-render needed after sending message
            }
            Msg::WebSocketReady => {
                self.ws_is_ready = true;
                false
            }
            Msg::WebSocketClosed => {
                console::log_1(&"WebSocketChatComponent update: WebSocketClosed".into());
                self.ws_is_ready = false;
                false
            }
            Msg::WebSocketErrored => {
                console::log_1(&"WebSocketChatComponent update: WebSocketErrored".into());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"WebSocket Messages:"}</h1>
                <ul>
                    { for self.messages.iter().map(|msg| html! { <li>{ msg }</li> }) }
                </ul>

                <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{ "Send Message" }</button>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // FAIL
        //         panicked at frontend/src/websocket_component.rs:72:14:
        // rendered send_with_str failed: JsValue(InvalidStateError: An attempt was made to use an object that is not, or is no longer, usable
        // __wbg_get_imports/imports.wbg.__wbg_send_115b7e92eb793bd9/<@http://localhost:8080/frontend-5051f9a5cc4539c3.js:632:25
        // self.ws
        //     .send_with_str("hello from rendered")
        //     .expect("rendered send_with_str failed");

        console::log_1(&"WebSocketChatComponent rendered: done!".into());
    }
}
