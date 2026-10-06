use leptonic::{components::prelude::*, hooks::Key};
use leptos::prelude::*;

#[component]
pub fn RadioLabeledDemo() -> impl IntoView {
    let shipping = RwSignal::new(Some(Key::from("standard")));

    view! {
        <RadioGroup label="Shipping" description="Express arrives the next day." value=shipping set_value=shipping>
            <Radio value="standard">"Standard shipping"</Radio>
            <Radio value="express">"Express shipping"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || shipping.get().map_or_else(|| "Nothing selected.".to_owned(), |shipping| format!("Shipping: {shipping}."))}
        </p>
    }
}
