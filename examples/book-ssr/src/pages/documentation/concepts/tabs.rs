use leptos::prelude::*;

use super::demos::tabs::TabsConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTabsOverview() -> impl IntoView {
    view! {
        <DocPage title="Tabs">
            <p>
                "Tabs organize content into panels, showing one panel at a time. "
                "The user switches panels by selecting a tab from a horizontal (or vertical) tab list. "
                "Tabs are ideal when content is parallel in structure \u{2014} "
                "settings categories, data views, or step-by-step sections."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Switch between parallel content panels"</TableCell><TableCell><b>"Tabs"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Expand/collapse independent sections"</TableCell>
                        <TableCell><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Navigate between pages"</TableCell><TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Choose a value from options"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Select.materialize()>"Select"</Link>" / "
                            <Link href=routes::doc::Radio.materialize()>"Radio"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Tabs exist as hooks and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link></TableCell>
                        <TableCell>
                            "Behavior and ARIA attributes for a tab list, tabs and panels you render yourself: "
                            <Code inline=true>"use_tab_list_state"</Code>", "<Code inline=true>"use_tab_list"</Code>", "
                            <Code inline=true>"use_tab"</Code>", "<Code inline=true>"use_tab_panel"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tabs::Atom.materialize()>"Tabs Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Tabs"</Code>", "<Code inline=true>"TabList"</Code>", "
                            <Code inline=true>"Tab"</Code>" and "<Code inline=true>"TabPanel"</Code>" built on the hooks, "
                            "styled through data attributes. The recommended starting point."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms: a collection of tabs, a "<Code inline=true>"TabList"</Code>" with a "<Code inline=true>"Tab"</Code>
                    " per tab and a "<Code inline=true>"TabPanel"</Code>" per tab. The classes are the book\u{2019}s own; the "
                    <Link href=format!("{}#styling", routes::doc::tabs::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows how they style the tabs through their data attributes."
                </p>

                <Demo description="Tabbed content panels built with the tab atoms" source=include_str!("demos/tabs.rs") source_open=true>
                    <TabsConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The tab hooks and atoms follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/tabs/" target=LinkTarget::Blank>"Tabs pattern"</Link>
                    ":"
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"tablist\""</Code>" on the tab list (with "<Code inline=true>"aria-orientation"</Code>
                        " and a label you provide), "<Code inline=true>"role=\"tab\""</Code>" on each tab, "
                        <Code inline=true>"role=\"tabpanel\""</Code>" on the panel"
                    </li>
                    <li><Code inline=true>"aria-selected"</Code>" on every tab, "<Code inline=true>"aria-disabled"</Code>" on disabled tabs"</li>
                    <li>
                        <Code inline=true>"aria-controls"</Code>" on the selected tab points to its panel, the panel\u{2019}s "
                        <Code inline=true>"aria-labelledby"</Code>" back to the tab"
                    </li>
                    <li>"The tab list is one tab stop; the panel is the next one unless it contains tabbable elements"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">
                        "Move focus to the selected tab (with manual activation: the tab focused last), then into the panel."
                    </KeyRow>
                    <KeyRow keys="ArrowRight / ArrowLeft">
                        "Move to the next or previous enabled tab, wrapping around (mirrored in right-to-left locales)."
                    </KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">
                        "Move to the next or previous enabled tab (vertical tab lists, where the left and right arrows keep working)."
                    </KeyRow>
                    <KeyRow keys="Home / End">"Move to the first or last enabled tab."</KeyRow>
                    <KeyRow keys="Enter / Space">"Select the focused tab (manual activation; otherwise moving selects)."</KeyRow>
                </KeyboardTable>

            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::tabs::Hook.materialize()>"Tabs Hooks"</Link></li>
                <li><Link href=routes::doc::tabs::Atom.materialize()>"Tabs Atoms"</Link></li>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
