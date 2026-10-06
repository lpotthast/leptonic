use leptonic::{
    components::prelude::*,
    hooks::{Orientation, collections::Key},
};
use leptos::prelude::*;

#[component]
pub fn RadioGroupDemo() -> impl IntoView {
    let (selected, set_selected) = signal(Some(Key::from("Small")));

    view! {
        <RadioGroup
            label="Size"
            orientation=Orientation::Horizontal
            default_value="Small"
            on_change=move |value| set_selected.set(value)
        >
            <Radio value="Small">"Small"</Radio>
            <Radio value="Medium">"Medium"</Radio>
            <Radio value="Large">"Large"</Radio>
        </RadioGroup>
        <p class="demo-status">
            "Selected size: "{move || selected.get().map_or_else(|| "None".to_owned(), |v| v.to_string())}
        </p>
    }
}
