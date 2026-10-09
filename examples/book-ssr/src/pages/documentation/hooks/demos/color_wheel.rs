use leptonic::{
    ColorValue, HSV, HsvChannel, IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        color::{
            UseColorWheelInput, UseColorWheelStateInput, use_color_wheel, use_color_wheel_state,
        },
        focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
    },
    leptos_styles::{css::CssColor, property::BackgroundColorProperty},
};
use leptos::prelude::*;

#[component]
pub fn ColorWheelDemo() -> impl IntoView {
    let locale = leptonic::use_locale();
    let disabled = RwSignal::new(false);

    let state = use_color_wheel_state(UseColorWheelStateInput {
        is_disabled: disabled.into(),
        default_value: HSV {
            hue: 210.0,
            saturation: 1.0,
            brightness: 1.0,
        },
        value: None,
        on_change: None,
        on_change_end: None,
    });
    // A ring between the radii 100 and 74 pixels.
    let wheel = use_color_wheel(UseColorWheelInput {
        state,
        outer_radius: Signal::stored(100.0),
        inner_radius: Signal::stored(74.0),
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        name: None,
        form: None,
    });

    // The focus is on the hidden input inside the thumb: `within` reports it on the thumb as
    // `data-focus-visible`.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });

    // The props carry the ring's size, shape and gradient, the thumb's position and the input's
    // hiding as styles. The thumb is filled with the hue at full saturation and brightness.
    let (track_attrs, track_styles) = wheel.track_props.into_parts();
    let (thumb_attrs, thumb_styles) = wheel.thumb_props.into_parts();
    let (input_attrs, input_styles) = wheel.input_props.into_parts();
    let display_color = state.display_color();
    let thumb_styles = thumb_styles.add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(display_color.get().to_rgb8()))
    });

    view! {
        // The thumb is a sibling of the track: the track's clip path would cut it off.
        <div class="demo-color-wheel">
            <div class="demo-color-wheel-track" {..track_attrs} style=track_styles></div>
            <div class="demo-color-wheel-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                <input {..input_attrs} style=input_styles/>
            </div>
        </div>

        <p class="demo-status">
            {move || format!("Hue: {}", state.value.get().format_channel_value(HsvChannel::Hue, &locale.get()))}
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
