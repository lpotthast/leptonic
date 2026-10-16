//! `use_menu` alone, for its performance test (`leptonic/tests/perf_tests/hooks/use_menu.rs`): its
//! props on a plain list (`ul`, `role="menu"`, "Actions") of [`SMALL`] or [`LARGE`] plain items
//! (`li`, through `use_menu_item`: "Item 0", "Item 1", ..., and the last one "Zebra"; key `i`),
//! with a `use_list_state` without selection, between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first with [`SMALL`]. Its controls ([`Control`]) mount it with either
//! size and unmount it, and read how many actions it reported (`actions`).

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{
            CloseOnSelect, CollectionOptions, Key, SelectionOptions, UseListStateInput,
            use_collection, use_list_state,
        },
        menu::{MenuData, UseMenuInput, UseMenuItemInput, use_menu, use_menu_item},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small menu.
pub const SMALL: i32 = 10;
/// The items of the large menu.
pub const LARGE: i32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let size = RwSignal::new(SMALL);
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount-small", move || {
            size.set(SMALL);
            mounted.set(true);
        }),
        Control::action("mount-large", move || {
            size.set(LARGE);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    view! {
        <h1>"use_menu performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject size=size.get_untracked() actions />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain list of `size` items.
#[component]
fn Subject(size: i32, actions: StoredValue<u32>) -> impl IntoView {
    let collection = use_collection(move |b| {
        for i in 0..size - 1 {
            b.item(i, format!("Item {i}"));
        }
        // The only item starting with "z", for type-ahead.
        b.item(size - 1, "Zebra");
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
        options: CollectionOptions {
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        keyboard_delegate: None,
        on_action: Some(Callback::new(move |_| {
            actions.update_value(|count| *count += 1);
        })),
        on_close: None,
        submenu: None,
    });
    let data = menu.data;
    view! {
        <ul {..menu.props.into_attrs()}>
            {(0..size)
                .map(|i| {
                    let text = if i == size - 1 { "Zebra".to_owned() } else { format!("Item {i}") };
                    view! { <Item menu=data.clone() key=Key::from(i) text /> }
                })
                .collect_view()}
        </ul>
    }
}

/// A plain item of the menu (`use_menu_item`).
#[component]
fn Item(menu: MenuData, key: Key, text: String) -> impl IntoView {
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
