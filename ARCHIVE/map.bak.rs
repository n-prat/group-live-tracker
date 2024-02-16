use gloo_utils::format::JsValueSerdeExt;
use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::HtmlElement;
use web_sys::Window;
use yew::prelude::*;

pub struct Map {
    marker_layer: Option<JsValue>, // Hold the reference to the marker layer
}

impl Component for Map {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Initialize the map here
        // You can use JavaScript interop to create the map
        // Initialize the marker layer as None initially
        Map { marker_layer: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Handle updates to the map, if necessary
        true // Return true to force re-rendering
    }

    // fn changed(&mut self, ctx: &Context<Self>, _: Self::Properties) -> bool {
    //     // Handle changes to properties, if any
    //     true // Return true to force re-rendering
    // }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        log::warn!("rendered : {:?}", first_render);
        println!("rendered");
        console::log_1(&"rendered".into());

        // Initialize the map after the component has been mounted
        let map_container = document().get_element_by_id("map").unwrap();
        let map_container: HtmlElement = map_container.dyn_into().unwrap();
        console::log_1(&"map_container".into());

        // Import the Leaflet library from JavaScript
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();
        log::warn!("leaflet : {:?}", leaflet);
        console::log_1(&leaflet);

        // Initialize the map using Leaflet
        let map = Reflect::get(&leaflet, &JsValue::from_str("map")).unwrap();
        // console::log_1(&"map1".into());

        let map = map
            .dyn_ref::<js_sys::Function>()
            .unwrap()
            .call1(&JsValue::NULL, &JsValue::from(map_container))
            .unwrap();
        console::log_1(&"map2".into());
        console::log_1(&map);

        let map = Reflect::get(&map, &JsValue::from_str("setView")).unwrap();
        console::log_1(&"map3".into());
        console::log_1(&map);
        let set_view_function = map.dyn_ref::<js_sys::Function>().unwrap();

        //     .call3(
        //         &JsValue::NULL,
        //         &JsValue::from_f64(51.505),
        //         &JsValue::from_f64(-0.09),
        //         &JsValue::from_f64(13.0),
        //     )
        //     .unwrap();
        // console::log_1(&"map4".into());

        // Get a reference to the setView function
        // let set_view_function = map
        //     .dyn_ref::<Window>()
        //     .unwrap()
        //     .get("L")
        //     .unwrap()
        //     .dyn_ref::<js_sys::Object>()
        //     .unwrap()
        //     .get("map")
        //     .unwrap()
        //     .get("setView")
        //     .unwrap()
        //     .dyn_ref::<js_sys::Function>()
        //     .unwrap();

        // Define the arguments to pass to setView
        let lat_lon_arr = js_sys::Array::new();
        lat_lon_arr.push(&JsValue::from(51.505)); // Push latitude
        lat_lon_arr.push(&JsValue::from(-0.09)); // Push longitude

        // Call the setView function
        // Zoom level
        set_view_function
            // .call2(&map, &lat_lon_arr, &JsValue::from(13))
            .call3(&map, &lat_lon_arr, &JsValue::from(13), &JsValue::null())
            // .call1(&map, &lat_lon_arr)
            .unwrap();
        // set_view_function
        //     .call3(&map, &args.get(0).unwrap(), &args.get(1).unwrap())
        //     .unwrap();

        // Add the tile layer to the map
        let tile_layer = Reflect::get(&leaflet, &JsValue::from_str("tileLayer")).unwrap();
        let tile_layer = tile_layer
            .dyn_ref::<js_sys::Function>()
            .unwrap()
            .call1(
                &JsValue::NULL,
                &JsValue::from_str("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"),
            )
            .unwrap();
        Reflect::get(&tile_layer, &JsValue::from_str("addTo"))
            .unwrap()
            .dyn_ref::<js_sys::Function>()
            .unwrap()
            .call1(&JsValue::NULL, &map)
            .unwrap();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            // Add a placeholder for the map
            // <div id="map"></div>
        }
    }
}

// Define a struct to represent location data
#[derive(Serialize, Deserialize)]
struct Location {
    user_id: String,
    latitude: f64,
    longitude: f64,
}

// WebSocket communication logic
// Implement WebSocket communication to send and receive location data

// Function to send location data to the server via WebSocket
fn send_location_data(location: Location) {
    // Send the location data to the server via WebSocket
}

// Function to receive location data from the server via WebSocket
fn receive_location_data() -> Vec<Location> {
    // Receive location data from the server via WebSocket
    // Return the received location data
    todo!("receive_location_data");
}

// Update the map component with location data
fn update_map_with_location_data(map: &Map, locations: Vec<Location>) {
    // Update the map with the received location data
    // You can use Leaflet.js to add/update markers on the map based on the location data
}

fn document() -> web_sys::Document {
    web_sys::window()
        .unwrap()
        .document()
        .expect("Failed to get document")
}

fn window() -> Window {
    web_sys::window().expect("Failed to get window")
}
