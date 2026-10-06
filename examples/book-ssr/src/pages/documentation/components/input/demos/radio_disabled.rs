use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioDisabledDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        <RadioGroup label="Plan" default_value="free" is_disabled=disabled>
            <Radio value="free">"Free"</Radio>
            <Radio value="pro">"Pro"</Radio>
            <Radio value="enterprise" is_disabled=true>"Enterprise (contact sales)"</Radio>
        </RadioGroup>
        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disable the group"</Checkbox>
        </div>
    }
}
