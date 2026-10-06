use leptonic::{components::prelude::*, hooks::Key};
use leptos::prelude::*;

#[component]
pub fn RadioDisabledDemo() -> impl IntoView {
    let plan = RwSignal::new(Some(Key::from("free")));
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        <RadioGroup label="Plan" value=plan set_value=plan is_disabled=disabled is_read_only=read_only>
            <Radio value="free">"Free"</Radio>
            <Radio value="pro">"Pro"</Radio>
            <Radio value="enterprise" is_disabled=true>"Enterprise (contact sales)"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || plan.get().map_or_else(|| "No plan selected.".to_owned(), |plan| format!("Plan: {plan}."))}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
        </div>
    }
}
