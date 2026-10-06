use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TabBasicDemo() -> impl IntoView {
    view! {
        <Tabs>
            <Tab name="overview" label=|| "Overview">"A short summary of the project."</Tab>
            <Tab name="activity" label=|| "Activity">"The latest changes."</Tab>
            <Tab name="settings" label=|| "Settings">"Project settings."</Tab>
        </Tabs>
    }
}
