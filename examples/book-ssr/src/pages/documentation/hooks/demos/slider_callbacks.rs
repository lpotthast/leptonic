use leptonic::{
    hooks::*,
    utils::{
        css::{computed_pct, computed_size},
        style::WidthProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderCallbacksDemo() -> impl IntoView {
    let (last_change, set_last_change) = signal(None::<f64>);
    let (last_change_end, set_last_change_end) = signal(None::<f64>);

    let state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![25.0]),
        step: Signal::stored(5.0),
        // Every change, also while dragging.
        on_change: Some(Callback::new(move |values: Vec<f64>| {
            set_last_change.set(values.first().copied());
        })),
        // When the user lets go (or after a keyboard change).
        on_change_end: Some(Callback::new(move |values: Vec<f64>| {
            set_last_change_end.set(values.first().copied());
        })),
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
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    let fill_styles = Styles::new().add_reactive(move || {
        WidthProperty.declare(computed_size(computed_pct(state.thumb_percent(0) * 100.0)))
    });
    let show = |value: Option<f64>| value.map_or_else(|| "none yet".to_owned(), |value| format!("{value:.0}"));

    view! {
        <div class="demo-slider demo-slider-orange" {..slider.group_props.into_attrs()}>
            <span class="demo-slider-label" {..slider.label_props.into_attrs()}>"Brightness"</span>
            <div class="demo-slider-track" {..track_attrs} style=track_styles>
                <div class="demo-slider-fill" style=fill_styles></div>
                <div class="demo-slider-thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                    <input class="demo-visually-hidden-input" {..thumb.input_props.into_attrs()}/>
                </div>
            </div>
            <output class="demo-slider-output" {..slider.output_props.into_attrs()}>{move || state.formatted_values()}</output>
        </div>

        <p class="demo-status">
            {move || format!(
                "Last on_change: {}. Last on_change_end: {}.",
                show(last_change.get()),
                show(last_change_end.get()),
            )}
        </p>
    }
}
