use std::sync::Arc;

use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        radio::{Radio, RadioGroup},
    },
    hooks::Key,
};
use leptos::prelude::*;

const OPTIONS: [(&str, &str); 3] = [
    ("standard", "Standard"),
    ("express", "Express"),
    ("overnight", "Overnight"),
];

#[component]
pub fn FieldPartsDemo() -> impl IntoView {
    let (shipping, set_shipping) = signal(Some(Key::from("standard")));

    view! {
        <RadioGroup
            default_value="standard"
            on_change=move |value| set_shipping.set(value)
            validate=Arc::new(|value: &Option<Key>| {
                if value.as_ref().is_some_and(|key| key.to_string() == "overnight") {
                    Err(vec!["Overnight shipping isn\u{2019}t available for your address.".to_owned()])
                } else {
                    Ok(())
                }
            })
            classes="demo-choice-group"
        >
            // Inside a group, the label is a `<span>` the group references with `aria-labelledby`.
            <Label classes="demo-choice-group-label">"Shipping"</Label>
            {OPTIONS
                .into_iter()
                .map(|(value, label)| view! {
                    <Radio value classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        {label}
                    </Radio>
                })
                .collect_view()}
            // Referenced by the group (`aria-describedby`) while it is rendered.
            <Description classes="demo-choice-group-description">
                "Express arrives within two business days."
            </Description>
            // Rendered only while the group is invalid, showing the validation errors.
            <FieldError classes="demo-choice-group-error"/>
        </RadioGroup>

        <p class="demo-status">
            {move || shipping.get().map_or_else(|| "No shipping".to_owned(), |key| format!("Shipping: {key}"))}
        </p>
    }
}
