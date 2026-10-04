use std::collections::HashSet;

use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItem},
        select::{HiddenSelect, Select, SelectLabel, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::{PlacementX, PlacementY, Selection, SelectionMode},
    utils::locale::WritingDirection,
};
use leptos::prelude::*;

use super::listbox::describe;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// An uncontrolled single select with a default value. "Cherry" is disabled.
#[component]
pub fn PageAtomSelect() -> impl IntoView {
    let items = Signal::stored(FRUITS.map(String::from).to_vec());
    let changes = RwSignal::new(Vec::<String>::new());

    view! {
        <div id="test-page-atom-select">
            <h1>"Select"</h1>
            <button id="test-sel-before">"Before"</button>
            <form id="test-sel-form">
                <Select<String>
                    items=items
                    selection_mode=SelectionMode::Single
                    default_selected_keys=Selection::Keys(["Banana".to_owned()].into_iter().collect())
                    disabled_keys=Signal::stored(HashSet::from(["Cherry".to_owned()]))
                    get_text_value=Callback::new(|key: String| key)
                    label="Fruit"
                    name="fruit"
                    placeholder="Pick a fruit"
                    on_selection_change=Callback::new(move |s: Selection<String>| {
                        changes.update(|c| c.push(describe(&s)));
                    })
                >
                    <SelectLabel<String>>"Fruit"</SelectLabel<String>>
                    <SelectTrigger<String>>
                        <SelectValue<String> />
                    </SelectTrigger<String>>
                    <SelectPopover<String>
                        placement_x=Signal::derive(|| PlacementX::Left)
                        placement_y=Signal::derive(|| PlacementY::Below)
                        writing_direction=Signal::derive(|| WritingDirection::Ltr)
                    >
                        <ListBox<String>>
                            {FRUITS
                                .map(|fruit| {
                                    view! {
                                        <ListBoxItem<String>
                                            key=fruit.to_owned()
                                        >
                                            {fruit}
                                        </ListBoxItem<String>>
                                    }
                                })
                                .collect_view()}
                        </ListBox<String>>
                    </SelectPopover<String>>
                    <HiddenSelect<String> get_text_value=Callback::new(|key: String| key) />
                </Select<String>>
            </form>
            <button id="test-sel-after">"After"</button>
            <div>"Changes: " <span id="test-sel-changes">{move || changes.get().join(" | ")}</span></div>
        </div>
    }
}
