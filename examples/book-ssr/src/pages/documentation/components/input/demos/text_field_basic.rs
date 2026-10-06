use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldBasicDemo() -> impl IntoView {
    let (name, set_name) = signal(String::new());

    view! {
        <div class="demo-form">
            <TextField label="Name" placeholder="Ferris" on_change=move |value: String| set_name.set(value)/>
        </div>
        <p class="demo-status">
            {move || name.with(|name| if name.is_empty() { "Hello, stranger!".to_owned() } else { format!("Hello, {name}!") })}
        </p>
    }
}
