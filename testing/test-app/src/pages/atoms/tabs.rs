use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::{
        KeyboardActivation, Orientation,
        collections::{Collection, Key},
    },
};
use leptos::prelude::*;

const TABS: [&str; 3] = ["a", "b", "c"];

/// Tabs A, B and C (react-aria-components' `Tabs.test.js` setup), configured per case. `name`
/// labels the tab list and prefixes the ids of the selection display and the button before it.
#[component]
fn TestTabs(
    name: &'static str,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(optional)] keyboard_activation: KeyboardActivation,
    #[prop(optional)] disabled_keys: Vec<&'static str>,
    /// Disabled in the collection.
    #[prop(optional)]
    disabled_tabs: Vec<&'static str>,
) -> impl IntoView {
    let collection = Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for tab in TABS {
                b.item(tab, tab.to_uppercase())
                    .disabled(disabled_tabs.contains(&tab));
            }
        }))
    });
    let selection = RwSignal::new(String::new());
    let disabled_keys: HashSet<Key> = disabled_keys.into_iter().map(Key::from).collect();
    view! {
        <button id=format!("test-tabs-{name}-before")>"Before"</button>
        <Tabs
            collection=collection
            orientation=orientation
            keyboard_activation=keyboard_activation
            disabled_keys=Signal::stored(disabled_keys)
            on_selection_change=Callback::new(move |key: Key| selection.set(key.to_string()))
        >
            <TabList aria_label=name>
                {TABS
                    .map(|tab| view! { <Tab key=tab>{tab.to_uppercase()}</Tab> })
                    .collect_view()}
            </TabList>
            {TABS
                .map(|tab| {
                    view! { <TabPanel key=tab>{format!("Panel {}", tab.to_uppercase())}</TabPanel> }
                })
                .collect_view()}
        </Tabs>
        <div>"Selection: " <span id=format!("test-tabs-{name}-selection")>{selection}</span></div>
    }
}

/// Tabs in different configurations: basic, a disabled tab, a disabled first tab, all tabs
/// disabled, vertical, manual activation.
#[component]
pub fn PageAtomTabs() -> impl IntoView {
    view! {
        <div id="test-page-atom-tabs">
            <h1>"Tabs"</h1>
            <TestTabs name="basic" />
            <TestTabs name="disabled-tab" disabled_tabs=vec!["b"] />
            <TestTabs name="first-disabled" disabled_keys=vec!["a"] />
            <TestTabs name="all-disabled" disabled_tabs=TABS.to_vec() />
            <TestTabs name="vertical" orientation=Orientation::Vertical />
            <TestTabs name="manual" keyboard_activation=KeyboardActivation::Manual />
        </div>
    }
}
