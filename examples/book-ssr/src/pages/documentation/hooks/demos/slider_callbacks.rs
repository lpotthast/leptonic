use leptonic::{
    hooks::*,
    utils::{
        css::{CssDimension, LengthPercentageAuto, NonNegativeLengthPercentage, Size, try_pct},
        style::{LeftProperty, WidthProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderCallbacksDemo() -> impl IntoView {
    let (last_change, set_last_change) = signal(None::<f64>);
    let (last_change_end, set_last_change_end) = signal(None::<f64>);

    let state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Uncontrolled(vec![25.0]),
        min_value: 0.0,
        max_value: 100.0,
        step: Some(5.0),
        is_disabled: false.into(),
        orientation: Orientation::Horizontal.into(),
        on_change: Some(Callback::new(move |values: Vec<f64>| {
            set_last_change.set(values.first().copied());
        })),
        on_change_end: Some(Callback::new(move |values: Vec<f64>| {
            set_last_change_end.set(values.first().copied());
        })),
    });

    let UseSliderReturn {
        group_props,
        output_props,
        track_props,
        track_ref,
        ..
    } = use_slider(UseSliderInput {
        state,
        aria_label: Some("Brightness"),
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
        aria_label: Some("Brightness".into()),
        aria_labelledby: None,
        is_disabled: state.is_disabled,
        decimal_places: None,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: None,
    });

    let fill_styles = Styles::new()
        .add_reactive(move || WidthProperty.declare(computed_size(computed_pct(percentage.get()))));
    let thumb_styles = Styles::new().add_reactive(move || {
        LeftProperty.declare(LengthPercentageAuto::from(computed_pct(percentage.get())))
    });

    let show = |value: Option<f64>| {
        value.map_or_else(|| "\u{2014}".to_owned(), |value| format!("{value:.0}"))
    };

    view! {
        <div class="demo-slider demo-slider-orange" {..group_props.into_attrs()}>
            <span class="demo-slider-label" aria-hidden="true">"Brightness"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill demo-slider-fill-positioned" style=fill_styles></div>
                <div class="demo-slider-thumb demo-slider-thumb-positioned" {..thumb_props.into_attrs()} style=thumb_styles>
                    <input class="demo-visually-hidden-input" {..input_props.into_attrs()}/>
                </div>
            </div>
            <output class="demo-slider-output" {..output_props.into_attrs()}>
                {move || format!("{:.0}%", value.get())}
            </output>
        </div>

        <div class="demo-mono-log">
            <div>"on_change: "{move || show(last_change.get())}</div>
            <div>"on_change_end: "{move || show(last_change_end.get())}</div>
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
