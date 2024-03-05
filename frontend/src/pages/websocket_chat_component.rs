/// `https://chat.openai.com`
/// See also `https://github.com/jetli/yew-hooks/blob/e31debde4ce3c8c524c56303255baa833a0f0b79/crates/yew-hooks/src/hooks/use_websocket.rs#L163`
// TODO maybe switch to tungstenite cf https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs
// b/c the whole "initial delay" sucks...
// https://github.com/snapview/tokio-tungstenite/issues/278 related ?
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::use_store;

use crate::{app::WS_ROOT, store::PersistentStore};

#[function_component(WebSocketChatComponent)]
pub(crate) fn websocket_chat_component() -> Html {
    let history = use_list(vec![]);
    let (store, _dispatch) = use_store::<PersistentStore>();
    let token = store.token.clone().unwrap_or_default();

    // TODO?
    // if auth_user.is_none() {
    //     return html! {<LoginPage />};
    // }

    let ws = {
        let history = history.clone();
        let ws_handle: UseWebSocketHandle = use_websocket_with_options(
            format!("{WS_ROOT}?token={token}",),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("[recv]: {message}",));
                })),
                manual: Some(false),
                protocols: Some(vec!["chat".to_string()]),
                ..Default::default()
            },
        );

        // TODO set header eg "Authorization Bearer"?
        // let my_websocket: web_sys::WebSocket = TODO;
        // if let Some(ref ws) = *ws_handle {
        //     let my_websocket: web_sys::WebSocket = unsafe { std::mem::transmute(ws) };
        //     // Now you can use my_websocket
        // }
        // if let Some(ref ws) = *ws_handle.ws.borrow() {
        //     let my_websocket: web_sys::WebSocket = ws.clone();
        //     // Now you can use my_websocket
        // }

        ws_handle
    };
    let onclick = {
        let ws = ws.clone();
        // let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, world!".to_string();
            ws.send(message.clone());
            // history.push(format!("[send]: {}", message));
        })
    };
    let onopen = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.open();
        })
    };

    // *ws.;

    html! {
        <>
        // <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
        //     <div class="w-full">

                <h1>{"WebSocket Messages:"}</h1>

        //     </div>
        // </section>


        <div class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4">
        // <div class="shrink-0">
        //     <img class="h-12 w-12" src="/img/logo.svg" alt="ChitChat Logo">
        // </div>
            <div>
                <button class="bg-violet-500 hover:bg-violet-600 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 disabled:opacity-75" onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect" }</button>
                <button class="bg-violet-500 hover:bg-violet-600 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 disabled:opacity-75" {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send" }</button>
                <p class="text-slate-500">{ "WebSocket Messages:" }</p>
                <ul>
                    { for history.current().iter().map(|message| html! { <p class="text-slate-500">{ message }</p> }) }
                </ul>
            </div>
        </div>

        </>
    }
}

// pub struct WebSocketChatComponent {
//     // link: ComponentLink<Self>,
//     ws: WebSocket,
//     messages: Vec<String>,
//     ws_is_ready: bool,
// }

// pub enum Msg {
//     WebSocketMessage(String),
//     SendMessage,
//     WebSocketReady,
//     WebSocketClosed,
//     WebSocketErrored,
// }

// impl WebSocketChatComponent {
//     fn new_websocket(ctx: &Context<Self>) -> WebSocket {
//         let on_message_callback = ctx.link().callback(|event: MessageEvent| {
//             let msg = event.data().as_string().unwrap(); // Handle potential errors here
//             Msg::WebSocketMessage(msg)
//         });
//         let on_open_callback = ctx
//             .link()
//             .callback(|_event: MessageEvent| Msg::WebSocketReady);
//         let on_close_callback = ctx
//             .link()
//             .callback(|_event: MessageEvent| Msg::WebSocketClosed);
//         let on_error_callback = ctx
//             .link()
//             .callback(|_event: MessageEvent| Msg::WebSocketErrored);
//         let ws = new_websocket(
//             "chat",
//             on_message_callback,
//             on_open_callback,
//             on_close_callback,
//             on_error_callback,
//         );

//         ws
//     }
// }

// impl Component for WebSocketChatComponent {
//     type Message = Msg;
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         console::log_1(&"WebSocketChatComponent create".into());

//         let ws = Self::new_websocket(ctx);

//         Self {
//             // link,
//             ws,
//             messages: Vec::new(),
//             ws_is_ready: false,
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::WebSocketMessage(message) => {
//                 self.messages.push(message);
//                 true // Re-render
//             }
//             Msg::SendMessage => {
//                 if self.ws_is_ready {
//                     self.ws
//                         .clone()
//                         .send_with_str("Hello from Yew update")
//                         .expect("update send_with_str failed");
//                 } else {
//                     console::log_1(&"WebSocketChatComponent update: websocket not ready".into());
//                 }

//                 false // No re-render needed after sending message
//             }
//             Msg::WebSocketReady => {
//                 self.ws_is_ready = true;

//                 // this will be used as "username" cf server/src/ws_handler.rs:L77
//                 let username = get_username_from_context(ctx);
//                 console::log_1(
//                     &format!("WebSocketChatComponent Msg::WebSocketReady username: {username:?}")
//                         .into(),
//                 );
//                 match username {
//                     Some(username) => self
//                         .ws
//                         .clone()
//                         .send_with_str(&username)
//                         .expect("update send_with_str failed"),
//                     None => {}
//                 };

//                 false
//             }
//             Msg::WebSocketClosed => {
//                 console::log_1(&"WebSocketChatComponent update: WebSocketClosed".into());
//                 self.ws_is_ready = false;
//                 // This will in practice retry until it works; NO NEED for a while/loop etc
//                 // if the server is still down it will again reach `WebSocketErrored` then `WebSocketClosed`
//                 // which will in turn call `new_websocket`
//                 self.ws = Self::new_websocket(ctx);
//                 false
//             }
//             Msg::WebSocketErrored => {
//                 console::log_1(&"WebSocketChatComponent update: WebSocketErrored".into());
//                 false
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <h1>{"WebSocket Messages:"}</h1>
//                 <ul>
//                     { for self.messages.iter().map(|msg| html! { <li>{ msg }</li> }) }
//                 </ul>

//                 <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{ "Send Message" }</button>
//             </div>
//         }
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
//         // FAIL
//         //         panicked at frontend/src/websocket_component.rs:72:14:
//         // rendered send_with_str failed: JsValue(InvalidStateError: An attempt was made to use an object that is not, or is no longer, usable
//         // __wbg_get_imports/imports.wbg.__wbg_send_115b7e92eb793bd9/<@http://localhost:8080/frontend-5051f9a5cc4539c3.js:632:25
//         // self.ws
//         //     .send_with_str("hello from rendered")
//         //     .expect("rendered send_with_str failed");

//         console::log_1(&"WebSocketChatComponent rendered: done!".into());
//     }
// }
