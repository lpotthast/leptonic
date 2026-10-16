//! `use_slider` on plain elements: the group (`#perf-slider`), its label, a 200 × 20 pixel track
//! (`#perf-slider-track`) and an output of the values (`#perf-slider-output`), with what it needs:
//! a `use_slider_state` (one value at 30, 0 to 100 in steps of 1) and a thumb (`use_slider_thumb`
//! on a `<div>` with its `<input>`, which the output names), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_slider.rs`), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first. Its controls ([`Control`]) mount and unmount it, disable it,
//! and read the value (`value`) and how often a change ended (`change-ends`).

use leptonic::{
    IntoAttrs, Orientation,
    hooks::slider::{
        SliderState, UseSliderInput, UseSliderStateInput, UseSliderThumbInput, use_slider,
        use_slider_state, use_slider_thumb,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels) and its thumb.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; } \
                     .perf-thumb input { opacity: 0; width: 0; height: 0; margin: 0; }";

/// The slider: `use_slider` on plain elements, its state published to `state` for the controls
/// while mounted.
#[component]
fn Subject(
    state: StoredValue<Option<SliderState<f64>>>,
    #[prop(into)] disabled: Signal<bool>,
    change_ends: StoredValue<u32>,
) -> impl IntoView {
    let slider_state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![30.0]),
        value: None,
        min_value: Signal::stored(0.0),
        max_value: Signal::stored(100.0),
        step: Signal::stored(1.0),
        is_disabled: disabled,
        orientation: Signal::stored(Orientation::Horizontal),
        format_options: Signal::default(),
        value_label: None,
        page_size: None,
        on_change: None,
        on_change_end: Some(Callback::new(move |_| {
            change_ends.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(slider_state));
    on_cleanup(move || state.set_value(None));
    let slider = use_slider(UseSliderInput {
        state: slider_state,
        id: Some("perf-slider".to_owned()),
        has_label: Signal::stored(true),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    let thumb = use_slider_thumb(UseSliderThumbInput {
        state: slider_state,
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
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_errormessage: Signal::default(),
        aria_details: Signal::default(),
    });
    let (track_attrs, track_styles) = slider.track_props.into_parts();
    let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
    view! {
        <div {..slider.group_props.into_attrs()}>
            <span {..slider.label_props.into_attrs()}>"Volume"</span>
            <div id="perf-slider-track" {..track_attrs} style=track_styles>
                <div class="perf-thumb" {..thumb_attrs} style=thumb_styles>
                    <input {..thumb.input_props.into_attrs()} />
                </div>
            </div>
            <output id="perf-slider-output" {..slider.output_props.into_attrs()}>
                {move || slider_state.formatted_values()}
            </output>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<SliderState<f64>>);
    let disabled = RwSignal::new(false);
    let change_ends = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::read("value", move || {
            state.get_value().map_or(f64::NAN, |state| {
                state
                    .values
                    .with_untracked(|values| values.first().copied().unwrap_or(f64::NAN))
            })
        }),
        Control::read("change-ends", move || f64::from(change_ends.get_value())),
    ];
    view! {
        <h1>"use_slider performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject state disabled change_ends />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
