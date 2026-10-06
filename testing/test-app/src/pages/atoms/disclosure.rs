use leptonic::{
    atoms::prelude::{
        Button, Disclosure, DisclosureGroup, DisclosurePanel, DisclosurePanelRole,
        DisclosureTrigger, Menu, MenuItems, MenuTrigger, Popover,
    },
    hooks::{DisclosureGroupExpansion, collections::use_collection},
};
use leptos::prelude::*;

/// Disclosure atoms (react-aria-components `Disclosure.test.js` setups):
/// - `#test-disc-main`: "Shipping" (`#test-disc-trigger`), a menu button next to its heading
///   (`#test-disc-menu-trigger`) and its panel ("Shipping content"); every expanded change is
///   counted in `#test-disc-changes`.
/// - Nested: "Outer" with a panel holding the "Inner" disclosure ("Inner content").
/// - A single-expansion group ("Group A", "Group B"), a multiple-expansion group ("Multi C",
///   "Multi D") and a disabled group ("Disabled E").
#[component]
pub fn PageAtomDisclosure() -> impl IntoView {
    let changes = RwSignal::new(0u32);
    let actions = use_collection(|b| {
        b.item("rename", "Rename");
        b.item("delete", "Delete");
    });

    view! {
        <div id="test-page-atom-disclosure">
            <Disclosure attr:id="test-disc-main" on_expanded_change=move |_| changes.update(|c| *c += 1)>
                <h3>
                    <DisclosureTrigger>
                        <Button attr:id="test-disc-trigger">"Shipping"</Button>
                    </DisclosureTrigger>
                </h3>
                <MenuTrigger>
                    <Button attr:id="test-disc-menu-trigger" aria_label="Menu">"\u{2630}"</Button>
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

            <DisclosureGroup>
                <Disclosure id="a">
                    <DisclosureTrigger><Button>"Group A"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"A content"</p></DisclosurePanel>
                </Disclosure>
                <Disclosure id="b">
                    <DisclosureTrigger><Button>"Group B"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"B content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>

            <DisclosureGroup expansion=DisclosureGroupExpansion::Multiple>
                <Disclosure id="c">
                    <DisclosureTrigger><Button>"Multi C"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"C content"</p></DisclosurePanel>
                </Disclosure>
                <Disclosure id="d">
                    <DisclosureTrigger><Button>"Multi D"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"D content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>

            <Disclosure>
                <DisclosureTrigger><Button>"Region"</Button></DisclosureTrigger>
                <DisclosurePanel role=DisclosurePanelRole::Region><p>"Region content"</p></DisclosurePanel>
            </Disclosure>

            <DisclosureGroup is_disabled=true>
                <Disclosure id="e">
                    <DisclosureTrigger><Button>"Disabled E"</Button></DisclosureTrigger>
                    <DisclosurePanel><p>"E content"</p></DisclosurePanel>
                </Disclosure>
            </DisclosureGroup>
        </div>
    }
}
