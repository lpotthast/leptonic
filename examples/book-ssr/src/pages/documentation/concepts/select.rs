use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::select::SelectConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSelectOverview() -> impl IntoView {
    view! {
        <DocPage title="Select">
            <p>
                "A select is a dropdown for choosing from a predefined list of options. "
                "It\u{2019}s the go-to control when the option set is too large for radio buttons "
                "but the user still needs to pick from a constrained list. "
                "The component layer offers three variants: "<Code inline=true>"Select"</Code>
                " (exactly one option), "<Code inline=true>"OptionalSelect"</Code>
                " (one or none), and "<Code inline=true>"Multiselect"</Code>"."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Choose one from many predefined options"</TableCell><TableCell><b>"Select"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Choose one or none"</TableCell><TableCell><b>"OptionalSelect"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Choose multiple from a list"</TableCell><TableCell><b>"Multiselect"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Search-and-select with free text"</TableCell><TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Choose from 2\u{2013}5 visible options"</TableCell><TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Show the options without a popover"</TableCell><TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Selects exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::select::Hook.materialize()>"use_select"</Link></TableCell>
                        <TableCell>"Behavior and ARIA attributes for a trigger, a listbox popover and a hidden native select you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::select::Atom.materialize()>"Select atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled parts ("<Code inline=true>"SelectTrigger"</Code>", "<Code inline=true>"SelectValue"</Code>", "
                            <Code inline=true>"SelectPopover"</Code>", \u{2026}) with that behavior, a positioned popover and a "
                            "hidden form element. The options are a "<Code inline=true>"ListBox"</Code>", with sections and "
                            "descriptions if you need them."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::select::Component.materialize()>"Select component"</Link></TableCell>
                        <TableCell>"Themed, searchable "<Code inline=true>"Select"</Code>", "<Code inline=true>"OptionalSelect"</Code>" and "<Code inline=true>"Multiselect"</Code>" over any option type."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The simplest way to use a select is the component layer. Options can be any type that implements "
                    <Code inline=true>"Clone + PartialEq + Display + Send + Sync"</Code>". The displayed text identifies an "
                    "option, so it must be unique among the options. "<Code inline=true>"search_text_provider"</Code>
                    " gives the text to search by, "<Code inline=true>"render_option"</Code>" renders an option."
                </p>

                <Demo description="Coffee size select" source=include_str!("demos/select.rs") source_open=true>
                    <SelectConceptDemo/>
                </Demo>

                <p>
                    "For full control over markup and styling, compose the headless "
                    <Link href=routes::doc::select::Atom.materialize()>"Select atoms"</Link>" instead: "
                    <Code inline=true>"Select"</Code>" with a "<Link href=routes::doc::atoms::Field.materialize()>"Label"</Link>", "
                    <Code inline=true>"SelectTrigger"</Code>" (containing "<Code inline=true>"SelectValue"</Code>"), "
                    <Code inline=true>"SelectPopover"</Code>" (containing a "<Code inline=true>"ListBox"</Code>
                    " with one "<Code inline=true>"ListBoxItem"</Code>" per option) and "
                    <Code inline=true>"HiddenSelect"</Code>" for forms. The options come from a collection, "
                    "just like for a "<Link href=routes::doc::listbox::Atom.materialize()>"ListBox"</Link>
                    ", so they can have sections and descriptions. A minimal select in a form:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let fruits = use_list_collection(
                            Signal::stored(vec!["Apple", "Banana", "Cherry"]),
                            |fruit| Key::from(*fruit),
                            |fruit| (*fruit).to_owned(),
                        );

                        view! {
                            <Select collection=fruits name="fruit" on_change=Callback::new(|keys: Vec<Key>| log!("{keys:?}"))>
                                <Label>"Fruit"</Label>
                                <SelectTrigger><SelectValue placeholder="Pick a fruit" /></SelectTrigger>
                                <SelectPopover>
                                    <ListBox>
                                        <ListBoxItem key="Apple">"Apple"</ListBoxItem>
                                        <ListBoxItem key="Banana">"Banana"</ListBoxItem>
                                        <ListBoxItem key="Cherry">"Cherry"</ListBoxItem>
                                    </ListBox>
                                </SelectPopover>
                                <HiddenSelect />
                            </Select>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The trigger is a button that opens a listbox popover, following the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox/" target=LinkTarget::_Blank>"Listbox pattern"</LinkExt>
                    " once open."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"aria-haspopup=\"listbox\""</Code>" and "<Code inline=true>"aria-expanded"</Code>
                        " \u{2014} on the trigger, reflecting the popover state"
                    </li>
                    <li>"The trigger is labelled by the selected value and the label"</li>
                    <li><Code inline=true>"role=\"listbox\""</Code>" \u{2014} on the options container"</li>
                    <li><Code inline=true>"role=\"option\""</Code>" + "<Code inline=true>"aria-selected"</Code>" \u{2014} on each option"</li>
                    <li>"A visually hidden native "<Code inline=true>"<select>"</Code>" takes part in forms and autofill"</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"Open the popover."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move between options."</KeyRow>
                    <KeyRow keys="Enter / Space">"Select the focused option."</KeyRow>
                    <KeyRow keys="Escape">"Close the popover."</KeyRow>
                    <KeyRow keys="Any character">"Type-ahead: jump to the first option starting with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
