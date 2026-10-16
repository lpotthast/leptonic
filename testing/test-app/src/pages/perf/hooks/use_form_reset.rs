//! `use_form_reset` on an `<input>` (`#perf-input`) in a `<form>` (`#perf-form`), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_form_reset.rs`): mounted at first, its
//! value "Ada" at first and restored to it on reset. Between two plain buttons (`#perf-before`,
//! `#perf-after`). Its controls ([`Control`]) mount and unmount it, make the form cancel its
//! resets (`prevent-resets`), and read how often the hook restored the value (`resets`).

use leptonic::{
    CapturedElement,
    hooks::form::{UseFormResetInput, use_form_reset},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new("Ada".to_owned());
    let prevent_resets = StoredValue::new(false);
    let resets = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("prevent-resets", move || prevent_resets.set_value(true)),
        Control::read("resets", move || f64::from(resets.get_value())),
    ];
    view! {
        <h1>"use_form_reset performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <form
            id="perf-form"
            on:reset=move |e| {
                if prevent_resets.get_value() {
                    e.prevent_default();
                }
            }
        >
            <Show when=move || mounted.get()>
                <ResetInput value resets />
            </Show>
        </form>
        <button id="perf-after">"After"</button>
    }
}

/// The input whose value a form reset restores.
#[component]
fn ResetInput(value: RwSignal<String>, resets: StoredValue<u32>) -> impl IntoView {
    let element = CapturedElement::new();
    use_form_reset(UseFormResetInput {
        element,
        initial_value: "Ada".to_owned(),
        on_reset: Callback::new(move |initial: String| {
            resets.update_value(|resets| *resets += 1);
            value.set(initial);
        }),
    });
    view! {
        <input
            id="perf-input"
            aria-label="Name"
            {..element.attr()}
            prop:value=value
            on:input=move |e| value.set(event_target_value(&e))
        />
    }
}
