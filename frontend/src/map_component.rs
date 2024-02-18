use std::ops::Add;

/// https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs
use gloo_utils::document;
use leaflet::{CircleMarker, LatLng, Map, MapOptions, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::console::{self, log};
use web_sys::{Element, HtmlElement, Node};
use yew::Properties;
use yew::{html::ImplicitClone, prelude::*};

const PARIS_LAT: f64 = 48.866667;
const PARIS_LNG: f64 = 2.333333;

pub enum Msg {}

pub struct MapComponent {
    map: Map,
    container: HtmlElement,
}

#[derive(Properties)]
pub struct MapProperties {
    pub markers: Vec<LatLng>,
}

impl PartialEq for MapProperties {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;
        for (self_lat_lng, other_lat_lng) in self.markers.iter().zip(other.markers.iter()) {
            eq &= self_lat_lng.distance_to(other_lat_lng) == 0.0;
        }
        eq
    }
}

impl MapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = MapProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &MapOptions::default());
        Self {
            map: leaflet_map,
            container,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.map.set_view(&LatLng::new(PARIS_LAT, PARIS_LNG), 11.0);
            add_tile_layer(&self.map);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        console::log_1(&"changed: called".into());
        let props = ctx.props();

        let circle_marker = CircleMarker::new(&props.markers[0]);
        // Set the radius of the circle marker
        circle_marker.set_radius(2.0);
        // .set("#3388ff") // Set the color of the circle marker
        // .fill_opacity(0.5); // Set the opacity of the fill
        self.map.add_layer(&circle_marker);

        console::log_1(&"changed: done".into());
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="map" class="map-container component-container">
                {self.render_map()}
            </div>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
