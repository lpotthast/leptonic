use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxDisabledDemo() -> impl IntoView {
    let remember = RwSignal::new(true);
    let disabled = RwSignal::new(true);
    let read_only = RwSignal::new(false);

    view! {
        <Checkbox is_selected=remember set_selected=remember is_disabled=disabled is_read_only=read_only>"Remember me"</Checkbox>
        <p class="demo-status">{move || if remember.get() { "Remembered." } else { "Not remembered." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
        </div>
    }
}
