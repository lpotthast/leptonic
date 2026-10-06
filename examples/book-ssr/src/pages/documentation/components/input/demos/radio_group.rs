use leptonic::{
    components::prelude::*,
    hooks::{Key, Orientation},
};
use leptos::prelude::*;

#[component]
pub fn RadioGroupDemo() -> impl IntoView {
    let size = RwSignal::new(Some(Key::from("Small")));

    view! {
        <RadioGroup label="Size" orientation=Orientation::Horizontal value=size set_value=size>
            <Radio value="Small">"Small"</Radio>
            <Radio value="Medium">"Medium"</Radio>
            <Radio value="Large">"Large"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || size.get().map_or_else(|| "No size selected.".to_owned(), |size| format!("Size: {size}."))}
        </p>
    }
}
