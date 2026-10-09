use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        input::Input,
        search_field::{SearchField, SearchFieldClearButton},
    },
    hooks::form::InputType,
};
use leptos::{ev::SubmitEvent, prelude::*};

/// SearchField atoms, as react-aria-components' and react-aria's search field tests render them.
#[component]
pub fn PageAtomSearchField() -> impl IntoView {
    let clears = RwSignal::new(0);
    let submitted = RwSignal::new(String::new());
    let escapes_bubbled = RwSignal::new(0);
    let form_submits = RwSignal::new(0);
    let disabled_events = RwSignal::new(0);

    view! {
        <div id="test-page-atom-search-field">
            <h1>"SearchField"</h1>

            <div id="sf-slots">
                <SearchField default_value="test" is_invalid=true attr:data-foo="bar">
                    <Label>"Test"</Label>
                    <Input />
                    <SearchFieldClearButton>"x"</SearchFieldClearButton>
                    <Description>"Description"</Description>
                    <FieldError>"Error"</FieldError>
                </SearchField>
            </div>

            // Enter submits, Escape and the clear button clear. Escape keydowns reaching the
            // wrapper are counted.
            <div
                id="sf-keys"
                on:keydown=move |e| {
                    if e.key() == "Escape" {
                        escapes_bubbled.update(|n| *n += 1);
                    }
                }
            >
                <SearchField
                    on_submit=move |value| submitted.set(value)
                    on_clear=move |()| clears.update(|n| *n += 1)
                >
                    <Label>"Search"</Label>
                    <Input />
                    <SearchFieldClearButton>"x"</SearchFieldClearButton>
                </SearchField>
            </div>
            <div>"Clears: " <span id="sf-clears">{move || clears.get()}</span></div>
            <div>"Submitted: " <span id="sf-submitted">{move || submitted.get()}</span></div>
            <div>"Escapes bubbled: " <span id="sf-escapes-bubbled">{move || escapes_bubbled.get()}</span></div>

            // Without `on_submit`, Enter submits the form.
            <form
                id="sf-form"
                on:submit=move |e: SubmitEvent| {
                    e.prevent_default();
                    form_submits.update(|n| *n += 1);
                }
            >
                <SearchField name="q">
                    <Label>"Search"</Label>
                    <Input />
                </SearchField>
            </form>
            <div>"Form submits: " <span id="sf-form-submits">{move || form_submits.get()}</span></div>

            <form id="sf-native">
                <SearchField is_required=true>
                    <Label>"Test"</Label>
                    <Input />
                    <FieldError />
                </SearchField>
            </form>

            <div id="sf-disabled">
                <SearchField
                    is_disabled=true
                    default_value="test"
                    on_submit=move |_| disabled_events.update(|n| *n += 1)
                    on_clear=move |()| disabled_events.update(|n| *n += 1)
                >
                    <Label>"Disabled"</Label>
                    <Input />
                    <SearchFieldClearButton>"x"</SearchFieldClearButton>
                </SearchField>
            </div>
            <div>"Disabled events: " <span id="sf-disabled-events">{move || disabled_events.get()}</span></div>

            <div id="sf-read-only">
                <SearchField is_read_only=true default_value="test">
                    <Label>"Test"</Label>
                    <Input />
                    <SearchFieldClearButton>"x"</SearchFieldClearButton>
                </SearchField>
            </div>

            // Another input type than `search`.
            <div id="sf-type">
                <SearchField input_type=InputType::Text>
                    <Label>"Text type"</Label>
                    <Input />
                </SearchField>
            </div>

            // The field outside the form it belongs to.
            <form id="sf-attribute-form"></form>
            <div id="sf-form-attribute">
                <SearchField form="sf-attribute-form">
                    <Label>"Test"</Label>
                    <Input />
                </SearchField>
            </div>
        </div>
    }
}
