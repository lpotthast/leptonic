use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    number_field_basic::NumberFieldBasicDemo, number_field_disabled::NumberFieldDisabledDemo,
    number_field_fractional::NumberFieldFractionalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseNumberField() -> impl IntoView {
    view! {
        <DocPage title="use_number_field">
            <p>
                "The "<Code inline=true>"use_number_field"</Code>" hook builds a number input with increment and decrement "
                "buttons, keyboard and scroll wheel stepping, and locale-aware formatting. See the "
                <Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useNumberField"/>

            <Section title="Architecture">
                <p>"The number field is split into two hooks:"</p>

                <DocTable headers=&["Hook", "Responsibility"]>
                    <TableRow>
                        <TableCell><Code inline=true>"use_number_field_state"</Code></TableCell>
                        <TableCell>
                            "Owns the value and the displayed text, commits typed text (parse, snap or validate, format), "
                            "steps the value and validates it."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_number_field"</Code></TableCell>
                        <TableCell>
                            "Labelling, ARIA attributes, keyboard and scroll wheel handling, input filtering, stepper button "
                            "configuration and commit on blur and "<Keys keys="Enter"/>"."
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The keyboard handling, hold-to-spin buttons and screen reader announcements come from "
                    <Link href=routes::doc::hooks::UseSpinButton.materialize()>"use_spin_button"</Link>". For a ready-made "
                    "field, use the "<Link href=routes::doc::text_field::NumberFieldAtom.materialize()>"NumberField atom"</Link>"."
                </p>
            </Section>

            <Section title="Value Types">
                <p>
                    "Both hooks are generic over the value type "<Code inline=true>"T"</Code>": every primitive integer ("
                    <Code inline=true>"i8"</Code>" to "<Code inline=true>"i128"</Code>", "<Code inline=true>"u8"</Code>" to "
                    <Code inline=true>"u128"</Code>", "<Code inline=true>"isize"</Code>", "<Code inline=true>"usize"</Code>
                    ") and float ("<Code inline=true>"f32"</Code>", "<Code inline=true>"f64"</Code>"), through the "
                    <Code inline=true>"NumberValue"</Code>" trait. Usually "<Code inline=true>"T"</Code>" is inferred from "
                    <Code inline=true>"default_value"</Code>" or the bound state."
                </p>
                <ul>
                    <li>
                        "Integers are exact, also beyond 2"<sup>"53"</sup>". Stepping saturates at the type\u{2019}s bounds, "
                        "which also apply without "<Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>
                        ": a "<Code inline=true>"u8"</Code>" field stops at 0 and 255, and "<Keys keys="Home"/>" / "
                        <Keys keys="End"/>" jump there. Text with fraction digits or out of range isn\u{2019}t a value of the type."
                    </li>
                    <li>
                        "Floats are rounded to the step\u{2019}s precision, so stepping by 0.1 three times gives exactly 0.3."
                    </li>
                </ul>
            </Section>

            <Section title="use_number_field_state">
                <Section title="Input" id="use-number-field-state-input">
                    <p>
                        <Code inline=true>"UseNumberFieldStateInput<T>"</Code>" implements "<Code inline=true>"Default"</Code>
                        ". Formatting and parsing follow the locale of the enclosing "<Code inline=true>"I18nProvider"</Code>
                        " (the default locale without one)."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseNumberFieldStateInput">
                        <ApiRow name="default_value" ty="Option<T>" default="None">
                            "The initial value, restored on form reset. "<Code inline=true>"None"</Code>" starts empty."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<T>>>" default="None">
                            "The value as app state, replacing "<Code inline=true>"default_value"</Code>" (see "
                            <a href="#binding-app-state">"Binding App State"</a>")."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">
                            "Called when the committed value changes."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<T>>" default="None">
                            "The allowed range. Integer types are also bounded by their own range."
                        </ApiRow>
                        <ApiRow name="step" ty="Signal<Option<T>>" default="None">
                            "The step of increments; "<Code inline=true>"None"</Code>" steps by 1 (0.01 for percentages). "
                            "Committed values snap to it only when it is set."
                        </ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="decimal, grouped">
                            "Formatting: style (decimal, percent, currency, unit), grouping, integer, fraction and significant "
                            "digits, sign display."
                        </ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior" default="Snap">
                            "What happens to a typed value outside the range or between steps: "<Code inline=true>"Snap"</Code>
                            " clamps it and rounds it to the step, "<Code inline=true>"Validate"</Code>" keeps it and reports "
                            "it as invalid (range overflow, underflow, step mismatch)."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">"Validates the committed value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as the value changes, "<Code inline=true>"Native"</Code>
                            " on form submission."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The field\u{2019}s name, matching server errors of a surrounding form."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="State" id="use-number-field-state-state">
                    <p>
                        "The hook returns a "<Code inline=true>"NumberFieldState<T>"</Code>" ("<Code inline=true>"Copy"</Code>
                        "), which you pass to "<Code inline=true>"use_number_field"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Fields of="NumberFieldState">
                        <ApiRow name="number_value" ty="Signal<Option<T>>">
                            "The value of the typed text; "<Code inline=true>"None"</Code>" while it is empty or unparsable."
                        </ApiRow>
                        <ApiRow name="input_value" ty="Signal<String>">"The text in the input: the formatted value, or what the user is typing."</ApiRow>
                        <ApiRow name="can_increment, can_decrement" ty="Signal<bool>">"Whether stepping is possible."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<T>>">"The range."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>">"The effective step."</ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>">"The format options."</ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior">"How typed values are committed."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>">"Disabled and read-only state."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The validation state."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"The validation behavior."</ApiRow>
                        <ApiRow name="default_number_value" ty="Option<T>">"The value a form reset restores."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Purpose"]>
                        <TableRow>
                            <TableCell><Code inline=true>"value()"</Code></TableCell>
                            <TableCell>"The committed value, a "<Code inline=true>"Signal<Option<T>>"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_number_value(value)"</Code></TableCell>
                            <TableCell>"Sets the value and formats the text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_input_value(text)"</Code></TableCell>
                            <TableCell>"Sets the typed text without committing it."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"commit(text)"</Code></TableCell>
                            <TableCell>
                                "Commits the typed text ("<Code inline=true>"None"</Code>") or the given text (a paste): parses, "
                                "snaps or validates and formats it. Unparsable text reverts to the committed value."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"increment()"</Code>", "<Code inline=true>"decrement()"</Code>", "
                                <Code inline=true>"increment_to_max()"</Code>", "<Code inline=true>"decrement_to_min()"</Code>
                            </TableCell>
                            <TableCell>"Step the value; to the maximum or minimum, the type\u{2019}s bounds without explicit ones."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validate(text)"</Code></TableCell>
                            <TableCell>"Whether the text may be typed: a number, or the beginning of one, in the current locale."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_number_field">
                <Section title="Input" id="use-number-field-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseNumberFieldInput::new(state)"</Code>" and set further "
                        "fields with struct update syntax. Range, step, format, disabled and read-only state come from the state."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseNumberFieldInput">
                        <ApiRow name="state" ty="NumberFieldState<T>">"The state from "<Code inline=true>"use_number_field_state"</Code>"."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the field when there is no visible label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">
                            "Further labelling and describing elements."
                        </ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Sets "<Code inline=true>"aria-required"</Code>"."</ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focus the input when it mounts."</ApiRow>
                        <ApiRow name="is_wheel_disabled" ty="bool" default="false">
                            "Leave the value alone on scroll. Otherwise the wheel steps while the field has focus."
                        </ApiRow>
                        <ApiRow name="increment_aria_label, decrement_aria_label" ty="MaybeProp<String>" default="None">
                            "Replace the stepper buttons\u{2019} names (\u{201c}Increase \u{2026}\u{201d}, \u{201c}Decrease \u{2026}\u{201d})."
                        </ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Focus callbacks."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Keyboard callbacks."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-number-field-return">
                    <ApiTable kind=ApiKind::Return of="UseNumberFieldReturn">
                        <ApiRow name="group_props" ty="UseNumberFieldGroupProps">
                            <Code inline=true>"role=\"group\""</Code>", "<Code inline=true>"aria-disabled"</Code>" and "
                            <Code inline=true>"aria-invalid"</Code>" for the element wrapping input and buttons."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"Id and "<Code inline=true>"for"</Code>" of the label."</ApiRow>
                        <ApiRow name="input_props" ty="UseNumberFieldInputProps">
                            "Attributes and handlers for the "<Code inline=true>"<input>"</Code>", including its value: the input "
                            "shows the state\u{2019}s text by itself."
                        </ApiRow>
                        <ApiRow name="increment_button, decrement_button" ty="UseButtonInput">
                            "Configuration for the stepper buttons. Pass them to "<Code inline=true>"use_button"</Code>"."
                        </ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">"For the description; referenced while it is rendered."</ApiRow>
                        <ApiRow name="error_message_props" ty="SlotProps">"For the error message. Render it only while "<Code inline=true>"is_invalid"</Code>"."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The input, once rendered."</ApiRow>
                        <ApiRow name="is_focused, is_focus_visible" ty="Signal<bool>">"Whether the input has focus, and whether to show a focus ring."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity state."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Example">
                <Demo
                    description="Quantity number field from 0 to 100 with stepper buttons"
                    source=include_str!("demos/number_field_basic.rs")
                    source_open=true
                >
                    <NumberFieldBasicDemo/>
                </Demo>
            </Section>

            <Section title="Stepper Buttons">
                <p>
                    "The hook doesn\u{2019}t hand you DOM props for the increment and decrement buttons. It returns "
                    <Code inline=true>"increment_button"</Code>" and "<Code inline=true>"decrement_button"</Code>", two "
                    <Code inline=true>"UseButtonInput"</Code>"s that you pass to "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                    ". They come preconfigured the way a number field\u{2019}s steppers should behave:"
                </p>

                <ul>
                    <li>"They are left out of the tab order. Keyboard users change the value with the arrow keys in the input instead."</li>
                    <li>
                        "They don\u{2019}t take focus when pressed. With a mouse, focus moves to the input, so you can keep typing. "
                        "If the input already has focus, it keeps it, so tapping a button doesn\u{2019}t close the software keyboard."
                    </li>
                    <li>
                        "They are named \u{201c}Increase "<i>"aria_label"</i>"\u{201d} and \u{201c}Decrease "<i>"aria_label"</i>
                        "\u{201d}, or \u{201c}Increase\u{201d} and \u{201c}Decrease\u{201d} together with the visible label, "
                        "and point to the input with "<Code inline=true>"aria-controls"</Code>". Use "
                        <Code inline=true>"increment_aria_label"</Code>" and "<Code inline=true>"decrement_aria_label"</Code>
                        " to pick your own labels."
                    </li>
                    <li>"They are disabled once the value reaches the minimum or maximum, but stay focusable, so focus isn\u{2019}t lost."</li>
                    <li>"Holding a button keeps stepping until you let go or the limit is reached."</li>
                </ul>

                <p>
                    "Since they are plain inputs, you can add your own settings with struct update syntax, e.g. "
                    <Code inline=true>"use_button(UseButtonInput { on_hover_change: .., ..field.increment_button })"</Code>"."
                </p>
            </Section>

            <Section title="Binding App State">
                <p>
                    "To keep the value in your app\u{2019}s state, pass a "<Code inline=true>"ValueBinding"</Code>" as "
                    <Code inline=true>"value"</Code>": "<Code inline=true>"ValueBinding::from(rw_signal)"</Code>" for an "
                    <Code inline=true>"RwSignal<Option<T>>"</Code>", from a "<Code inline=true>"(ReadSignal, WriteSignal)"</Code>
                    " pair, or "<Code inline=true>"ValueBinding::new(signal, setter)"</Code>". The field writes committed "
                    "values to it, and its text follows changes you make to the signal."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        let quantity = RwSignal::new(Some(1_u32));
                        let state = use_number_field_state(UseNumberFieldStateInput {
                            value: Some(quantity.into()),
                            ..UseNumberFieldStateInput::default()
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Forms">
                <p>
                    "The input shows the formatted text (e.g. \u{201c}1,024\u{201d}), so it has no "<Code inline=true>"name"</Code>
                    ". To submit the value with a form, render a hidden input with the field\u{2019}s name and "
                    <Code inline=true>"state.number_value"</Code>"; the "
                    <Link href=routes::doc::text_field::NumberFieldAtom.materialize()>"NumberField atom"</Link>" does this when "
                    "you give it a "<Code inline=true>"name"</Code>". Form resets restore "<Code inline=true>"default_value"</Code>"."
                </p>
            </Section>

            <Section title="Fractional Step">
                <p>
                    "Floating-point precision is handled for you: incrementing by 0.1 three times gives exactly 0.3, not "
                    "0.30000000000000004."
                </p>

                <Demo description="Number field from 0 to 1 with step 0.1" source=include_str!("demos/number_field_fractional.rs")>
                    <NumberFieldFractionalDemo/>
                </Demo>
            </Section>

            <Section title="Disabled State">
                <p>
                    "Set "<Code inline=true>"is_disabled"</Code>" on the state; the field reads it. The group gets "<Code inline=true>"aria-disabled"</Code>
                    ", the input and the stepper buttons are disabled."
                </p>

                <Demo description="Number field with a disabled toggle" source=include_str!("demos/number_field_disabled.rs")>
                    <NumberFieldDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Increment or decrement by one step."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increment or decrement by one step."</KeyRow>
                    <KeyRow keys="Home">"Set to the minimum (for integers without one: the type\u{2019}s minimum)."</KeyRow>
                    <KeyRow keys="End">"Set to the maximum (for integers without one: the type\u{2019}s maximum)."</KeyRow>
                    <KeyRow keys="Enter">
                        "Commit the typed value. Inside a form, "<Keys keys="Enter"/>" still submits it, with the committed value."
                    </KeyRow>
                </KeyboardTable>

                <p>"Keys pressed with Ctrl, Shift, Alt or Meta are ignored. Typed text is also committed on blur."</p>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The input has "<Code inline=true>"aria-roledescription=\"Number field\""</Code>
                        " (except on iOS, where it interferes with VoiceOver)."
                    </li>
                    <li>
                        "The group wrapper has "<Code inline=true>"role=\"group\""</Code>" with "
                        <Code inline=true>"aria-disabled"</Code>" and "<Code inline=true>"aria-invalid"</Code>"."
                    </li>
                    <li>"The stepper buttons point to the input with "<Code inline=true>"aria-controls"</Code>"."</li>
                    <li>"Autocorrect and spellcheck are off, and the virtual keyboard layout (" <Code inline=true>"inputmode"</Code>") depends on the range, the step and the platform."</li>
                    <li>"Value changes are announced to screen readers."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::NumberFieldAtom.materialize()>"Number Field atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseSpinButton.materialize()>"use_spin_button"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
