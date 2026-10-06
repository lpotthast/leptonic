use leptonic::{components::prelude::*, hooks::Key};
use leptos::prelude::*;

#[component]
pub fn RadioConceptDemo() -> impl IntoView {
    let shipping = RwSignal::new(Some(Key::from("standard")));
    let disabled = RwSignal::new(false);

    view! {
        <RadioGroup label="Shipping" value=shipping set_value=shipping is_disabled=disabled>
            <Radio value="standard">"Standard shipping"</Radio>
            <Radio value="express">"Express shipping"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || shipping.get().map_or_else(|| "Nothing selected.".to_owned(), |shipping| format!("Selected: {shipping}."))}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
