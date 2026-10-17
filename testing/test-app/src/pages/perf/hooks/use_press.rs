//! `use_press` on a plain `<div role="button">` (`#perf-press`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_press.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, its pressed state as `data-pressed`.
//! Its controls ([`Control`]) mount and unmount it, change its reactive inputs, and read how
//! often it was pressed (`presses`) and double-pressed (`double-presses`).

use leptonic::{
    flag,
    hooks::interactions::{UsePressInput, UsePressReturn, use_press},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The press handling's reactive inputs and counters.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    forced: RwSignal<bool>,
    press_count: StoredValue<u32>,
    double_press_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        forced: RwSignal::new(false),
        press_count: StoredValue::new(0),
        double_press_count: StoredValue::new(0),
    };
    let Inputs {
        disabled,
        forced,
        press_count,
        double_press_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("force-pressed", move || forced.set(true)),
        Control::action("unforce-pressed", move || forced.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
        Control::read("double-presses", move || {
            f64::from(double_press_count.get_value())
        }),
    ];
    view! {
        <h1>"use_press performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Pressed inputs />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The element with `use_press`.
#[component]
fn Pressed(inputs: Inputs) -> impl IntoView {
    let UsePressReturn {
        target: props,
        is_pressed,
    } = use_press(UsePressInput {
        is_disabled: inputs.disabled.into(),
        force_is_pressed: inputs.forced.into(),
        on_press: Some(Callback::new(move |_| {
            inputs.press_count.update_value(|count| *count += 1);
        })),
        on_double_press: Some(Callback::new(move |_| {
            inputs.double_press_count.update_value(|count| *count += 1);
        })),
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.attrs();
    view! {
        <div
            id="perf-press"
            role="button"
            tabindex="0"
            {..attrs}
            style=styles
            data-pressed=flag(is_pressed)
        >
            "Press me"
        </div>
    }
}
