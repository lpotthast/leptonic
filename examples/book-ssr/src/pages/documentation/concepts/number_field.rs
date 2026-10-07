use leptos::prelude::*;

use super::demos::number_field::NumberFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageNumberFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Number Field">
            <p>
                "A number field lets users enter a number by typing or by stepping it up and down with buttons, the arrow keys "
                "or the scroll wheel. It formats the number for the user\u{2019}s locale (decimal separators, percentages, "
                "currencies, units) and keeps it within a minimum and maximum."
            </p>
            <p>
                "Leptonic\u{2019}s number fields are generic over the number type: any primitive integer or float. Integers "
                "are exact and also stop at their type\u{2019}s bounds; the value is an "<Code inline=true>"Option"</Code>
                ", "<Code inline=true>"None"</Code>" while the field is empty. Typed text is committed when the field loses "
                "focus or on "<Keys keys="Enter"/>": it is parsed, clamped to the range and rounded to the step."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Enter an exact number, such as a quantity, a price or a percentage"</TableCell>
                        <TableCell><b>"Number Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a value from a range where the position matters more than the exact number"</TableCell>
                        <TableCell><Link href=routes::doc::Slider.materialize()>"Slider"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter digits that aren\u{2019}t a quantity, such as a phone number or a postal code"</TableCell>
                        <TableCell><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose one of a few fixed values"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Select.materialize()>"Select"</Link>" / "
                            <Link href=routes::doc::Radio.materialize()>"Radio"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter one channel of a color"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-color-channel-field", routes::doc::color_field::Hook.materialize())>"Color Field"</Link>
                            " ("<Code inline=true>"use_color_channel_field"</Code>")"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a date or a time"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::DateField.materialize()>"Date Field"</Link>", "
                            <Link href=routes::doc::TimeField.materialize()>"Time Field"</Link>" / "
                            <Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "A number field and a slider go well together: the slider for quick, rough changes, the field for the "
                    "exact value."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Number fields exist as hooks and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_number_field_state"</Code>" and "<Code inline=true>"use_number_field"</Code>
                            ": parsing, formatting, stepping, validation and the attributes of the input and its stepper buttons."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"NumberField"</Code>", "<Code inline=true>"NumberFieldGroup"</Code>
                            " and stepper buttons around an "<Code inline=true>"Input"</Code>", with the "
                            <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" for the label, description "
                            "and error message. Styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Compose the "<Code inline=true>"NumberField"</Code>" atom from a "<Code inline=true>"Label"</Code>
                    " and a "<Code inline=true>"NumberFieldGroup"</Code>" holding the "<Code inline=true>"Input"</Code>
                    " and the stepper buttons (the CSS is on the "
                    <Link href=format!("{}#styling", routes::doc::number_field::Atom.materialize())>"Number Field Atoms"</Link>
                    " page). Pass it a signal as "<Code inline=true>"value"</Code>" and its setter as "
                    <Code inline=true>"set_value"</Code>". The signal\u{2019}s type decides the number type, here "
                    <Code inline=true>"u8"</Code>". Type a number and press "<Keys keys="Enter"/>", or step it with the "
                    "buttons or the arrow keys:"
                </p>
                <Demo
                    description="Ticket count number field from 1 to 10 kept in a signal, with a disabled toggle"
                    source=include_str!("demos/number_field.rs")
                    source_open=true
                >
                    <NumberFieldConceptDemo/>
                </Demo>
                <p>
                    <Code inline=true>"format_options"</Code>" shows the number as a currency, a percentage or a unit, and "
                    <Code inline=true>"step"</Code>" sets the increment, e.g. "<Code inline=true>"0.05"</Code>" for "
                    "fractions."
                </p>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The input is a text box announced as \u{201c}Number field\u{201d} ("
                        <Code inline=true>"aria-roledescription"</Code>", except on iOS, where it interferes with VoiceOver). "
                        "Its keys follow the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/" target=LinkTarget::Blank>"Spinbutton pattern"</Link>
                        ", and value changes are announced to screen readers."
                    </li>
                    <li>
                        "The input and its stepper buttons are a "<Code inline=true>"role=\"group\""</Code>". The buttons are "
                        "named \u{201c}Increase\u{201d} and \u{201c}Decrease\u{201d} with the field\u{2019}s label, point to "
                        "the input with "<Code inline=true>"aria-controls"</Code>", aren\u{2019}t in the tab order and are "
                        "disabled at the limits."
                    </li>
                    <li>
                        "The label, description and error message are connected like those of any "
                        <Link href=routes::doc::Field.materialize()>"field"</Link>". On touch devices, the input asks for a "
                        "virtual keyboard that fits the range and the step: digits, a decimal separator or, where a "
                        "minus sign is needed on an iPhone, the full keyboard. Autocorrect and spell checking are off."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Increment or decrement by one step."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increment or decrement by one step."</KeyRow>
                    <KeyRow keys="Home / End">"Set to the minimum or maximum (for integers without one: the type\u{2019}s)."</KeyRow>
                    <KeyRow keys="Enter">"Commit the typed value. Inside a form, also submit it."</KeyRow>
                    <KeyRow keys="Tab">"Commit the typed value and move focus on, past the stepper buttons."</KeyRow>
                </KeyboardTable>
                <p>
                    "The stepping keys act only on their own: pressed together with "<Keys keys="Control"/>", "
                    <Keys keys="Shift"/>", "<Keys keys="Alt"/>" or "<Keys keys="Meta"/>", they are left to the browser."
                </p>
                <p>
                    "While the input has focus, the scroll wheel steps the value too. The atom and the hook turn this off with "
                    <Code inline=true>"is_wheel_disabled"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
                <li><Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link></li>
                <li><Link href=routes::doc::Slider.materialize()>"Slider"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></li>
                <li><Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
