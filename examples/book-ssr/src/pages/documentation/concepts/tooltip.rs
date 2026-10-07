use leptos::prelude::*;

use super::demos::tooltip::TooltipConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTooltipOverview() -> impl IntoView {
    view! {
        <DocPage title="Tooltip">
            <p>
                "A tooltip is a short text that describes an element while it is hovered or has keyboard focus. It is "
                "purely informational: content the user interacts with belongs in a "
                <Link href=routes::doc::Popover.materialize()>"Popover"</Link>"."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a short description on hover or focus"</TableCell><TableCell><b>"Tooltip"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Show interactive content in a popup"</TableCell><TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Provide a permanently visible label"</TableCell><TableCell>"The "<Link href=routes::doc::Field.materialize()>"Field"</Link>"\u{2019}s label"</TableCell></TableRow>
                    <TableRow><TableCell>"Use the browser\u{2019}s native tooltip"</TableCell><TableCell>"The "<Code inline=true>"title"</Code>" attribute"</TableCell></TableRow>
                </DocTable>

                <p>
                    "Tooltips opened by hovering don\u{2019}t appear at once: the first one waits for a delay, so that they "
                    "don\u{2019}t flash up while the pointer passes over the page. Once a tooltip was open, the next ones "
                    "appear at once, until a short cooldown after one closed. Focusing a trigger with the keyboard opens its "
                    "tooltip at once. Touch doesn\u{2019}t hover, so don\u{2019}t put information only touch users need into "
                    "a tooltip."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Tooltips exist as hooks and atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_tooltip_trigger_state"</Code>", "<Code inline=true>"use_tooltip_trigger"</Code>
                            " and "<Code inline=true>"use_tooltip"</Code>": state, trigger behavior and ARIA attributes for "
                            "elements you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"TooltipTrigger"</Code>" and an unstyled "<Code inline=true>"Tooltip"</Code>
                            ": wrap a button, a link or any element made focusable, with an optional arrow."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms are the quickest way to a tooltip: a "<Code inline=true>"TooltipTrigger"</Code>" around a "
                    <Code inline=true>"Button"</Code>" and its "<Code inline=true>"Tooltip"</Code>". See the "
                    <Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link>" for all options, and the "
                    <Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip Hooks"</Link>" to build your own."
                </p>

                <Demo description="Tooltip above a button" source=include_str!("demos/tooltip.rs") source_open=true>
                    <TooltipConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Tooltips follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/" target=LinkTarget::Blank>"Tooltip pattern"</Link>
                    "."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"tooltip\""</Code>" on the tooltip element."</li>
                    <li>
                        <Code inline=true>"aria-describedby"</Code>" on the trigger, pointing to the tooltip while it is visible."
                    </li>
                    <li>"Keyboard focus opens the tooltip; focusing the trigger by clicking it doesn\u{2019}t."</li>
                    <li>"Pressing the trigger hides the tooltip."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Focusing the trigger shows its tooltip at once; moving focus away hides it."</KeyRow>
                    <KeyRow keys="Escape">"Hides the tooltip. Only the tooltip closes: an enclosing dialog stays open."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip Hooks"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover"</Link></li>
                <li><Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
