use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TabReactiveLabelDemo() -> impl IntoView {
    let (notifications, set_notifications) = signal(true);

    view! {
        <Tabs>
            // The label function runs once. Return a closure from it to keep the label reactive.
            <Tab
                name="notifications"
                label=move || move || if notifications.get() { "Notifications (on)" } else { "Notifications (off)" }
            >
                <Switch is_selected=notifications set_selected=set_notifications>"Notifications"</Switch>
            </Tab>
            <Tab name="privacy" label=|| "Privacy">"Privacy settings."</Tab>
        </Tabs>
    }
}
