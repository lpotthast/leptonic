#![recursion_limit = "256"]

pub mod app;
#[cfg(feature = "ssr")]
pub mod markdown;
pub mod pages;
#[path = "routes.rs"]
mod route_defs;
pub use route_defs::routes;
pub mod search;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    leptos::mount::hydrate_body(App);
}
