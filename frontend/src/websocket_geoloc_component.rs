//! `https://chat.openai.com`
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console::{self};
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;

// TODO maybe switch to tungstenite cf https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs
// b/c the whole "initial delay" sucks...
// https://github.com/snapview/tokio-tungstenite/issues/278 related ?

pub struct WebSocketGeoLocComponent {
    // link: ComponentLink<Self>,
    ws: WebSocket,
    ws_is_ready: bool,
}

pub enum Msg {
    WebSocketMessage(String),
    SendMessage,
    WebSocketReady,
}

impl Component for WebSocketGeoLocComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let protocols = js_sys::Array::new();
        protocols.push(&JsValue::from("geolocation"));
        // let ws = WebSocket::new("ws://localhost:8081/ws")
        //     .expect(" WebSocket::new_with_str_sequence failed!");
        let ws = WebSocket::new_with_str_sequence("ws://localhost:8081/ws", &protocols)
            .expect(" WebSocket::new_with_str_sequence failed!");

        let on_message_callback = ctx.link().callback(|event: MessageEvent| {
            let msg = event.data().as_string().unwrap(); // Handle potential errors here
            Msg::WebSocketMessage(msg)
        });

        let on_message_callback_rc = Rc::new(on_message_callback);
        let on_message_closure = Closure::wrap(Box::new(move |event: MessageEvent| {
            let callback = on_message_callback_rc.clone();
            callback.emit(event);
            // if let Err(err) = result {
            //     log::error!("Failed to process WebSocket message event: {:?}", err);
            // }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.clone()
            .set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        on_message_closure.forget(); // Ensure closure is not dropped prematurely

        // MUST wait for the cx to be ready ???
        // ws.send_with_str("hello from rendered")
        //     .expect("rendered send_with_str failed");
        let on_open_callback = ctx
            .link()
            .callback(|_event: MessageEvent| Msg::WebSocketReady);
        let on_open_callback_rc = Rc::new(on_open_callback);
        let on_open_closure: Closure<dyn FnMut(MessageEvent)> =
            Closure::wrap(Box::new(move |event: MessageEvent| {
                let callback = on_open_callback_rc.clone();
                callback.emit(event);
                // if let Err(err) = result {
                //     log::error!("Failed to process WebSocket message event: {:?}", err);
                // }
            }) as Box<dyn FnMut(MessageEvent)>);
        ws.clone()
            .set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));
        on_open_closure.forget();

        console::log_1(&"WebSocketGeoLocComponent create: done!".into());

        Self {
            // link,
            ws,
            // messages: Vec::new(),
            ws_is_ready: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WebSocketMessage(message) => {
                // self.messages.push(message);
                console::log_1(&format!("WebSocketGeoLocComponent update: {message}").into());
                false // DO NOT Re-render
            }
            Msg::SendMessage => {
                if self.ws_is_ready {
                    self.ws
                        .clone()
                        .send_with_str("TODO WebSocketGeoLocComponent Location")
                        .expect("update send_with_str failed");
                } else {
                    console::log_1(&"WebSocketGeoLocComponent update: websocket not ready".into());
                }

                false // No re-render needed after sending message
            }
            Msg::WebSocketReady => {
                self.ws_is_ready = true;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{ "TEST SEND Location Message" }</button>
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

        console::log_1(&"WebSocketGeoLocComponent rendered: done!".into());
    }
}
