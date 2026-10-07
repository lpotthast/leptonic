use leptonic::atoms::{
    checkbox::Checkbox,
    field::{Description, Label},
    input::Input,
    text_field::TextField,
};
use leptos::prelude::*;

#[component]
pub fn TextFieldConceptDemo() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);

    view! {
        <TextField value=name set_value=name placeholder="Your name" is_disabled=disabled classes="demo-field">
            <Label classes="demo-field-label">"Name"</Label>
            <Input classes="demo-atom-input"/>
            <Description classes="demo-field-description">"Pressing the label focuses the input."</Description>
        </TextField>
        <p class="demo-status">
            {move || name.with(|name| if name.is_empty() { "Hello, stranger!".to_owned() } else { format!("Hello, {name}!") })}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
