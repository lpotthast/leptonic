use std::collections::HashSet;

use leptonic::atoms::field::Label;
use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItem},
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{Key, use_collection, use_list_collection},
    utils::ValueBinding,
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A single select with a default value. "Cherry" is disabled.
#[component]
pub fn PageAtomSelect() -> impl IntoView {
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let changes = RwSignal::new(Vec::<String>::new());
    // As crudkit's page size select: options from a memo, a derived value binding, `<For>` items.
    let page_size = RwSignal::new(10_u32);
    let options = Memo::new(|_| vec![10_u32, 25, 50]);
    let sizes = use_collection(move |b| {
        for option in options.get() {
            b.item(Key::from(option.to_string()), option.to_string());
        }
    });
    let bound = ValueBinding::new(
        Signal::derive(move || vec![Key::from(page_size.get().to_string())]),
        Callback::new(move |keys: Vec<Key>| {
            if let Some(size) = keys
                .first()
                .and_then(|k| k.as_str())
                .and_then(|k| k.parse().ok())
            {
                page_size.set(size);
            }
        }),
    );

    view! {
        <div id="test-page-atom-select">
            <h1>"Select"</h1>
            <button id="test-sel-before">"Before"</button>
            <form id="test-sel-form">
                <Select
                    collection=fruits
                    default_value=vec![Key::from("Banana")]
                    disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                    name="fruit"
                    on_change=Callback::new(move |keys: Vec<Key>| {
                        let keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                        changes.update(|c| c.push(keys.join(",")));
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
                <Select collection=sizes value=bound>
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
                                    view! { <ListBoxItem key=Key::from(option.to_string())>{option}</ListBoxItem> }
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
            <div>"Changes: " <span id="test-sel-changes">{move || changes.get().join(" | ")}</span></div>
        </div>
    }
}
