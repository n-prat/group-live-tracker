use leaflet::LatLng;
use web_sys::console;
/// `https://chat.openai.com`
/// See also https://github.com/jetli/yew-hooks/blob/e31debde4ce3c8c524c56303255baa833a0f0b79/crates/yew-hooks/src/hooks/use_websocket.rs#L163
// TODO maybe switch to tungstenite cf https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs
// b/c the whole "initial delay" sucks...
// https://github.com/snapview/tokio-tungstenite/issues/278 related ?
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::use_store;

use crate::{app::WS_ROOT, store::PersistentStore};

#[function_component(WebSocketGeoLocComponent)]
pub(crate) fn websocket_geolocation_component() -> Html {
    let history = use_list(vec![]);

    let (store, _dispatch) = use_store::<PersistentStore>();
    let token = store.token.clone().unwrap_or_default();

    // Create a state for the geolocation status
    let geolocation_state = use_state(|| None);

    // Use the geolocation hook
    // Options for retrieving geolocation position
    let mut position_options = web_sys::PositionOptions::new();
    position_options.enable_high_accuracy(true);
    position_options.timeout(10_000); // Timeout in milliseconds
    let geolocation = use_geolocation_with_options(position_options);

    // Clone the state handle to move it into the closure
    let geolocation_state_clone = geolocation_state.clone();

    // TODO?
    // if auth_user.is_none() {
    //     return html! {<LoginPage />};
    // }

    let ws = {
        // let history = history.clone();
        use_websocket_with_options(
            format!("{}?token={}", WS_ROOT, token),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("[recv]: {}", message));
                    console::log_1(
                        &format!("WebSocketGeoLocComponent: [recv]: {}", message).into(),
                    );
                })),
                manual: Some(false),
                protocols: Some(vec!["geolocation".to_string()]),
                ..Default::default()
            },
        )
    };

    // let onclick = {
    //     let ws = ws.clone();
    //     // let history = history.clone();
    //     Callback::from(move |_| {
    //         let message = "Hello, world!".to_string();
    //         ws.send(message.clone());
    //         // history.push(format!("[send]: {}", message));
    //     })
    // };
    // let onopen = {
    //     let ws = ws.clone();
    //     Callback::from(move |_| {
    //         ws.open();
    //     })
    // };

    // Use the effect hook to perform side effects when the geolocation state changes
    use_effect_with((geolocation.clone(),), move |(geolocation,)| {
        // Perform side effects when the position changes
        geolocation_state_clone.set(Some(LatLng::new(
            geolocation.latitude,
            geolocation.longitude,
        )));
        console::log_1(
            &format!(
                "websocket_geolocation_component: {} {}",
                geolocation.latitude, geolocation.longitude
            )
            .into(),
        );

        let message = format!("{},{}", geolocation.latitude, geolocation.longitude);
        ws.send(message.clone());
        // Return an effect cleanup function if needed
        || {}
    });

    html! {
        <>
        // <div class="w-full">
        //     <p>
        //         <button onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect" }</button>
        //         <button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send with options" }</button>
        //     </p>
        //     <h1>{"WebSocket Messages:"}</h1>
        //     <ul>
        //         { for history.current().iter().map(|message| html! { <p>{ message }</p> }) }
        //     </ul>
        // </div>
        </>
    }
}

// pub struct WebSocketGeoLocComponent {
//     // link: ComponentLink<Self>,
//     ws: WebSocket,
//     ws_is_ready: bool,
// }

// pub enum Msg {
//     WebSocketMessage(String),
//     SendMessage,
//     WebSocketReady,
//     WebSocketClosed,
//     WebSocketErrored,
// }

// impl WebSocketGeoLocComponent {
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
//             "geolocation",
//             on_message_callback,
//             on_open_callback,
//             on_close_callback,
//             on_error_callback,
//         );

//         ws
//     }
// }

// impl Component for WebSocketGeoLocComponent {
//     type Message = Msg;
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         console::log_1(&"WebSocketGeoLocComponent create".into());

//         let ws = Self::new_websocket(ctx);

//         Self {
//             // link,
//             ws,
//             // messages: Vec::new(),
//             ws_is_ready: false,
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::WebSocketMessage(message) => {
//                 // self.messages.push(message);
//                 console::log_1(&format!("WebSocketGeoLocComponent update: {message}").into());
//                 false // DO NOT Re-render
//             }
//             Msg::SendMessage => {
//                 if self.ws_is_ready {
//                     self.ws
//                         .clone()
//                         .send_with_str("TODO WebSocketGeoLocComponent Location")
//                         .expect("update send_with_str failed");
//                 } else {
//                     console::log_1(&"WebSocketGeoLocComponent update: websocket not ready".into());
//                 }

//                 false // No re-render needed after sending message
//             }
//             Msg::WebSocketReady => {
//                 self.ws_is_ready = true;

//                 // this will be used as "username" cf server/src/ws_handler.rs:L77
//                 let username = get_username_from_context(ctx);
//                 console::log_1(
//                     &format!("WebSocketGeoLocComponent Msg::WebSocketReady username: {username:?}")
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
//                 console::log_1(&"WebSocketGeoLocComponent update: WebSocketClosed".into());
//                 self.ws_is_ready = false;
//                 // This will in practice retry until it works; NO NEED for a while/loop etc
//                 // if the server is still down it will again reach `WebSocketErrored` then `WebSocketClosed`
//                 // which will in turn call `new_websocket`
//                 self.ws = Self::new_websocket(ctx);
//                 false
//             }
//             Msg::WebSocketErrored => {
//                 console::log_1(&"WebSocketGeoLocComponent update: WebSocketErrored".into());
//                 false
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{ "TEST SEND Location Message" }</button>
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

//         console::log_1(&"WebSocketGeoLocComponent rendered: done!".into());
//     }
// }
