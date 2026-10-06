use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::toggle_button::ToggleButtonConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageToggleButtonOverview() -> impl IntoView {
    view! {
        <DocPage title="Toggle Button">
            <p>
                "A toggle button is a button that stays pressed until it is pressed again, like the bold button of a text "
                "editor. Toggle button groups combine several of them: either any number can be pressed (bold, italic, "
                "underline), or exactly one (left, center or right alignment)."
            </p>
            <p>
                "Leptonic builds toggle buttons on its "<Link href=routes::doc::Button.materialize()>"button"</Link>
                " behavior and groups on the "<Link href=routes::doc::hooks::UseToolbar.materialize()>"toolbar"</Link>
                ": a group is one tab stop, and the arrow keys move between its buttons."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Toggle a mode or formatting option, often in a toolbar"</TableCell><TableCell><b>"Toggle Button"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Turn a setting on or off with immediate effect"</TableCell>
                        <TableCell><Link href=routes::doc::Switch.materialize()>"Switch"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Set a form option that is submitted later"</TableCell>
                        <TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose one of a few options in a form"</TableCell>
                        <TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Trigger a one-time action"</TableCell>
                        <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Toggle buttons are compact and fit toolbars and headers. Unlike checkboxes, switches and radios, they "
                    "don\u{2019}t submit with forms."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle button hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_toggle_button"</Code>" and the group hooks: the input of "
                            <Code inline=true>"use_button"</Code>" for buttons you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle button atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ToggleButton"</Code>" and "<Code inline=true>"ToggleButtonGroup"</Code>
                            ", styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>"There is no themed toggle button component yet."</p>
            </Section>

            <Section title="Quick Start">
                <p>"Bind the atom to a signal with "<Code inline=true>"state"</Code>" and style it through "<Code inline=true>"data-selected"</Code>":"</p>
                <Demo
                    description="Star toggle button bound to a signal, showing its state"
                    source=include_str!("demos/toggle_button.rs")
                    source_open=true
                >
                    <ToggleButtonConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "A toggle button is a "<Code inline=true>"<button>"</Code>" with "<Code inline=true>"aria-pressed"</Code>
                        ", following the WAI-ARIA "
                        <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/button/" target=LinkTarget::_Blank>"Button pattern"</LinkExt>
                        ". Keep its label the same in both states; the pressed state is announced separately."
                    </li>
                    <li>
                        "A group is a "<Code inline=true>"role=\"toolbar\""</Code>" (WAI-ARIA "
                        <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/" target=LinkTarget::_Blank>"Toolbar pattern"</LinkExt>
                        "). With single selection, it is a "<Code inline=true>"role=\"radiogroup\""</Code>" of buttons with "
                        <Code inline=true>"role=\"radio\""</Code>" and "<Code inline=true>"aria-checked"</Code>"."
                    </li>
                    <li>"Give the group an accessible name with "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>"."</li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the button, or into and out of a group."</KeyRow>
                    <KeyRow keys="Space / Enter">"Toggles the focused button."</KeyRow>
                    <KeyRow keys="Arrow keys">"In a group: focuses the next or previous button along the group\u{2019}s orientation."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
