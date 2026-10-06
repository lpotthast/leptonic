use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldConceptDemo() -> impl IntoView {
    let name = RwSignal::new(String::new());

    view! {
        <div class="demo-form">
            <TextField label="Name" description="Pressing the label focuses the input." placeholder="Your name" state=name/>
        </div>
        <p class="demo-status">
            {move || name.with(|name| if name.is_empty() { "Hello, stranger!".to_owned() } else { format!("Hello, {name}!") })}
        </p>
    }
}
