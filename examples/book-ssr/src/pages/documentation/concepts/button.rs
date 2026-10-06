use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{button_basic::ButtonBasicConceptDemo, button_styled::ButtonStyledConceptDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageButtonOverview() -> impl IntoView {
    view! {
        <DocPage title="Button">
            <p>
                "Buttons trigger an action when activated: submitting a form, opening a dialog, deleting an item. "
                "They can be pressed with a mouse, touch, pen, the keyboard and assistive technology."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Trigger an action (submit, delete, open)"</TableCell><TableCell><b>"Button"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Navigate to another page or URL"</TableCell><TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Toggle a binary state on or off"</TableCell><TableCell><Link href=routes::doc::Switch.materialize()>"Toggle"</Link></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Select from a set of options"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link>" / "
                            <Link href=routes::doc::Radio.materialize()>"Radio"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "If an element looks like a button but navigates, use a "<Code inline=true>"LinkButton"</Code>
                    ". If an element looks like a link but triggers an action, use a button."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Buttons exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></TableCell>
                        <TableCell>"Behavior and ARIA attributes for any element you render, e.g. a "<Code inline=true>"<div>"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"<button>"</Code>" with that behavior, for your own design."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::button::Component.materialize()>"Button component"</Link></TableCell>
                        <TableCell>"A themed button with colors, variants, sizes and groups."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to a button:"</p>

                <Demo description="Basic button" source=include_str!("demos/button_basic.rs") source_open=true>
                    <ButtonBasicConceptDemo/>
                </Demo>

                <p>"Give it a color and a variant:"</p>

                <Demo description="Primary filled button" source=include_str!("demos/button_styled.rs") source_open=true>
                    <ButtonStyledConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "All layers follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/button/" target=LinkTarget::_Blank>"Button pattern"</LinkExt>
                    " and behave the same."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"button\""</Code>
                        " is set on elements that aren\u{2019}t native buttons; native buttons keep their implicit role."
                    </li>
                    <li>
                        <Code inline=true>"aria-disabled"</Code>" marks disabled non-native buttons; native ones use the "
                        <Code inline=true>"disabled"</Code>" attribute."
                    </li>
                    <li>
                        <Code inline=true>"aria-haspopup"</Code>" and "<Code inline=true>"aria-expanded"</Code>
                        " describe a popup (menu, dialog, \u{2026}) the button opens."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the button."</KeyRow>
                    <KeyRow keys="Enter / Space">"Activates the button."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
