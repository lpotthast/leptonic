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
    let value = RwSignal::new(None::<Key>);

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
            "Selected: "
            {move || {
                // The status names the country; the keys are its ISO codes.
                let name = |key: &Key| countries.with(|c| c.get(key).map(|node| node.text_value.to_string()));
                value.with(|key| key.as_ref().and_then(name)).unwrap_or_else(|| "none".to_owned())
            }}
            "."
        </p>
    }
}
