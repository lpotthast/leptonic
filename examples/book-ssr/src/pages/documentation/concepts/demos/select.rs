use leptonic::{
    atoms::{
        checkbox::Checkbox,
        field::Label,
        listbox::{ListBox, ListBoxItems},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

#[component]
pub fn SelectConceptDemo() -> impl IntoView {
    // The options: a key each, and the text shown.
    let sizes = use_collection(|b| {
        b.item("small", "Small");
        b.item("medium", "Medium");
        b.item("large", "Large");
    });
    let size = RwSignal::new(vec![Key::from("medium")]);
    let disabled = RwSignal::new(false);

    view! {
        <Select collection=sizes value=size set_value=size is_disabled=disabled classes="demo-sel">
            <Label classes="demo-sel-label">"Coffee size"</Label>
            <SelectTrigger classes="demo-sel-trigger">
                <SelectValue classes="demo-sel-value"/>
                <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
            </SelectTrigger>
            <SelectPopover classes="demo-sel-popover">
                <ListBox classes="demo-sel-listbox">
                    <ListBoxItems classes="demo-sel-item" let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </SelectPopover>
        </Select>
        <p class="demo-status">
            "Selected: "{move || size.with(|keys| keys.first().map_or_else(|| "none".to_owned(), ToString::to_string))}"."
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
