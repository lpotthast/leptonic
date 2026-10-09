use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    field_email::FieldEmailDemo, label_basic::LabelBasicDemo, label_span::LabelSpanDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseLabel() -> impl IntoView {
    view! {
        <DocPage title="Field Hooks">
            <p>
                <Code inline=true>"use_label"</Code>" connects a field with its visible label; "
                <Code inline=true>"use_field"</Code>" also connects a description and an error message. See the "
                <Link href=routes::doc::Field.materialize()>"Field overview"</Link>"."
            </p>

            <Section title="use_label">
                <ReactAria hook="useLabel"/>

                <Section title="Input" id="use-label-input">
                    <p><Code inline=true>"UseLabelInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="UseLabelInput">
                        <ApiRow name="id" ty="Option<String>" default="None">"The field\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". Without one, set "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                            "; in debug builds, the hook logs a warning otherwise."
                        </ApiRow>
                        <ApiRow name="label_element_type" ty="LabelElementType" default="Label">
                            <Code inline=true>"Label"</Code>" for a "<Code inline=true>"<label>"</Code>", "
                            <Code inline=true>"Span"</Code>" for any other element (see "
                            <AnchorLink href="#span-labels">"Span Labels"</AnchorLink>")."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the field when there is no visible label. Next to a visible label, it is added to the field\u{2019}s name."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "Ids of further labelling elements, separated by spaces."
                        </ApiRow>
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
                            use leptonic::hooks::form::{UseLabelInput, UseLabelReturn, use_label};

                            let UseLabelReturn { label_props, field_props } = use_label(UseLabelInput {
                                has_label: true.into(),
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
                        ", and the field references it with "<Code inline=true>"aria-labelledby"</Code>" instead. Clicking such "
                        "a label doesn\u{2019}t focus the field."
                    </p>

                    <Demo description="Span label for a custom contenteditable text box" source=include_str!("demos/label_span.rs")>
                        <LabelSpanDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_field">
                <ReactAria hook="useField"/>
                <p>
                    "Connects a field with its label, description and error message. Tell it whether you render a visible "
                    "label; the description and the error message are noticed when you render them, and only then referenced "
                    "in the field\u{2019}s "<Code inline=true>"aria-describedby"</Code>"."
                </p>
                <p>
                    "The "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" "
                    <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                    <Code inline=true>"FieldError"</Code>" render these parts: inside a field atom, they take the props of "
                    "its hook ("<Code inline=true>"label_props"</Code>", "<Code inline=true>"description_props"</Code>", "
                    <Code inline=true>"error_message_props"</Code>") from context."
                </p>

                <Section title="Input" id="use-field-input">
                    <p><Code inline=true>"UseFieldInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="UseFieldInput">
                        <ApiRow name="id" ty="Option<String>" default="None">"The field\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". Without one, set "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                            "; in debug builds, the hook logs a warning otherwise."
                        </ApiRow>
                        <ApiRow name="label_element_type" ty="LabelElementType" default="Label">
                            "Whether the label is a "<Code inline=true>"<label>"</Code>" (gets "<Code inline=true>"for"</Code>
                            ") or another element."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the field when there is no visible label. Next to a visible label, it is added to the field\u{2019}s name."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "Ids of further labelling elements, separated by spaces."
                        </ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                            "Ids of further describing elements, separated by spaces. Referenced after the description and the "
                            "error message."
                        </ApiRow>
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
                    <p>
                        "Validity is not part of "<Code inline=true>"use_field"</Code>": mark the field itself invalid (with "
                        <Code inline=true>"aria-invalid"</Code>") and render the error message only while it is. The "
                        "demo shows the ids the input references: type a valid address and the error message\u{2019}s id "
                        "disappears with it."
                    </p>

                    <Demo
                        description="Email field with a description and an error message rendered while the address is invalid"
                        source=include_str!("demos/field_email.rs")
                        source_open=true
                    >
                        <FieldEmailDemo/>
                    </Demo>

                    <p>
                        "To validate the value like leptonic\u{2019}s fields do, and to take part in a form\u{2019}s "
                        "submission and reset, add the "<Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link>"."
                    </p>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Field.materialize()>"Field overview"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
