use leptos::*;

mod app;
mod error_template;
mod pages;

use crate::app::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    mount_to_body(|| {
        view! { <App/> }
    });
}
