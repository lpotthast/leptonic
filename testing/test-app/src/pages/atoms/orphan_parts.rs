//! Atom parts outside their parent: each warns (debug builds) and renders nothing, instead of
//! panicking. A family's parts mount when its button is pressed, so the page loads without
//! warnings. `#orphans-checkbox` mounts a `CheckboxField` without the `value` its
//! `CheckboxGroup` needs.

use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField, CheckboxGroup},
        combobox::ComboBoxPopover,
        listbox::ListBoxItems,
        menu::{
            MenuItem, MenuItemDescription, MenuItemLabel, MenuItemShortcut, MenuItems, MenuSection,
            SubmenuTrigger,
        },
        number_field::{NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton},
        radio::RadioField,
        search_field::SearchFieldClearButton,
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

/// A family's misplaced parts in `#orphans-<name>`, mounted by `#orphans-<name>-mount`.
#[component]
fn Orphans(name: &'static str, children: ChildrenFn) -> impl IntoView {
    let mounted = RwSignal::new(false);
    view! {
        <button id=format!("orphans-{name}-mount") on:click=move |_| mounted.set(true)>
            {format!("Mount {name} parts without a parent")}
        </button>
        <div id=format!("orphans-{name}") data-mounted=move || mounted.get().then_some("true")>
            <Show when=move || mounted.get()>{children()}</Show>
        </div>
    }
}

#[component]
pub fn PageAtomOrphanParts() -> impl IntoView {
    view! {
        <h1>"Atom parts without a parent"</h1>
        <Orphans name="search-field">
            <SearchFieldClearButton>"Clear"</SearchFieldClearButton>
        </Orphans>
        <Orphans name="number-field">
            <NumberFieldGroup>"Group"</NumberFieldGroup>
            <NumberFieldIncrementButton>"+"</NumberFieldIncrementButton>
            <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
        </Orphans>
        <Orphans name="list-box">
            <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
        </Orphans>
        <Orphans name="combo-box">
            <ComboBoxPopover>"Popover"</ComboBoxPopover>
        </Orphans>
        <Orphans name="radio">
            <RadioField value="dogs">"Dogs"</RadioField>
        </Orphans>
        <Orphans name="menu">
            <MenuItem key="copy">"Copy"</MenuItem>
            <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            <MenuSection key="section">"Section"</MenuSection>
            <SubmenuTrigger key="share">"Share"</SubmenuTrigger>
            <MenuItemLabel>"Label"</MenuItemLabel>
            <MenuItemDescription>"Description"</MenuItemDescription>
            <MenuItemShortcut>"Ctrl+C"</MenuItemShortcut>
        </Orphans>
        <Orphans name="checkbox">
            <CheckboxGroup<Key> aria_label="Pets">
                <CheckboxField>
                    <CheckboxButton>"Dogs"</CheckboxButton>
                </CheckboxField>
            </CheckboxGroup<Key>>
        </Orphans>
    }
}
