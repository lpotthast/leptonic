use std::{collections::HashMap, sync::Arc};

use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        form::Form,
        input::{Input, TextArea},
        text_field::TextField,
    },
    hooks::form::{ValidateFn, ValidationBehavior, ValidationResult},
};
use leptos::{ev::SubmitEvent, prelude::*};

/// TextField atoms, as react-aria-components' `TextField` and `Form` tests render them. Each
/// case sits in its own element with an id.
#[component]
pub fn PageAtomTextField() -> impl IntoView {
    view! {
        <div id="test-page-atom-text-field">
            <h1>"TextField"</h1>

            // Label, description and (invalid) error message around an input and a textarea.
            <div id="tf-slots-input">
                <TextField default_value="test" is_invalid=true attr:data-foo="bar">
                    <Label>"Test"</Label>
                    <Input />
                    <Description>"Description"</Description>
                    <FieldError>"Error"</FieldError>
                </TextField>
            </div>
            <div id="tf-slots-textarea">
                <TextField default_value="test" is_invalid=true attr:data-foo="bar">
                    <Label>"Test"</Label>
                    <TextArea />
                    <Description>"Description"</Description>
                    <FieldError>"Error"</FieldError>
                </TextField>
            </div>

            <div id="tf-read-only">
                <TextField default_value="test" is_read_only=true>
                    <Label>"Read-only"</Label>
                    <Input />
                </TextField>
            </div>
            <div id="tf-required">
                <TextField is_required=true validation_behavior=ValidationBehavior::Aria>
                    <Label>"Required"</Label>
                    <Input />
                </TextField>
            </div>

            // Native validation: the error shows once the form is checked.
            <form id="tf-native-input">
                <TextField is_required=true>
                    <Label>"Test"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </form>
            <form id="tf-native-textarea">
                <TextField is_required=true>
                    <Label>"Test"</Label>
                    <TextArea />
                    <FieldError />
                </TextField>
            </form>
            // An error message chosen by the validation details.
            <form id="tf-custom-error">
                <TextField is_required=true>
                    <Label>"Test"</Label>
                    <Input />
                    <FieldError message=Arc::new(|result: &ValidationResult| {
                        result
                            .validation_details
                            .value_missing
                            .then(|| "Please enter a name".to_owned())
                    }) />
                </TextField>
            </form>
            // Invalid without an error message: no error element.
            <form id="tf-invalid-without-message">
                <TextField is_required=true is_invalid=true>
                    <Label>"Test"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </form>

            <div id="tf-id">
                <TextField default_value="test" id="name">
                    <Label>"Test"</Label>
                    <Input />
                </TextField>
            </div>
            // The field outside the form it belongs to.
            <form id="tf-form"></form>
            <div id="tf-form-attribute">
                <TextField form="tf-form">
                    <Label>"Test"</Label>
                    <Input />
                </TextField>
            </div>

            // Bound values: one without a setter rejects every change, one whose setter
            // uppercases changes the text.
            <div id="tf-rejecting">
                <TextField value=Signal::stored("fixed".to_owned())>
                    <Label>"Fixed"</Label>
                    <Input />
                </TextField>
            </div>
            <UppercaseField />

            <ServerErrorsForm />
            <TextFieldValidation />

            <Form attr:id="form-native">
                <TextField name="name" is_required=true>
                    <Label>"Name"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </Form>
            <Form attr:id="form-aria" validation_behavior=ValidationBehavior::Aria>
                <TextField name="name" is_required=true>
                    <Label>"Name"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </Form>
            <Form attr:id="form-native-field-aria">
                <TextField name="name" is_required=true validation_behavior=ValidationBehavior::Aria>
                    <Label>"Name"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </Form>
            <Form attr:id="form-aria-field-native" validation_behavior=ValidationBehavior::Aria>
                <TextField
                    name="name"
                    is_required=true
                    validation_behavior=ValidationBehavior::Native
                >
                    <Label>"Name"</Label>
                    <Input />
                    <FieldError />
                </TextField>
            </Form>

            <StandaloneInputs />
        </div>
    }
}

/// `Input` and `TextArea` outside a field: plain elements (the first input mirrors its value),
/// and a disabled, invalid input.
#[component]
fn StandaloneInputs() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <div id="tf-standalone">
            <Input
                attr:aria-label="Standalone"
                on:input=move |e| value.set(event_target_value(&e))
            />
            <output>{value}</output>
            <Input is_disabled=true is_invalid=true attr:aria-label="Standalone disabled" />
            <TextArea attr:aria-label="Standalone notes" />
        </div>
    }
}

/// Submitting sets a server error for the field `name`.
#[component]
fn ServerErrorsForm() -> impl IntoView {
    let errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        errors.set(HashMap::from([(
            "name".to_owned(),
            vec!["Invalid name.".to_owned()],
        )]));
    };
    view! {
        <Form attr:id="form-server" validation_errors=errors on:submit=on_submit>
            <TextField name="name">
                <Label>"Name"</Label>
                <Input />
                <FieldError />
            </TextField>
            <button id="form-server-submit" type="submit">"Submit"</button>
        </Form>
    }
}

/// A text field bound to a signal whose setter stores the text uppercased.
#[component]
fn UppercaseField() -> impl IntoView {
    let text = RwSignal::new(String::new());
    view! {
        <div id="tf-uppercase">
            <TextField value=text set_value=move |value: String| text.set(value.to_uppercase())>
                <Label>"Uppercase"</Label>
                <Input />
            </TextField>
        </div>
    }
}

/// Validators, a canceled `invalid` event, server errors and a disabled field (react-spectrum's
/// `TextField.test.js` validation and state cases).
#[component]
fn TextFieldValidation() -> impl IntoView {
    let not_foo = || -> ValidateFn<String> {
        Arc::new(|v: &String| {
            if v == "Foo" {
                Err(vec!["Invalid name".to_owned()])
            } else {
                Ok(())
            }
        })
    };
    view! {
        <Form attr:id="tfv-validate">
            <TextField default_value="Foo" validate=not_foo()>
                <Label>"Name"</Label>
                <Input />
                <FieldError />
            </TextField>
        </Form>
        // The form cancels its fields' `invalid` events: no field takes the focus.
        <Form attr:id="tfv-prevented" on:invalid:capture=|e| e.prevent_default()>
            <TextField is_required=true>
                <Label>"Name"</Label>
                <Input />
                <FieldError />
            </TextField>
        </Form>
        <Form attr:id="tfv-aria-validate" validation_behavior=ValidationBehavior::Aria>
            <TextField default_value="Foo" validate=not_foo()>
                <Label>"Name"</Label>
                <Input />
                <FieldError />
            </TextField>
        </Form>
        <Form
            attr:id="tfv-aria-server"
            validation_behavior=ValidationBehavior::Aria
            validation_errors=Signal::stored(HashMap::from([(
                "name".to_owned(),
                vec!["Invalid name".to_owned()],
            )]))
        >
            <TextField name="name">
                <Label>"Name"</Label>
                <Input />
                <FieldError />
            </TextField>
        </Form>
        <div id="tf-disabled">
            <TextField default_value="test" is_disabled=true>
                <Label>"Disabled"</Label>
                <Input />
            </TextField>
        </div>
    }
}
