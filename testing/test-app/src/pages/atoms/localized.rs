use leptonic::{
    I18nProvider, Locale,
    atoms::{
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItem},
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
        search_field::{SearchField, SearchFieldClearButton},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
        tag_group::{TagGroup, TagItems, TagList, TagRemoveButton},
    },
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

/// Atoms whose own texts (labels, placeholders, role descriptions) come from the localized
/// strings, in a de-DE `I18nProvider`: a search field, a number field without a label of its
/// own, removable tags and a select without a placeholder.
#[component]
pub fn PageAtomLocalized() -> impl IntoView {
    let german: Locale = "de-DE".parse().expect("a locale");
    let tags = use_collection(|b| {
        b.item("cat", "Katze");
    });
    let fruits = use_collection(|b| {
        b.item("apple", "Apfel");
    });
    view! {
        <div id="test-page-atom-localized">
            <h1>"Localized atoms"</h1>
            <I18nProvider locale=german>
                <SearchField default_value="Suche">
                    <Label>"Suchen"</Label>
                    <Input />
                    <SearchFieldClearButton>"x"</SearchFieldClearButton>
                </SearchField>
                <NumberField<i32> aria_label="Menge">
                    <NumberFieldGroup>
                        <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
                        <Input />
                        <NumberFieldIncrementButton>"+"</NumberFieldIncrementButton>
                    </NumberFieldGroup>
                </NumberField<i32>>
                <TagGroup collection=tags on_remove=|_| {}>
                    <Label>"Tiere"</Label>
                    <TagList>
                        <TagItems let:node>
                            {node.text_value.to_string()}
                            <TagRemoveButton>"x"</TagRemoveButton>
                        </TagItems>
                    </TagList>
                </TagGroup>
                <Select<Option<Key>> collection=fruits>
                    <Label>"Obst"</Label>
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectPopover>
                        <ListBox>
                            <ListBoxItem key="apple">"Apfel"</ListBoxItem>
                        </ListBox>
                    </SelectPopover>
                </Select<Option<Key>>>
            </I18nProvider>
        </div>
    }
}
