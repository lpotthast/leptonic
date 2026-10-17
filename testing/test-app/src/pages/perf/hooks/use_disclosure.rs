//! `use_disclosure` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_disclosure.rs`): a plain trigger button (through
//! `use_button`) and panel (a `div`, role `group`, through the hook's panel props) in
//! `#perf-disclosure`, collapsed, with its own `use_disclosure_state`, between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and unmount
//! it, expand and collapse its state, and disable and enable it.

use leptonic::hooks::{
    button::use_button,
    disclosure::{
        DisclosureState, UseDisclosureInput, UseDisclosureStateInput, use_disclosure,
        use_disclosure_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<DisclosureState>);
    let call = move |method: fn(&DisclosureState)| {
        if let Some(state) = state.get_value() {
            method(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("expand", move || call(DisclosureState::expand)),
        Control::action("collapse", move || call(DisclosureState::collapse)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_disclosure performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject state disabled=disabled.into() />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain trigger and panel, and hands its state to the page's controls.
#[component]
fn Subject(state: StoredValue<Option<DisclosureState>>, disabled: Signal<bool>) -> impl IntoView {
    let disclosure_state = use_disclosure_state(UseDisclosureStateInput::default());
    state.set_value(Some(disclosure_state));
    on_cleanup(move || state.set_value(None));
    let disclosure = use_disclosure(UseDisclosureInput {
        state: disclosure_state,
        is_disabled: disabled,
    });
    let (button_attrs, button_styles) = use_button(disclosure.button).button.attrs();
    let panel_attrs = disclosure.panel.attrs();
    view! {
        <div id="perf-disclosure">
            <button {..button_attrs} style=button_styles>
                "Details"
            </button>
            <div {..panel_attrs}>"Content"</div>
        </div>
    }
}
