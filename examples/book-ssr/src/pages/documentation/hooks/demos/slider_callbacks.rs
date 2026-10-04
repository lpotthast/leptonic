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
    let step_state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Uncontrolled(vec![25.0]),
        min_value: 0.0,
        max_value: 100.0,
        step: Some(5.0),
        disabled: false.into(),
        orientation: Signal::default(),
        on_change: None,
        on_change_end: None,
    });

    let UseSliderReturn {
        track_props: step_track_props_with_styles,
        track_ref: step_track_ref,
        label_props: step_label_props,
        ..
    } = use_slider(UseSliderInput {
        state: step_state,
        is_rtl: false,
        aria_label: None,
        aria_labelledby: None,
    });
    let (step_track_props, step_track_styles) = step_track_props_with_styles.into_parts();

    let UseSliderThumbReturn {
        thumb_props: step_thumb_props,
        input_props: step_input_props,
        percentage: step_percent,
        value: step_value,
        ..
    } = use_slider_thumb(UseSliderThumbInput {
        state: step_state,
        track: step_track_ref,
        index: 0,
        name: None,
        aria_label: None,
        aria_labelledby: None,
        disabled: false.into(),
        validation_state: ValidationState::Valid,
        is_rtl: false,
        decimal_places: None,
        is_required: false,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: None,
    });

    // The fill and thumb positions are the only dynamic styles; everything else is in CSS classes.
    let fill_styles = Styles::new().add_reactive(move || {
        WidthProperty.declare(computed_size(computed_pct(step_percent.get())))
    });
    let thumb_styles = Styles::new().add_reactive(move || {
        LeftProperty.declare(LengthPercentageAuto::from(computed_pct(step_percent.get())))
    });

    view! {
        <div class="demo-frame">
            <div class="demo-slider demo-slider-orange">
                <label id=step_label_props.id class="demo-slider-label">
                    "Brightness"
                </label>

                <div {..step_track_props} class="demo-slider-track" style=step_track_styles>
                    <div
                        class="demo-slider-fill demo-slider-fill-positioned"
                        style=fill_styles
                    ></div>

                    <div
                        {..step_thumb_props.into_attrs()}
                        class="demo-slider-thumb demo-slider-thumb-positioned"
                        style=thumb_styles
                    >
                        <input
                            {..step_input_props.into_attrs()}
                            class="demo-visually-hidden-input"
                        />
                    </div>
                </div>

                <output class="demo-slider-output">
                    {move || format!("{:.0}%", step_value.get())}
                </output>
            </div>

            // Step markers
            <div class="demo-slider-step-labels">
                {(0..=20)
                    .map(|i| view! { <span class="demo-slider-step-label">{i * 5}</span> })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Percentages computed from slider state can be NaN (e.g. when `min == max`),
/// so use the fallible `try_pct` instead of `pct`, which panics on non-finite input.
fn computed_pct(value: f64) -> CssDimension {
    try_pct(value).unwrap_or(CssDimension::Zero)
}

/// `width`/`height` only accept non-negative values; fall back to `0` otherwise.
fn computed_size(value: CssDimension) -> Size {
    NonNegativeLengthPercentage::try_from(value)
        .unwrap_or_else(|_| NonNegativeLengthPercentage::new(CssDimension::Zero))
        .into()
}
