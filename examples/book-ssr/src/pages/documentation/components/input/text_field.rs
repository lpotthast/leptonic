use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    number_field::NumberFieldDemo, search_field::SearchFieldDemo,
    text_field_basic::TextFieldBasicDemo, text_field_description::TextFieldDescriptionDemo,
    text_field_password::TextFieldPasswordDemo, text_field_state::TextFieldStateDemo,
    text_field_validation::TextFieldValidationDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Components">
            <p>
                "The themed "<Code inline=true>"TextField"</Code>", "<Code inline=true>"SearchField"</Code>" and "
                <Code inline=true>"NumberField"</Code>": an input with its label, description and validation errors, "
                "styled by leptonic\u{2019}s theme. See the "<Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>
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
                        <ApiRow name="default_value" ty="String" default="empty">"The initial value, restored on form reset."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="state" ty="Option<TextFieldState>" default="None">
                            "Binds the field to your state, replacing "<Code inline=true>"default_value"</Code>": an "
                            <Code inline=true>"RwSignal<String>"</Code>", a "<Code inline=true>"(ReadSignal, WriteSignal)"</Code>
                            " pair or a "<Code inline=true>"TextFieldState"</Code>"."
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
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the field as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown, see "<a href="#validation">"Validation"</a>"."
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

                <Section title="SearchField">
                    <ApiTable kind=ApiKind::Props of="components::text_field::SearchField">
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="MaybeProp<String>" default="None">"Help text below the input."</ApiRow>
                        <ApiRow name="default_value" ty="String" default="empty">"The initial value, restored on form reset."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="state" ty="Option<TextFieldState>" default="None">
                            "Binds the field to your state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_submit" ty="Option<Callback<String>>" default="None">
                            "Called with the value when "<Keys keys="Enter"/>" is pressed. Without it, "<Keys keys="Enter"/>
                            " submits the form."
                        </ApiRow>
                        <ApiRow name="on_clear" ty="Option<Callback<()>>" default="None">
                            "Called when "<Keys keys="Escape"/>" or the clear button empties the field."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"Shown while the field is empty."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the field as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the field as invalid, regardless of "<Code inline=true>"validate"</Code>"."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">"Validates the value; returns the error messages."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "Native or ARIA validation. Default: the surrounding "<Code inline=true>"Form"</Code>"\u{2019}s, else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
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

                <Section title="NumberField">
                    <ApiTable kind=ApiKind::Props of="components::number_field::NumberField">
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="MaybeProp<String>" default="None">"Help text below the input."</ApiRow>
                        <ApiRow name="default_value" ty="Option<T>" default="None">"The initial value; "<Code inline=true>"None"</Code>" starts empty."</ApiRow>
                        <ApiRow name="state" ty="Option<ValueBinding<Option<T>>>" default="None">
                            "Binds the field to your state, replacing "<Code inline=true>"default_value"</Code>": an "
                            <Code inline=true>"RwSignal<Option<T>>"</Code>" or a "<Code inline=true>"(ReadSignal, WriteSignal)"</Code>" pair."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">"Called with the value when a committed value changes."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="MaybeProp<T>" default="None">
                            "The range. Without them, integer types stop at their own bounds."
                        </ApiRow>
                        <ApiRow name="step" ty="MaybeProp<T>" default="None">
                            "The step of increments; without one, 1 (0.01 for percentages). Typed values snap to it only when it is set."
                        </ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="decimal, grouped">
                            "Formatting: style (decimal, percent, currency, unit), grouping, digits, sign display."
                        </ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior" default="Snap">
                            <Code inline=true>"Snap"</Code>" clamps typed values and rounds them to the step; "
                            <Code inline=true>"Validate"</Code>" keeps them and reports them as invalid."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"Shown while the field is empty."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the field as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown, see "<a href="#validation">"Validation"</a>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The name of a hidden input holding the value, for form submission and server errors."
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
                    "changes through "<Code inline=true>"on_change"</Code>" (as in the first demo), or works on your state, "
                    "given as "<Code inline=true>"state"</Code>". A bound signal receives every change, and the field shows "
                    "whatever you write to it:"
                </p>
                <Demo
                    description="City field bound to a signal that buttons change from outside"
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
                    "takes the behavior of the surrounding "<Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>
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

            <Section title="Search">
                <p>
                    <Code inline=true>"SearchField"</Code>" is a text field for queries: "<Keys keys="Enter"/>" calls "
                    <Code inline=true>"on_submit"</Code>", and "<Keys keys="Escape"/>" or the clear button, shown while "
                    "there is a value, empty it and call "<Code inline=true>"on_clear"</Code>"."
                </p>
                <Demo
                    description="Recipe search listing the recipes that match the submitted query"
                    source=include_str!("demos/search_field.rs")
                >
                    <SearchFieldDemo/>
                </Demo>
            </Section>

            <Section title="Numbers">
                <p>
                    <Code inline=true>"NumberField<T>"</Code>" works on any primitive integer or float type "
                    <Code inline=true>"T"</Code>", taken from its "<Code inline=true>"state"</Code>" or "
                    <Code inline=true>"default_value"</Code>". Its value is an "<Code inline=true>"Option<T>"</Code>": "
                    <Code inline=true>"None"</Code>" while the field is empty. The stepper buttons, "<Keys keys="ArrowUp"/>" and "
                    <Keys keys="ArrowDown"/>" change it by "<Code inline=true>"step"</Code>" within "
                    <Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>"; "<Keys keys="Home"/>" and "
                    <Keys keys="End"/>" jump to the limits. "<Code inline=true>"format_options"</Code>" formats it for the "
                    "current locale, as a currency, a percentage or a unit."
                </p>
                <Demo
                    description="Order form with an integer quantity, a price in euros and a percentage discount, with a disabled toggle"
                    source=include_str!("demos/number_field.rs")
                >
                    <NumberFieldDemo/>
                </Demo>
                <p>
                    "Typed values are committed when the input loses focus or on "<Keys keys="Enter"/>". By default ("
                    <Code inline=true>"CommitBehavior::Snap"</Code>"), a value outside the range is clamped and rounded to the "
                    "step; with "<Code inline=true>"CommitBehavior::Validate"</Code>" it is kept and reported as invalid."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The fields share the input styling of leptonic\u{2019}s theme. Override any of these CSS variables to "
                    "adapt them to your design:"
                </p>
                <CssVariables prefix="--input-" scss=theme_scss!("input")/>
                <p>
                    "The field\u{2019}s "<Code inline=true>"<div>"</Code>" has the class "<Code inline=true>"leptonic-text-field"</Code>
                    " (and "<Code inline=true>"leptonic-search-field"</Code>" or "<Code inline=true>"leptonic-number-field"</Code>
                    ") and the data attributes of the "<Link href=routes::doc::text_field::Atom.materialize()>"Text Field atoms"</Link>
                    ", e.g. "<Code inline=true>"data-invalid"</Code>" and "<Code inline=true>"data-disabled"</Code>". For full "
                    "control over the markup, build the field from the atoms."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field atoms"</Link></li>
                <li><Link href=routes::doc::text_field::NumberFieldAtom.materialize()>"Number Field atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"use_text_field and use_search_field"</Link></li>
                <li><Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
