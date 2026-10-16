//! `use_menu_trigger_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_menu_trigger_state.rs`): the state (closed at first)
//! shown in `#perf-open` (`true`/`false`) and its open submenus in `#perf-submenus` (their trigger
//! keys, comma-separated), mounted at first. Its controls ([`Control`]) mount and unmount it, call
//! its methods (`open` focusing the first item, `close`, `toggle`, `open_submenu` and
//! `close_submenu` of the trigger `share` on the first level), and read how often it reported a
//! change (`changes`).

use leptonic::hooks::{
    collections::{FocusStrategy, Key},
    menu::{MenuTriggerState, UseMenuTriggerStateInput, use_menu_trigger_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<MenuTriggerState>);
    let changes = StoredValue::new(0_u32);
    let call = move |method: &dyn Fn(&MenuTriggerState)| {
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
        Control::action("close", move || call(&MenuTriggerState::close)),
        Control::action("toggle", move || call(&|state| state.toggle(None))),
        Control::action("open-submenu", move || {
            call(&|state| state.open_submenu(Key::from("share"), 0));
        }),
        Control::action("close-submenu", move || {
            call(&|state| state.close_submenu(&Key::from("share"), 0));
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"use_menu_trigger_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject state changes />
        </Show>
    }
}

/// Calls the hook, hands its state to the page's controls and shows it.
#[component]
fn Subject(
    state: StoredValue<Option<MenuTriggerState>>,
    changes: StoredValue<u32>,
) -> impl IntoView {
    let menu = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open: false,
        value: None,
        on_open_change: Some(Callback::new(move |_| {
            changes.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(menu));
    on_cleanup(move || state.set_value(None));
    let is_open = menu.overlay.is_open;
    let stack = menu.expanded_keys_stack;
    let submenus = move || {
        stack.with(|keys| {
            keys.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        })
    };
    view! {
        <span id="perf-open">{move || is_open.get().to_string()}</span>
        <span id="perf-submenus">{submenus}</span>
    }
}
