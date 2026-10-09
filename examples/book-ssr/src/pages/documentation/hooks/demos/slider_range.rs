use leptonic::{
    IntoAttrs, Orientation, computed_pct, computed_size,
    hooks::{
        focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
        slider::{
            UseSliderInput, UseSliderStateInput, UseSliderThumbInput, use_slider, use_slider_state,
            use_slider_thumb,
        },
    },
    leptos_styles::{
        Styles,
        css::LengthPercentageAuto,
        property::{LeftProperty, WidthProperty},
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![20.0, 80.0]),
        value: None,
        min_value: Signal::stored(0.0),
        max_value: Signal::stored(100.0),
        step: Signal::stored(1.0),
        is_disabled: Signal::default(),
        orientation: Signal::stored(Orientation::Horizontal),
        format_options: Signal::default(),
        value_label: None,
        page_size: None,
        on_change: None,
        on_change_end: None,
    });
    let slider = use_slider(UseSliderInput {
        has_label: Signal::stored(true),
        state,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_details: None,
    });
    // One `use_slider_thumb` per value, each with a focus ring that shows while its input has keyboard focus.
    let thumb = |index: usize, label: &'static str| {
        let thumb = use_slider_thumb(UseSliderThumbInput {
            index,
            aria_label: label.into(),
            state,
            slider: slider.data.clone(),
            track: slider.track_element,
            is_disabled: Signal::default(),
            is_required: Signal::default(),
            is_invalid: Signal::default(),
            name: None,
            form: None,
            has_label: Signal::stored(false),
            aria_labelledby: None,
            aria_describedby: None,
            aria_errormessage: None,
            aria_details: None,
        });
        let focus_ring = use_focus_ring(UseFocusRingInput {
            target: FocusRingTarget::Within,
            ..UseFocusRingInput::default()
        });
        let (attrs, styles) = thumb.thumb_props.into_parts();
        view! {
            <div class="demo-slider-thumb" {..attrs} {..focus_ring.props.into_attrs()} style=styles>
                <input class="demo-visually-hidden-input" {..thumb.input_props.into_attrs()}/>
            </div>
        }
    };
    let minimum = thumb(0, "Minimum price");
    let maximum = thumb(1, "Maximum price");
    let (track_attrs, track_styles) = slider.track_props.into_parts();
    // The fill spans the two thumbs.
    let fill_styles = Styles::new()
        .add_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(computed_pct(
                state.thumb_percent(0).as_percent(),
            )))
        })
        .add_reactive(move || {
            let width = state.thumb_percent(1).as_percent() - state.thumb_percent(0).as_percent();
            WidthProperty.declare(computed_size(computed_pct(width)))
        });

    view! {
        <div class="demo-slider demo-slider-blue" {..slider.group_props.into_attrs()}>
            <span class="demo-slider-label" {..slider.label_props.into_attrs()}>"Price"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill" style=fill_styles></div>
                {minimum}
                {maximum}
            </div>
            // "20 – 80"
            <output class="demo-slider-output demo-slider-output-wide" {..slider.output_props.into_attrs()}>
                {move || state.formatted_values()}
            </output>
        </div>
    }
}
