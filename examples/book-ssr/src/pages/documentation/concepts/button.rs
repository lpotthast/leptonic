use leptos::prelude::*;

use super::demos::button_basic::ButtonConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageButtonOverview() -> impl IntoView {
    view! {
        <DocPage title="Button">
            <p>
                "Buttons trigger an action when activated: submitting a form, opening a dialog, deleting an item. "
                "Users press them with a mouse, touch, a pen, the keyboard or assistive technology, and a button reacts "
                "to all of them in the same way."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Trigger an action, such as submit, delete or open"</TableCell><TableCell><b>"Button"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Navigate to another page or URL"</TableCell><TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Turn a setting on or off"</TableCell><TableCell><Link href=routes::doc::Switch.materialize()>"Switch"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Keep a button pressed, such as \u{201c}Bold\u{201d} in a toolbar"</TableCell><TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Select from a set of options"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link>" / "
                            <Link href=routes::doc::Radio.materialize()>"Radio"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "An element that looks like a button but navigates is a "<Link href=routes::doc::Link.materialize()>"link"</Link>
                    ", styled like your buttons. An element that looks like a link but triggers an action is a button."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Buttons exist as a hook and an atom. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></TableCell>
                        <TableCell>"Behavior and ARIA attributes for any element you render, such as a "<Code inline=true>"<div>"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"<button>"</Code>" with that behavior, styled through data attributes."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atom is the quickest way to a button. Pass "<Code inline=true>"on_press"</Code>
                    " to react to presses and "<Code inline=true>"is_disabled"</Code>" to turn it off, and style it "
                    "through its data attributes (the demo\u{2019}s CSS is on the "
                    <Link href=format!("{}#styling", routes::doc::button::Atom.materialize())>"Button Atom"</Link>" page):"
                </p>

                <Demo description="Button counting saves, with a disabled toggle" source=include_str!("demos/button_basic.rs") source_open=true>
                    <ButtonConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "All layers follow the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/button/" target=LinkTarget::Blank>"Button pattern"</Link>
                    " and behave the same."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"button\""</Code>
                        " is set on elements that aren\u{2019}t native buttons; native buttons keep their implicit role."
                    </li>
                    <li>
                        "A disabled native button gets the "<Code inline=true>"disabled"</Code>" attribute; other elements get "
                        <Code inline=true>"aria-disabled"</Code>"."
                    </li>
                    <li>
                        "A button whose content doesn\u{2019}t name it, such as an icon button, needs an "
                        <Code inline=true>"aria-label"</Code>"."
                    </li>
                    <li>
                        <Code inline=true>"aria-haspopup"</Code>" and "<Code inline=true>"aria-expanded"</Code>
                        " describe a popup (menu, dialog, \u{2026}) the button opens."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">
                        "Moves focus to the button, unless "<Code inline=true>"exclude_from_tab_order"</Code>" is set."
                    </KeyRow>
                    <KeyRow keys="Enter / Space">"Presses the button."</KeyRow>
                    <KeyRow keys="Shift + F10 / ContextMenu">
                        "Requests a context menu, when "<Code inline=true>"use_button"</Code>"\u{2019}s "
                        <Code inline=true>"on_context_menu"</Code>" is set. On macOS, "<Keys keys="Control + Enter"/>" does too."
                    </KeyRow>
                </KeyboardTable>
                <p>
                    "The "<Code inline=true>"shortcuts"</Code>" you pass to "<Code inline=true>"use_button"</Code>" are handled "
                    "first. Holding a key down doesn\u{2019}t trigger them again; see "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" for how shortcuts work."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
