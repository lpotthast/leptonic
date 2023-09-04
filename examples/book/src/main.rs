use leptos::*;

mod app;
mod pages;
mod wasm_tracing_layer;

use crate::app::*;

fn main() {
    console_error_panic_hook::set_once();
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Initialize a tracing subscriber logging to the browser console.
    wasm_tracing_layer::set_as_global_default_with_config(
        wasm_tracing_layer::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    mount_to_body(|| {
        view! { <App/> }
    })
}
