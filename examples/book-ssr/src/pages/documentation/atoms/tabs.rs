use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{tabs::TabsAtomDemo, tabs_vertical::TabsVerticalAtomDemo};
use crate::{kit::*, routes};

/// A link to a section of the tabs hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::tabs::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTabs() -> impl IntoView {
    view! {
        <DocPage title="Tabs Atoms">
            <p>
                "The unstyled tab atoms: "<AnchorLink href="#tabs">"Tabs"</AnchorLink>" holds the state, "
                <AnchorLink href="#tablist">"TabList"</AnchorLink>" contains a "<AnchorLink href="#tab">"Tab"</AnchorLink>
                " per tab, and a "<AnchorLink href="#tabpanel">"TabPanel"</AnchorLink>" per tab shows its content while it is "
                "selected. See the "
                <Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hook"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Tabs"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-tab-list-state")>"use_tab_list_state"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TabList"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-tab-list")>"use_tab_list"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Tab"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-tab")>"use_tab"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TabPanel"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-tab-panel")>"use_tab_panel"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "The tabs are a "<Link href=routes::doc::CollectionState.materialize()>"collection"</Link>" from "
                    <Link href=format!("{}#use-collection", routes::doc::CollectionState.materialize())>"use_collection"</Link>
                    ". Render one "<Code inline=true>"Tab"</Code>" per collection item, in collection order, and a "
                    <Code inline=true>"TabPanel"</Code>" with the same key for each:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::tabs::{Tab, TabList, TabPanel, Tabs}, hooks::use_collection};

                        let collection = use_collection(|b| {
                            b.item("details", "Details");
                            b.item("reviews", "Reviews").disabled(true);
                            b.item("shipping", "Shipping");
                        });

                        view! {
                            <Tabs collection=collection>
                                <TabList aria_label="Product">
                                    <Tab key="details">"Details"</Tab>
                                    <Tab key="reviews">"Reviews"</Tab>
                                    <Tab key="shipping">"Shipping"</Tab>
                                </TabList>
                                <TabPanel key="details">"A foldable reading lamp."</TabPanel>
                                <TabPanel key="reviews">"4.6 out of 5 stars."</TabPanel>
                                <TabPanel key="shipping">"Ships within two days."</TabPanel>
                            </Tabs>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Click a tab, or tab into the tab list and use the arrow keys. \u{201c}Reviews\u{201d} is disabled through "
                    <Code inline=true>"disabled_keys"</Code>"; uncheck the box to enable it. The tabs are styled through the "
                    <AnchorLink href="#data-attributes">"data attributes"</AnchorLink>". The selected tab lives in an "
                    <Code inline=true>"RwSignal<Key>"</Code>" of the demo, passed as "<Code inline=true>"selected_key"</Code>
                    " and "<Code inline=true>"set_selected_key"</Code>"; \u{201c}Show shipping\u{201d} selects a tab by setting "
                    "it (see "<AnchorLink href="#controlled-selection">"Controlled Selection"</AnchorLink>")."
                </p>
                <Demo
                    description="Horizontal tabs styled through data attributes, with a Disabled checkbox, a toggleable disabled tab and a button selecting a tab"
                    source=include_str!("demos/tabs.rs")
                >
                    <TabsAtomDemo/>
                </Demo>

                <p>
                    "Vertical tabs with manual activation: "<Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" move focus, "
                    <Keys keys="Enter"/>" or "<Keys keys="Space"/>" selects the focused tab. The layout comes from the "
                    <Code inline=true>"data-orientation"</Code>" of "<Code inline=true>"Tabs"</Code>"."
                </p>
                <Demo
                    description="Vertical tabs with manual activation"
                    source=include_str!("demos/tabs_vertical.rs")
                >
                    <TabsVerticalAtomDemo/>
                </Demo>
            </Section>

            <Section title="Controlled Selection">
                <p>
                    "Pass "<Code inline=true>"selected_key"</Code>" and "<Code inline=true>"set_selected_key"</Code>" to keep the "
                    "selected tab in your app\u{2019}s state, e.g. to restore it or to switch tabs from elsewhere: the tabs "
                    "show the key "<Code inline=true>"selected_key"</Code>" reads, selecting a tab calls "
                    <Code inline=true>"set_selected_key"</Code>", and changing your state selects that tab. "
                    <Code inline=true>"selected_key"</Code>" takes any signal, "<Code inline=true>"set_selected_key"</Code>
                    " any setter: an "<Code inline=true>"RwSignal"</Code>", a "<Code inline=true>"WriteSignal"</Code>
                    ", a closure or a "<Code inline=true>"Callback"</Code>". Without "<Code inline=true>"set_selected_key"</Code>
                    ", the selection is read-only."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{components::prelude::Button, hooks::Key};

                        let selected = RwSignal::new(Key::from("details"));

                        view! {
                            <Tabs collection=collection selected_key=selected set_selected_key=selected>
                                // TabList, Tab and TabPanel as above
                            </Tabs>
                            <Button on_press=move |_| selected.set(Key::from("shipping"))>"Show shipping"</Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Tabs">
                <p>
                    "Creates the tab list state and renders a "<Code inline=true>"<div>"</Code>" around its children, which "
                    "may contain any markup besides the "<Code inline=true>"TabList"</Code>" and the panels."
                </p>
                <Section title="Props" id="tabs-props">
                    <ApiTable kind=ApiKind::Props of="atoms::tabs::Tabs">
                        <ApiRow name="collection" ty="CollectionMemo">"The tabs, e.g. from "<Code inline=true>"use_collection"</Code>"."</ApiRow>
                        <ApiRow name="default_selected_key" ty="Option<Key>" default="None">
                            "The initially selected tab. "<Code inline=true>"None"</Code>": the first enabled tab."
                        </ApiRow>
                        <ApiRow name="selected_key" ty="Option<Signal<Key>>" default="None">
                            "The selected tab (controlled), replacing "<Code inline=true>"default_selected_key"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selected_key" ty="Option<Out<Key>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of the tab the user selects."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Tabs that can\u{2019}t be selected, besides those disabled in the collection. The arrow keys skip them."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all tabs."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Horizontal">
                            "Which arrow keys move between the tabs. Exposed as "<Code inline=true>"data-orientation"</Code>"."
                        </ApiRow>
                        <ApiRow name="keyboard_activation" ty="KeyboardActivation" default="Automatic">
                            <Code inline=true>"Automatic"</Code>": moving focus with the arrow keys selects the tab. "
                            <Code inline=true>"Manual"</Code>": "<Keys keys="Enter"/>" or "<Keys keys="Space"/>" selects it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the wrapping "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The tab list, the panels, and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=hook_section("use-tab-list-state")>"use_tab_list_state"</Link>" and "
                        <Link href=hook_section("keyboard-activation")>"Keyboard Activation"</Link>" for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="TabList">
                <p>
                    "The tab list, a "<Code inline=true>"<div role=\"tablist\">"</Code>" containing the "
                    <Code inline=true>"Tab"</Code>"s. It is not a tab stop itself; the selected tab is."
                </p>
                <Section title="Props" id="tab-list-props">
                    <ApiTable kind=ApiKind::Props of="TabList">
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">"Names the tab list."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tab list."</ApiRow>
                        <ApiRow name="children" ty="Children">"One "<Code inline=true>"Tab"</Code>" per collection item, in collection order."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Tab">
                <p>
                    "A tab, as a "<Code inline=true>"<div role=\"tab\">"</Code>". It has no "<Code inline=true>"is_disabled"</Code>
                    " prop on purpose: the keyboard navigation only skips tabs disabled in the collection ("
                    <Code inline=true>"ItemBuilder::disabled"</Code>") or through "<Code inline=true>"disabled_keys"</Code>"."
                </p>
                <Section title="Props" id="tab-props">
                    <ApiTable kind=ApiKind::Props of="atoms::tabs::Tab">
                        <ApiRow name="key" ty="Key">"The tab\u{2019}s key in the collection."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tab."</ApiRow>
                        <ApiRow name="children" ty="Children">"The tab\u{2019}s label."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TabPanel">
                <p>
                    "The content of the tab with the same key, as a "<Code inline=true>"<div role=\"tabpanel\">"</Code>
                    ". It is rendered only while its tab is selected, so switching tabs unmounts the previous panel and "
                    "resets its state. Without tabbable content, the panel is a tab stop."
                </p>
                <Section title="Props" id="tab-panel-props">
                    <ApiTable kind=ApiKind::Props of="TabPanel">
                        <ApiRow name="key" ty="Key">"The panel\u{2019}s tab."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the panel."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The panel content, rendered each time the tab is selected."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-orientation" ty="horizontal | vertical">
                        "On "<Code inline=true>"Tabs"</Code>": the orientation."
                    </ApiRow>
                    <ApiRow name="data-selected" ty="true">"On "<Code inline=true>"Tab"</Code>": the tab is selected."</ApiRow>
                    <ApiRow name="data-focused" ty="true">
                        "On "<Code inline=true>"Tab"</Code>": the tab has focus, from the keyboard or a click."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On "<Code inline=true>"Tab"</Code>": the tab is disabled, by the collection, "
                        <Code inline=true>"disabled_keys"</Code>" or "<Code inline=true>"is_disabled"</Code>"."
                    </ApiRow>
                    <ApiRow name="data-pressed" ty="true">"On "<Code inline=true>"Tab"</Code>": the tab is being pressed."</ApiRow>
                </ApiTable>
                <p>
                    "A "<Code inline=true>"Tab"</Code>" currently has no "<Code inline=true>"data-hovered"</Code>" or "
                    <Code inline=true>"data-focus-visible"</Code>"; use "<Code inline=true>":hover"</Code>" and "
                    <Code inline=true>":focus-visible"</Code>" for those."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with attribute "
                    "selectors. "<Code inline=true>"data-focused"</Code>" is also set after a click, so draw focus rings with "
                    <Code inline=true>":focus-visible"</Code>". Lay out a vertical tab list through the "
                    <Code inline=true>"data-orientation"</Code>" of "<Code inline=true>"Tabs"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-tab-list { display: flex; border-bottom: 2px solid var(--border); }
                        .my-tabs[data-orientation="vertical"] { display: flex; }
                        .my-tabs[data-orientation="vertical"] .my-tab-list { flex-direction: column; }
                        .my-tab[data-selected] { border-bottom: 2px solid var(--accent); }
                        .my-tab[data-disabled] { color: var(--muted); }
                        .my-tab:focus-visible { outline: 2px solid var(--focus); }
                    "#)}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find their tab list through the context: "<Code inline=true>"TabListData"</Code>" (provided by "
                    <Code inline=true>"Tabs"</Code>") and "<Code inline=true>"TabListItemData"</Code>" (provided by "
                    <Code inline=true>"TabList"</Code>"). Your own Leptos components inside "<Code inline=true>"Tabs"</Code>
                    " can read the state, e.g. to show the selected tab elsewhere:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::TabListData;

                        #[component]
                        fn SelectedTab() -> impl IntoView {
                            let state = expect_context::<TabListData>().state;
                            view! { <p>"Showing: " {move || state.selected_key().map(|key| key.to_string())}</p> }
                        }
                    "#)}
                </Code>
                <p>
                    "Need panels that stay mounted while hidden, or a tab built from your own element? Use the "
                    <Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link>" directly."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link></li>
                <li><Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link></li>
                <li><Link href=routes::doc::tabs::Component.materialize()>"Tabs Components"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
