use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tooltip::TooltipConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTooltipOverview() -> impl IntoView {
    view! {
        <DocPage title="Tooltip">
            <p>
                "Tooltips are small, non-interactive labels that appear on hover or keyboard focus to describe an element. "
                "They are purely informational. If the user needs to interact with the popup content, use a "
                <Link href=routes::doc::Popover.materialize()>"Popover"</Link>" instead."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a short description on hover or focus"</TableCell><TableCell><b>"Tooltip"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Show interactive content in a popup"</TableCell><TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Provide a permanently visible label"</TableCell><TableCell><Link href=routes::doc::hooks::UseLabel.materialize()>"Label"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Use the browser\u{2019}s native tooltip"</TableCell><TableCell>"The "<Code inline=true>"title"</Code>" attribute"</TableCell></TableRow>
                </DocTable>

                <p>
                    "Tooltips opened by hovering don\u{2019}t appear instantly: the first one waits for a warmup delay, so "
                    "they don\u{2019}t flash up while the pointer passes over the page. Once a tooltip has been shown, the next "
                    "ones appear immediately, until a short cooldown after the last one closed. Focusing a trigger opens its "
                    "tooltip immediately."
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
                        <TableCell><Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_tooltip_trigger_state"</Code>", "<Code inline=true>"use_tooltip_trigger"</Code>
                            " and "<Code inline=true>"use_tooltip"</Code>": state, trigger behavior and ARIA attributes for "
                            "elements you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"TooltipTrigger"</Code>" and an unstyled "<Code inline=true>"Tooltip"</Code>
                            ": wrap any button or link, no wiring needed."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms are the quickest way to a tooltip: a "<Code inline=true>"TooltipTrigger"</Code>" around a "
                    <Code inline=true>"Button"</Code>" and its "<Code inline=true>"Tooltip"</Code>". See the "
                    <Link href=routes::doc::tooltip::Atom.materialize()>"tooltip atoms"</Link>" for all options, and the "
                    <Link href=routes::doc::tooltip::Hook.materialize()>"tooltip hooks"</Link>" to build your own."
                </p>

                <Demo description="Tooltip above a button" source=include_str!("demos/tooltip.rs") source_open=true>
                    <TooltipConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Tooltips follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/" target=LinkTarget::_Blank>"Tooltip pattern"</LinkExt>
                    "."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"tooltip\""</Code>" on the tooltip element."</li>
                    <li>
                        <Code inline=true>"aria-describedby"</Code>" on the trigger, pointing to the tooltip while it is visible."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Focusing the trigger shows the tooltip immediately."</KeyRow>
                    <KeyRow keys="Escape">"Hides the tooltip. The Escape press is not passed on, so an enclosing dialog stays open."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
