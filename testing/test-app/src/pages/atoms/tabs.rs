use std::{collections::HashSet, sync::Arc, time::Duration};

use leptonic::{
    I18nProvider, Locale, Orientation,
    atoms::{
        tabs::{Tab, TabList, TabPanel, TabPanels, Tabs},
        tooltip::{Tooltip, TooltipTrigger},
    },
    hooks::{
        collections::{Collection, Key},
        tabs::KeyboardActivation,
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
    /// Disabled by the `Tab`'s `is_disabled`.
    #[prop(optional)]
    tab_disabled: Vec<&'static str>,
    #[prop(optional)] should_force_mount: bool,
    /// The selected key, bound.
    #[prop(optional)]
    selected: Option<RwSignal<Key>>,
    /// Disables all tabs (`Tabs::is_disabled`).
    #[prop(optional)]
    is_disabled: bool,
    /// Names the panels (`TabPanel::aria_label`: "Panel A", ...).
    #[prop(optional)]
    panel_labels: bool,
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
                <TabPanel
                    key=tab
                    should_force_mount=should_force_mount
                    aria_label=panel_labels.then(|| format!("Panel {}", tab.to_uppercase()))
                >
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
                on_selected_key_change=on_selection_change
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
                is_disabled=is_disabled
                on_selected_key_change=on_selection_change
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
            on_selected_key_change=move |_| changes.update(|c| *c += 1)
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

/// Tabs "First", "Second" and "Third" with "Third" selected by default.
#[component]
fn DefaultSelectedTabs() -> impl IntoView {
    let collection = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.item("first", "First");
            b.item("second", "Second");
            b.item("third", "Third");
        }))
    });
    view! {
        <button id="test-tabs-default-before">"Before"</button>
        <Tabs collection=collection default_selected_key="third">
            <TabList aria_label="default">
                <Tab key="first">"First"</Tab>
                <Tab key="second">"Second"</Tab>
                <Tab key="third">"Third"</Tab>
            </TabList>
            <TabPanel key="first">"Panel First"</TabPanel>
            <TabPanel key="second">"Panel Second"</TabPanel>
            <TabPanel key="third">"Panel Third"</TabPanel>
        </Tabs>
    }
}

/// Tabs whose "Tooltip tab" is wrapped in a `TooltipTrigger` (react-aria-components' "supports
/// tooltips"), and whose panels hold an input ("Input panel") or only a disabled input
/// ("Disabled input panel").
#[component]
fn TooltipAndInputTabs() -> impl IntoView {
    let collection = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            b.item("plain", "Plain");
            b.item("tooltip", "Tooltip tab");
        }))
    });
    view! {
        <Tabs collection=collection>
            <TabList aria_label="Tooltip and inputs">
                <Tab key="plain">"Plain"</Tab>
                <TooltipTrigger delay=Duration::from_millis(100) close_delay=Duration::from_millis(100)>
                    <Tab key="tooltip">"Tooltip tab"</Tab>
                    <Tooltip>"Test"</Tooltip>
                </TooltipTrigger>
            </TabList>
            <TabPanel key="plain">
                <input aria-label="Input panel" />
            </TabPanel>
            <TabPanel key="tooltip">
                <input aria-label="Disabled input panel" disabled=true />
            </TabPanel>
        </Tabs>
    }
}

/// Tabs in different configurations: basic, a disabled tab, a disabled first tab, all tabs
/// disabled, vertical, manual activation, vertical right-to-left, force-mounted panels, tabs
/// disabled by `Tab::is_disabled`, a bound selected key (`#test-tabs-controlled-select-c`
/// selects C from outside; B is disabled), dynamic, nested and animated tabs; all tabs disabled by
/// `Tabs::is_disabled` ("all-tabs-disabled"), a third tab selected by default ("default"),
/// named panels ("labelled-panels"), right-to-left horizontal ("rtl"),
/// and a tab with a tooltip next to panels with inputs.
#[component]
pub fn PageAtomTabs() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    let rtl_horizontal: Locale = "ar-AE".parse().expect("a valid locale");
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
            <TestTabs name="all-tabs-disabled" is_disabled=true />
            <DefaultSelectedTabs />
            <TestTabs name="labelled-panels" panel_labels=true />
            <I18nProvider locale=rtl_horizontal>
                <TestTabs name="rtl" />
            </I18nProvider>
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
            <TooltipAndInputTabs />
        </div>
    }
}
