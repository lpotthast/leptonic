use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    label_basic::LabelBasicDemo, label_invalid::LabelInvalidDemo, label_span::LabelSpanDemo,
    label_valid::LabelValidDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseLabel() -> impl IntoView {
    view! {
        <DocPage title="use_label & use_field">
            <p>
                "Every form field needs an accessible name, and many have a description or an error message. Screen readers "
                "only announce them when the field references them by id. "<Code inline=true>"use_label"</Code>
                " connects a label with its field; "<Code inline=true>"use_field"</Code>
                " also connects a description and an error message."
            </p>

            <ReactAria hook="useLabel"/>

            <Section title="use_label">
                <Section title="Input" id="use-label-input">
                    <p><Code inline=true>"UseLabelInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="UseLabelInput">
                        <ApiRow name="id" ty="Option<String>" default="None">"The field\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". Without one, set "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                            "; otherwise the hook logs a warning."
                        </ApiRow>
                        <ApiRow name="label_element_type" ty="LabelElementType" default="Label">
                            <Code inline=true>"Label"</Code>" for a "<Code inline=true>"<label>"</Code>", "
                            <Code inline=true>"Span"</Code>" for any other element."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the field when there is no visible label. Next to a visible label, it is added to the field\u{2019}s name."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Further labelling elements."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-label-return">
                    <ApiTable kind=ApiKind::Return of="UseLabelReturn">
                        <ApiRow name="label_props" ty="UseLabelProps">
                            "The label\u{2019}s "<Code inline=true>"id"</Code>" and, for "<Code inline=true>"<label>"</Code>
                            " elements, "<Code inline=true>"for"</Code>". Spread with "<Code inline=true>"{..label_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="field_props" ty="UseLabelFieldProps">
                            "The field\u{2019}s "<Code inline=true>"id"</Code>", "<Code inline=true>"aria-label"</Code>" and "
                            <Code inline=true>"aria-labelledby"</Code>" (the visible label and further labelling elements)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-label-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseLabelReturn { label_props, field_props } = use_label(UseLabelInput {
                                has_label: true,
                                ..UseLabelInput::default()
                            });

                            view! {
                                <label {..label_props.into_attrs()}>"Username"</label>
                                <input type="text" {..field_props.into_attrs()}/>
                            }
                        "#)}
                    </Code>

                    <Demo description="Native label connected to a text input" source=include_str!("demos/label_basic.rs")>
                        <LabelBasicDemo/>
                    </Demo>
                </Section>

                <Section title="Span Labels">
                    <p>
                        "A "<Code inline=true>"<label>"</Code>" only labels native form controls. For custom controls, use "
                        <Code inline=true>"LabelElementType::Span"</Code>": the label gets no "<Code inline=true>"for"</Code>
                        ", and the field references it with "<Code inline=true>"aria-labelledby"</Code>" instead."
                    </p>

                    <Demo description="Span label for a custom contenteditable text box" source=include_str!("demos/label_span.rs")>
                        <LabelSpanDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_field">
                <p>
                    "Connects a field with its label, description and error message. Tell it whether you render a visible "
                    "label; the description and error message are noticed when you render them, and only then referenced in "
                    "the field\u{2019}s "<Code inline=true>"aria-describedby"</Code>"."
                </p>
                <p>
                    "The "<Link href=routes::doc::atoms::Field.materialize()>"field atoms"</Link>" "
                    <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                    <Code inline=true>"FieldError"</Code>" render these parts from the props of "
                    <Code inline=true>"use_field"</Code>"."
                </p>

                <Section title="Input" id="use-field-input">
                    <p><Code inline=true>"UseFieldInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="UseFieldInput">
                        <ApiRow name="id" ty="Option<String>" default="None">"The field\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". Without one, set "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                            "; otherwise the hook logs a warning."
                        </ApiRow>
                        <ApiRow name="label_element_type" ty="LabelElementType" default="Label">
                            "Whether the label is a "<Code inline=true>"<label>"</Code>" (gets "<Code inline=true>"for"</Code>
                            ") or another element."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the field when there is no visible label. Next to a visible label, it is added to the field\u{2019}s name."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Further labelling elements."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Further describing elements."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-field-return">
                    <ApiTable kind=ApiKind::Return of="UseFieldReturn">
                        <ApiRow name="label_props" ty="UseLabelProps">
                            "The label\u{2019}s "<Code inline=true>"id"</Code>" and, for "<Code inline=true>"<label>"</Code>
                            " elements, "<Code inline=true>"for"</Code>"."
                        </ApiRow>
                        <ApiRow name="field_props" ty="UseFieldProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"aria-label"</Code>", "
                            <Code inline=true>"aria-labelledby"</Code>" and a reactive "<Code inline=true>"aria-describedby"</Code>"."
                        </ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">
                            "Id and element capture for the description. Referenced while the element is rendered."
                        </ApiRow>
                        <ApiRow name="error_message_props" ty="SlotProps">
                            "Id and element capture for the error message. Render it only while the field is invalid."
                        </ApiRow>
                        <ApiRow name="description_id" ty="Signal<Option<String>>">
                            "The description\u{2019}s id while it is rendered, for further elements it describes (e.g. the "
                            "checkboxes of a group)."
                        </ApiRow>
                        <ApiRow name="error_message_id" ty="Signal<Option<String>>">"The error message\u{2019}s id while it is rendered."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-field-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseFieldReturn { label_props, field_props, description_props, error_message_props, .. } =
                                use_field(UseFieldInput {
                                    has_label: true,
                                    ..UseFieldInput::default()
                                });

                            view! {
                                <label {..label_props.into_attrs()}>"Email"</label>
                                <input type="email" {..field_props.into_attrs()}/>
                                <p {..description_props.into_attrs()}>"We'll never share your email."</p>
                                <Show when=move || is_invalid.get()>
                                    <p {..error_message_props.clone().into_attrs()}>"Please enter an email address."</p>
                                </Show>
                            }
                        "#)}
                    </Code>

                    <p>
                        "Validity is not part of "<Code inline=true>"use_field"</Code>": mark the field itself invalid (for "
                        "example with "<Code inline=true>"aria-invalid"</Code>") and render the error message only while it is."
                    </p>

                    <p>"A valid field with a description:"</p>

                    <Demo description="Email field with a label and a description" source=include_str!("demos/label_valid.rs")>
                        <LabelValidDemo/>
                    </Demo>

                    <p>"An invalid field with a description and an error message:"</p>

                    <Demo description="Invalid password field with a description and an error message" source=include_str!("demos/label_invalid.rs")>
                        <LabelInvalidDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="ARIA Attributes">
                <p>"The hooks set up these associations for you:"</p>
                <ul>
                    <li><Code inline=true>"id"</Code>" and "<Code inline=true>"for"</Code>" link a "<Code inline=true>"<label>"</Code>" to its field."</li>
                    <li>
                        <Code inline=true>"aria-labelledby"</Code>" connects the field with its label, also when the "
                        "label is a "<Code inline=true>"<label>"</Code>"."
                    </li>
                    <li>
                        <Code inline=true>"aria-describedby"</Code>" points to the description and the error message while they "
                        "are rendered."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::InputCategory.materialize()>"Input"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"use_text_field"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
