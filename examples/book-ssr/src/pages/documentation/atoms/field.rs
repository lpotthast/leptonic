use indoc::indoc;
use leptos::prelude::*;

use super::demos::{field_custom::FieldCustomDemo, field_parts::FieldPartsDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomField() -> impl IntoView {
    view! {
        <DocPage title="Field Atoms">
            <p>
                <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                <Code inline=true>"FieldError"</Code>" are the unstyled label, description and error message of every field "
                "atom: place them anywhere inside the field, and it connects them to its control. See the "
                <Link href=routes::doc::Field.materialize()>"Field overview"</Link>"."
            </p>
            <p>
                "These field atoms provide them: "
                <Link href=routes::doc::checkbox::Atom.materialize()>"CheckboxGroup"</Link>", "
                <Link href=routes::doc::radio::Atom.materialize()>"RadioGroup"</Link>", "
                <Link href=routes::doc::select::Atom.materialize()>"Select"</Link>", "
                <Link href=routes::doc::combobox::Atom.materialize()>"ComboBox"</Link>", "
                <Link href=routes::doc::text_field::Atom.materialize()>"TextField"</Link>", "
                <Link href=routes::doc::search_field::Atom.materialize()>"SearchField"</Link>", "
                <Link href=routes::doc::number_field::Atom.materialize()>"NumberField"</Link>", "
                <Link href=routes::doc::slider::Atom.materialize()>"Slider"</Link>" and "
                <Link href=routes::doc::color_field::Atom.materialize()>"ColorField"</Link>
                ". Fields you build from hooks can provide them too (see "
                <AnchorLink href="#composition">"Composition"</AnchorLink>")."
            </p>

            <Section title="Hooks Used">
                <p>
                    "The parts render the label, description and error message props of the field\u{2019}s hook, which "
                    "come from "<Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link>" ("
                    <Code inline=true>"use_label"</Code>" and "<Code inline=true>"use_field"</Code>
                    "). The field passes the label props on in a "<Code inline=true>"LabelContext"</Code>", the description "
                    "and error message props in a "<Code inline=true>"FieldContext"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            field::{Description, FieldError, Label},
                            radio::{Radio, RadioGroup},
                        };

                        view! {
                            <RadioGroup is_required=true>
                                <Label>"Shipping"</Label>
                                <Radio value="standard">"Standard"</Radio>
                                <Radio value="express">"Express"</Radio>
                                <Description>"Express arrives within two business days."</Description>
                                <FieldError/>
                            </RadioGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"Choose Overnight to see the error message; it disappears when you choose another option."</p>
                <Demo
                    description="Shipping radio group with a label, a description and an error message shown after a failed validation"
                    source=include_str!("demos/field_parts.rs")
                >
                    <FieldPartsDemo/>
                </Demo>
            </Section>

            <Section title="Label">
                <p>
                    "The visible label of the field around it. The field decides the element: a "<Code inline=true>"<label>"</Code>
                    " pointing at the input for fields with a text input ("<Code inline=true>"TextField"</Code>", "
                    <Code inline=true>"SearchField"</Code>", "<Code inline=true>"NumberField"</Code>", "
                    <Code inline=true>"ComboBox"</Code>"), a "
                    <Code inline=true>"<span>"</Code>" the field references with "<Code inline=true>"aria-labelledby"</Code>
                    " for groups and the select, whose trigger the label focuses when clicked. Outside a field, it is a plain "
                    <Code inline=true>"<label>"</Code>"."
                </p>
                <p>
                    "A field without a visible label needs "<Code inline=true>"aria_label"</Code>" or "
                    <Code inline=true>"aria_labelledby"</Code>" instead."
                </p>
                <Section title="Props" id="label-props">
                    <ApiTable kind=ApiKind::Props of="atoms::field::Label">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Description">
                <p>
                    "A description of the field around it, as a "<Code inline=true>"<span>"</Code>". The field references it "
                    "with "<Code inline=true>"aria-describedby"</Code>" while it is rendered, so you can show it conditionally. "
                    "In a group, it also describes each checkbox or radio. Use "<Code inline=true>"element=TextElement::Div"</Code>
                    " for block content."
                </p>
                <Section title="Props" id="description-props">
                    <ApiTable kind=ApiKind::Props of="atoms::field::Description">
                        <ApiRow name="element" ty="TextElement" default="Span">
                            <Code inline=true>"Span"</Code>" or "<Code inline=true>"Div"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The description."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="FieldError">
                <p>
                    "The error message of the field around it, rendered only while the field is invalid. The field references "
                    "it with "<Code inline=true>"aria-describedby"</Code>" while it is rendered. Without children, it shows the "
                    "field\u{2019}s validation errors, separated by spaces, and renders nothing when there are none (e.g. "
                    "when only "<Code inline=true>"is_invalid"</Code>" is set). Pass "<Code inline=true>"message"</Code>
                    " to word the error yourself from the validation result, e.g. from "
                    <Code inline=true>"validation_details.value_missing"</Code>" (return "<Code inline=true>"None"</Code>
                    " to show no error), or children for a fixed message, and "
                    <Code inline=true>"element=TextElement::Div"</Code>" for block content such as a list. Outside a field, "
                    "it renders nothing."
                </p>
                <Section title="Props" id="field-error-props">
                    <ApiTable kind=ApiKind::Props of="atoms::field::FieldError">
                        <ApiRow name="element" ty="TextElement" default="Span">
                            <Code inline=true>"Span"</Code>" or "<Code inline=true>"Div"</Code>"."
                        </ApiRow>
                        <ApiRow name="message" ty="Option<FieldErrorMessage>" default="None">
                            "Words the error for the field\u{2019}s "<Code inline=true>"ValidationResult"</Code>": an "
                            <Code inline=true>"Arc<dyn Fn(&ValidationResult) -> Option<String>>"</Code>". Used without children."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the element."</ApiRow>
                        <ApiRow name="children" ty="Option<ChildrenFn>" default="None">
                            "The message. Defaults to the validation errors."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "The parts bring no styles and render no data attributes of their own. Their default classes are "
                    <Code inline=true>"leptonic-Label"</Code>" (a "<Code inline=true>"<label>"</Code>", or a "
                    <Code inline=true>"<span>"</Code>" inside a group), "<Code inline=true>"leptonic-Description"</Code>" and "
                    <Code inline=true>"leptonic-FieldError"</Code>" (both inline "<Code inline=true>"<span>"</Code>
                    "s: display them as blocks, or put them in a flex column, to give them their own line). Their content is "
                    "their children; a "<Code inline=true>"FieldError"</Code>" without children shows the validation errors."
                </p>
                <p>
                    "Select the field\u{2019}s state through the field atom\u{2019}s attributes, e.g. "
                    <Code inline=true>"data-invalid"</Code>". The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-field { display: flex; flex-direction: column; gap: 0.25rem; }
                        .demo-field-label { font-weight: 600; }
                        .demo-field-description { font-size: 0.875rem; color: var(--muted); }
                        .demo-field-error { font-size: 0.875rem; color: var(--danger); }

                        .demo-choice-group { display: flex; flex-direction: column; align-items: flex-start; gap: 0.5rem; }
                        .demo-choice-group-label { font-weight: 600; }
                        .demo-choice-group-description { display: block; font-size: 0.875rem; color: var(--muted); }
                        .demo-choice-group-error { display: block; font-size: 0.875rem; color: var(--danger); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find their field through context: "<Code inline=true>"Label"</Code>" through a "
                    <Code inline=true>"LabelContext"</Code>", "<Code inline=true>"Description"</Code>" and "
                    <Code inline=true>"FieldError"</Code>" through a "<Code inline=true>"FieldContext"</Code>". To use them "
                    "with a field you build from hooks, provide both with the props of "
                    <Link href=format!("{}#use-field", routes::doc::field::Hook.materialize())>"use_field"</Link>
                    " (or of a field hook such as "<Code inline=true>"use_text_field"</Code>") around them. For a text "
                    "field, use the "<Link href=routes::doc::text_field::Atom.materialize()>"TextField"</Link>
                    " atom; the demo builds one only to show the contexts."
                </p>
                <Demo
                    description="Username text field built from use_field, providing a LabelContext to its label and a FieldContext to its description and list of errors"
                    source=include_str!("demos/field_custom.rs")
                    source_open=true
                >
                    <FieldCustomDemo/>
                </Demo>

                <Section title="FieldContext">
                    <ApiTable kind=ApiKind::Fields of="FieldContext">
                        <ApiRow name="description" ty="SlotProps">"The hook\u{2019}s "<Code inline=true>"description_props"</Code>"."</ApiRow>
                        <ApiRow name="error_message" ty="SlotProps">"The hook\u{2019}s "<Code inline=true>"error_message_props"</Code>"."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the field is invalid; "<Code inline=true>"FieldError"</Code>" is rendered only while it is."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The errors a "<Code inline=true>"FieldError"</Code>" without children shows."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">
                            "Which constraints fail, for a "<Code inline=true>"FieldError"</Code>"\u{2019}s "<Code inline=true>"message"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="LabelContext">
                    <ApiTable kind=ApiKind::Fields of="LabelContext">
                        <ApiRow name="props" ty="UseLabelProps">"The hook\u{2019}s "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="element_type" ty="LabelElementType">
                            <Code inline=true>"Label"</Code>" or "<Code inline=true>"Span"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_click" ty="EventHandler<MouseEvent>">"Handles clicks on the label."</ApiRow>
                    </ApiTable>
                    <p>"Create it with one of its constructors:"</p>
                    <DocTable headers=&["Constructor", "Renders"]>
                        <TableRow>
                            <TableCell><Code inline=true>"LabelContext::label(props)"</Code></TableCell>
                            <TableCell>
                                "A "<Code inline=true>"<label>"</Code>", for fields with a native input. Pass "
                                <Code inline=true>"LabelElementType::Label"</Code>" to the hook."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"LabelContext::span(props)"</Code></TableCell>
                            <TableCell>
                                "A "<Code inline=true>"<span>"</Code>", for fields a "<Code inline=true>"<label>"</Code>
                                " can\u{2019}t label (groups, custom elements). Pass "<Code inline=true>"LabelElementType::Span"</Code>
                                " to the hook."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>".with_on_click(handler)"</Code></TableCell>
                            <TableCell>"Runs the handler on clicks on the label, e.g. to focus a custom element."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Field.materialize()>"Field overview"</Link></li>
                <li><Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link></li>
                <li><Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::radio::Atom.materialize()>"Radio Atoms"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
