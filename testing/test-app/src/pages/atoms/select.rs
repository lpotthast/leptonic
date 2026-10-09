use std::collections::HashSet;

use leptonic::{
    atoms::{
        field::Label,
        listbox::{ListBox, ListBoxItem},
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{Key, UseListCollectionInput, use_collection, use_list_collection},
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A single select with a default value. "Cherry" is disabled.
#[component]
pub fn PageAtomSelect() -> impl IntoView {
    let fruits = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FRUITS.to_vec()),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    });
    let changes = RwSignal::new(Vec::<String>::new());
    let show_orphans = RwSignal::new(false);
    // As crudkit's page size select: options from a memo, a derived value with its setter, `<For>` items.
    let page_size = RwSignal::new(10_u32);
    let bound_changes = RwSignal::new(0_u32);
    let options = Memo::new(|_| vec![10_u32, 25, 50]);
    // Typed: the options' keys are the sizes' (`SelectionValue for u32`).
    let sizes = use_collection(move |b| {
        for option in options.get() {
            b.item(option, option.to_string());
        }
    });
    let selected_size = Signal::derive(move || Some(page_size.get()));
    let set_page_size = Callback::new(move |size: Option<u32>| {
        if let Some(size) = size {
            page_size.set(size);
        }
    });

    view! {
        <div id="test-page-atom-select">
            <h1>"Select"</h1>
            <button id="test-sel-before">"Before"</button>
            <form id="test-sel-form">
                <Select
                    collection=fruits
                    default_value=Some(Key::from("Banana"))
                    disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                    name="fruit"
                    on_change=Callback::new(move |key: Option<Key>| {
                        changes.update(|c| c.push(key.map(|k| k.to_string()).unwrap_or_default()));
                    })
                >
                    <Label>"Fruit"</Label>
                    <SelectTrigger>
                        <SelectValue placeholder="Pick a fruit" />
                    </SelectTrigger>
                    <SelectPopover>
                        <ListBox>
                            {FRUITS
                                .map(|fruit| view! { <ListBoxItem key=fruit>{fruit}</ListBoxItem> })
                                .collect_view()}
                        </ListBox>
                    </SelectPopover>
                    <HiddenSelect />
                </Select>
            </form>
            <button id="test-sel-after">"After"</button>
            // Bound to app state (`value`), as crudkit's page size select.
            <div id="test-sel-bound">
                <Select
                    collection=sizes
                    value=selected_size
                    set_value=set_page_size
                    on_change=move |_| bound_changes.update(|c| *c += 1)
                >
                    <Label>"Page size"</Label>
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectPopover>
                        <ListBox>
                            <For
                                each=move || options.get()
                                key=|option| *option
                                children=|option| {
                                    view! { <ListBoxItem key=option>{option}</ListBoxItem> }
                                }
                            />
                        </ListBox>
                    </SelectPopover>
                </Select>
            </div>
            <div>
                "Bound: "
                <span id="test-sel-bound-value">
                    {page_size}
                </span>
            </div>
            <div>"Bound changes: " <span id="test-sel-bound-changes">{bound_changes}</span></div>
            <button id="test-sel-bound-reset" on:click=move |_| page_size.set(10)>"Reset page size"</button>
            <div>"Changes: " <span id="test-sel-changes">{move || changes.get().join(" | ")}</span></div>
            <button id="test-sel-mount-orphans" on:click=move |_| show_orphans.set(true)>
                "Mount select parts without a parent"
            </button>
            <div id="test-sel-orphans" data-mounted=move || show_orphans.get().then_some("true")>
                <Show when=move || show_orphans.get()>
                    <SelectTrigger>"Misplaced trigger"</SelectTrigger>
                    <SelectValue placeholder="Misplaced value" />
                    <SelectPopover><span>"Misplaced popover"</span></SelectPopover>
                    <HiddenSelect />
                </Show>
            </div>
        </div>
    }
}
