use leptonic::{
    components::color_picker::ColorPicker,
    utils::color::{Color, RGB8},
};
use leptos::prelude::*;

/// The styled `ColorPicker` bound to app state (shown as hex in `#test-ccp-value`, set from
/// outside by `#test-ccp-set`).
#[component]
pub fn PageComponentColorPicker() -> impl IntoView {
    let color = RwSignal::new(Color::Rgb(RGB8::from_hex_int(0xFF_00_00)));
    view! {
        <div id="test-page-component-color-picker">
            <h1>"Color picker component"</h1>
            <div id="test-ccp">
                <ColorPicker value=color set_value=color />
            </div>
            <button id="test-ccp-set" on:click=move |_| color.set(Color::Rgb(RGB8::from_hex_int(0x00_80_00)))>
                "Set green"
            </button>
            <div>"Value: " <span id="test-ccp-value">{move || format!("{:X}", color.get().to::<RGB8>())}</span></div>
        </div>
    }
}
