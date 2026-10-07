use leptos::prelude::*;

use super::demos::field::FieldConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Field">
            <p>
                "A field is the frame around a form control: its label, an optional description and an error message, all "
                "connected to the control so that screen readers announce them together with it. Sighted users see the "
                "label next to the input; a screen reader user only hears it when the input references it by id. The field "
                "sets up these references, and keeps them correct as the description and the error message come and go."
            </p>
            <p>
                "Every form control of leptonic is a field: text, search and number fields, selects, comboboxes, sliders, "
                "and checkbox and radio groups. You use the field directly when you build a control of your own, or when you "
                "style one of leptonic\u{2019}s field atoms and place its label, description and error message yourself."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Label a control you build yourself, describe it and show its errors"</TableCell>
                        <TableCell><b>"Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Place the label, description and error message of a field atom"</TableCell>
                        <TableCell><b>"Field"</b>" atoms"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users enter text, with all parts already wired"</TableCell>
                        <TableCell><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users enter a number"</TableCell>
                        <TableCell><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Validate several fields together and submit them"</TableCell>
                        <TableCell><Link href=routes::doc::Form.materialize()>"Form"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "See "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>" for how fields, validation and "
                    "submission work together."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_label"</Code>" and "<Code inline=true>"use_field"</Code>": the ids and ARIA "
                            "attributes that connect a label, a description and an error message to a control you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                            <Code inline=true>"FieldError"</Code>", which connect themselves to the field atom around them ("
                            <Code inline=true>"TextField"</Code>", "<Code inline=true>"NumberField"</Code>", "
                            <Code inline=true>"Select"</Code>", \u{2026}). Your own fields provide them through a "
                            <Code inline=true>"LabelContext"</Code>" for the label and a "<Code inline=true>"FieldContext"</Code>
                            " for the description and the error message."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Put the field atoms inside a field atom, in any order and with any markup between them. The field gives "
                    "them their ids and references them from its input. Clear the input to see the error message:"
                </p>
                <Demo
                    description="Username text field with a label, a description and an error message shown while the value is too short"
                    source=include_str!("demos/field.rs")
                    source_open=true
                >
                    <FieldConceptDemo/>
                </Demo>
                <p>
                    "For a control you render yourself, call "
                    <Link href=format!("{}#use-field", routes::doc::field::Hook.materialize())>"use_field"</Link>
                    " and spread its props onto the label, the control, the description and the error message."
                </p>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A field follows the WAI-ARIA guidance on "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/practices/names-and-descriptions/" target=LinkTarget::Blank>
                        "names and descriptions"
                    </Link>":"
                </p>
                <ul>
                    <li>
                        "The label names the control: a "<Code inline=true>"<label for>"</Code>" for native inputs, a "
                        <Code inline=true>"<span>"</Code>" referenced with "<Code inline=true>"aria-labelledby"</Code>" for "
                        "groups and custom controls. Clicking a "<Code inline=true>"<label>"</Code>" focuses its input."
                    </li>
                    <li>
                        "The description and the error message describe the control through "
                        <Code inline=true>"aria-describedby"</Code>", but only while they are rendered, so screen readers never "
                        "point to a missing element."
                    </li>
                    <li>
                        "The control itself carries its validity ("<Code inline=true>"aria-invalid"</Code>") and whether it is "
                        "required. The error message is rendered only while the field is invalid."
                    </li>
                    <li>
                        "A field without a visible label needs an "<Code inline=true>"aria_label"</Code>" or "
                        <Code inline=true>"aria_labelledby"</Code>"; in debug builds, the hooks log a warning otherwise."
                    </li>
                </ul>
                <p>
                    "A field adds no keyboard interaction of its own: the keys are those of its control. The label, the "
                    "description and the error message are not focusable."
                </p>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the field\u{2019}s control, which announces its label, description and error message."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::Form.materialize()>"Form"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
