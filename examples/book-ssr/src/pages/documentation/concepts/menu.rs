use leptos::prelude::*;

use super::demos::menu::MenuConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageMenuOverview() -> impl IntoView {
    view! {
        <DocPage title="Menu">
            <p>
                "A menu presents a list of actions, or of options to check, in a popover opened from a button. Items can "
                "open submenus or small dialogs, and a menu can also open as a context menu, where the user right clicks "
                "an element. Unlike a select, a menu runs actions instead of holding a value."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a list of actions triggered by a button"</TableCell><TableCell><b>"Menu"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Offer actions for an element on right click, "<Keys keys="Shift + F10"/>" or long press"</TableCell>
                        <TableCell>
                            <b>"Menu"</b>" as a "<Link href=format!("{}#context-menus", routes::doc::menu::Atom.materialize())>"context menu"</Link>
                        </TableCell>
                    </TableRow>
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
                        <TableCell><Link href=routes::doc::menu::Hook.materialize()>"Menu Hooks"</Link></TableCell>
                        <TableCell>"Trigger, menu, item, section and submenu behavior for elements you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled trigger, menu, items, sections and submenus with that behavior, styled through data "
                            "attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms are the quickest way to a menu: a "<Code inline=true>"MenuTrigger"</Code>" around a button and a "
                    "popover with the menu, whose items come from a collection. See the "<Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link>" for every "
                    "option, and the "<Link href=routes::doc::menu::Hook.materialize()>"Menu Hooks"</Link>" to build your own."
                </p>

                <Demo description="An Edit menu with three actions, showing the last action" source=include_str!("demos/menu.rs") source_open=true>
                    <MenuConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Leptonic menus follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/menu-button/" target=LinkTarget::Blank>"Menu Button pattern"</Link>
                    "."
                </p>

                <ul>
                    <li>
                        "The menu has "<Code inline=true>"role=\"menu\""</Code>"; its items are "<Code inline=true>"menuitem"</Code>
                        "s, or "<Code inline=true>"menuitemradio"</Code>" / "<Code inline=true>"menuitemcheckbox"</Code>
                        " with "<Code inline=true>"aria-checked"</Code>" in a menu with selection. Sections are groups "
                        "labelled by their heading."
                    </li>
                    <li>
                        "The trigger has "<Code inline=true>"aria-haspopup"</Code>", "<Code inline=true>"aria-expanded"</Code>
                        " and "<Code inline=true>"aria-controls"</Code>"; the menu is labelled by the trigger. A context "
                        "menu\u{2019}s trigger has none of these, as a press doesn\u{2019}t open it."
                    </li>
                    <li>"A submenu trigger item has "<Code inline=true>"aria-haspopup"</Code>" and "<Code inline=true>"aria-expanded"</Code>"."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"On the trigger: open the menu and focus the first item."</KeyRow>
                    <KeyRow keys="ArrowUp">"On the trigger: open the menu and focus the last item."</KeyRow>
                    <KeyRow keys="Alt + ArrowDown / Alt + ArrowUp">
                        "On the trigger: open the menu. On a long press trigger, this (or "<Keys keys="Alt + Enter"/>" / "
                        <Keys keys="Alt + Space"/>") is the keyboard\u{2019}s way to open it, as a plain press runs the "
                        "button\u{2019}s own action."
                    </KeyRow>
                    <KeyRow keys="Shift + F10">
                        "On a context menu trigger: open the menu (also the context menu key, and "<Keys keys="Control + Enter"/>
                        " on macOS)."
                    </KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move between items."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last item."</KeyRow>
                    <KeyRow keys="Enter">"Activate the focused item and close the menu."</KeyRow>
                    <KeyRow keys="Space">"Activate the focused item; in a menu with selection, check it and keep the menu open."</KeyRow>
                    <KeyRow keys="Any character">"Focus the next item starting with the typed text."</KeyRow>
                    <KeyRow keys="ArrowRight / Enter / Space">
                        "On a submenu trigger item: open the submenu and focus its first item ("<Keys keys="ArrowLeft"/>
                        " in right-to-left languages)."
                    </KeyRow>
                    <KeyRow keys="ArrowLeft">
                        "In a submenu: close it and return focus to its trigger item ("<Keys keys="ArrowRight"/>" in "
                        "right-to-left languages)."
                    </KeyRow>
                    <KeyRow keys="Escape">"Close the menu, or only the submenu that has focus, and return focus to its trigger."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::menu::Hook.materialize()>"Menu Hooks"</Link></li>
                <li><Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></li>
                <li><Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
