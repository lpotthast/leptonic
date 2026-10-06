use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{field_custom::FieldCustomDemo, field_parts::FieldPartsDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomField() -> impl IntoView {
    view! {
        <DocPage title="Field Atoms">
            <p>
                "A form field needs a visible label, often a description, and an error message while its value is invalid. "
                "Screen readers announce them only when the field references them by id. "<Code inline=true>"Label"</Code>", "
                <Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>" are these parts for every "
                "field atom: place them anywhere inside the field, and the field wires up the ids. They are unstyled."
            </p>
            <p>
                "These field atoms provide them: "
                <Link href=routes::doc::checkbox::Atom.materialize()>"CheckboxGroup"</Link>", "
                <Link href=routes::doc::radio::Atom.materialize()>"RadioGroup"</Link>", "
                <Link href=routes::doc::select::Atom.materialize()>"Select"</Link>", "
                <Link href=routes::doc::combobox::Atom.materialize()>"ComboBox"</Link>", "
                <Link href=routes::doc::text_field::Atom.materialize()>"TextField"</Link>" and "
                <Link href=format!("{}#searchfield", routes::doc::text_field::Atom.materialize())>"SearchField"</Link>
                ". Your own fields can provide them too, see "<a href="#fieldcontext">"FieldContext"</a>"."
            </p>

            <Section title="Hooks Used">
                <p>
                    "The parts render the label, description and error message props of the field\u{2019}s hook, which "
                    "come from "<Link href=routes::doc::hooks::UseLabel.materialize()>"use_label and use_field"</Link>
                    ". The field passes them on in a "<Code inline=true>"FieldContext"</Code>"."
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
                            <RadioGroup default_value="standard" validate=validate_shipping>
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
                    <Code inline=true>"SearchField"</Code>", "<Code inline=true>"ComboBox"</Code>"), a "
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
                    "The parts render no data attributes of their own. Give them classes, and select the field\u{2019}s state "
                    "through the field atom\u{2019}s attributes, e.g. "<Code inline=true>"data-invalid"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-label { font-weight: 600; }
                        .my-description { display: block; font-size: 0.85em; color: gray; }
                        .my-error { display: block; font-size: 0.85em; color: crimson; }
                        .my-group[data-invalid] .my-label { color: crimson; }
                    ")}
                </Code>
                <p>
                    <Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>" are inline "
                    <Code inline=true>"<span>"</Code>"s by default: display them as blocks to put them on their own line."
                </p>
            </Section>

            <Section title="FieldContext">
                <p>
                    "The parts find their field through a "<Code inline=true>"FieldContext"</Code>". To use them with a field "
                    "you build from hooks, provide one with the props of "
                    <Link href=format!("{}#use-field", routes::doc::hooks::UseLabel.materialize())>"use_field"</Link>
                    " (or of a field hook such as "<Code inline=true>"use_text_field"</Code>") around them. (For a text "
                    "field, use the "<Link href=routes::doc::text_field::Atom.materialize()>"TextField"</Link>
                    " atom; the demo builds one to show the context.)"
                </p>
                <Demo
                    description="Username text field built from use_field, providing a FieldContext to its label, description and list of errors"
                    source=include_str!("demos/field_custom.rs")
                    source_open=true
                >
                    <FieldCustomDemo/>
                </Demo>

                <Section title="Fields" id="field-context-fields">
                    <ApiTable kind=ApiKind::Fields of="FieldContext">
                        <ApiRow name="label" ty="FieldLabelProps">"How the "<Code inline=true>"Label"</Code>" is rendered."</ApiRow>
                        <ApiRow name="description" ty="SlotProps">"The hook\u{2019}s "<Code inline=true>"description_props"</Code>"."</ApiRow>
                        <ApiRow name="error_message" ty="SlotProps">"The hook\u{2019}s "<Code inline=true>"error_message_props"</Code>"."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the field is invalid; "<Code inline=true>"FieldError"</Code>" is rendered only while it is."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The errors a "<Code inline=true>"FieldError"</Code>" without children shows."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">
                            "Which constraints fail, for a "<Code inline=true>"FieldError"</Code>"\u{2019}s "<Code inline=true>"message"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="FieldLabelProps">
                    <ApiTable kind=ApiKind::Fields of="FieldLabelProps">
                        <ApiRow name="props" ty="UseLabelProps">"The hook\u{2019}s "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="element_type" ty="LabelElementType">
                            <Code inline=true>"Label"</Code>" or "<Code inline=true>"Span"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_click" ty="EventHandler<MouseEvent>">"Handles clicks on the label."</ApiRow>
                    </ApiTable>
                    <p>"Create it with one of its constructors:"</p>
                    <DocTable headers=&["Constructor", "Renders"]>
                        <TableRow>
                            <TableCell><Code inline=true>"FieldLabelProps::label(props)"</Code></TableCell>
                            <TableCell>
                                "A "<Code inline=true>"<label>"</Code>", for fields with a native input. Pass "
                                <Code inline=true>"LabelElementType::Label"</Code>" to the hook."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"FieldLabelProps::span(props)"</Code></TableCell>
                            <TableCell>
                                "A "<Code inline=true>"<span>"</Code>", for fields a "<Code inline=true>"<label>"</Code>
                                " can\u{2019}t label (groups, custom widgets). Pass "<Code inline=true>"LabelElementType::Span"</Code>
                                " to the hook."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>".with_on_click(handler)"</Code></TableCell>
                            <TableCell>"Runs the handler on clicks on the label, e.g. to focus a custom widget."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::hooks::UseLabel.materialize()>"use_label and use_field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox atoms"</Link></li>
                <li><Link href=routes::doc::radio::Atom.materialize()>"Radio atoms"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select atoms"</Link></li>
                <li><Link href=routes::doc::combobox::Atom.materialize()>"Combobox atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text field atoms"</Link></li>
                <li><Link href=routes::doc::atoms::Form.materialize()>"Form atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
