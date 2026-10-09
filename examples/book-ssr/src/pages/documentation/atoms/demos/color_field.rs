use leptonic::{
    ColorValue, RGB8, RgbChannel,
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        color_field::{ColorChannelField, ColorField},
        color_swatch::ColorSwatch,
        field::Label,
        input::Input,
    },
    use_locale,
};
use leptos::prelude::*;

#[component]
pub fn ColorFieldAtomDemo() -> impl IntoView {
    let locale = use_locale();
    // The hex field and the three channel fields edit the same color.
    let color = RwSignal::new(Some(RGB8 {
        r: 30,
        g: 110,
        b: 200,
    }));
    let disabled = RwSignal::new(false);

    let channel_fields = [RgbChannel::Red, RgbChannel::Green, RgbChannel::Blue]
        .into_iter()
        .map(|channel| {
            view! {
                <ColorChannelField channel value=color set_value=color is_disabled=disabled classes="demo-field">
                    <Label classes="demo-field-label">{move || RGB8::channel_name(channel, &locale.get())}</Label>
                    <Input classes="demo-color-atoms-input"/>
                </ColorChannelField>
            }
        })
        .collect_view();

    view! {
        <div class="demo-color-atoms">
            <div class="demo-color-atoms-fields">
                <ColorField value=color set_value=color is_disabled=disabled classes="demo-field">
                    <Label classes="demo-field-label">"Hex"</Label>
                    <Input classes="demo-color-atoms-input"/>
                </ColorField>
                <div class="demo-color-atoms-channel-fields">{channel_fields}</div>
            </div>
            <Show when=move || color.get().is_some()>
                <ColorSwatch
                    color=Signal::derive(move || color.get().unwrap_or_default())
                    classes="demo-color-atoms-swatch"
                />
            </Show>
        </div>

        <p class="demo-status">
            {move || color.get().map_or_else(|| "No color".to_owned(), |c| format!("Color: {c}, {}", c.color_name(&locale.get())))}
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
