use leptonic::{
    components::prelude::Checkbox,
    hooks::*,
    utils::{
        css::{computed_pct, computed_size},
        style::WidthProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![50.0]),
        is_disabled: disabled.into(),
        value: None,
        min_value: Signal::stored(0.0),
        max_value: Signal::stored(100.0),
        step: Signal::stored(1.0),
        orientation: Signal::stored(Orientation::Horizontal),
        format_options: Signal::default(),
        value_label: None,
        page_size: None,
        on_change: None,
        on_change_end: None,
    });
    let slider = use_slider(UseSliderInput {
        // A visible label names the slider: spread `label_props` on it.
        has_label: Signal::stored(true),
        state,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
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
    });
    // The slider hooks don't track keyboard focus: a focus ring on the thumb sets `data-focus-visible` while its
    // input has keyboard focus.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let (track_attrs, track_styles) = slider.track_props.into_parts();
    // The thumb positions itself on the track.
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    // The fill covers the track up to the thumb.
    let fill_styles = Styles::new().add_reactive(move || {
        WidthProperty.declare(computed_size(computed_pct(state.thumb_percent(0) * 100.0)))
    });

    view! {
        <div class="demo-slider" {..slider.group_props.into_attrs()}>
            <span class="demo-slider-label" {..slider.label_props.into_attrs()}>"Volume"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill" style=fill_styles></div>
                <div class="demo-slider-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                    <input class="demo-visually-hidden-input" {..thumb.input_props.into_attrs()}/>
                </div>
            </div>
            <output class="demo-slider-output" {..slider.output_props.into_attrs()}>{move || state.formatted_values()}</output>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
