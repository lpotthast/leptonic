//! `use_menu_item` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_menu_item.rs`): its props on a plain item (`li`, "Item 1",
//! key `1`) between two other plain items ("Item 0", "Item 2") of a plain menu (`ul`, through
//! `use_menu`, with a `use_list_state` without selection), next to a plain button
//! (`#perf-before`) to tab from, mounted at first. Its controls ([`Control`]) mount and unmount it
//! (the menu stays), switch to single selection and back, disable and enable it, and read how many
//! actions the menu reported (`actions`).

use std::collections::HashSet;

use leptonic::{
    CapturedElement,
    hooks::{
        collections::{
            CloseOnSelect, CollectionOptions, Key, SelectionBehavior, SelectionMode,
            SelectionOptions, UseListStateInput, use_collection, use_list_state,
        },
        menu::{MenuData, UseMenuInput, UseMenuItemInput, use_menu, use_menu_item},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let selection_mode = RwSignal::new(SelectionMode::None);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("single-selection", move || {
            selection_mode.set(SelectionMode::Single);
        }),
        Control::action("no-selection", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("disable", move || {
            disabled_keys.set(HashSet::from([Key::from(1)]));
        }),
        Control::action("enable", move || disabled_keys.set(HashSet::new())),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let collection = use_collection(|b| {
        for i in 0..3 {
            b.item(i, format!("Item {i}"));
        }
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: selection_mode.into(),
            selection_behavior: Signal::stored(SelectionBehavior::Toggle),
            disabled_keys: disabled_keys.into(),
            ..SelectionOptions::default()
        },
    });
    let menu = use_menu(UseMenuInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Actions".into(),
        aria_labelledby: Signal::stored(None),
        options: CollectionOptions::default(),
        keyboard_delegate: None,
        on_action: Some(Callback::new(move |_| {
            actions.update_value(|count| *count += 1);
        })),
        on_close: None,
        submenu: None,
    });
    let data = menu.data;
    let shown = data.clone();
    view! {
        <h1>"use_menu_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ul {..menu.menu.attrs()}>
            <Item menu=data.clone() key=Key::from(0) />
            <Show when=move || mounted.get()>
                <Item menu=shown.clone() key=Key::from(1) />
            </Show>
            <Item menu=data.clone() key=Key::from(2) />
        </ul>
    }
}

/// Calls the hook on a plain item.
#[component]
fn Item(menu: MenuData, key: Key) -> impl IntoView {
    let text = format!("Item {key}");
    let item = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: CloseOnSelect::Auto,
        submenu_trigger: None,
    });
    let (attrs, styles) = item.item.attrs();
    let (pressed, hovered, focus_visible) =
        (item.is_pressed, item.is_hovered, item.is_focus_visible);
    view! {
        <li
            {..attrs}
            style=styles
            data-pressed=move || pressed.get().then_some("true")
            data-hovered=move || hovered.get().then_some("true")
            data-focus-visible=move || focus_visible.get().then_some("true")
        >
            {text}
        </li>
    }
}
