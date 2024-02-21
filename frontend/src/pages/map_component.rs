/// `https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs`
use gloo_utils::document;
use leaflet::{CircleMarker, LatLng, Map, MapOptions, PathOptions, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;
use yew::Properties;
use yewdux::use_store;

use crate::store::Store;

const PARIS_LAT: f64 = 48.866_667;
const PARIS_LNG: f64 = 2.333_333;

#[function_component(MapComponent)]
pub(crate) fn map_component() -> Html {
    let container: Element = document().create_element("div").unwrap();
    let container: HtmlElement = container.dyn_into().unwrap();
    container.set_class_name("map h-full");
    let leaflet_map = Map::new_with_element(&container, &MapOptions::default());

    leaflet_map.set_view(&LatLng::new(PARIS_LAT, PARIS_LNG), 11.0);
    add_tile_layer(&leaflet_map);

    let (store, _dispatch) = use_store::<Store>();

    // let circle_marker = CircleMarker::new(&LatLng::new(
    //     props.markers[0].0 + 0.1,
    //     props.markers[0].1 + 0.1,
    // ));
    // // Set the radius of the circle marker
    // circle_marker.set_radius(100.0);
    // circle_marker.set_style(&PathOptions::default());
    // // .set("#3388ff") // Set the color of the circle marker
    // // .fill_opacity(0.5); // Set the opacity of the fill
    // leaflet_map.add_layer(&circle_marker);

    for (username, (lat, lng)) in &store.locations {
        add_circle_with_options(&leaflet_map, *lat, *lng);
    }

    html! {
        // <div id="map" class="map-container component-container">
        //     {render_map(&container)}
        // </div>

        // <section class="bg-ct-blue-600 min-h-screen pt-20">
        //  max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center
            <div class="h-full">
                // <div id="map">
                    {render_map(&container)}
                // </div>
            </div>
        // </section>
    }
}

/// https://github.com/slowtec/leaflet-rs/blob/09d02e74bc30d519a5a30bb130516aa161f0415a/examples/basic/src/lib.rs#L76
fn add_circle_with_options(map: &Map, lat: f64, lng: f64) {
    let options = leaflet::CircleOptions::default();
    options.set_radius(100.0);
    leaflet::Circle::new_with_options(&LatLng::new(lat, lng), &options).add_to(map);
}

fn render_map(container: &HtmlElement) -> Html {
    let node: &Node = &container.clone().into();
    Html::VRef(node.clone())
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
