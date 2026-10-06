use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxDisabledDemo() -> impl IntoView {
    let (remember, set_remember) = signal(true);
    let disabled = RwSignal::new(true);
    let read_only = RwSignal::new(false);

    view! {
        <Checkbox state=(remember, set_remember) is_disabled=disabled is_read_only=read_only>"Remember me"</Checkbox>
        <p class="demo-status">{move || if remember.get() { "Remembered" } else { "Not remembered" }}</p>
        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
            <Checkbox state=read_only>"Read-only"</Checkbox>
        </div>
    }
}
