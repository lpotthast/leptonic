//! `use_toggle_button` on a plain `<button>` (`#perf-toggle`, through `use_button`), its state a
//! `use_toggle_state` of the button's component (not selected at first), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_toggle_button.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, and toggle its state from outside
//! (`toggle`).

use leptonic::hooks::{
    button::{UseButtonInput, UseToggleButtonInput, use_button, use_toggle_button},
    form::{ToggleState, UseToggleStateInput, use_toggle_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<ToggleState>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("toggle", move || {
            if let Some(state) = state.get_value() {
                state.toggle();
            }
        }),
    ];
    view! {
        <h1>"use_toggle_button performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Toggle disabled state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The `<button>` with `use_toggle_button`.
#[component]
fn Toggle(disabled: RwSignal<bool>, state: StoredValue<Option<ToggleState>>) -> impl IntoView {
    let toggle_state = use_toggle_state(UseToggleStateInput::default());
    state.set_value(Some(toggle_state));
    let button = use_button(use_toggle_button(UseToggleButtonInput {
        state: toggle_state,
        button: UseButtonInput {
            id: Some("perf-toggle".into()),
            is_disabled: disabled.into(),
            ..UseButtonInput::default()
        },
    }));
    let (attrs, styles) = button.button.attrs();
    view! {
        <button {..attrs} style=styles>
            "Toggle me"
        </button>
    }
}
