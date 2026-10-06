use leptonic::{components::prelude::*, hooks::collections::Key};
use leptos::prelude::*;

#[component]
pub fn RadioBasicDemo() -> impl IntoView {
    let (selected, set_selected) = signal(None::<Key>);

    view! {
        <RadioGroup aria_label="Answer" on_change=move |value| set_selected.set(value)>
            <Radio value="yes">"Yes"</Radio>
            <Radio value="no">"No"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || selected.get().map_or_else(|| "Nothing selected".to_owned(), |v| format!("Selected: {v}"))}
        </p>
    }
}
