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

mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::new().render();
}
