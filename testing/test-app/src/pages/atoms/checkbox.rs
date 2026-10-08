use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField, CheckboxGroup},
        field::{Description, FieldError, Label},
    },
    hooks::{ValidationBehavior, collections::Key},
};
use leptos::prelude::*;

/// A checkbox labelled `label`; its selection is shown in `#test-cb-{name}-value`.
#[component]
fn TestCheckbox(
    name: &'static str,
    label: &'static str,
    #[prop(optional)] default_selected: bool,
    #[prop(optional)] is_indeterminate: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] is_invalid: bool,
) -> impl IntoView {
    let value = RwSignal::new(default_selected);
    view! {
        <CheckboxField
            default_selected
            on_change=move |selected| value.set(selected)
            is_indeterminate
            is_disabled
            is_read_only
            is_invalid><CheckboxButton>
            {label}
        </CheckboxButton></CheckboxField>
        <div>"Selected: " <span id=format!("test-cb-{name}-value")>{move || value.get().to_string()}</span></div>
    }
}

/// Checkboxes and checkbox groups (react-aria-components' `Checkbox.test.js` and
/// `CheckboxGroup.test.js` setups).
#[component]
pub fn PageAtomCheckbox() -> impl IntoView {
    let bound = RwSignal::new(false);
    let bound_read_only = RwSignal::new(false);
    let bound_reported = RwSignal::new(false);
    let reported = RwSignal::new(String::new());
    let group_value = RwSignal::new(String::new());
    let show_group = move |keys: Vec<Key>| {
        let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
        keys.sort();
        group_value.set(keys.join(","));
    };

    view! {
        <h1>"Checkbox"</h1>
        <button id="test-cb-before">"Before"</button>
        <TestCheckbox name="basic" label="Basic" />
        <TestCheckbox name="indeterminate" label="Indeterminate" is_indeterminate=true />
        <TestCheckbox name="disabled" label="Disabled" is_disabled=true />
        <TestCheckbox name="readonly" label="Read only" default_selected=true is_read_only=true />
        <TestCheckbox name="invalid" label="Invalid" is_invalid=true />

        // Bound to a signal, which a button changes from outside.
        <CheckboxField is_selected=bound set_selected=bound><CheckboxButton>"Bound"</CheckboxButton></CheckboxField>
        <button id="test-cb-bound-flip" on:click=move |_| bound.update(|b| *b = !*b)>"Flip"</button>
        // Bound and read-only; `on_change` reports changes of a bound state too.
        <CheckboxField is_selected=bound_read_only set_selected=bound_read_only is_read_only=true><CheckboxButton>"Bound read only"</CheckboxButton></CheckboxField>
        <CheckboxField
            is_selected=bound_reported
            set_selected=bound_reported
            on_change=move |selected: bool| reported.set(selected.to_string())><CheckboxButton>
            "Bound reported"
        </CheckboxButton></CheckboxField>
        <div>"Reported: " <span id="test-cb-reported">{reported}</span></div>

        <form id="test-cb-required-form">
            <CheckboxField is_required=true validation_behavior=ValidationBehavior::Native><CheckboxButton>"Required native"</CheckboxButton></CheckboxField>
            <CheckboxField is_required=true validation_behavior=ValidationBehavior::Aria><CheckboxButton>"Required aria"</CheckboxButton></CheckboxField>
        </form>

        <CheckboxGroup name="pets" on_change=show_group>
            <Label>"Pets"</Label>
            <CheckboxField value="dogs"><CheckboxButton>"Dogs"</CheckboxButton></CheckboxField>
            <CheckboxField value="cats"><CheckboxButton>"Cats"</CheckboxButton></CheckboxField>
            <CheckboxField value="dragons" is_disabled=true><CheckboxButton>"Dragons"</CheckboxButton></CheckboxField>
            <Description>"Pick your pets."</Description>
        </CheckboxGroup>
        <div>"Group: " <span id="test-cb-group-value">{group_value}</span></div>

        <CheckboxGroup<Key> aria_label="Disabled group" is_disabled=true>
            <CheckboxField value="a"><CheckboxButton>"Disabled group A"</CheckboxButton></CheckboxField>
        </CheckboxGroup<Key>>
        <CheckboxGroup aria_label="Read-only group" is_read_only=true default_value=vec![Key::from("a")]>
            <CheckboxField value="a"><CheckboxButton>"Read-only group A"</CheckboxButton></CheckboxField>
            <CheckboxField value="b"><CheckboxButton>"Read-only group B"</CheckboxButton></CheckboxField>
        </CheckboxGroup>

        <form id="test-cb-group-form">
            <CheckboxGroup<Key> is_required=true validation_behavior=ValidationBehavior::Native>
                <Label>"Required group"</Label>
                <CheckboxField value="a"><CheckboxButton>"Required group A"</CheckboxButton></CheckboxField>
                <CheckboxField value="b"><CheckboxButton>"Required group B"</CheckboxButton></CheckboxField>
                <FieldError />
            </CheckboxGroup<Key>>
        </form>
    }
}
