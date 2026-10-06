use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldConceptDemo() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-form">
            <TextField
                label="Name"
                description="Pressing the label focuses the input."
                placeholder="Your name"
                value=name
                set_value=name
                is_disabled=disabled
            />
        </div>
        <p class="demo-status">
            {move || name.with(|name| if name.is_empty() { "Hello, stranger!".to_owned() } else { format!("Hello, {name}!") })}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
