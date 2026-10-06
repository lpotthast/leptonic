use std::{collections::HashMap, sync::Arc};

use leptonic::{
    atoms::prelude::Form,
    components::prelude::*,
    hooks::{ButtonType, InputType, ValidationBehavior},
};
use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn FormsDemo() -> impl IntoView {
    // Errors reported by the server, by field name.
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let (is_submitted, set_submitted) = signal(false);

    let simulate_server_error = move |_| {
        server_errors.set(HashMap::from([(
            "username".to_owned(),
            vec!["This username is taken.".to_owned()],
        )]));
    };

    view! {
        <Form
            validation_errors=server_errors
            classes="demo-form"
            on:submit=move |e: SubmitEvent| {
                // A real app would send the form to its server here.
                e.prevent_default();
                set_submitted.set(true);
            }
            on:reset=move |_| {
                server_errors.set(HashMap::new());
                set_submitted.set(false);
            }
        >
            // Shows errors while you type.
            <TextField
                label="Username"
                name="username"
                is_required=true
                validation_behavior=ValidationBehavior::Aria
                validate=Arc::new(|value: &String| {
                    if value.chars().count() >= 3 { Ok(()) } else { Err(vec!["At least 3 characters.".to_owned()]) }
                })
            />
            // Shows errors when the form is submitted, using the browser's constraint validation (the form's default).
            <TextField
                label="Email"
                name="email"
                input_type=InputType::Email
                is_required=true
                validate=Arc::new(|value: &String| {
                    if value.ends_with(".example") { Err(vec!["Use a real domain.".to_owned()]) } else { Ok(()) }
                })
            />
            <div class="demo-controls">
                <Button button_type=ButtonType::Submit>"Submit"</Button>
                <Button button_type=ButtonType::Reset variant=ButtonVariant::Outlined>"Reset"</Button>
                <Button variant=ButtonVariant::Flat on_press=simulate_server_error>"Simulate server error"</Button>
            </div>
        </Form>
        <p class="demo-status">{move || if is_submitted.get() { "Submitted." } else { "Not submitted yet." }}</p>
    }
}
