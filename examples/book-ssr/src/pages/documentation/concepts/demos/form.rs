use leptonic::{
    atoms::{
        button::Button,
        field::{FieldError, Label},
        form::Form,
        input::Input,
        text_field::TextField,
    },
    hooks::{button::ButtonType, form::InputType},
};
use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn FormConceptDemo() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let (invited, set_invited) = signal(None::<String>);

    view! {
        // Submitting runs the fields' validation; `on:submit` only runs once all of them are valid.
        <Form
            classes="demo-form"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                set_invited.set(Some(format!("{} <{}>", name.get_untracked(), email.get_untracked())));
            }
        >
            <TextField name="name" is_required=true value=name set_value=name classes="demo-field">
                <Label classes="demo-field-label">"Name"</Label>
                <Input classes="demo-atom-input"/>
                <FieldError classes="demo-field-error"/>
            </TextField>
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
                <FieldError classes="demo-field-error"/>
            </TextField>
            <div>
                <Button button_type=ButtonType::Submit classes="demo-btn-primary">"Send invitation"</Button>
            </div>
        </Form>

        <p class="demo-status">
            {move || invited.get().map_or_else(|| "No invitation sent yet.".to_owned(), |to| format!("Invited {to}."))}
        </p>
    }
}
