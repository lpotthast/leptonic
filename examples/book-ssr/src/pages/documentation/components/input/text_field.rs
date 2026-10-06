use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    text_field_basic::TextFieldBasicDemo, text_field_description::TextFieldDescriptionDemo,
    text_field_password::TextFieldPasswordDemo, text_field_state::TextFieldStateDemo,
    text_field_validation::TextFieldValidationDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Component">
            <p>
                "The themed "<Code inline=true>"TextField"</Code>" component: an input or text area with its label, description and "
                "validation errors, styled by leptonic\u{2019}s theme. See the "<Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Name field greeting the entered name" source=include_str!("demos/text_field_basic.rs")>
                <TextFieldBasicDemo/>
            </Demo>

            <Section title="Props">
                <Section title="TextField">
                    <ApiTable kind=ApiKind::Props of="components::text_field::TextField">
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="MaybeProp<String>" default="None">"Help text below the input."</ApiRow>
                        <ApiRow name="default_value" ty="String" default="empty">
                            "The initial value, unless "<Code inline=true>"value"</Code>" is set. A form reset restores the value "
                            "the field started with."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<String>>" default="None">
                            "The value (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<String>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Text">
                            "The "<Code inline=true>"<input>"</Code>"\u{2019}s type: "<Code inline=true>"Text"</Code>", "
                            <Code inline=true>"Email"</Code>", "<Code inline=true>"Password"</Code>", "<Code inline=true>"Url"</Code>
                            ", "<Code inline=true>"Tel"</Code>" or "<Code inline=true>"Search"</Code>". Reactive, e.g. for a "
                            "\u{201c}show password\u{201d} toggle."
                        </ApiRow>
                        <ApiRow name="multiline" ty="bool" default="false">
                            "Renders a "<Code inline=true>"<textarea>"</Code>" instead of an "<Code inline=true>"<input>"</Code>"."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"Shown while the field is empty."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Marks the field as required (the native "<Code inline=true>"required"</Code>" with "
                            <Code inline=true>"Native"</Code>" validation, "<Code inline=true>"aria-required"</Code>" with "
                            <Code inline=true>"Aria"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown, see "<AnchorLink href="#validation">"Validation"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission and server errors."
                        </ApiRow>
                        <ApiRow name="min_length, max_length" ty="Option<u32>" default="None">"Length constraints."</ApiRow>
                        <ApiRow name="auto_complete" ty="Option<String>" default="None">
                            "The "<Code inline=true>"autocomplete"</Code>" hint, e.g. "<Code inline=true>"\"email\""</Code>" or "
                            <Code inline=true>"\"off\""</Code>"."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="generated">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">
                            "The id of the "<Code inline=true>"<form>"</Code>" the input belongs to, when it isn\u{2019}t inside it."
                        </ApiRow>
                        <ApiRow name="pattern" ty="Option<String>" default="None">"A regular expression the value must match (native validation)."</ApiRow>
                        <ApiRow name="input_mode" ty="Option<InputMode>" default="None">"Which virtual keyboard to show."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                            "Ids of further elements describing the field, in addition to its description and errors."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a field without label."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Labels and Descriptions">
                <p>
                    "A field\u{2019}s "<Code inline=true>"label"</Code>" is a "<Code inline=true>"<label>"</Code>" for its input: "
                    "pressing it focuses the input. The "<Code inline=true>"description"</Code>" below the input is read by "
                    "screen readers together with the label. A field without visible label needs an "
                    <Code inline=true>"aria_label"</Code>". Set "<Code inline=true>"multiline"</Code>" for longer text: the "
                    "field renders a "<Code inline=true>"<textarea>"</Code>", which the user can resize vertically."
                </p>
                <Demo
                    description="Email field and multi-line message field with descriptions, with disabled and read-only toggles"
                    source=include_str!("demos/text_field_description.rs")
                >
                    <TextFieldDescriptionDemo/>
                </Demo>
            </Section>

            <Section title="State">
                <p>
                    "A field either keeps its own value, starting at "<Code inline=true>"default_value"</Code>" and reporting "
                    "changes through "<Code inline=true>"on_change"</Code>" (as in the first demo), or shows your state, "
                    "given as "<Code inline=true>"value"</Code>", and hands every change to "<Code inline=true>"set_value"</Code>
                    ". Pass the same "<Code inline=true>"RwSignal"</Code>" to both, and the field shows whatever you write to it:"
                </p>
                <Demo
                    description="City field controlled by a signal that buttons change from outside"
                    source=include_str!("demos/text_field_state.rs")
                    source_open=true
                >
                    <TextFieldStateDemo/>
                </Demo>
            </Section>

            <Section title="Input Types">
                <p>
                    <Code inline=true>"input_type"</Code>" sets the type of the "<Code inline=true>"<input>"</Code>": "
                    <Code inline=true>"InputType::Password"</Code>" masks the entered text, "<Code inline=true>"Email"</Code>", "
                    <Code inline=true>"Url"</Code>" and "<Code inline=true>"Tel"</Code>" show matching virtual keyboards and "
                    "let the browser check the format. Combine it with "<Code inline=true>"auto_complete"</Code>" so password "
                    "managers fill the right fields."
                </p>
                <Demo description="Login form with a masked password field" source=include_str!("demos/text_field_password.rs")>
                    <TextFieldPasswordDemo/>
                </Demo>
            </Section>

            <Section title="Validation">
                <p>
                    "A field is invalid while a native constraint fails ("<Code inline=true>"is_required"</Code>", "
                    <Code inline=true>"min_length"</Code>", the input type, \u{2026}) or "<Code inline=true>"validate"</Code>
                    " returns errors. Its messages appear below the input, and the input gets "
                    <Code inline=true>"aria-invalid"</Code>" and a red border. "<Code inline=true>"validation_behavior"</Code>
                    " decides when: "<Code inline=true>"Aria"</Code>" shows errors as you type, "<Code inline=true>"Native"</Code>
                    " when the value is committed (you leave the changed field) or the form is submitted. Without one, a field "
                    "takes the behavior of the surrounding "<Link href=routes::doc::Form.materialize()>"Form"</Link>
                    ", else "<Code inline=true>"Native"</Code>"."
                </p>
                <Demo
                    description="Required username validated as you type, and a website address checked by the browser"
                    source=include_str!("demos/text_field_validation.rs")
                >
                    <TextFieldValidationDemo/>
                </Demo>
                <p>
                    "See "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>" for server errors, form "
                    "reset and the order of validation sources."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The text field, the "<Link href=routes::doc::search_field::Component.materialize()>"Search Field Component"</Link>
                    " and the "<Link href=routes::doc::number_field::Component.materialize()>"Number Field Component"</Link>
                    " share the input styling of leptonic\u{2019}s theme. Override any of these CSS variables to adapt them "
                    "to your design:"
                </p>
                <CssVariables prefix="--input-" scss=theme_scss!("input")/>
                <p>
                    "The field\u{2019}s "<Code inline=true>"<div>"</Code>" has the class "<Code inline=true>"leptonic-text-field"</Code>
                    " and the data attributes of the "<Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link>
                    ", e.g. "<Code inline=true>"data-invalid"</Code>" and "<Code inline=true>"data-disabled"</Code>". For full "
                    "control over the markup, build the field from the atoms."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::search_field::Component.materialize()>"Search Field Component"</Link></li>
                <li><Link href=routes::doc::number_field::Component.materialize()>"Number Field Component"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
