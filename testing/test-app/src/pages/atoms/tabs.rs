use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, TabPanels, Tabs},
    hooks::{
        KeyboardActivation, Orientation,
        collections::{Collection, Key},
    },
    utils::i18n::{I18nProvider, Locale},
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
    /// Disabled by the `Tab`'s `is_disabled`.
    #[prop(optional)]
    tab_disabled: Vec<&'static str>,
    #[prop(optional)] should_force_mount: bool,
    /// The selected key, bound.
    #[prop(optional)]
    selected: Option<RwSignal<Key>>,
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
    // Rendered inside the `Tabs` (their contexts).
    let tabs = move || {
        TABS.map(|tab| {
            view! {
                <Tab key=tab is_disabled=tab_disabled.contains(&tab)>{tab.to_uppercase()}</Tab>
            }
        })
        .collect_view()
    };
    let panels = move || {
        TABS.map(|tab| {
            view! {
                <TabPanel key=tab should_force_mount=should_force_mount>
                    {format!("Panel {}", tab.to_uppercase())}
                </TabPanel>
            }
        })
        .collect_view()
    };
    let on_selection_change = Callback::new(move |key: Key| selection.set(key.to_string()));
    let tabs = match selected {
        Some(selected) => view! {
            <Tabs
                collection=collection
                orientation=orientation
                keyboard_activation=keyboard_activation
                disabled_keys=Signal::stored(disabled_keys)
                selected_key=selected
                set_selected_key=selected
                on_selection_change=on_selection_change
            >
                <TabList aria_label=name>{tabs()}</TabList>
                {panels()}
            </Tabs>
        }
        .into_any(),
        None => view! {
            <Tabs
                collection=collection
                orientation=orientation
                keyboard_activation=keyboard_activation
                disabled_keys=Signal::stored(disabled_keys)
                on_selection_change=on_selection_change
            >
                <TabList aria_label=name>{tabs()}</TabList>
                {panels()}
            </Tabs>
        }
        .into_any(),
    };
    view! {
        <button id=format!("test-tabs-{name}-before")>"Before"</button>
        {tabs}
        <div>"Selection: " <span id=format!("test-tabs-{name}-selection")>{selection}</span></div>
    }
}

/// "can add tabs and keep the current selected key": tabs "Tab 1".."Tab n" with a bound selected
/// key; `#test-tabs-dynamic-add` adds a tab and selects it, `#test-tabs-dynamic-remove` removes
/// the last one and selects the new last one. Selections by the user are counted in
/// `#test-tabs-dynamic-changes`.
#[component]
fn DynamicTabs() -> impl IntoView {
    let count = RwSignal::new(3usize);
    let selected = RwSignal::new(Key::from("1"));
    let changes = RwSignal::new(0u32);
    let collection = Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 1..=count.get() {
                b.item(i.to_string(), format!("Tab {i}"));
            }
        }))
    });
    view! {
        <Tabs
            collection=collection
            selected_key=selected
            set_selected_key=selected
            on_selection_change=move |_| changes.update(|c| *c += 1)
        >
            <TabList aria_label="Dynamic tabs">
                <For each=move || 1..=count.get() key=|i| *i let:i>
                    <Tab key=i.to_string()>{format!("Tab {i}")}</Tab>
                </For>
            </TabList>
            <button
                id="test-tabs-dynamic-add"
                on:click=move |_| {
                    count.update(|c| *c += 1);
                    selected.set(Key::from(count.get_untracked().to_string()));
                }
            >
                "Add tab"
            </button>
            <button
                id="test-tabs-dynamic-remove"
                on:click=move |_| {
                    if count.get_untracked() > 1 {
                        count.update(|c| *c -= 1);
                        selected.set(Key::from(count.get_untracked().to_string()));
                    }
                }
            >
                "Remove tab"
            </button>
            <For each=move || 1..=count.get() key=|i| *i let:i>
                <TabPanel key=i.to_string()>{format!("Tab body {i}")}</TabPanel>
            </For>
        </Tabs>
        <div>"Changes: " <span id="test-tabs-dynamic-changes">{changes}</span></div>
    }
}

/// "supports nested tabs": tabs Foo and Bar, Foo's panel holding tabs One and Two.
#[component]
fn NestedTabs() -> impl IntoView {
    let outer = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.item("foo", "Foo");
            b.item("bar", "Bar");
        }))
    });
    let inner = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.item("one", "One");
            b.item("two", "Two");
        }))
    });
    view! {
        <Tabs collection=outer>
            <TabList aria_label="Outer">
                <Tab key="foo">"Foo"</Tab>
                <Tab key="bar">"Bar"</Tab>
            </TabList>
            <TabPanel key="foo">
                <Tabs collection=inner>
                    <TabList aria_label="Inner">
                        <Tab key="one">"One"</Tab>
                        <Tab key="two">"Two"</Tab>
                    </TabList>
                    <TabPanel key="one">"Panel One"</TabPanel>
                    <TabPanel key="two">"Panel Two"</TabPanel>
                </Tabs>
            </TabPanel>
            <TabPanel key="bar">"Panel Bar"</TabPanel>
        </Tabs>
    }
}

/// "Short" and "Tall" tabs whose panels are in `TabPanels` (`.test-tab-panels`), animating its
/// height for a second.
#[component]
fn AnimatedTabs() -> impl IntoView {
    let collection = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.item("short", "Short");
            b.item("tall", "Tall");
        }))
    });
    view! {
        <style>
            ".test-tab-panels { overflow: hidden; height: var(--tab-panel-height); transition: height 1s; }"
        </style>
        <Tabs collection=collection>
            <TabList aria_label="Animated">
                <Tab key="short">"Short"</Tab>
                <Tab key="tall">"Tall"</Tab>
            </TabList>
            <TabPanels classes="test-tab-panels">
                <TabPanel key="short">"Short panel"</TabPanel>
                <TabPanel key="tall">
                    <div style="height: 300px">"Tall panel"</div>
                </TabPanel>
            </TabPanels>
        </Tabs>
    }
}

/// Tabs in different configurations: basic, a disabled tab, a disabled first tab, all tabs
/// disabled, vertical, manual activation, vertical right-to-left, force-mounted panels, tabs
/// disabled by `Tab::is_disabled`, a bound selected key (`#test-tabs-controlled-select-c`
/// selects C from outside; B is disabled), dynamic, nested and animated tabs.
#[component]
pub fn PageAtomTabs() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    let controlled = RwSignal::new(Key::from("b"));
    view! {
        <div id="test-page-atom-tabs">
            <h1>"Tabs"</h1>
            <TestTabs name="basic" />
            <TestTabs name="disabled-tab" disabled_tabs=vec!["b"] />
            <TestTabs name="first-disabled" disabled_keys=vec!["a"] />
            <TestTabs name="all-disabled" disabled_tabs=TABS.to_vec() />
            <TestTabs name="vertical" orientation=Orientation::Vertical />
            <TestTabs name="manual" keyboard_activation=KeyboardActivation::Manual />
            <I18nProvider locale=rtl>
                <TestTabs name="rtl-vertical" orientation=Orientation::Vertical />
            </I18nProvider>
            <TestTabs name="force" should_force_mount=true />
            <TestTabs name="tab-disabled" tab_disabled=vec!["b"] />
            <TestTabs name="tab-first-disabled" tab_disabled=vec!["a"] />
            <TestTabs name="controlled" disabled_keys=vec!["b"] selected=controlled />
            <button id="test-tabs-controlled-select-c" on:click=move |_| controlled.set(Key::from("c"))>
                "Select C"
            </button>
            <div>
                "Controlled: "
                <span id="test-tabs-controlled-key">{move || controlled.get().to_string()}</span>
            </div>
            <DynamicTabs />
            <NestedTabs />
            <AnimatedTabs />
        </div>
    }
}
