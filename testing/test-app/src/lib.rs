pub mod app;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    leptos::mount::hydrate_body(App);

    // Browser tests wait for this marker before interacting with a page. Deferring it by one frame
    // lets the effects scheduled during hydration run first.
    leptos::prelude::request_animation_frame(|| {
        if let Some(body) = leptos::prelude::document().body() {
            let _ = body.set_attribute("data-hydrated", "true");
        }
    });
}
