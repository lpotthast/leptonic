//! `use_spin_button` (1 to 10) on a focusable `<div>` (`#perf-spin-button`) and its increment
//! stepper (`#perf-increment`, `use_button`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_spin_button.rs`): mounted at first, its value the page's
//! (5 at first; the callbacks step it by 1, Page Up and Page Down by 2), between two plain
//! buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it,
//! disable and enable it, set the value (`set-value`: 8) and read it (`value`).

use leptonic::{
    IntoAttrs,
    hooks::{
        button::{UseButtonInput, use_button},
        spinbutton::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(5.0_f64);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-value", move || value.set(8.0)),
        Control::read("value", move || value.get_untracked()),
    ];
    view! {
        <h1>"use_spin_button performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <SpinButton value disabled />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The spin button and its increment stepper.
#[component]
fn SpinButton(value: RwSignal<f64>, disabled: RwSignal<bool>) -> impl IntoView {
    let step = move |change: f64| {
        Some(Callback::new(move |()| {
            value.update(|value| *value = (*value + change).clamp(1.0, 10.0));
        }))
    };
    let UseSpinButtonReturn {
        props,
        increment_button,
        ..
    } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || Some(value.get())),
        text_value: Signal::default(),
        min_value: Signal::stored(Some(1.0)),
        max_value: Signal::stored(Some(10.0)),
        is_disabled: disabled.into(),
        is_read_only: Signal::stored(false),
        is_required: Signal::stored(false),
        on_increment: step(1.0),
        on_increment_page: step(2.0),
        on_decrement: step(-1.0),
        on_decrement_page: step(-2.0),
        on_decrement_to_min: Some(Callback::new(move |()| value.set(1.0))),
        on_increment_to_max: Some(Callback::new(move |()| value.set(10.0))),
    });
    let (increment_attrs, increment_styles) = use_button(UseButtonInput {
        id: Some("perf-increment".to_owned()),
        aria_label: "Increase".into(),
        ..increment_button
    })
    .props
    .into_parts();
    view! {
        <div id="perf-spin-button" {..props.into_attrs()} tabindex="0" aria-label="Volume"></div>
        <button {..increment_attrs} style=increment_styles>"+"</button>
    }
}
