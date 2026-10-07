use leptonic::{
    atoms::{
        checkbox::Checkbox,
        field::Label,
        radio::{Radio, RadioGroup},
    },
    hooks::Key,
};
use leptos::prelude::*;

#[component]
pub fn RadioConceptDemo() -> impl IntoView {
    let shipping = RwSignal::new(Some(Key::from("standard")));
    let disabled = RwSignal::new(false);

    view! {
        // Each `Radio` renders a `<label>` around a visually hidden input; the children draw the circle.
        <RadioGroup value=shipping set_value=shipping is_disabled=disabled classes="demo-choice-group">
            <Label classes="demo-choice-group-label">"Shipping"</Label>
            <div class="demo-choice-group-items">
                <Radio value="standard" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "Standard shipping"
                </Radio>
                <Radio value="express" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "Express shipping"
                </Radio>
            </div>
        </RadioGroup>
        <p class="demo-status">
            {move || shipping.get().map_or_else(|| "Nothing selected.".to_owned(), |shipping| format!("Selected: {shipping}."))}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
