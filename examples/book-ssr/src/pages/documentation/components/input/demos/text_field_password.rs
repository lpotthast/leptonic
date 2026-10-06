use leptonic::{components::prelude::*, hooks::InputType};
use leptos::prelude::*;

#[component]
pub fn TextFieldPasswordDemo() -> impl IntoView {
    let password = RwSignal::new(String::new());

    view! {
        <div class="demo-form">
            <TextField label="Username" auto_complete="username"/>
            <TextField label="Password" input_type=InputType::Password auto_complete="current-password" state=password/>
        </div>
        <p class="demo-status">
            {move || format!("The password has {} characters.", password.with(|password| password.chars().count()))}
        </p>
    }
}
