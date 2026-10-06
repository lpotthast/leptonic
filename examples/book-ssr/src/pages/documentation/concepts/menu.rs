use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::atoms::demos::menu::MenuDemo, routes};

#[component]
pub fn PageMenuOverview() -> impl IntoView {
    view! {
        <DocPage title="Menu">
            <p>
                "Menus present a list of actions or options in an overlay triggered by a button. "
                "They support single-select, multi-select, and purely action-based modes. "
                "Menus are built from multiple cooperating hooks: "
                <Code inline=true>"use_menu_trigger"</Code>", "
                <Code inline=true>"use_menu"</Code>", "
                <Code inline=true>"use_menu_item"</Code>", and "
                <Code inline=true>"use_menu_section"</Code>"."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a list of actions triggered by a button"</TableCell><TableCell><b>"Menu"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Choose a value from a dropdown"</TableCell><TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show rich contextual content"</TableCell><TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Menus exist as hooks and atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for a detailed explanation of each layer."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::menu::Hook.materialize()>"Menu hooks"</Link></TableCell>
                        <TableCell>"Trigger, menu, item and section behavior for elements you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::menu::Atom.materialize()>"Menu atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"MenuTrigger"</Code>", "<Code inline=true>"Menu"</Code>", "<Code inline=true>"MenuItem"</Code>
                            " and "<Code inline=true>"MenuSection"</Code>" with the complete behavior, unstyled."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms are the quickest way to a menu: a "<Code inline=true>"MenuTrigger"</Code>" around a button and a "
                    <Code inline=true>"Popover"</Code>" with the "<Code inline=true>"Menu"</Code>", whose items come from a "
                    "collection. See the "<Link href=routes::doc::menu::Atom.materialize()>"menu atoms"</Link>" for every "
                    "option, and the "<Link href=routes::doc::menu::Hook.materialize()>"menu hooks"</Link>" to build your own."
                </p>

                <Demo
                    description="An action menu and a menu with checkable items, with a disabled toggle"
                    source=include_str!("../atoms/demos/menu.rs")
                    source_open=true
                >
                    <MenuDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>"Leptonic menus follow the WAI-ARIA Menu pattern."</p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"menu\""</Code>" on the container, "
                        <Code inline=true>"role=\"menuitem\""</Code>" (or "<Code inline=true>"\"menuitemradio\""</Code>
                        " / "<Code inline=true>"\"menuitemcheckbox\""</Code>")"
                    </li>
                    <li>
                        <Code inline=true>"aria-haspopup=\"true\""</Code>", "<Code inline=true>"aria-expanded"</Code>" and "
                        <Code inline=true>"aria-controls"</Code>" on the trigger; the menu is labelled by the trigger"
                    </li>
                    <li><Code inline=true>"role=\"group\""</Code>" for sections, labelled by their heading"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"On the trigger: open the menu and focus the first item."</KeyRow>
                    <KeyRow keys="ArrowUp">"On the trigger: open the menu and focus the last item."</KeyRow>
                    <KeyRow keys="Alt + ArrowDown / Alt + ArrowUp">
                        "On the trigger: open the menu. Long press triggers also accept "<Code inline=true>"Alt + Enter / Space"</Code>
                        ", since a plain press is reserved for the button\u{2019}s own action."
                    </KeyRow>
                    <KeyRow keys="Enter / Space">"Activate the focused item."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Navigate between items."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last item."</KeyRow>
                    <KeyRow keys="Escape">"Close the menu."</KeyRow>
                    <KeyRow keys="Letter keys">"Focus the next item starting with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
