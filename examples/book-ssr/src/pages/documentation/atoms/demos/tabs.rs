use std::collections::HashSet;

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    components::prelude::{Button, ButtonColor, Checkbox},
    hooks::{Key, use_collection},
};
use leptos::prelude::*;

const TABS: [(&str, &str); 4] = [
    ("details", "Details"),
    ("specs", "Specs"),
    ("reviews", "Reviews"),
    ("shipping", "Shipping"),
];

#[component]
pub fn TabsAtomDemo() -> impl IntoView {
    let reviews_disabled = RwSignal::new(true);
    let all_disabled = RwSignal::new(false);
    // App state: the selected tab. The tabs show it, and selecting a tab writes it.
    let selected = RwSignal::new(Key::from("details"));

    let collection = use_collection(|b| {
        for (key, label) in TABS {
            b.item(key, label);
        }
    });
    // Disabled tabs can't be selected, and the arrow keys skip them.
    let disabled_keys = Signal::derive(move || {
        if reviews_disabled.get() {
            HashSet::from([Key::from("reviews")])
        } else {
            HashSet::new()
        }
    });

    view! {
        <Tabs
            collection=collection
            disabled_keys=disabled_keys
            is_disabled=all_disabled
            selected_key=selected
        >
            <TabList aria_label="Product" classes="demo-tab-list">
                {TABS
                    .map(|(key, label)| view! { <Tab key=key classes=["demo-tab", "demo-atom-tab"]>{label}</Tab> })
                    .collect_view()}
            </TabList>
            <TabPanel key="details" classes="demo-tab-panel">
                <h4>"Details"</h4>
                <p>"A lightweight, foldable reading lamp."</p>
            </TabPanel>
            <TabPanel key="specs" classes="demo-tab-panel">
                <h4>"Specs"</h4>
                <p>"Aluminium frame, 1.2 kg, USB-C charging."</p>
            </TabPanel>
            <TabPanel key="reviews" classes="demo-tab-panel">
                <h4>"Reviews"</h4>
                <p>"4.6 out of 5 stars from 212 reviews."</p>
            </TabPanel>
            <TabPanel key="shipping" classes="demo-tab-panel">
                <h4>"Shipping"</h4>
                <p>"Ships within two business days."</p>
            </TabPanel>
        </Tabs>

        <div class="demo-tabs-controls">
            <Checkbox state=reviews_disabled>"Disable \u{201c}Reviews\u{201d}"</Checkbox>
            <Checkbox state=all_disabled>"Disable all tabs"</Checkbox>
            // The app changes the selected tab by writing its state.
            <Button on_press=move |_| selected.set(Key::from("shipping")) color=ButtonColor::Secondary>
                "Show shipping"
            </Button>
            <p class="demo-tabs-status">"Selected: " {move || selected.get().to_string()}</p>
        </div>
    }
}
