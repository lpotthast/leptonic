use leptonic::{
    atoms::{
        checkbox::{Checkbox, CheckboxGroup},
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
        <Checkbox
            default_selected
            on_change=move |selected| value.set(selected)
            is_indeterminate
            is_disabled
            is_read_only
            is_invalid
        >
            {label}
        </Checkbox>
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
        <Checkbox is_selected=bound set_selected=bound>"Bound"</Checkbox>
        <button id="test-cb-bound-flip" on:click=move |_| bound.update(|b| *b = !*b)>"Flip"</button>
        // Bound and read-only; `on_change` reports changes of a bound state too.
        <Checkbox is_selected=bound_read_only set_selected=bound_read_only is_read_only=true>"Bound read only"</Checkbox>
        <Checkbox
            is_selected=bound_reported
            set_selected=bound_reported
            on_change=move |selected: bool| reported.set(selected.to_string())
        >
            "Bound reported"
        </Checkbox>
        <div>"Reported: " <span id="test-cb-reported">{reported}</span></div>

        <form id="test-cb-required-form">
            <Checkbox is_required=true validation_behavior=ValidationBehavior::Native>"Required native"</Checkbox>
            <Checkbox is_required=true validation_behavior=ValidationBehavior::Aria>"Required aria"</Checkbox>
        </form>

        <CheckboxGroup name="pets" on_change=show_group>
            <Label>"Pets"</Label>
            <Checkbox value="dogs">"Dogs"</Checkbox>
            <Checkbox value="cats">"Cats"</Checkbox>
            <Checkbox value="dragons" is_disabled=true>"Dragons"</Checkbox>
            <Description>"Pick your pets."</Description>
        </CheckboxGroup>
        <div>"Group: " <span id="test-cb-group-value">{group_value}</span></div>

        <CheckboxGroup aria_label="Disabled group" is_disabled=true>
            <Checkbox value="a">"Disabled group A"</Checkbox>
        </CheckboxGroup>
        <CheckboxGroup aria_label="Read-only group" is_read_only=true default_value=vec![Key::from("a")]>
            <Checkbox value="a">"Read-only group A"</Checkbox>
            <Checkbox value="b">"Read-only group B"</Checkbox>
        </CheckboxGroup>

        <form id="test-cb-group-form">
            <CheckboxGroup is_required=true validation_behavior=ValidationBehavior::Native>
                <Label>"Required group"</Label>
                <Checkbox value="a">"Required group A"</Checkbox>
                <Checkbox value="b">"Required group B"</Checkbox>
                <FieldError />
            </CheckboxGroup>
        </form>
    }
}
