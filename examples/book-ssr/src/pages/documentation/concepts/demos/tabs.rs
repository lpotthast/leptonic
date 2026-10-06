use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::use_collection,
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

    view! {
        <Tabs collection=collection>
            <TabList aria_label="Project" classes="demo-tab-list">
                <Tab key="overview" classes=["demo-tab", "demo-atom-tab"]>"Overview"</Tab>
                <Tab key="activity" classes=["demo-tab", "demo-atom-tab"]>"Activity"</Tab>
                <Tab key="members" classes=["demo-tab", "demo-atom-tab"]>"Members"</Tab>
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
    }
}
