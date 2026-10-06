use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageButtons() -> impl IntoView {
    view! {
        <DocPage title="Buttons">
            <p>
                "Buttons trigger an action when pressed: submitting a form, opening a dialog, deleting an item. Users press "
                "them with a mouse, touch, a pen, the keyboard ("<Keys keys="Enter"/>" or "<Keys keys="Space"/>
                ") or a screen reader, and every button reacts to all of these in the same way."
            </p>
            <p>
                "The members of this group share that press behavior. A toggle button is a button that additionally "
                "stays pressed until it is pressed again, and screen readers announce it as pressed or not pressed."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Buttons.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        "A toggle button is built on the button: "
                        <Link href=format!("{}#use-toggle-button", routes::doc::toggle_button::Hook.materialize())>"use_toggle_button"</Link>
                        " configures "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                        " so that each press flips a toggle state, and "<Code inline=true>"aria-pressed"</Code>
                        " tells screen readers whether the button is pressed."
                    </li>
                    <li>
                        "A toggle button group is a "<Link href=routes::doc::Toolbar.materialize()>"toolbar"</Link>
                        ": the whole group is one tab stop, and the arrow keys move between its buttons. A group that "
                        "allows only one pressed button is announced as a radio group."
                    </li>
                    <li>
                        "A button that navigates is a link. "
                        <Link href=format!("{}#linkbutton", routes::doc::link::Atom.materialize())>
                            <Code inline=true>"LinkButton"</Code>
                        </Link>
                        " renders a "<Link href=routes::doc::Link.materialize()>"link"</Link>
                        " that looks and presses like a button; its themed version is one of the "
                        <Link href=format!("{}#linkbutton", routes::doc::link::Component.materialize())>"Link Components"</Link>"."
                    </li>
                    <li>
                        "Triggers of other concepts hand their behavior to a button: put a button into a "
                        <Code inline=true>"DialogTrigger"</Code>", "<Code inline=true>"MenuTrigger"</Code>" or "
                        <Code inline=true>"TooltipTrigger"</Code>", and it opens the "
                        <Link href=routes::doc::Dialog.materialize()>"dialog"</Link>", "
                        <Link href=routes::doc::Menu.materialize()>"menu"</Link>" or "
                        <Link href=routes::doc::Tooltip.materialize()>"tooltip"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Trigger an action, such as submit, delete or open"</TableCell>
                        <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show several buttons side by side as one visual unit"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#buttongroup", routes::doc::button::Component.materialize())>
                                <Code inline=true>"ButtonGroup"</Code>
                            </Link>" of the Button Components"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Lay out separate buttons in a row that wraps when space runs out"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#buttonwrapper", routes::doc::button::Component.materialize())>
                                <Code inline=true>"ButtonWrapper"</Code>
                            </Link>" of the Button Components"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Switch a mode or a formatting option on and off, such as bold text"</TableCell>
                        <TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose one of a few compact options, such as the text alignment"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link>
                            " group with single selection"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate to another page, with an element that looks like a button"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#linkbutton", routes::doc::link::Atom.materialize())>
                                <Code inline=true>"LinkButton"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Turn a setting on or off with immediate effect"</TableCell>
                        <TableCell><Link href=routes::doc::Switch.materialize()>"Switch"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Set an option that is submitted with a form"</TableCell>
                        <TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "Toggle buttons fit toolbars and headers. They don\u{2019}t submit with forms; for a choice that "
                    "belongs to a form, use a checkbox, a switch or a radio group from "
                    <Link href=routes::doc::Fields.materialize()>"Fields"</Link>"."
                </p>
            </Section>
        </DocPage>
    }
}
