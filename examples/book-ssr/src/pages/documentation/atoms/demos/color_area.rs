use leptonic::{
    atoms::{color_area::ColorArea, color_swatch::ColorSwatch},
    components::prelude::Checkbox,
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

/// A blue. The area keeps the hue and lets the user pick saturation (X) and brightness (Y).
const INITIAL: HSV = HSV {
    hue: 210.0,
    saturation: 0.6,
    value: 0.8,
};

#[component]
pub fn ColorAreaAtomDemo() -> impl IntoView {
    // The area owns its color. `on_change` mirrors every change, `on_change_end` only the value at the end of a
    // drag or key press.
    let color = RwSignal::new(INITIAL);
    let committed = RwSignal::new(INITIAL);
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-color-atoms">
            <ColorArea<HSV>
                default_value=INITIAL
                x_channel=HsvChannel::Saturation
                y_channel=HsvChannel::Brightness
                aria_label="Saturation and brightness"
                is_disabled=disabled
                on_change=Callback::new(move |c| color.set(c))
                on_change_end=Callback::new(move |c| committed.set(c))
                classes="demo-color-atoms-area"
            />

            <div class="demo-color-atoms-result">
                <ColorSwatch<HSV> color=color classes="demo-color-atoms-swatch"/>
                <dl class="demo-color-atoms-values">
                    <dt>"Saturation"</dt>
                    <dd>{move || color.get().format_channel_value(HsvChannel::Saturation)}</dd>
                    <dt>"Brightness"</dt>
                    <dd>{move || color.get().format_channel_value(HsvChannel::Brightness)}</dd>
                    <dt>"CSS"</dt>
                    <dd><code>{move || color.get().to_css_string()}</code></dd>
                    <dt>"Hex"</dt>
                    <dd><code>{move || color.get().into_rgb8().to_string()}</code></dd>
                    <dt>"Committed"</dt>
                    <dd><code>{move || committed.get().into_rgb8().to_string()}</code></dd>
                </dl>
            </div>
        </div>

        <Checkbox state=disabled>"Disabled"</Checkbox>
    }
}
