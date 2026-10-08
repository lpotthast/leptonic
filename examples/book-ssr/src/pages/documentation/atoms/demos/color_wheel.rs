use leptonic::{
    atoms::prelude::{CheckboxButton, CheckboxField, ColorSwatch, ColorThumb, ColorWheel, ColorWheelTrack},
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

/// A blue. The wheel changes its hue and keeps its saturation and brightness.
const INITIAL: HSV = HSV {
    hue: 210.0,
    saturation: 0.6,
    brightness: 0.8,
};

#[component]
pub fn ColorWheelAtomDemo() -> impl IntoView {
    let locale = leptonic::utils::i18n::use_locale();
    // The wheel owns its color (`default_value`). `on_change` reports every change, `on_change_end`
    // only the color at the end of a drag or key press.
    let color = RwSignal::new(INITIAL);
    let committed = RwSignal::new(INITIAL);
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-color-atoms">
            <ColorWheel
                outer_radius=100.0
                inner_radius=74.0
                default_value=INITIAL
                on_change=move |c| color.set(c)
                on_change_end=move |c| committed.set(c)
                is_disabled=disabled
                classes="demo-color-atoms-wheel"
            >
                <ColorWheelTrack/>
                <ColorThumb classes="demo-color-atoms-thumb"/>
            </ColorWheel>
            <ColorSwatch color=color classes="demo-color-atoms-swatch"/>
        </div>

        <p class="demo-status">
            {move || format!("Committed hue: {}", committed.get().format_channel_value(HsvChannel::Hue, &locale.get()))}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
