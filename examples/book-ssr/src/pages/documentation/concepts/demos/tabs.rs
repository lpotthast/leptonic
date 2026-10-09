use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        tabs::{Tab, TabList, TabPanel, Tabs},
    },
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

#[component]
pub fn TabsConceptDemo() -> impl IntoView {
    // The tabs are a collection: a key and a text per tab.
    let collection = use_collection(|b| {
        b.item("overview", "Overview");
        b.item("activity", "Activity");
        b.item("members", "Members");
    });
    let selected = RwSignal::new(Key::from("overview"));
    let disabled = RwSignal::new(false);

    view! {
        <Tabs collection=collection selected_key=selected set_selected_key=selected is_disabled=disabled classes="demo-tabs">
            <TabList aria_label="Project" classes="demo-tab-list">
                <Tab key="overview" classes="demo-tab">"Overview"</Tab>
                <Tab key="activity" classes="demo-tab">"Activity"</Tab>
                <Tab key="members" classes="demo-tab">"Members"</Tab>
            </TabList>
            <TabPanel key="overview" classes="demo-tab-panel">
                <p>"Three open milestones, due next month."</p>
            </TabPanel>
            <TabPanel key="activity" classes="demo-tab-panel">
                <p>"Ada merged a pull request an hour ago."</p>
            </TabPanel>
            <TabPanel key="members" classes="demo-tab-panel">
                <p>"Ada, Grace and Linus."</p>
            </TabPanel>
        </Tabs>
        <p class="demo-status">{move || format!("Selected tab: {}.", selected.get())}</p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
