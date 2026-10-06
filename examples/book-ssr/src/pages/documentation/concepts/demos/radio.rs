use leptonic::{components::prelude::*, hooks::collections::Key};
use leptos::prelude::*;

#[component]
pub fn RadioConceptDemo() -> impl IntoView {
    let (selected, set_selected) = signal(Some(Key::from("standard")));

    view! {
        <RadioGroup label="Shipping" default_value="standard" on_change=move |value| set_selected.set(value)>
            <Radio value="standard">"Standard shipping"</Radio>
            <Radio value="express">"Express shipping"</Radio>
        </RadioGroup>
        <p>
            "Selected: "
            {move || selected.get().map_or_else(|| "none".to_owned(), |v| v.to_string())}
        </p>
    }
}
