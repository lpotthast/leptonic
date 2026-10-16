//! `use_safely_mouse_to_submenu` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_safely_mouse_to_submenu.rs`): watching the pointer move
//! from a plain menu (`#perf-menu`, 200px square at (8, 100)) towards a plain submenu
//! (`#perf-submenu`, 200px square at (240, 100), to its right), mounted at first, closed. Its
//! controls ([`Control`]) mount and unmount it, and open and close the submenu (the hook's
//! `is_open`).

use leptonic::{
    CapturedElement,
    hooks::menu::{UseSafelyMouseToSubmenuInput, use_safely_mouse_to_submenu},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
    ];
    view! {
        <h1>"use_safely_mouse_to_submenu performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject open=open.into() />
        </Show>
    }
}

/// Calls the hook on a plain menu and submenu.
#[component]
fn Subject(open: Signal<bool>) -> impl IntoView {
    let (menu, submenu) = (CapturedElement::new(), CapturedElement::new());
    use_safely_mouse_to_submenu(UseSafelyMouseToSubmenuInput {
        menu,
        submenu,
        is_open: open,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div
            id="perf-menu"
            style="position: absolute; top: 100px; left: 8px; width: 200px; height: 200px;"
            {..menu.attr()}
        >
            "Menu"
        </div>
        <div
            id="perf-submenu"
            style="position: absolute; top: 100px; left: 240px; width: 200px; height: 200px;"
            {..submenu.attr()}
        >
            "Submenu"
        </div>
    }
}
