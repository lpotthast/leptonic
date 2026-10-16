//! `use_submenu_trigger_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_submenu_trigger_state.rs`): the state of the submenu
//! triggered by `share` on the first level of an open root menu (`use_menu_trigger_state`),
//! closed at first, shown in `#perf-open` (`true`/`false`; the root's in `#perf-root-open`),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, and call its methods
//! (`open` focusing the first item, `close`, `toggle`, `close_all`).

use leptonic::hooks::{
    collections::{FocusStrategy, Key},
    menu::{
        MenuTriggerState, SubmenuTriggerState, UseMenuTriggerStateInput,
        UseSubmenuTriggerStateInput, use_menu_trigger_state, use_submenu_trigger_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let root = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open: true,
        ..UseMenuTriggerStateInput::default()
    });
    let state = StoredValue::new(None::<SubmenuTriggerState>);
    let call = move |method: &dyn Fn(&SubmenuTriggerState)| {
        if let Some(state) = state.get_value() {
            method(&state);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || {
            call(&|state| state.open(Some(FocusStrategy::First)));
        }),
        Control::action("close", move || call(&SubmenuTriggerState::close)),
        Control::action("toggle", move || call(&|state| state.toggle(None))),
        Control::action("close-all", move || call(&SubmenuTriggerState::close_all)),
    ];
    let root_open = root.overlay.is_open;
    view! {
        <h1>"use_submenu_trigger_state performance"</h1>
        <PerfControls controls />
        <span id="perf-root-open">{move || root_open.get().to_string()}</span>
        <Show when=move || mounted.get()>
            <Subject root state />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows it.
#[component]
fn Subject(
    root: MenuTriggerState,
    state: StoredValue<Option<SubmenuTriggerState>>,
) -> impl IntoView {
    let submenu = use_submenu_trigger_state(UseSubmenuTriggerStateInput {
        trigger_key: Key::from("share"),
        root,
        level: 0,
    });
    state.set_value(Some(submenu));
    on_cleanup(move || state.set_value(None));
    let is_open = submenu.is_open;
    view! { <span id="perf-open">{move || is_open.get().to_string()}</span> }
}
