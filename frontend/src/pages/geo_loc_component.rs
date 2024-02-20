use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::console::{self};
use web_sys::PositionOptions;
use yew::prelude::*;

#[function_component]
pub(crate) fn GeoLocComponent() -> Html {
    console::log_1(&"GeoLocComponent: rendered start".into());
    request_geolocation();

    html! {}
}

/// `https://chat.openai.com`
// TODO MAYBE use yew-hooks "use_location"?
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
        console::log_1(&format!("Latitude: {latitude}, Longitude: {longitude}",).into());
    }) as Box<dyn FnMut(web_sys::Position)>);

    // Set up a callback function for geolocation retrieval errors
    let error_callback = Closure::wrap(Box::new(|error| {
        // Handle geolocation retrieval errors
        console::error_1(&format!("Error getting geolocation: {error:?}",).into());
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
                console::error_1(&format!("Error watching geolocation: {error:?}",).into());
            }

            // Prevent the callbacks from being dropped prematurely
            success_callback.forget();
            error_callback.forget();
        }
        Err(err) => {
            console::error_1(&format!("Failed to cast to Geolocation: {err:?}",).into());
        }
    };
}
