/// `https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs`
use gloo_utils::document;
use leaflet::{CircleMarker, LatLng, Map, MapOptions, PathOptions, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::console::{self};
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;
use yew::Properties;

pub(crate) const PARIS_LAT: f64 = 48.866_667;
pub(crate) const PARIS_LNG: f64 = 2.333_333;

#[derive(Properties)]
pub struct MapProps {
    pub markers: Vec<(f64, f64)>,
}

impl PartialEq for MapProps {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;
        for (self_lat_lng, other_lat_lng) in self.markers.iter().zip(other.markers.iter()) {
            // eq &= self_lat_lng.distance_to(other_lat_lng) == 0.0;
            eq &= self_lat_lng == other_lat_lng;
        }
        eq
    }
}

#[function_component(MapComponent)]
pub(crate) fn map_component(props: &MapProps) -> Html {
    let container: Element = document().create_element("div").unwrap();
    let container: HtmlElement = container.dyn_into().unwrap();
    container.set_class_name("map");
    let leaflet_map = Map::new_with_element(&container, &MapOptions::default());

    leaflet_map.set_view(&LatLng::new(PARIS_LAT, PARIS_LNG), 11.0);
    add_tile_layer(&leaflet_map);

    let circle_marker = CircleMarker::new(&LatLng::new(
        props.markers[0].0 + 0.1,
        props.markers[0].1 + 0.1,
    ));
    // Set the radius of the circle marker
    circle_marker.set_radius(2000.0);
    circle_marker.set_style(&PathOptions::default());
    // .set("#3388ff") // Set the color of the circle marker
    // .fill_opacity(0.5); // Set the opacity of the fill
    leaflet_map.add_layer(&circle_marker);

    add_circle_with_options(&leaflet_map, props.markers[0].0, props.markers[0].1);

    html! {
        <div id="map" class="map-container component-container">
            {render_map(&container)}
        </div>
    }
}

/// https://github.com/slowtec/leaflet-rs/blob/09d02e74bc30d519a5a30bb130516aa161f0415a/examples/basic/src/lib.rs#L76
fn add_circle_with_options(map: &Map, lat: f64, lng: f64) {
    let options = leaflet::CircleOptions::default();
    options.set_radius(2000.0);
    leaflet::Circle::new_with_options(&LatLng::new(lat, lng), &options).add_to(map);
}

fn render_map(container: &HtmlElement) -> Html {
    let node: &Node = &container.clone().into();
    Html::VRef(node.clone())
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
