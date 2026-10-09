use leptonic::{
    ColorValue, HSL, HslChannel,
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        color_slider::{ColorSlider, ColorSliderOutput, ColorSliderTrack},
        color_swatch::ColorSwatch,
        color_thumb::ColorThumb,
        field::Label,
    },
    use_locale,
};
use leptos::prelude::*;

#[component]
pub fn ColorSliderAtomDemo() -> impl IntoView {
    let locale = use_locale();
    // All three sliders edit the same color: each changes one of its channels.
    let color = RwSignal::new(HSL {
        hue: 210.0,
        saturation: 0.6,
        lightness: 0.5,
    });
    let committed = RwSignal::new(color.get_untracked());
    let disabled = RwSignal::new(false);

    let sliders = [
        HslChannel::Hue,
        HslChannel::Saturation,
        HslChannel::Lightness,
    ]
    .into_iter()
    .map(|channel| {
        view! {
            <ColorSlider
                channel
                value=color
                set_value=color
                on_change_end=move |c| committed.set(c)
                is_disabled=disabled
                classes="demo-color-atoms-slider"
            >
                <Label>{move || HSL::channel_name(channel, &locale.get())}</Label>
                <ColorSliderOutput classes="demo-color-atoms-slider-output"/>
                <ColorSliderTrack classes="demo-color-atoms-slider-track">
                    <ColorThumb classes="demo-color-atoms-thumb"/>
                </ColorSliderTrack>
            </ColorSlider>
        }
    })
    .collect_view();

    view! {
        <div class="demo-color-atoms">
            <div class="demo-color-atoms-sliders">{sliders}</div>
            <ColorSwatch color=color classes="demo-color-atoms-swatch"/>
        </div>

        <p class="demo-status">{move || format!("Committed: {}", committed.get().to_css_string())}</p>
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
