use leptonic::{
    components::prelude::Checkbox,
    hooks::*,
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        css::CssColor,
        style::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorSliderDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // Disabled and orientation live on the state; the slider changes the hue of a blue.
    let state = use_color_slider_state(UseColorSliderStateInput {
        is_disabled: disabled.into(),
        ..UseColorSliderStateInput::new(
            HSV {
                hue: 210.0,
                saturation: 0.6,
                value: 0.8,
            },
            HsvChannel::Hue,
        )
    });
    let UseColorSliderReturn {
        slider,
        thumb,
        track_styles,
        input_styles,
    } = use_color_slider(UseColorSliderInput {
        has_label: true.into(),
        ..UseColorSliderInput::new(state)
    });

    // The focus is on the hidden input inside the thumb: `within` reports it on the thumb as
    // `data-focus-visible`.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });

    // The track's own styles (position, touch action) plus the gradient of the channel.
    let (track_attrs, slider_track_styles) = slider.track_props.into_parts();
    // The thumb positions itself on the track; its fill is the color the track shows at its position.
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    let display_color = state.display_color();
    let thumb_styles = thumb_styles.add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(display_color.get().into_rgb8()))
    });
    let formatted = state.formatted_value();

    view! {
        <div class="demo-color-slider" {..slider.group_props.into_attrs()}>
            <span {..slider.label_props.into_attrs()}>"Hue"</span>
            <output class="demo-color-slider-output" {..slider.output_props.into_attrs()}>{formatted}</output>
            <div class="demo-color-slider-track" {..track_attrs} style=slider_track_styles.merge(track_styles)>
                <div class="demo-color-slider-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                    <input {..thumb.input_props.into_attrs()} style=input_styles/>
                </div>
            </div>
        </div>

        <p class="demo-status">{move || format!("Color: {}, hue: {}", state.value.get().into_rgb8(), state.value.get().hue_name())}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
