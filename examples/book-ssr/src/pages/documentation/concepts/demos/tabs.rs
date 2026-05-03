use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TabsConceptDemo() -> impl IntoView {
    view! {
        <Tabs>
            <Tab name="tab-1" label=|| "First">"Content of first tab"</Tab>
            <Tab name="tab-2" label=|| "Second">"Content of second tab"</Tab>
            <Tab name="tab-3" label=|| "Third">"Content of third tab"</Tab>
        </Tabs>
    }
}
