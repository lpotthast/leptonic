use leptonic::{
    hooks::*,
    utils::{
        css::{computed_pct, computed_size},
        style::HeightProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![60.0]),
        orientation: Signal::stored(Orientation::Vertical),
        ..UseSliderStateInput::new(0.0, 100.0)
    });
    let slider = use_slider(UseSliderInput {
        has_label: Signal::stored(true),
        ..UseSliderInput::new(state)
    });
    let thumb = use_slider_thumb(UseSliderThumbInput::new(state, &slider));
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let (track_attrs, track_styles) = slider.track_props.into_parts();
    // A vertical thumb positions itself from the top; the fill grows from the bottom.
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    let fill_styles = Styles::new().add_reactive(move || {
        HeightProperty.declare(computed_size(computed_pct(state.thumb_percent(0) * 100.0)))
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
