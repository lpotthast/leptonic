use std::collections::HashMap;

use leptonic::{
    atoms::{
        button::Button,
        field::{Description, FieldError, Label},
        form::Form,
        input::Input,
        text_field::TextField,
    },
    hooks::{ButtonType, InputType},
};
use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn FormAtomDemo() -> impl IntoView {
    let email = RwSignal::new(String::new());
    // Errors reported by the server, by field name.
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let (subscribed, set_subscribed) = signal(None::<String>);

    view! {
        // Native validation (the default): errors appear when the form is submitted, and focus
        // moves to the first invalid field.
        <Form
            validation_errors=server_errors
            classes="demo-form"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                set_subscribed.set(Some(email.get_untracked()));
            }
            on:reset=move |_| {
                server_errors.set(HashMap::new());
                set_subscribed.set(None);
            }
        >
            <TextField
                name="email"
                input_type=InputType::Email
                is_required=true
                value=email
                set_value=email
                classes="demo-field"
            >
                <Label classes="demo-field-label">"Email"</Label>
                <Input classes="demo-atom-input"/>
                <Description classes="demo-field-description">"We send the newsletter to this address."</Description>
                <FieldError classes="demo-field-error"/>
            </TextField>
            <div class="demo-flex-center-row">
                <Button button_type=ButtonType::Submit classes="demo-btn-primary">"Subscribe"</Button>
                <Button button_type=ButtonType::Reset classes="demo-btn">"Reset"</Button>
                <Button
                    classes="demo-btn"
                    on_press=move |_| server_errors.set(HashMap::from([(
                        "email".to_owned(),
                        vec!["This address is already subscribed.".to_owned()],
                    )]))
                >
                    "Simulate server error"
                </Button>
            </div>
        </Form>

        <p class="demo-status">
            {move || subscribed.get().map_or_else(|| "Not subscribed yet.".to_owned(), |email| format!("Subscribed: {email}"))}
        </p>
    }
}
