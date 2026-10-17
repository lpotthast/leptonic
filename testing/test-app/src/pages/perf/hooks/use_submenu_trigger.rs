//! `use_submenu_trigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_submenu_trigger.rs`): the trigger item "Share" (first, a
//! plain `li` through `use_menu_item`) of a plain menu (`ul`, through `use_menu`; then "Copy"),
//! opening a plain submenu (`.perf-submenu`, through `use_menu` with the hook's submenu props:
//! Email, Link; rendered while open, to the right of the menu) without delay, with its state
//! (`use_submenu_trigger_state`) under the menu's (`use_menu_trigger_state`, open), next to a plain
//! button (`#perf-before`) to tab from, mounted at first, closed. Its controls ([`Control`]) mount
//! and unmount it.

use std::time::Duration;

use leptonic::{
    CapturedElement,
    hooks::{
        collections::{
            CloseOnSelect, CollectionOptions, Key, SelectionOptions, UseListStateInput,
            use_collection, use_list_state,
        },
        menu::{
            MenuData, SubmenuKind, SubmenuTriggerItem, UseMenuInput, UseMenuItemInput,
            UseMenuTriggerStateInput, UseSubmenuTriggerInput, UseSubmenuTriggerStateInput,
            use_menu, use_menu_item, use_menu_trigger_state, use_submenu_trigger,
            use_submenu_trigger_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"use_submenu_trigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject />
        </Show>
    }
}

/// The menu with the submenu trigger.
#[component]
fn Subject() -> impl IntoView {
    let root = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open: true,
        ..UseMenuTriggerStateInput::default()
    });
    let collection = use_collection(|b| {
        b.item("share", "Share");
        b.item("copy", "Copy");
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions::default(),
    });
    let parent_menu = CapturedElement::new();
    let menu = use_menu(UseMenuInput {
        state,
        element: parent_menu,
        id: None,
        aria_label: "Actions".into(),
        aria_labelledby: Signal::stored(None),
        options: CollectionOptions::default(),
        keyboard_delegate: None,
        on_action: None,
        on_close: None,
        submenu: None,
    });
    let data = menu.data;
    let submenu_state = use_submenu_trigger_state(UseSubmenuTriggerStateInput {
        trigger_key: Key::from("share"),
        root,
        level: data.submenu_level,
    });
    let submenu_element = CapturedElement::new();
    let trigger = use_submenu_trigger(UseSubmenuTriggerInput {
        state: submenu_state,
        trigger: CapturedElement::new(),
        parent_menu,
        submenu: submenu_element,
        kind: SubmenuKind::Menu,
        is_disabled: Signal::stored(false),
        delay: Duration::ZERO,
    });
    let submenu_props = StoredValue::new(trigger.submenu);
    let is_open = submenu_state.is_open;
    view! {
        <ul {..menu.menu.attrs()} style="width: 200px;">
            <Item menu=data.clone() key=Key::from("share") submenu_trigger=Some(trigger.trigger) />
            <Item menu=data.clone() key=Key::from("copy") submenu_trigger=None />
        </ul>
        <Show when=move || is_open.get()>
            <Submenu props=submenu_props element=submenu_element />
        </Show>
    }
}

/// The submenu (`use_menu` with the trigger's submenu props).
#[component]
fn Submenu(
    props: StoredValue<leptonic::Part<leptonic::hooks::menu::SubmenuProps>>,
    element: CapturedElement,
) -> impl IntoView {
    let collection = use_collection(|b| {
        b.item("email", "Email");
        b.item("link", "Link");
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions::default(),
    });
    let menu = use_menu(UseMenuInput {
        state,
        element,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::stored(None),
        options: CollectionOptions::default(),
        keyboard_delegate: None,
        on_action: None,
        on_close: None,
        submenu: props.with_value(leptonic::Part::props),
    });
    let data = menu.data;
    view! {
        <ul
            class="perf-submenu"
            style="position: absolute; top: 100px; left: 260px; width: 200px;"
            {..menu.menu.attrs()}
        >
            <Item menu=data.clone() key=Key::from("email") submenu_trigger=None />
            <Item menu=data.clone() key=Key::from("link") submenu_trigger=None />
        </ul>
    }
}

/// A plain item (`use_menu_item`), maybe a submenu trigger.
#[component]
fn Item(menu: MenuData, key: Key, submenu_trigger: Option<SubmenuTriggerItem>) -> impl IntoView {
    let text = menu
        .state
        .collection
        .with_untracked(|c| c.get(&key).map(|node| node.text_value.to_string()))
        .unwrap_or_default();
    let item = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: CloseOnSelect::Auto,
        submenu_trigger,
    });
    let (attrs, styles) = item.item.attrs();
    view! {
        <li {..attrs} style=styles>
            {text}
        </li>
    }
}
