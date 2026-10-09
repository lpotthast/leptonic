use leptonic::{
    IntoAttrs, Orientation, computed_pct, computed_size,
    hooks::{
        focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
        slider::{
            UseSliderInput, UseSliderStateInput, UseSliderThumbInput, use_slider, use_slider_state,
            use_slider_thumb,
        },
    },
    leptos_styles::{Styles, property::HeightProperty},
};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![60.0]),
        orientation: Signal::stored(Orientation::Vertical),
        value: None,
        min_value: Signal::stored(0.0),
        max_value: Signal::stored(100.0),
        step: Signal::stored(1.0),
        is_disabled: Signal::default(),
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
    let thumb = use_slider_thumb(UseSliderThumbInput {
        state,
        slider: slider.data.clone(),
        track: slider.track_element,
        index: 0,
        is_disabled: Signal::default(),
        is_required: Signal::default(),
        is_invalid: Signal::default(),
        name: None,
        form: None,
        has_label: Signal::stored(false),
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_errormessage: None,
        aria_details: None,
    });
    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    let (track_attrs, track_styles) = slider.track_props.into_parts();
    // A vertical thumb positions itself from the top; the fill grows from the bottom.
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    let fill_styles = Styles::new().add_reactive(move || {
        HeightProperty.declare(computed_size(computed_pct(
            state.thumb_percent(0).as_percent(),
        )))
    });

    view! {
        <div class="demo-slider demo-slider-vertical" {..slider.group_props.into_attrs()}>
            <span class="demo-slider-label" {..slider.label_props.into_attrs()}>"Temperature"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill" style=fill_styles></div>
                <div class="demo-slider-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                    <input class="demo-visually-hidden-input" {..thumb.input_props.into_attrs()}/>
                </div>
            </div>
            <output class="demo-slider-output" {..slider.output_props.into_attrs()}>{move || state.formatted_values()}</output>
        </div>
    }
}
