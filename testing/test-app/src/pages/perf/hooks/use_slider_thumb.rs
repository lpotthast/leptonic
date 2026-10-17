//! `use_slider_thumb` on a plain `<div>` (`#perf-slider-thumb`, 16 × 16 pixels, with
//! `data-dragging` and `data-focused`) with its `<input>`, with what it needs: a
//! `use_slider_state` (one value at 30, 0 to 100 in steps of 1) and a `use_slider` (a group, a
//! label and a 200 × 20 pixel track), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_slider_thumb.rs`), between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and unmount
//! the slider, change the thumb's inputs (disabled, required, invalid), and read the value
//! (`value`) and how often a change ended (`change-ends`).

use leptonic::{
    Orientation,
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
                     #perf-slider-thumb { width: 16px; height: 16px; background: #333; top: 50%; } \
                     #perf-slider-thumb input { opacity: 0; width: 0; height: 0; margin: 0; }";

/// The thumb's inputs the controls change.
#[derive(Clone, Copy)]
struct ThumbProps {
    disabled: RwSignal<bool>,
    required: RwSignal<bool>,
    invalid: RwSignal<bool>,
}

/// The slider: `use_slider_thumb` on a `<div>` in a `use_slider` track, the state published to
/// `state` for the controls while mounted.
#[component]
fn Subject(
    state: StoredValue<Option<SliderState<f64>>>,
    props: ThumbProps,
    change_ends: StoredValue<u32>,
) -> impl IntoView {
    let slider_state = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![30.0]),
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
        on_change_end: Some(Callback::new(move |_| {
            change_ends.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(slider_state));
    on_cleanup(move || state.set_value(None));
    let slider = use_slider(UseSliderInput {
        state: slider_state,
        id: None,
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
        is_disabled: props.disabled.into(),
        is_required: props.required.into(),
        is_invalid: props.invalid.into(),
        name: None,
        form: None,
        has_label: Signal::stored(false),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_errormessage: Signal::default(),
        aria_details: Signal::default(),
    });
    let (track_attrs, track_styles) = slider
        .track
        .props()
        .expect("the first track takes the props")
        .into_parts();
    let (thumb_attrs, thumb_styles) = thumb.thumb.attrs();
    let (is_dragging, is_focused) = (thumb.is_dragging, thumb.is_focused);
    view! {
        <div {..slider.group.attrs()}>
            <span {..slider.label.attrs()}>"Volume"</span>
            <div id="perf-slider-track" {..track_attrs} style=track_styles>
                <div
                    id="perf-slider-thumb"
                    {..thumb_attrs}
                    style=thumb_styles
                    data-dragging=move || is_dragging.get().then_some("true")
                    data-focused=move || is_focused.get().then_some("true")
                >
                    <input {..thumb.input.attrs()} />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<SliderState<f64>>);
    let props = ThumbProps {
        disabled: RwSignal::new(false),
        required: RwSignal::new(false),
        invalid: RwSignal::new(false),
    };
    let change_ends = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || props.disabled.set(true)),
        Control::action("enable", move || props.disabled.set(false)),
        Control::action("require", move || props.required.set(true)),
        Control::action("make-optional", move || props.required.set(false)),
        Control::action("invalidate", move || props.invalid.set(true)),
        Control::action("validate", move || props.invalid.set(false)),
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
        <h1>"use_slider_thumb performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject state props change_ends />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
