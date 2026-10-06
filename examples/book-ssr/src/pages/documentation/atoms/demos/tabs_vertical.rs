use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::{Key, KeyboardActivation, Orientation, use_collection},
};
use leptos::prelude::*;

const TABS: [(&str, &str, &str); 3] = [
    ("profile", "Profile", "Your name, photo and bio."),
    (
        "security",
        "Security",
        "Password and two-factor authentication.",
    ),
    ("billing", "Billing", "Payment methods and invoices."),
];

#[component]
pub fn TabsVerticalAtomDemo() -> impl IntoView {
    let selected = RwSignal::new(Key::from("profile"));
    let collection = use_collection(|b| {
        for (key, label, _) in TABS {
            b.item(key, label);
        }
    });

    // Vertical tabs with manual activation: the arrow keys move focus, Enter or Space selects.
    view! {
        <Tabs
            collection=collection
            orientation=Orientation::Vertical
            keyboard_activation=KeyboardActivation::Manual
            selected_key=selected
            set_selected_key=selected
            classes="demo-tabs"
        >
            <TabList aria_label="Settings" classes="demo-tab-list">
                {TABS
                    .map(|(key, label, _)| view! { <Tab key=key classes="demo-tab">{label}</Tab> })
                    .collect_view()}
            </TabList>
            {TABS
                .map(|(key, _, text)| {
                    view! {
                        <TabPanel key=key classes="demo-tab-panel">
                            <p>{text}</p>
                        </TabPanel>
                    }
                })
                .collect_view()}
        </Tabs>

        <p class="demo-status">"Selected: " {move || selected.get().to_string()}</p>
    }
}
