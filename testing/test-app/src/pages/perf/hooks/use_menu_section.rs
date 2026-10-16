//! `use_menu_section` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_menu_section.rs`): its props on a plain section (`li` with
//! a heading `span` and a group `ul`) "edit" (heading "Edit", or "Clipboard" when renamed; Cut,
//! Copy) of a plain menu (`ul`, through `use_menu`) after the section "file" (Open), mounted at
//! first. Its controls ([`Control`]) mount and unmount it (the menu stays), and rename its heading
//! (in the collection) and back.

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{
            CloseOnSelect, CollectionOptions, Key, SelectionOptions, UseListStateInput,
            use_collection, use_list_state,
        },
        menu::{
            MenuData, UseMenuInput, UseMenuItemInput, UseMenuSectionInput, use_menu, use_menu_item,
            use_menu_section,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let renamed = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("rename", move || renamed.set(true)),
        Control::action("unrename", move || renamed.set(false)),
    ];
    let collection = use_collection(move |b| {
        b.section("file", |s| {
            s.header("file-header", "File");
            s.item("Open", "Open");
        });
        b.section("edit", |s| {
            s.header(
                "edit-header",
                if renamed.get() { "Clipboard" } else { "Edit" },
            );
            s.item("Cut", "Cut");
            s.item("Copy", "Copy");
        });
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions::default(),
    });
    let menu = use_menu(UseMenuInput {
        state,
        element: CapturedElement::new(),
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
    let shown = data.clone();
    view! {
        <h1>"use_menu_section performance"</h1>
        <PerfControls controls />
        <ul {..menu.props.into_attrs()}>
            <Section menu=data.clone() key=Key::from("file") items=vec!["Open"] />
            <Show when=move || mounted.get()>
                <Section menu=shown.clone() key=Key::from("edit") items=vec!["Cut", "Copy"] />
            </Show>
        </ul>
    }
}

/// Calls the hook on a plain section with its items.
#[component]
fn Section(menu: MenuData, key: Key, items: Vec<&'static str>) -> impl IntoView {
    let section = use_menu_section(UseMenuSectionInput {
        menu: menu.clone(),
        key,
    });
    view! {
        <li {..section.item_props.into_attrs()}>
            <span class="perf-heading" {..section.heading_props.into_attrs()}>
                {section.heading}
            </span>
            <ul {..section.group_props.into_attrs()}>
                {items
                    .into_iter()
                    .map(|item| view! { <Item menu=menu.clone() key=Key::from(item) /> })
                    .collect_view()}
            </ul>
        </li>
    }
}

/// A plain item of the menu (`use_menu_item`).
#[component]
fn Item(menu: MenuData, key: Key) -> impl IntoView {
    let text = key.to_string();
    let item = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: CloseOnSelect::Auto,
        submenu_trigger: None,
    });
    let (attrs, styles) = item.props.into_parts();
    view! {
        <li {..attrs} style=styles>
            {text}
        </li>
    }
}
