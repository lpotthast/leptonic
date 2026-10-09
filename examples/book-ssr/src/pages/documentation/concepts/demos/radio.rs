use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        field::Label,
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

#[component]
pub fn RadioConceptDemo() -> impl IntoView {
    let shipping = RwSignal::new(Some(Key::from("standard")));
    let disabled = RwSignal::new(false);

    view! {
        // Each `RadioButton` renders a `<label>` around a visually hidden input; its children draw the circle.
        <RadioGroup value=shipping set_value=shipping is_disabled=disabled classes="demo-choice-group">
            <Label classes="demo-choice-group-label">"Shipping"</Label>
            <div class="demo-choice-group-items">
                <RadioField value="standard">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "Standard shipping"
                    </RadioButton>
                </RadioField>
                <RadioField value="express">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "Express shipping"
                    </RadioButton>
                </RadioField>
            </div>
        </RadioGroup>
        <p class="demo-status">
            {move || shipping.get().map_or_else(|| "Nothing selected.".to_owned(), |shipping| format!("Selected: {shipping}."))}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
