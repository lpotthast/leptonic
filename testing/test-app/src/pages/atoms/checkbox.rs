use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField, CheckboxGroup},
        field::{Description, FieldError, Label},
        form::Form,
    },
    hooks::{
        collections::Key,
        form::{ValidateFn, ValidationBehavior, ValidationResult},
    },
};
use leptos::{ev::SubmitEvent, prelude::*};

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
    let show_group = move |keys: HashSet<Key>| {
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
        <CheckboxGroup aria_label="Read-only group" is_read_only=true default_value=HashSet::from([Key::from("a")])>
            <CheckboxField value="a"><CheckboxButton>"Read-only group A"</CheckboxButton></CheckboxField>
            <CheckboxField value="b"><CheckboxButton>"Read-only group B"</CheckboxButton></CheckboxField>
        </CheckboxGroup>

        <GroupValidation />

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

/// Three terms the group's validator requires all of.
fn terms_group(prefix: &'static str, behavior: ValidationBehavior) -> impl IntoView {
    let all_terms: ValidateFn<HashSet<Key>> = Arc::new(|terms: &HashSet<Key>| {
        if terms.len() < 3 {
            Err(vec!["You must accept all terms".to_owned()])
        } else {
            Ok(())
        }
    });
    view! {
        <CheckboxGroup<Key> validate=all_terms validation_behavior=behavior>
            <Label>"Agree to the following"</Label>
            <CheckboxField value="terms"><CheckboxButton>{format!("{prefix} terms")}</CheckboxButton></CheckboxField>
            <CheckboxField value="cookies"><CheckboxButton>{format!("{prefix} cookies")}</CheckboxButton></CheckboxField>
            <CheckboxField value="privacy"><CheckboxButton>{format!("{prefix} privacy")}</CheckboxButton></CheckboxField>
            <FieldError />
        </CheckboxGroup<Key>>
    }
}

/// Three terms, two of which validate themselves.
fn terms_items(prefix: &'static str, behavior: ValidationBehavior) -> impl IntoView {
    let accepted = |message: &'static str| -> ValidateFn<bool> {
        Arc::new(move |checked: &bool| {
            if *checked {
                Ok(())
            } else {
                Err(vec![message.to_owned()])
            }
        })
    };
    view! {
        <CheckboxGroup<Key> validation_behavior=behavior>
            <Label>"Agree to the following"</Label>
            <CheckboxField value="terms" validate=accepted("You must accept the terms.")><CheckboxButton>{format!("{prefix} terms")}</CheckboxButton></CheckboxField>
            <CheckboxField value="cookies" validate=accepted("You must accept the cookies.")><CheckboxButton>{format!("{prefix} cookies")}</CheckboxButton></CheckboxField>
            <CheckboxField value="privacy"><CheckboxButton>{format!("{prefix} privacy")}</CheckboxButton></CheckboxField>
            <FieldError />
        </CheckboxGroup<Key>>
    }
}

/// Checkbox groups validated by a function (the group's or the checkboxes' own), by the server
/// and with a custom message, with native and ARIA validation (react-spectrum's
/// `CheckboxGroup.test.js` validation cases).
#[component]
fn GroupValidation() -> impl IntoView {
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        server_errors.set(HashMap::from([(
            "terms".to_owned(),
            vec!["You must accept the terms.".to_owned()],
        )]));
    };
    view! {
        <Form attr:id="gv-native-group">{terms_group("Native group", ValidationBehavior::Native)}</Form>
        <Form attr:id="gv-native-items">{terms_items("Native items", ValidationBehavior::Native)}</Form>
        <Form attr:id="gv-native-server" validation_errors=server_errors on:submit=on_submit>
            <CheckboxGroup<Key> name="terms">
                <Label>"Server terms"</Label>
                <CheckboxField value="terms"><CheckboxButton>"Server terms A"</CheckboxButton></CheckboxField>
                <CheckboxField value="cookies"><CheckboxButton>"Server terms B"</CheckboxButton></CheckboxField>
                <FieldError />
            </CheckboxGroup<Key>>
            <button id="gv-native-server-submit" type="submit">"Submit"</button>
        </Form>
        <Form attr:id="gv-custom-message">
            <CheckboxGroup<Key> is_required=true>
                <Label>"Custom message"</Label>
                <CheckboxField value="terms"><CheckboxButton>"Custom message terms"</CheckboxButton></CheckboxField>
                <FieldError message=Arc::new(|result: &ValidationResult| {
                    result
                        .validation_details
                        .value_missing
                        .then(|| "Please select at least one item".to_owned())
                }) />
            </CheckboxGroup<Key>>
        </Form>
        <div id="gv-aria-group">{terms_group("Aria group", ValidationBehavior::Aria)}</div>
        <div id="gv-aria-items">{terms_items("Aria items", ValidationBehavior::Aria)}</div>
        <Form attr:id="gv-aria-server" validation_behavior=ValidationBehavior::Aria validation_errors=Signal::stored(HashMap::from([(
            "terms".to_owned(),
            vec!["You must accept the terms".to_owned()],
        )]))>
            <CheckboxGroup<Key> name="terms">
                <Label>"Aria server terms"</Label>
                <CheckboxField value="terms"><CheckboxButton>"Aria server terms A"</CheckboxButton></CheckboxField>
                <FieldError />
            </CheckboxGroup<Key>>
        </Form>
    }
}
