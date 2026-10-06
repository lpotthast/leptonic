use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchDisabledDemo() -> impl IntoView {
    let backups = RwSignal::new(true);
    let disabled = RwSignal::new(true);
    let read_only = RwSignal::new(false);

    view! {
        <Switch is_selected=backups set_selected=backups is_disabled=disabled is_read_only=read_only>"Automatic backups"</Switch>
        <p class="demo-status">{move || if backups.get() { "Backups are on." } else { "Backups are off." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
        </div>
    }
}
