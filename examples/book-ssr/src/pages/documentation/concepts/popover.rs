use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover::PopoverConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PagePopoverOverview() -> impl IntoView {
    view! {
        <DocPage title="Popover">
            <p>
                "Popovers are floating overlays anchored to a trigger element. They show contextual content: rich "
                "descriptions, small forms, previews."
            </p>

            <p>
                "Under the hood, a popover combines dismissal (Escape, outside interaction, blur) with placement next to "
                "the trigger that adapts to the available space."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show rich contextual content anchored to an element"</TableCell><TableCell><b>"Popover"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Block the page and require a decision"</TableCell><TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show a short text label on hover or focus"</TableCell><TableCell><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show a list of actions triggered by a button"</TableCell><TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell></TableRow>
                </DocTable>

                <p>
                    "Popovers are modal or non-modal. A non-modal popover lets you keep interacting with the page behind "
                    "it (the Popover component\u{2019}s default). A modal popover traps focus, prevents scrolling and closes "
                    "on any outside interaction (the atom\u{2019}s default, as for menus and selects)."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Popovers exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                        <TableCell>"Positioning and dismiss behavior for elements you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Atom.materialize()>"Popover atom"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"Popover"</Code>" opened by a "<Code inline=true>"DialogTrigger"</Code>
                            " around a button, or by your own state."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Component.materialize()>"Popover component"</Link></TableCell>
                        <TableCell>"A themed popover that opens when its trigger button is pressed, with dialog semantics."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The component is the quickest way to a popover. A "<Code inline=true>"Button"</Code>
                    " inside the trigger slot toggles it without any wiring:"
                </p>

                <Demo description="Popover component toggled by a button" source=include_str!("demos/popover.rs") source_open=true>
                    <PopoverConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>"The popover component behaves as follows:"</p>

                <ul>
                    <li>
                        "The content has "<Code inline=true>"role=\"dialog\""</Code>" (or "<Code inline=true>"\"alertdialog\""</Code>
                        "). Hidden dismiss buttons at its start and end let screen reader users close it."
                    </li>
                    <li>
                        "The trigger has "<Code inline=true>"aria-expanded"</Code>" and, while the popover is open, "
                        <Code inline=true>"aria-controls"</Code>" pointing to it."
                    </li>
                    <li>
                        "A modal popover traps focus and hides the rest of the page from assistive technology. A non-modal "
                        "popover lets focus move freely."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the popover."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus. In a modal popover, focus stays inside."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
