use std::ops::Add;

/// https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs
use gloo_utils::document;
use leaflet::{CircleMarker, LatLng, Map, MapOptions, TileLayer};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console::{self, log};
use web_sys::js_sys::Reflect;
use web_sys::{Element, HtmlElement, Node};
use web_sys::{Geolocation, PositionOptions};
use yew::Properties;
use yew::{html::ImplicitClone, prelude::*};

pub enum Msg {
    RequestLocation,
}

pub struct GeoLocComponent {}

impl Component for GeoLocComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        console::log_1(&format!("GeoLocComponent: rendered start").into());
        request_geolocation();
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::RequestLocation)}>{ "Request Location" }</button>
            </div>
        }
    }
}

/// https://chat.openai.com
fn request_geolocation() {
    // Define options for geolocation request
    // let options = GeolocationOptions::new()
    //     .enable_high_accuracy()
    //     .timeout(10_000); // Timeout in milliseconds

    // Request geolocation

    // Set up a callback function for successful geolocation retrieval
    let success_callback = Closure::wrap(Box::new(|position: web_sys::Position| {
        // Handle the retrieved geolocation position
        let latitude = position.coords().latitude();
        let longitude = position.coords().longitude();
        console::log_1(&format!("Latitude: {}, Longitude: {}", latitude, longitude).into());
    }) as Box<dyn FnMut(web_sys::Position)>);

    // Set up a callback function for geolocation retrieval errors
    let error_callback = Closure::wrap(Box::new(|error| {
        // Handle geolocation retrieval errors
        console::error_1(&format!("Error getting geolocation: {:?}", error).into());
    }) as Box<dyn FnMut(web_sys::PositionError)>);

    // Options for retrieving geolocation position
    let mut position_options = PositionOptions::new();
    position_options.enable_high_accuracy(true);
    position_options.timeout(10_000); // Timeout in milliseconds

    // Request geolocation position
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    // let navigator = Reflect::get(&window, &JsValue::from_str("navigator")).unwrap();
    // Use Reflect to get the navigator object from the window
    // let navigator = Reflect::get(&window, &"navigator".into())
    //     .unwrap()
    //     .dyn_into::<web_sys::Navigator>()
    //     .unwrap();
    console::log_1(&navigator);
    // let geolocation = Reflect::get(&navigator, &JsValue::from_str("geolocation")).unwrap();
    let geolocation = navigator.geolocation();

    // Convert the JsValue to Geolocation using JsCast::dyn_into
    // match geolocation.dyn_into::<Geolocation>() {
    match geolocation {
        Ok(geolocation) => {
            console::log_1(&"Geolocation is supported!".into());
            // Now you can use the Geolocation object

            if let Err(error) = geolocation.watch_position_with_error_callback_and_options(
                success_callback.as_ref().unchecked_ref(),
                Some(error_callback.as_ref().unchecked_ref()),
                &position_options,
            ) {
                // Handle error while watching geolocation
                console::error_1(&format!("Error watching geolocation: {:?}", error).into());
            }

            // Prevent the callbacks from being dropped prematurely
            success_callback.forget();
            error_callback.forget();
        }
        Err(err) => {
            console::error_1(&format!("Failed to cast to Geolocation: {:?}", err).into());
        }
    };
}
