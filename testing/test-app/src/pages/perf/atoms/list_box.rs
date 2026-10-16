//! The `ListBox` alone, for its performance test (`leptonic/tests/perf_tests/atoms/list_box.rs`): a
//! multiple selection (controlled) of options "Option 0", "Option 1", ... (`ListBoxItem`s,
//! `data-perf-option` holding the number), [`SMALL`] at first, [`LARGE`] after the control `large`
//! (while unmounted), in a 200px high scroll view (for `PageDown` and `PageUp`). Between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, change the options and its reactive props, and set the
//! selection.

use std::collections::HashSet;

use leptonic::{
    Orientation,
    atoms::listbox::{ListBox, ListBoxItem},
    hooks::collections::{
        Key, Selection, SelectionBehavior, SelectionMode, UseListCollectionInput,
        use_list_collection,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options at first.
pub const SMALL: u32 = 10;
/// The options after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let selection = RwSignal::new(Selection::default());
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let selection_behavior = RwSignal::new(SelectionBehavior::Toggle);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let disallow_empty = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Vertical);
    let label = RwSignal::new("Options".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("append", move || {
            items.update(|items| {
                let next = items.last().map_or(0, |last| last + 1);
                items.push(next);
            });
        }),
        Control::action("remove-last", move || {
            items.update(|items| {
                items.pop();
            });
        }),
        Control::action("select-first", move || {
            selection.set(Selection::keys([Key::from(0_u32)]));
        }),
        Control::action("select-all", move || selection.set(Selection::All)),
        Control::action("clear-selection", move || {
            selection.set(Selection::default());
        }),
        Control::action("single", move || selection_mode.set(SelectionMode::Single)),
        Control::action("multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("replace", move || {
            selection_behavior.set(SelectionBehavior::Replace);
        }),
        Control::action("toggle", move || {
            selection_behavior.set(SelectionBehavior::Toggle);
        }),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("disallow-empty", move || disallow_empty.set(true)),
        Control::action("allow-empty", move || disallow_empty.set(false)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("rename", move || label.set("Choices".to_owned())),
        Control::action("restore-name", move || label.set("Options".to_owned())),
    ];
    view! {
        <h1>"ListBox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Options
                items=items
                selection=selection
                selection_mode=selection_mode
                selection_behavior=selection_behavior
                disabled_keys=disabled_keys
                disallow_empty=disallow_empty
                orientation=orientation
                label=label
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The listbox with its collection (built while mounted).
#[component]
#[allow(clippy::too_many_arguments, clippy::implicit_hasher)]
fn Options(
    items: RwSignal<Vec<u32>>,
    selection: RwSignal<Selection>,
    selection_mode: RwSignal<SelectionMode>,
    selection_behavior: RwSignal<SelectionBehavior>,
    disabled_keys: RwSignal<HashSet<Key>>,
    disallow_empty: RwSignal<bool>,
    orientation: RwSignal<Orientation>,
    label: RwSignal<String>,
) -> impl IntoView {
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    view! {
        <ListBox
            collection=collection
            selection=selection
            set_selection=selection
            selection_mode=selection_mode
            selection_behavior=selection_behavior
            disabled_keys=disabled_keys
            disallow_empty_selection=disallow_empty
            orientation=orientation
            aria_label=label
            styles=Styles::new()
                .add_unchecked("max-height", "200px")
                .add_unchecked("overflow", "auto")
        >
            <For each=move || items.get() key=|i| *i let:i>
                <ListBoxItem key=i attr:data-perf-option=i>
                    {format!("Option {i}")}
                </ListBoxItem>
            </For>
        </ListBox>
    }
}
