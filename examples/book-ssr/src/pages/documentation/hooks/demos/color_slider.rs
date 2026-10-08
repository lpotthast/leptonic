use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::*,
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        css::CssColor,
        i18n::use_locale,
        style::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorSliderDemo() -> impl IntoView {
    let locale = use_locale();
    let disabled = RwSignal::new(false);

    // Disabled and orientation live on the state; the slider changes the hue of a blue.
    let state = use_color_slider_state(UseColorSliderStateInput {
        is_disabled: disabled.into(),
        default_value: HSV {
            hue: 210.0,
            saturation: 0.6,
            brightness: 0.8,
        },
        value: None,
        channel: HsvChannel::Hue,
        orientation: Signal::stored(Orientation::Horizontal),
        on_change: None,
        on_change_end: None,
    });
    let UseColorSliderReturn {
        slider,
        thumb,
        track_styles,
        input_styles,
    } = use_color_slider(UseColorSliderInput {
        has_label: true.into(),
        state,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        name: None,
        form: None,
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
        BackgroundColorProperty.declare(CssColor::from(display_color.get().to_rgb8()))
    });
    let formatted = state.formatted_value();

    view! {
        <div class="demo-color-slider">
            <span {..slider.label_props.into_attrs()}>"Hue"</span>
            <output class="demo-color-slider-output" {..slider.output_props.into_attrs()}>{formatted}</output>
            // The track is the slider's group.
            <div
                class="demo-color-slider-track"
                {..slider.group_props.into_attrs()}
                {..track_attrs}
                style=slider_track_styles.merge(track_styles)
            >
                <div class="demo-color-slider-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                    <input {..thumb.input_props.into_attrs()} style=input_styles/>
                </div>
            </div>
        </div>

        <p class="demo-status">{move || format!("Color: {}, hue: {}", state.value.get().to_rgb8(), state.value.get().hue_name(&locale.get()))}</p>
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
