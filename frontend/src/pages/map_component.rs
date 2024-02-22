/// `https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs`
use gloo_utils::document;
use js_sys::Array;
use leaflet::{Circle, LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use leaflet::{Tooltip, TooltipOptions};
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlElement, Node};
use yew::prelude::*;
use yewdux::use_store;

use crate::store::Store;

const PARIS_LAT: f64 = 48.866_667;
const PARIS_LNG: f64 = 2.333_333;

#[function_component(MapComponent)]
pub(crate) fn map_component() -> Html {
    let (store, _dispatch) = use_store::<Store>();
    let leaflet_map_state = use_state(|| None);

    // "Provide a empty tuple `()` as dependencies when you need to do something only on the first render of a component."
    // let container_clone = container.clone();
    let leaflet_map_state_clone = leaflet_map_state.clone();
    use_effect_with((), move |_| {
        // let container: Element = document().create_element("div").unwrap();
        // let container: HtmlElement = container.dyn_into().unwrap();
        // container.set_class_name("map h-full");
        // container.set_id("map-container");

        let container: HtmlElement = document()
            .get_element_by_id("map-container")
            .unwrap()
            .dyn_into()
            .unwrap();

        // let leaflet_map = Map::new("map-container", &MapOptions::default());
        let leaflet_map = Map::new_with_element(&container, &MapOptions::default());

        leaflet_map.set_view(&LatLng::new(PARIS_LAT, PARIS_LNG), 11.0);
        add_tile_layer(&leaflet_map);

        add_geojson_trace(&leaflet_map);

        leaflet_map_state_clone.set(Some(leaflet_map));
    });

    // NOT the the first render: retrieve the existing map from the document
    // FAIL: get_element_by_id works but no way to cast it into "Map"???
    // match document().get_element_by_id("map-container") {
    //     Some(container) => {
    //         let container_clone = container.clone();
    //         match container.dyn_into::<Map>() {
    //             Ok(leaflet_map) => {
    //                 // For example, update the circles on the map
    //                 for (username, (lat, lng)) in &store.locations {
    //                     add_circle_with_options(&leaflet_map, *lat, *lng, username);
    //                 }
    //             }
    //             Err(err) => {
    //                 console::log_1(&"MapComponent: map-container can not be cast to Map".into());
    //                 console::log_1(&container_clone);
    //             }
    //         }
    //     }
    //     None => {
    //         console::log_1(&"MapComponent: map-container not found".into());
    //     }
    // }
    //
    // So use a State instead
    match leaflet_map_state.as_ref() {
        Some(leaflet_map) => {
            console::log_1(&"MapComponent: leaflet_map_state ready".into());

            // For example, update the circles on the map
            for (username, (lat, lng)) in &store.locations {
                add_circle_with_options(leaflet_map, *lat, *lng, username);
            }
        }
        None => {
            console::log_1(&"MapComponent: leaflet_map_state NOT ready".into());
        }
    }

    // NOTE??? apparently no need to do anything
    // and DO NOT try: we WANT the map to stay and NOT refresh; we only want markers to be added inside the Map!
    html! {
        // <div id="map" class="map-container component-container">
        //     {render_map(&container)}
        // </div>

        // <section class="bg-ct-blue-600 min-h-screen pt-20">
        //  max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center
            // <div class="h-full">
                // <div id="map">
                    // {render_map(&container)}
                // </div>
            // </div>
        // </section>
    }
}

/// https://github.com/slowtec/leaflet-rs/blob/09d02e74bc30d519a5a30bb130516aa161f0415a/examples/basic/src/lib.rs#L76
fn add_circle_with_options(map: &Map, lat: f64, lng: f64, username: &str) {
    console::log_1(
        &format!(
            "MapComponent: add_circle_with_options username: {}",
            username
        )
        .into(),
    );
    let lat_lng = LatLng::new(lat, lng);

    let options = leaflet::CircleOptions::default();
    let circle = Circle::new_with_options(&lat_lng, &options);
    // circle.set_style(&PathOptions::default());

    let tooltip_options: TooltipOptions = TooltipOptions::new();
    tooltip_options.set_permanent(true);
    let tooltip = Tooltip::new_with_lat_lng(&lat_lng, &tooltip_options);
    tooltip.set_content(&JsValue::from_str(username));
    circle.bind_tooltip(&tooltip);
    // circle.bind_tooltip(&Tooltip::new(&TooltipOptions::new(), map.lag));
    circle.add_to(map);
}

/// Add a .gpx (GeoJSON) track on the map
fn add_geojson_trace(map: &Map) {
    // Parse the GeoJSON string into a serde_json::Value
    let geojson_string: Value = serde_json::from_str(include_str!(
        "../../../server/tests/data/2024-02-19_1444960792_MJ 19_02.geojson"
    ))
    .unwrap();

    let lines = &geojson_string["geometries"][0]["coordinates"][0];
    console::log_1(&format!("MapComponent: add_geojson_trace: lines: {}", lines,).into());

    let latlngs = lines
        .as_array()
        .unwrap()
        .iter()
        .map(|line| {
            let arr = line.as_array().unwrap();
            let lng = arr[0].as_f64().expect("arr[0].as_f64");
            let lat = arr[1].as_f64().expect("arr[1].as_f64");
            let lat_lng = LatLng::new(lat, lng);
            lat_lng
        })
        .collect::<Array>();
    console::log_1(&format!("MapComponent: add_geojson_trace: latlngs: {:?}", latlngs,).into());

    let options = PolylineOptions::default();
    Polyline::new_with_options(
        &latlngs.iter().map(JsValue::from).collect::<Array>(),
        &options,
    )
    .add_to(map);

    // // Create a GeoJson layer from the parsed GeoJSON value
    // let geojson_value = JsValue::from_str(&geojson_string);
    // // let geojson_layer = GeoJson::geo_json(&geojson_value);

    // let geo = GeoJson::add_data(&geojson_value);

    // // Parse the GeoJSON string into a JavaScript object
    // let window = window();
    // let geojson_value = JsValue::from_str(&geojson_string);
    // let geojson_object = window
    //     .eval("JSON.parse")
    //     .call1(&JsValue::NULL, &geojson_value)
    //     .unwrap();

    // Create a GeoJson layer from the parsed GeoJSON object
    // let geojson_layer = GeoJSON::new(&geojson_value);

    // Add the GeoJson layer to the map
    // map.add_layer(geojson_layer);
}

// fn render_map(container: &HtmlElement) -> Html {
//     let node: &Node = &container.clone().into();
//     Html::VRef(node.clone())
// }

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
