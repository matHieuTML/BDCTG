#![recursion_limit = "512"]

pub mod app;
pub mod auth;
pub mod components;
pub mod data;
pub mod inscriptions;
pub mod models;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
