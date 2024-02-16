//! https://chat.openai.com
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, MessageEvent, WebSocket};
use yew::prelude::*;

pub struct WebSocketComponent {
    // link: ComponentLink<Self>,
    ws: WebSocket,
    messages: Vec<String>,
}

pub enum Msg {
    WebSocketMessage(String),
    // Add other message variants as needed
}

impl Component for WebSocketComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ws = WebSocket::new("ws://localhost:8081/ws").expect("WebSocket creation failed");
        let on_message_callback = ctx.link().callback(|event: MessageEvent| {
            let msg = event.data().as_string().unwrap(); // Handle potential errors here
            Msg::WebSocketMessage(msg)
        });

        let on_message_callback_rc = Rc::new(on_message_callback);
        let on_message_closure = Closure::wrap(Box::new(move |event: MessageEvent| {
            let callback = on_message_callback_rc.clone();
            let result = callback.emit(event);
            // if let Err(err) = result {
            //     log::error!("Failed to process WebSocket message event: {:?}", err);
            // }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        on_message_closure.forget(); // Ensure closure is not dropped prematurely

        Self {
            // link,
            ws,
            messages: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WebSocketMessage(message) => {
                self.messages.push(message);
                true // Re-render
            } // Handle other message variants here
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"WebSocket Messages:"}</h1>
                <ul>
                    { for self.messages.iter().map(|msg| html! { <li>{ msg }</li> }) }
                </ul>
            </div>
        }
    }
}
