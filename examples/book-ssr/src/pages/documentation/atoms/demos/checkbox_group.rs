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
    let days = RwSignal::new(vec![Key::from("Monday")]);
    let disabled = RwSignal::new(false);

    view! {
        <CheckboxGroup
            value=days
            set_value=days
            is_required=true
            validate=Arc::new(|days: &Vec<Key>| {
                if days.is_empty() { Err(vec!["Choose at least one day.".to_owned()]) } else { Ok(()) }
            })
            is_disabled=disabled
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
                if days.is_empty() { "No office days.".to_owned() } else { format!("Office days: {}.", days.join(", ")) }
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
