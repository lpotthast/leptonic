use std::collections::HashSet;

use leptonic::{
    atoms::{
        button::Button,
        disclosure::{
            Disclosure, DisclosureGroup, DisclosurePanel, DisclosurePanelRole, DisclosureTrigger,
        },
        menu::{Menu, MenuItems, MenuTrigger},
        popover::Popover,
    },
    hooks::{
        collections::{Key, use_collection},
        disclosure::DisclosureGroupExpansion,
    },
};
use leptos::prelude::*;

/// Disclosure atoms (react-aria-components `Disclosure.test.js` setups):
/// - `#test-disc-main`: "Shipping" (`#test-disc-trigger`), a menu button next to its heading
///   (`#test-disc-menu-trigger`) and its panel ("Shipping content"); every expanded change is
///   counted in `#test-disc-changes`.
/// - Nested: "Outer" with a panel holding the "Inner" disclosure ("Inner content").
/// - A single-expansion group ("Group A", "Group B"; its expanded keys in
///   `#test-disc-group-keys`), a multiple-expansion group ("Multi C", "Multi D") and a disabled
///   group ("Disabled E").
/// - Controlled: "Controlled" (expanded, without a setter; changes in `#test-disc-controlled-changes`),
///   "Disabled expanded", "Closed controlled" (collapsed without a setter; the requested states in
///   `#test-disc-closed-requests`).
/// - A controlled group ("Controlled 1"/"Controlled 2", `#test-disc-expand-2` expands the second)
///   and nested groups ("Nested 1" holding "Nested 2").
/// - "Remounted": its panel is rendered only while `#test-disc-remount-toggle` shows it (a remount
///   of the panel while the disclosure stays expanded or collapsed).
#[component]
pub fn PageAtomDisclosure() -> impl IntoView {
    let changes = RwSignal::new(0u32);
    let controlled_changes = RwSignal::new(String::new());
    let closed_requests = RwSignal::new(String::new());
    let group_keys = RwSignal::new(String::new());
    let controlled_keys = RwSignal::new(HashSet::from([Key::from("item1")]));
    let show_remounted_panel = RwSignal::new(true);
    let actions = use_collection(|b| {
        b.item("rename", "Rename");
        b.item("delete", "Delete");
    });

    view! {
        <div id="test-page-atom-disclosure">
            <Disclosure attr:id="test-disc-main" on_expanded_change=move |_| changes.update(|c| *c += 1)>
                <h3>
                    <DisclosureTrigger>
                        <Button id="test-disc-trigger">"Shipping"</Button>
                    </DisclosureTrigger>
                </h3>
                <MenuTrigger>
                    <Button id="test-disc-menu-trigger" aria_label="Menu">"\u{2630}"</Button>
                    <Popover>
                        <Menu collection=actions>
                            <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                        </Menu>
                    </Popover>
                </MenuTrigger>
                <DisclosurePanel>
                    <p>"Shipping content"</p>
                </DisclosurePanel>
            </Disclosure>
            <div>"Changes: " <span id="test-disc-changes">{changes}</span></div>

            <Disclosure>
                <DisclosureTrigger><Button>"Outer"</Button></DisclosureTrigger>
                <DisclosurePanel>
                    <Disclosure>
                        <DisclosureTrigger><Button>"Inner"</Button></DisclosureTrigger>
                        <DisclosurePanel><p>"Inner content"</p></DisclosurePanel>
                    </Disclosure>
                </DisclosurePanel>
            </Disclosure>

            <DisclosureGroup on_expanded_change={move |keys: HashSet<Key>| {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                group_keys.set(keys.join(","));
            }}>
                <Disclosure key="a">
                    <DisclosureTrigger><Button>"Group A"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"A content"</p></DisclosurePanel>
                </Disclosure>
                <Disclosure key="b">
                    <DisclosureTrigger><Button>"Group B"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"B content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>
            <div>"Group keys: " <span id="test-disc-group-keys">{group_keys}</span></div>

            <DisclosureGroup expansion=DisclosureGroupExpansion::Multiple>
                <Disclosure key="c">
                    <DisclosureTrigger><Button>"Multi C"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"C content"</p></DisclosurePanel>
                </Disclosure>
                <Disclosure key="d">
                    <DisclosureTrigger><Button>"Multi D"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"D content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>

            <Disclosure>
                <DisclosureTrigger><Button>"Region"</Button></DisclosureTrigger>
                <DisclosurePanel role=DisclosurePanelRole::Region><p>"Region content"</p></DisclosurePanel>
            </Disclosure>

            <Disclosure
                is_expanded=true
                on_expanded_change=move |expanded: bool| controlled_changes.update(|c| c.push_str(&expanded.to_string()))
                classes="test-disc-controlled"
            >
                <DisclosureTrigger><Button>"Controlled"</Button></DisclosureTrigger>
                <DisclosurePanel><p>"Controlled content"</p></DisclosurePanel>
            </Disclosure>
            <div>"Controlled changes: " <span id="test-disc-controlled-changes">{controlled_changes}</span></div>

            <Disclosure is_disabled=true is_expanded=true classes="test-disc-disabled-expanded">
                <DisclosureTrigger><Button>"Disabled expanded"</Button></DisclosureTrigger>
                <DisclosurePanel><p>"Disabled expanded content"</p></DisclosurePanel>
            </Disclosure>

            <Disclosure
                is_expanded=false
                on_expanded_change=move |expanded: bool| closed_requests.update(|c| c.push_str(&expanded.to_string()))
            >
                <DisclosureTrigger><Button>"Closed controlled"</Button></DisclosureTrigger>
                <DisclosurePanel><p>"Closed controlled content"</p></DisclosurePanel>
            </Disclosure>
            <div>"Closed requests: " <span id="test-disc-closed-requests">{closed_requests}</span></div>


            <button id="test-disc-expand-2" on:click=move |_| controlled_keys.set(HashSet::from([Key::from("item2")]))>
                "Expand item2"
            </button>
            <DisclosureGroup expanded_keys=controlled_keys>
                <Disclosure key="item1">
                    <DisclosureTrigger><Button>"Controlled 1"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"Controlled 1 content"</p></DisclosurePanel>
                </Disclosure>
                <Disclosure key="item2">
                    <DisclosureTrigger><Button>"Controlled 2"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"Controlled 2 content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>

            <DisclosureGroup>
                <Disclosure key="nested1">
                    <DisclosureTrigger><Button>"Nested 1"</Button></DisclosureTrigger>
                    <DisclosurePanel>
                        <DisclosureGroup>
                            <Disclosure key="nested2">
                                <DisclosureTrigger><Button>"Nested 2"</Button></DisclosureTrigger>
                                <DisclosurePanel><p>"Nested 2 content"</p></DisclosurePanel>
                            </Disclosure>
                        </DisclosureGroup>
                    </DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>

            <Disclosure>
                <DisclosureTrigger><Button>"Remounted"</Button></DisclosureTrigger>
                <Show when=move || show_remounted_panel.get()>
                    <DisclosurePanel><p>"Remounted content"</p></DisclosurePanel>
                </Show>
            </Disclosure>
            <button id="test-disc-remount-toggle" on:click=move |_| show_remounted_panel.update(|show| *show = !*show)>
                "Toggle the panel"
            </button>

            <DisclosureGroup is_disabled=true>
                <Disclosure key="e">
                    <DisclosureTrigger><Button>"Disabled E"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"E content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>
        </div>
    }
}
