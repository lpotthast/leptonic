use leptonic::{components::prelude::*, hooks::InputType};
use leptos::prelude::*;

#[component]
pub fn TextFieldPasswordDemo() -> impl IntoView {
    let password = RwSignal::new(String::new());

    view! {
        <div class="demo-form">
            <TextField label="Username" auto_complete="username"/>
            <TextField label="Password" input_type=InputType::Password auto_complete="current-password" value=password set_value=password/>
        </div>
        <p class="demo-status">
            {move || match password.with(|password| password.chars().count()) {
                0 => "No password entered.".to_owned(),
                1 => "The password has 1 character.".to_owned(),
                count => format!("The password has {count} characters."),
            }}
        </p>
    }
}
