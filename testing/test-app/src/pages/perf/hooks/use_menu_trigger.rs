//! `use_menu_trigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_menu_trigger.rs`): its button input on a plain button
//! (`button[aria-haspopup]`, through `use_button`) with a `use_menu_trigger_state`, and a plain
//! menu (`ul`, `role="menu"`, the hook's id, rendered while open), between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first, closed. The state's focus strategy is shown
//! in `#perf-focus-strategy` (`first`, `last` or empty). Its controls ([`Control`]) mount and
//! unmount it, and close the menu.

use leptonic::hooks::{
    button::use_button,
    collections::FocusStrategy,
    menu::{
        MenuTriggerState, MenuTriggerType, UseMenuTriggerInput, UseMenuTriggerStateInput,
        use_menu_trigger, use_menu_trigger_state,
    },
    overlay::OverlayTriggerType,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<MenuTriggerState>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("close", move || {
            if let Some(state) = state.get_value() {
                state.close();
            }
        }),
    ];
    view! {
        <h1>"use_menu_trigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain button, with a plain menu while open.
#[component]
fn Subject(state: StoredValue<Option<MenuTriggerState>>) -> impl IntoView {
    let menu_state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    state.set_value(Some(menu_state));
    on_cleanup(move || state.set_value(None));
    let trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled: Signal::stored(false),
        trigger: MenuTriggerType::Press,
        state: menu_state,
    });
    let menu_id = trigger.menu_props.id;
    let labelled_by = trigger.menu_props.aria_labelledby;
    let (attrs, styles) = use_button(trigger.button).props.into_parts();
    let is_open = menu_state.overlay.is_open;
    let strategy = menu_state.focus_strategy;
    let shown_strategy = move || match strategy.get() {
        Some(FocusStrategy::First) => "first",
        Some(FocusStrategy::Last) => "last",
        None => "",
    };
    view! {
        <button {..attrs} style=styles>
            "Actions"
        </button>
        <span id="perf-focus-strategy">{shown_strategy}</span>
        <Show when=move || is_open.get()>
            <ul id=menu_id.clone() role="menu" aria-labelledby=labelled_by>
                <li role="menuitem">"Copy"</li>
            </ul>
        </Show>
    }
}
