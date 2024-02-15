#![recursion_limit = "128000"]

use wasm_bindgen::prelude::*;

mod app_with_map;
mod chat;
// mod map;
mod map_component;

pub use app_with_map::AppWithMap;
use chat::chat_model::*;

use crate::chat::web_rtc_manager::WebRTCManager;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    env_logger::init();

    yew::Renderer::<AppWithMap>::new().render();
    Ok(())
}
