use std::sync::Arc;

use leptonic::{
    atoms::{
        checkbox::{Checkbox, CheckboxGroup},
        field::{Description, FieldError, Label},
    },
    hooks::Key,
};
use leptos::prelude::*;

const DAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

#[component]
pub fn CheckboxGroupAtomDemo() -> impl IntoView {
    let (days, set_days) = signal(vec![Key::from("Monday")]);

    view! {
        <CheckboxGroup
            default_value=vec![Key::from("Monday")]
            on_change=move |value| set_days.set(value)
            is_required=true
            validate=Arc::new(|days: &Vec<Key>| {
                if days.is_empty() { Err(vec!["Choose at least one day.".to_owned()]) } else { Ok(()) }
            })
            classes="demo-choice-group"
        >
            <Label classes="demo-choice-group-label">"Office days"</Label>
            {DAYS
                .into_iter()
                .map(|day| view! {
                    <Checkbox value=day classes="demo-check">
                        <span class="demo-check-box" aria-hidden="true"></span>
                        {day}
                    </Checkbox>
                })
                .collect_view()}
            <Description classes="demo-choice-group-description">
                "The days you work from the office."
            </Description>
            // Shown while the group is invalid; without children, it shows the validation errors.
            <FieldError classes="demo-choice-group-error"/>
        </CheckboxGroup>

        <p class="demo-status">
            {move || {
                let days = days.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                if days.is_empty() { "No office days".to_owned() } else { days.join(", ") }
            }}
        </p>
    }
}
