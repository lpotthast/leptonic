use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonDisabledDemo() -> impl IntoView {
    let disabled = RwSignal::new(true);
    let sent = RwSignal::new(false);

    view! {
        <Button is_disabled=disabled on_press=move |_| sent.set(true)>"Send"</Button>
        <p class="demo-status">{move || if sent.get() { "Sent." } else { "Not sent yet." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
