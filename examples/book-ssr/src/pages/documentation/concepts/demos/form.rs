use leptonic::{
    atoms::form::Form,
    components::prelude::*,
    hooks::{ButtonType, InputType},
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
            <TextField label="Name" name="name" is_required=true value=name set_value=name/>
            <TextField
                label="Email"
                name="email"
                input_type=InputType::Email
                is_required=true
                value=email
                set_value=email
            />
            <div>
                <Button button_type=ButtonType::Submit>"Send invitation"</Button>
            </div>
        </Form>

        <p class="demo-status">
            {move || invited.get().map_or_else(|| "No invitation sent yet.".to_owned(), |to| format!("Invited {to}."))}
        </p>
    }
}
