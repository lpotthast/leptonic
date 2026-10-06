use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItems},
    },
    hooks::{
        collections::{Key, use_collection},
        use_contains_filter,
    },
};
use leptos::prelude::*;

#[component]
pub fn ComboBoxConceptDemo() -> impl IntoView {
    let countries = use_collection(|b| {
        b.item("at", "Austria");
        b.item("de", "Germany");
        b.item("nl", "Netherlands");
        b.item("se", "Sweden");
    });
    let value = RwSignal::new(Vec::<Key>::new());

    view! {
        <ComboBox collection=countries filter=use_contains_filter() value=value set_value=value classes="demo-combo">
            <Label classes="demo-combo-label">"Ship to"</Label>
            <div class="demo-combo-field">
                <Input classes="demo-combo-atom-input"/>
                <ComboBoxButton classes="demo-combo-atom-button">
                    <span aria-hidden="true">"\u{25bc}"</span>
                </ComboBoxButton>
            </div>
            <ComboBoxPopover classes="demo-combo-popover">
                <ListBox classes="demo-combo-listbox">
                    <ListBoxItems classes="demo-combo-item" let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox>
        <p class="demo-status">
            "Selected: "{move || value.with(|keys| keys.first().map_or_else(|| "none".to_owned(), ToString::to_string))}"."
        </p>
    }
}
