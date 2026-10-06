use std::collections::HashSet;

use leptonic::{
    atoms::toggle_button::{ToggleButton, ToggleButtonGroup},
    hooks::*,
    utils::{
        color::{ColorValue, HSL, HSV, HslChannel, HsvChannel, RGB8},
        css::{
            CssColor, CssDimension, LengthPercentageAuto, NonNegativeLengthPercentage, Size, try_px,
        },
        style::{
            BackgroundColorProperty, HeightProperty, LeftProperty, TopProperty, WidthProperty,
        },
        styles::Styles,
    },
};
use leptos::prelude::*;

/// The color model the wheel edits the hue of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorModel {
    Hsv,
    Hsl,
}

#[component]
pub fn ColorWheelDemo() -> impl IntoView {
    let model = RwSignal::new(ColorModel::Hsv);
    let select_model = move |keys: HashSet<Key>| {
        model.set(if keys.contains(&Key::from("HSL")) {
            ColorModel::Hsl
        } else {
            ColorModel::Hsv
        });
    };

    view! {
        <div>
            <div class="demo-color-wheel-modes">
                <ToggleButtonGroup
                    selection_mode=ToggleGroupSelectionMode::Single
                    disallow_empty_selection=true
                    default_selected_keys=HashSet::from([Key::from("HSV")])
                    on_selection_change=select_model
                    aria_label="Color model"
                    classes="demo-toggle-group"
                >
                    <ToggleButton value="HSV" classes="demo-toggle-button">"HSV"</ToggleButton>
                    <ToggleButton value="HSL" classes="demo-toggle-button">"HSL"</ToggleButton>
                </ToggleButtonGroup>
            </div>

            // Both wheels stay mounted, so each keeps its value while hidden.
            <div hidden=move || model.get() != ColorModel::Hsv>
                <HsvWheel/>
            </div>
            <div hidden=move || model.get() != ColorModel::Hsl>
                <HslWheel/>
            </div>
        </div>
    }
}

#[component]
fn HsvWheel() -> impl IntoView {
    let state = use_color_wheel_state(UseColorWheelStateInput {
        default_value: HSV::new(),
        channel: HsvChannel::Hue,
        is_disabled: false.into(),
        on_change: None,
        on_change_end: None,
    });
    let value = state.value;
    let info = Signal::derive(move || {
        let c = value.get();
        format!(
            "HSV({:.0}\u{00b0}, {:.0}%, {:.0}%)",
            c.get_channel_value(HsvChannel::Hue),
            c.get_channel_value(HsvChannel::Saturation) * 100.0,
            c.get_channel_value(HsvChannel::Brightness) * 100.0,
        )
    });
    wheel_view(state, info)
}

#[component]
fn HslWheel() -> impl IntoView {
    let state = use_color_wheel_state(UseColorWheelStateInput {
        default_value: HSL::new(),
        channel: HslChannel::Hue,
        is_disabled: false.into(),
        on_change: None,
        on_change_end: None,
    });
    let value = state.value;
    let info = Signal::derive(move || {
        let c = value.get();
        format!(
            "HSL({:.0}\u{00b0}, {:.0}%, {:.0}%)",
            c.get_channel_value(HslChannel::Hue),
            c.get_channel_value(HslChannel::Saturation) * 100.0,
            c.get_channel_value(HslChannel::Lightness) * 100.0,
        )
    });
    wheel_view(state, info)
}

/// A pixel size for `width`/`height`. Negative or non-finite values render as `0px`.
fn px_size(value: f64) -> Size {
    let dimension = try_px(value).unwrap_or(CssDimension::Zero);
    NonNegativeLengthPercentage::try_from(dimension)
        .unwrap_or_else(|_| NonNegativeLengthPercentage::new(CssDimension::Zero))
        .into()
}

/// A pixel offset for `left`/`top`. Non-finite values render as `0px`.
fn px_offset(value: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(try_px(value).unwrap_or(CssDimension::Zero))
}

fn wheel_view<C>(state: UseColorWheelStateReturn<C>, info: Signal<String>) -> impl IntoView
where
    C: ColorValue,
    RGB8: From<C>,
{
    let hue = state.hue;
    let display_color = state.display_color;

    let wheel = use_color_wheel(UseColorWheelInput {
        state,
        outer_radius: 100.0,
        inner_radius: 70.0,
        is_disabled: false.into(),
        aria_label: Some("Hue wheel"),
        name: None,
        form: None,
    });

    let background = wheel.background;
    let thumb_x = wheel.thumb_x;
    let thumb_y = wheel.thumb_y;

    // The size comes from the radii, the annulus shape and the conic gradient are free-form
    // CSS values that leptos-css does not model, so they go through the unchecked escape hatch.
    let track_styles = Styles::builder()
        .with(WidthProperty.declare(px_size(wheel.track_size)))
        .with(HeightProperty.declare(px_size(wheel.track_size)))
        .with_unchecked("clip-path", wheel.clip_path)
        .with_optional_unchecked("background", move || Some(background.get()))
        .build();

    let thumb_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(px_offset(thumb_x.get())))
        .with_reactive(move || TopProperty.declare(px_offset(thumb_y.get())))
        .with_reactive(move || {
            BackgroundColorProperty.declare(CssColor::from(RGB8::from(display_color.get())))
        })
        .build();

    let preview_styles = Styles::new().add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(RGB8::from(display_color.get())))
    });

    view! {
        <div class="demo-color-wheel">
            <div {..wheel.track_props.into_attrs()} class="demo-color-wheel-track" style=track_styles>
                <div {..wheel.thumb_props.into_attrs()} class="demo-color-wheel-thumb" style=thumb_styles>
                    <input {..wheel.input_props.into_attrs()} class="demo-visually-hidden-input"/>
                </div>
            </div>

            <p class="demo-color-wheel-info">
                <span class="demo-color-preview" style=preview_styles></span>
                " "
                <code>{move || format!("{:.0}\u{00b0}", hue.get())}</code>
                <br/>
                <small>
                    <code>{info}</code>
                </small>
            </p>
        </div>
    }
}
