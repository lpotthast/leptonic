use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchDisabledDemo() -> impl IntoView {
    let (backups, set_backups) = signal(true);
    let disabled = RwSignal::new(true);
    let read_only = RwSignal::new(false);

    view! {
        <Switch state=(backups, set_backups) is_disabled=disabled is_read_only=read_only>"Automatic backups"</Switch>
        <p class="demo-status">{move || if backups.get() { "Backups are on." } else { "Backups are off." }}</p>
        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
            <Checkbox state=read_only>"Read-only"</Checkbox>
        </div>
    }
}
