use leptonic::{
    hooks::*,
    utils::{
        CapturedElement,
        css::{CssDimension, LengthPercentageAuto, NonNegativeLengthPercentage, Size, try_pct},
        style::{LeftProperty, WidthProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Uncontrolled(vec![20.0, 80.0]),
        min_value: 0.0,
        max_value: 100.0,
        step: Some(1.0),
        is_disabled: false.into(),
        orientation: Orientation::Horizontal.into(),
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
        aria_label: Some("Price range"),
        aria_labelledby: None,
    });
    let (track_attrs, track_styles) = track_props.into_parts();

    // Thumb positions in percent (0-100). `state.values` is tracked, so the fill follows the thumbs.
    let percent = move |index: usize| {
        let value = state
            .values
            .get()
            .get(index)
            .copied()
            .unwrap_or(state.min_value);
        state.get_value_percent.run(value) * 100.0
    };
    let fill_styles = Styles::new()
        .add_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(computed_pct(percent(0))))
        })
        .add_reactive(move || {
            WidthProperty.declare(computed_size(computed_pct(percent(1) - percent(0))))
        });

    view! {
        <div class="demo-slider demo-slider-blue" {..group_props.into_attrs()}>
            <span class="demo-slider-label" aria-hidden="true">"Price"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill demo-slider-fill-range" style=fill_styles></div>
                <Thumb state track=track_ref index=0 label="Minimum price"/>
                <Thumb state track=track_ref index=1 label="Maximum price"/>
            </div>
            <output class="demo-slider-output demo-slider-output-wide" {..output_props.into_attrs()}>
                {move || {
                    let values = state.values.get();
                    let (min, max) = (values.first().copied(), values.get(1).copied());
                    format!("{:.0} \u{2013} {:.0}", min.unwrap_or_default(), max.unwrap_or_default())
                }}
            </output>
        </div>
    }
}

/// One thumb of the range slider: a positioned element around the focusable range input.
#[component]
fn Thumb(
    state: UseSliderStateReturn,
    track: CapturedElement,
    index: usize,
    label: &'static str,
) -> impl IntoView {
    let UseSliderThumbReturn {
        thumb_props,
        input_props,
        percentage,
        ..
    } = use_slider_thumb(UseSliderThumbInput {
        state,
        track,
        index,
        name: None,
        aria_label: Some(label.into()),
        aria_labelledby: None,
        is_disabled: state.is_disabled,
        decimal_places: None,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: None,
    });

    let styles = Styles::new().add_reactive(move || {
        LeftProperty.declare(LengthPercentageAuto::from(computed_pct(percentage.get())))
    });

    view! {
        <div class="demo-slider-thumb demo-slider-thumb-positioned" {..thumb_props.into_attrs()} style=styles>
            <input class="demo-visually-hidden-input" {..input_props.into_attrs()}/>
        </div>
    }
}

/// Percentages computed at runtime can be NaN (e.g. when `min == max`); fall back to `0`.
fn computed_pct(value: f64) -> CssDimension {
    try_pct(value).unwrap_or(CssDimension::Zero)
}

/// `width` only accepts non-negative values; fall back to `0` otherwise.
fn computed_size(value: CssDimension) -> Size {
    NonNegativeLengthPercentage::try_from(value)
        .unwrap_or_else(|_| NonNegativeLengthPercentage::new(CssDimension::Zero))
        .into()
}
