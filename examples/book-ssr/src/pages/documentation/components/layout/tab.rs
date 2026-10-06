use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    tab_basic::TabBasicDemo, tab_mounting::TabMountingDemo,
    tab_reactive_label::TabReactiveLabelDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageTab() -> impl IntoView {
    view! {
        <DocPage title="Tabs Components">
            <p>
                "The themed "<AnchorLink href="#tabs">"Tabs"</AnchorLink>" component spreads content over several panels, of "
                "which only one is shown at a time. Every "<AnchorLink href="#tab">"Tab"</AnchorLink>" inside it is one panel "
                "with a label; selecting the label brings the panel into view. Its tabs can only be selected with a pointer "
                "(see "<AnchorLink href="#accessibility">"Accessibility"</AnchorLink>"). See the "
                <Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link>" for concept guidance."
            </p>

            <Demo description="Three tabs with static labels" source=include_str!("demos/tab_basic.rs")>
                <TabBasicDemo/>
            </Demo>

            <Section title="Tabs">
                <Section title="Props" id="tabs-props">
                    <ApiTable kind=ApiKind::Props of="components::tabs::Tabs">
                        <ApiRow name="mount" ty="Option<Mount>" default="None">
                            "Default mount mode of the contained tabs, see "<AnchorLink href="#mounting">"Mounting"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"Tab"</Code>"s. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Tab">
                <Section title="Props" id="tab-props">
                    <ApiTable kind=ApiKind::Props of="components::tab::Tab">
                        <ApiRow name="name" ty="Oco<'static, str>">
                            "Uniquely identifies the tab within its "<Code inline=true>"Tabs"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="label" ty="ViewFn">
                            "The label of the tab selector, for example "<Code inline=true>"|| \"Settings\""</Code>
                            ". The function runs once; return a closure from it ("
                            <Code inline=true>"move || move || ..."</Code>") to make the label reactive. Required."
                        </ApiRow>
                        <ApiRow name="mount" ty="Option<Mount>" default="None">
                            "Mount mode of this tab. Overrides the mode of the "<Code inline=true>"Tabs"</Code>
                            "; without either, "<Code inline=true>"Mount::Once"</Code>" applies."
                        </ApiRow>
                        <ApiRow name="on_show" ty="Option<Out<()>>" default="None">
                            "Called whenever the tab comes into view."
                        </ApiRow>
                        <ApiRow name="on_hide" ty="Option<Out<()>>" default="None">
                            "Called whenever the tab gets hidden."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<Oco<'static, str>>" default="None">
                            "The tab panel\u{2019}s id. A generated one, stable across server rendering and hydration, if omitted."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the panel."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn" default="empty">"The panel content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Reactive Labels">
                <p>
                    "The "<Code inline=true>"label"</Code>" function of a tab runs once. To update a label, return a closure "
                    "from it that reads your signals."
                </p>
                <Demo description="A tab label showing the state of a toggle" source=include_str!("demos/tab_reactive_label.rs") source_open=true>
                    <TabReactiveLabelDemo/>
                </Demo>
            </Section>

            <Section title="Mounting">
                <p>
                    "The "<Code inline=true>"mount"</Code>" prop decides when the content of a tab is rendered. "
                    "The first registered tab is shown initially."
                </p>
                <DocTable headers=&["Mount", "Behavior"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Mount::Once"</Code></TableCell>
                        <TableCell>
                            "The content is rendered once and hidden while the tab isn\u{2019}t shown. "
                            "State inside the tab, such as input values or the selection of nested tabs, "
                            "survives switching tabs."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Mount::WhenShown"</Code></TableCell>
                        <TableCell>
                            "The content is rendered every time the tab is shown and unmounted when it gets hidden, so "
                            "only the active tab exists in the DOM. State inside the tab is lost when switching away."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Both tab sets below nest tabs in "<Code inline=true>"Outer 1"</Code>". Select "
                    <Code inline=true>"Inner 2"</Code>", switch to "<Code inline=true>"Outer 2"</Code>" and back: with "
                    <Code inline=true>"Mount::Once"</Code>" the inner selection is kept, with "
                    <Code inline=true>"Mount::WhenShown"</Code>" it starts over."
                </p>
                <Demo description="Nested tabs with both mount modes" source=include_str!("demos/tab_mounting.rs")>
                    <TabMountingDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The component predates the tab hooks and isn\u{2019}t built on them. Its tab list and panels have the "
                    <Code inline=true>"tablist"</Code>" and "<Code inline=true>"tabpanel"</Code>" roles, but each tab is a "
                    <Code inline=true>"<div role=\"tab\">"</Code>" that reacts to clicks only: it isn\u{2019}t focusable, the "
                    "arrow keys don\u{2019}t move between the tabs, and it has no "<Code inline=true>"aria-selected"</Code>" or "
                    <Code inline=true>"aria-controls"</Code>". Keyboard and screen reader users can\u{2019}t switch tabs. Use the "
                    <Link href=routes::doc::tabs::Atom.materialize()>"Tabs Atoms"</Link>" (or the "
                    <Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link>") where that matters."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt tabs to your design:"</p>
                <CssVariables prefix="--tab-" scss=theme_scss!("tabs")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link></li>
                <li><Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link></li>
                <li><Link href=routes::doc::tabs::Atom.materialize()>"Tabs Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
