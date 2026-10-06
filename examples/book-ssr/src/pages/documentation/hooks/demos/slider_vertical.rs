use leptonic::{
    hooks::*,
    utils::{
        css::{CssDimension, LengthPercentageAuto, NonNegativeLengthPercentage, Size, try_pct},
        style::{BottomProperty, HeightProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Uncontrolled(vec![60.0]),
        min_value: 0.0,
        max_value: 100.0,
        step: Some(1.0),
        is_disabled: false.into(),
        orientation: Orientation::Vertical.into(),
        on_change: None,
        on_change_end: None,
    });

    let UseSliderReturn {
        group_props,
        output_props,
        track_props,
        track_ref,
        ..
    } = use_slider(UseSliderInput {
        state,
        aria_label: Some("Temperature"),
        aria_labelledby: None,
    });
    let (track_attrs, track_styles) = track_props.into_parts();

    let UseSliderThumbReturn {
        thumb_props,
        input_props,
        percentage,
        value,
        ..
    } = use_slider_thumb(UseSliderThumbInput {
        state,
        track: track_ref,
        index: 0,
        name: None,
        aria_label: Some("Temperature".into()),
        aria_labelledby: None,
        is_disabled: state.is_disabled,
        decimal_places: None,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: None,
    });

    // In a vertical slider, the fill grows from the bottom and the thumb moves up.
    let fill_styles = Styles::new().add_reactive(move || {
        HeightProperty.declare(computed_size(computed_pct(percentage.get())))
    });
    let thumb_styles = Styles::new().add_reactive(move || {
        BottomProperty.declare(LengthPercentageAuto::from(computed_pct(percentage.get())))
    });

    view! {
        <div class="demo-slider demo-slider-purple demo-slider-vertical" {..group_props.into_attrs()}>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill demo-slider-fill-vertical" style=fill_styles></div>
                <div class="demo-slider-thumb demo-slider-thumb-vertical" {..thumb_props.into_attrs()} style=thumb_styles>
                    <input class="demo-visually-hidden-input" {..input_props.into_attrs()}/>
                </div>
            </div>
            <output class="demo-slider-output" {..output_props.into_attrs()}>
                {move || format!("{:.0}\u{b0}", value.get())}
            </output>
        </div>
    }
}

/// Percentages computed at runtime can be NaN (e.g. when `min == max`); fall back to `0`.
fn computed_pct(value: f64) -> CssDimension {
    try_pct(value).unwrap_or(CssDimension::Zero)
}

/// `height` only accepts non-negative values; fall back to `0` otherwise.
fn computed_size(value: CssDimension) -> Size {
    NonNegativeLengthPercentage::try_from(value)
        .unwrap_or_else(|_| NonNegativeLengthPercentage::new(CssDimension::Zero))
        .into()
}
