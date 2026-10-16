//! `use_slider_state` alone (one thumb at 30, 0 to 100 in steps of 1), its values shown in an
//! `<output>` (`#perf-slider-state`, with `data-dragging`, `data-focused` and `data-disabled` of
//! the thumb), for its performance test (`leptonic/tests/perf_tests/hooks/use_slider_state.rs`),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, call the state's methods
//! (increment, decrement, set the value or the percentage, start and end dragging, focus and
//! blur the thumb), change its inputs (maximum, step, disabled), and read the value (`value`),
//! the thumb's percentage (`percent`) and how often a change ended (`change-ends`).

use leptonic::{
    Fraction, Orientation,
    hooks::slider::{SliderState, UseSliderStateInput, use_slider_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The slider state: `use_slider_state`, published to `state` for the controls while mounted.
#[component]
fn Subject(
    state: StoredValue<Option<SliderState<f64>>>,
    #[prop(into)] max: Signal<f64>,
    #[prop(into)] step: Signal<f64>,
    #[prop(into)] disabled: Signal<bool>,
    change_ends: StoredValue<u32>,
) -> impl IntoView {
    let slider = use_slider_state(UseSliderStateInput {
        default_values: Some(vec![30.0]),
        value: None,
        min_value: Signal::stored(0.0),
        max_value: max,
        step,
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
    state.set_value(Some(slider));
    on_cleanup(move || state.set_value(None));
    view! {
        <output
            id="perf-slider-state"
            data-dragging=move || slider.is_thumb_dragging(0).then_some("true")
            data-focused=move || (slider.focused_thumb.get() == Some(0)).then_some("true")
            data-disabled=move || slider.is_disabled.get().then_some("true")
        >
            {move || slider.formatted_values()}
        </output>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<SliderState<f64>>);
    let max = RwSignal::new(100.0_f64);
    let step = RwSignal::new(1.0_f64);
    let disabled = RwSignal::new(false);
    let change_ends = StoredValue::new(0_u32);
    // Calls `method` on the mounted state.
    let with_state = move |method: fn(SliderState<f64>)| {
        if let Some(state) = state.get_value() {
            method(state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("increment", move || {
            with_state(|state| state.increment_thumb(0, None));
        }),
        Control::action("decrement", move || {
            with_state(|state| state.decrement_thumb(0, None));
        }),
        Control::action("set-value", move || {
            with_state(|state| state.set_thumb_value(0, 60.0));
        }),
        Control::action("set-percent", move || {
            with_state(|state| state.set_thumb_percent(0, Fraction::new(0.25)));
        }),
        Control::action("start-drag", move || {
            with_state(|state| state.set_thumb_dragging(0, true));
        }),
        Control::action("end-drag", move || {
            with_state(|state| state.set_thumb_dragging(0, false));
        }),
        Control::action("focus-thumb", move || {
            with_state(|state| state.set_focused_thumb(Some(0)));
        }),
        Control::action("blur-thumb", move || {
            with_state(|state| state.set_focused_thumb(None));
        }),
        Control::action("widen", move || max.set(200.0)),
        Control::action("narrow", move || max.set(100.0)),
        Control::action("coarsen", move || step.set(5.0)),
        Control::action("refine", move || step.set(1.0)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || {
            state.get_value().map_or(f64::NAN, |state| {
                state
                    .values
                    .with_untracked(|values| values.first().copied().unwrap_or(f64::NAN))
            })
        }),
        Control::read("percent", move || {
            state.get_value().map_or(f64::NAN, |state| {
                untrack(|| state.thumb_percent(0).as_percent())
            })
        }),
        Control::read("change-ends", move || f64::from(change_ends.get_value())),
    ];
    view! {
        <h1>"use_slider_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state max step disabled change_ends />
        </Show>
    }
}
