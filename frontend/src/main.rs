// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
// #![warn(clippy::expect_used)]
// #![warn(clippy::panic)]
// #![warn(clippy::unwrap_used)]

use yew::prelude::*;
use yew_router::prelude::*;

mod geo_loc_component;
mod map_component;
mod websocket_chat_component;
mod websocket_geoloc_component;
mod websockets_common;

use crate::geo_loc_component::GeoLocComponent;
use crate::map_component::MapComponent;
use crate::websocket_chat_component::WebSocketChatComponent;
use crate::websocket_geoloc_component::WebSocketGeoLocComponent;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    // #[at("/hello-server")]
    // HelloServer,
}

#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        // Route::HelloServer => html! { <HelloServer /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            <MapComponent markers={vec![]}/>
            <WebSocketChatComponent />
            <GeoLocComponent />
            <WebSocketGeoLocComponent />
        </BrowserRouter>

    }
}

// #[function_component(HelloServer)]
// fn hello_server() -> Html {
//     let data = use_state(|| None);

//     // Request `/api/hello` once
//     {
//         let data = data.clone();
//         use_effect(move || {
//             if data.is_none() {
//                 spawn_local(async move {
//                     let resp = Request::get("/api/hello").send().await.unwrap();
//                     let result = {
//                         if !resp.ok() {
//                             Err(format!(
//                                 "Error fetching data {} ({})",
//                                 resp.status(),
//                                 resp.status_text()
//                             ))
//                         } else {
//                             resp.text().await.map_err(|err| err.to_string())
//                         }
//                     };
//                     data.set(Some(result));
//                 });
//             }

//             || {}
//         });
//     }

//     match data.as_ref() {
//         None => {
//             html! {
//                 <div>{"No server response"}</div>
//             }
//         }
//         Some(Ok(data)) => {
//             html! {
//                 <div>{"Got server response: "}{data}</div>
//             }
//         }
//         Some(Err(err)) => {
//             html! {
//                 <div>{"Error requesting data from server: "}{err}</div>
//             }
//         }
//     }
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
