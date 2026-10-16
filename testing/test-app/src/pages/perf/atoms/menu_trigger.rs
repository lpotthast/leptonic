//! The `MenuTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_trigger.rs`): its `Button` (`#perf-trigger`) opening a
//! `Menu` of 10 items ("Item 0", ...) in a `Popover`, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, closed. The open state is the app's
//! (`is_open` + `set_open`). Its controls ([`Control`]) mount and unmount it, open and close it
//! (the app's state), and read how many actions the menu reported (`actions`).

use leptonic::{
    atoms::{
        button::Button,
        menu::{Menu, MenuItems, MenuTrigger},
        popover::Popover,
    },
    hooks::collections::use_collection,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let items = use_collection(|b| {
        for i in 0..10 {
            b.item(i, format!("Item {i}"));
        }
    });
    view! {
        <h1>"MenuTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <MenuTrigger is_open=open set_open=open>
                <Button id="perf-trigger">"Actions"</Button>
                <Popover>
                    <Menu
                        collection=items
                        on_action=move |_| actions.update_value(|count| *count += 1)
                    >
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
