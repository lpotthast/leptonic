use leptonic::{
    atoms::prelude::{CheckboxButton, CheckboxField, ColorArea, ColorSwatch, ColorThumb},
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        i18n::use_locale,
    },
};
use leptos::prelude::*;

/// A blue. The area keeps the hue and lets the user pick saturation (X) and brightness (Y).
const INITIAL: HSV = HSV {
    hue: 210.0,
    saturation: 0.6,
    brightness: 0.8,
};

#[component]
pub fn ColorAreaAtomDemo() -> impl IntoView {
    let locale = use_locale();
    // The area owns its color (`default_value`). `on_change` reports every change, `on_change_end`
    // only the color at the end of a drag or key press.
    let color = RwSignal::new(INITIAL);
    let committed = RwSignal::new(INITIAL);
    let disabled = RwSignal::new(false);

    // A name for screen readers with the channel values, instead of the swatch's default, the color name.
    let color_name = Signal::derive(move || {
        let c = color.get();
        format!(
            "{}, saturation {}, brightness {}",
            c.hue_name(&locale.get()),
            c.format_channel_value(HsvChannel::Saturation, &locale.get()),
            c.format_channel_value(HsvChannel::Brightness, &locale.get()),
        )
    });

    view! {
        <div class="demo-color-atoms">
            <ColorArea
                default_value=INITIAL
                x_channel=HsvChannel::Saturation
                y_channel=HsvChannel::Brightness
                aria_label="Saturation and brightness"
                is_disabled=disabled
                on_change=Callback::new(move |c| color.set(c))
                on_change_end=Callback::new(move |c| committed.set(c))
                classes="demo-color-atoms-area"
            >
                <ColorThumb classes="demo-color-atoms-thumb"/>
            </ColorArea>

            <div class="demo-color-atoms-result">
                <ColorSwatch color=color color_name=color_name classes="demo-color-atoms-swatch"/>
                <dl class="demo-color-atoms-values">
                    <dt>"Saturation"</dt>
                    <dd>{move || color.get().format_channel_value(HsvChannel::Saturation, &locale.get())}</dd>
                    <dt>"Brightness"</dt>
                    <dd>{move || color.get().format_channel_value(HsvChannel::Brightness, &locale.get())}</dd>
                    <dt>"Hex"</dt>
                    <dd><code>{move || color.get().to_rgb8().to_string()}</code></dd>
                </dl>
            </div>
        </div>

        <p class="demo-status">{move || format!("Committed: {}", committed.get().to_rgb8())}</p>
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
