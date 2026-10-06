use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::date_picker::DatePickerDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDatePickerHooks() -> impl IntoView {
    view! {
        <DocPage title="Date Picker Hooks">
            <p>
                "The date picker hooks combine a "<Link href=routes::doc::date_time::DateFieldHooks.materialize()>"date field"</Link>
                " with a button that opens a calendar in a popover. See the "
                <Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link>" for how the date and time "
                "hooks fit together."
            </p>

            <ReactAria hook="useDatePicker"/>

            <Section title="Demo">
                <p>
                    "Type a date into the segments, or open the calendar with the button (or Alt + Arrow Down in the "
                    "field), move through the days with the arrow keys and press Enter. Escape or a click outside closes "
                    "the calendar."
                </p>

                <Demo
                    description="Date picker built from use_date_picker, use_date_field, use_popover and the calendar hooks"
                    source=include_str!("demos/date_picker.rs")
                >
                    <DatePickerDemo/>
                </Demo>
            </Section>

            <Section title="Composition">
                <p>"A date picker is four parts, each with its own hook:"</p>

                <DocTable headers=&["Part", "Hook", "Gets"]>
                    <TableRow>
                        <TableCell>"Group with label, description and error message"</TableCell>
                        <TableCell><Code inline=true>"use_date_picker"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"group_props"</Code>", "<Code inline=true>"label_props"</Code>", "
                            <Code inline=true>"description_props"</Code>", "<Code inline=true>"error_props"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Date field"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::date_time::DateFieldHooks.materialize()><Code inline=true>"use_date_field"</Code></Link>
                            " with "<Code inline=true>"is_date_picker: true"</Code>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"value: picker.state.value"</Code>", "
                            <Code inline=true>"on_change: Some(picker.state.set_value)"</Code>"; "
                            <Code inline=true>"picker.field_props"</Code>" on an element around it"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Calendar button"</TableCell>
                        <TableCell><Code inline=true>"use_date_picker"</Code></TableCell>
                        <TableCell><Code inline=true>"button_props"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Popover with a dialog and a calendar"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::popover::Hook.materialize()><Code inline=true>"use_popover"</Code></Link>
                            " and the "<Link href=routes::doc::date_time::CalendarHooks.materialize()>"calendar hooks"</Link>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"is_open"</Code>", "<Code inline=true>"close"</Code>", "
                            <Code inline=true>"dialog_props"</Code>", "<Code inline=true>"calendar_props"</Code>
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The popover is yours: the picker hooks only track whether it is open. "<Code inline=true>"use_popover"</Code>
                    " positions it below the field and closes it on Escape and on clicks outside. Return the focus to the "
                    "button when the popover closes, as the demo does."
                </p>

                <p>
                    "leptonic has no single "<Code inline=true>"use_calendar"</Code>" hook. Build the calendar from "
                    <Code inline=true>"use_calendar_state"</Code>", "<Code inline=true>"use_calendar_grid"</Code>" and "
                    <Code inline=true>"use_calendar_cell"</Code>" (documented on the "
                    <Link href=routes::doc::date_time::CalendarHooks.materialize()>"calendar hooks"</Link>" page) and "
                    "configure it from "<Code inline=true>"calendar_props"</Code>": its "<Code inline=true>"value"</Code>" "
                    "as the calendar\u{2019}s initial value, its "<Code inline=true>"on_change"</Code>" as the calendar\u{2019}s "
                    "selection callback, and "<Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>" and "
                    <Code inline=true>"default_focused_value"</Code>". Create the calendar when the popover opens, so that "
                    "it starts at the current value. The range calendar hooks don\u{2019}t fit: a date picker selects a "
                    "single date."
                </p>
            </Section>

            <Section title="use_date_picker_state">
                <p>
                    "The state behind "<Code inline=true>"use_date_picker"</Code>": the value, the open state, the dates "
                    "and times picked but not committed yet, and validation. "<Code inline=true>"use_date_picker"</Code>
                    " creates it and returns it as "<Code inline=true>"state"</Code>"."
                </p>

                <Section title="Input" id="use-date-picker-state-input">
                    <ApiTable kind=ApiKind::Input of="UseDatePickerStateInput">
                        <ApiRow name="default_value" ty="Option<OffsetDateTime>" default="None">
                            "The initial value. Its time and offset are also used for dates picked without a time."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>" default="None">"Values outside these bounds are invalid."</ApiRow>
                        <ApiRow name="show_time" ty="bool" default="false">
                            "Whether the value includes a time, which changes when a calendar selection is committed (see "
                            <a href="#committing-values">"Committing Values"</a>")."
                        </ApiRow>
                        <ApiRow name="should_close_on_select" ty="bool" default="true">"Close the popover when a date is picked."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<Date, bool>>" default="None">
                            "Marks dates as unavailable; an unavailable value is invalid."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<OffsetDateTime>>>" default="None">"Called when the value changes."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the popover opens or closes."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Prevent opening the popover."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<OffsetDateTime>>>" default="None">"Custom validation."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"When errors are displayed."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "Matches server-side errors provided through "<Code inline=true>"FormValidationContext"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-picker-state-return">
                    <ApiTable kind=ApiKind::Return of="UseDatePickerStateReturn">
                        <ApiRow name="value" ty="Signal<Option<OffsetDateTime>>">"The committed value."</ApiRow>
                        <ApiRow name="set_value" ty="Callback<Option<OffsetDateTime>>">
                            "Commits a value, drops picked but uncommitted parts and commits the validation. Wire the date "
                            "field\u{2019}s "<Code inline=true>"on_change"</Code>" to it."
                        </ApiRow>
                        <ApiRow name="date_value" ty="Signal<Option<Date>>">"The committed date, or the date picked but not committed yet."</ApiRow>
                        <ApiRow name="set_date_value" ty="Callback<Date>">"Picks a date (from the calendar)."</ApiRow>
                        <ApiRow name="time_value" ty="Signal<Option<TimeValue>>">"The committed time, or the time picked but not committed yet."</ApiRow>
                        <ApiRow name="set_time_value" ty="Callback<TimeValue>">"Picks a time (from a time field in the popover)."</ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the popover is open."</ApiRow>
                        <ApiRow name="open, close" ty="Callback<()>">"Open or close the popover."</ApiRow>
                        <ApiRow name="set_open" ty="Callback<bool>">
                            "Opens or closes the popover. Opening does nothing while disabled or read-only."
                        </ApiRow>
                        <ApiRow name="clear" ty="Callback<()>">"Clears the value and everything picked."</ApiRow>
                        <ApiRow name="has_time" ty="bool">"The "<Code inline=true>"show_time"</Code>" input."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The form validation state."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the value is invalid: form validation, bounds or an unavailable date."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">
                            "The error messages, including \u{201C}Date is before the minimum allowed date.\u{201D}, "
                            "\u{201C}Date is after the maximum allowed date.\u{201D} and \u{201C}Selected date is "
                            "unavailable.\u{201D}"
                        </ApiRow>
                        <ApiRow name="formatted_value" ty="Signal<String>">
                            "The value as text, e.g. \u{201C}October 13, 2026\u{201D} (with \u{201C} at 14:30\u{201D} when "
                            <Code inline=true>"show_time"</Code>" is set); empty without a value."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_date_picker">
                <p>
                    "Creates the state and adds IDs and ARIA attributes for the group, label, button, popover dialog and "
                    "the hidden \u{201C}Selected date: \u{2026}\u{201D} description, which it appends to the document body "
                    "and references from the group and the button."
                </p>

                <Section title="Input" id="use-date-picker-input">
                    <p>
                        <Code inline=true>"UseDatePickerInput"</Code>" implements "<Code inline=true>"Default"</Code>
                        ". It has the fields of "<Code inline=true>"use_date_picker_state"</Code>"\u{2019}s input, plus "
                        "the label, description and error message settings."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseDatePickerInput">
                        <ApiRow name="default_value" ty="Option<OffsetDateTime>" default="None">
                            "The initial value. Its time and offset are also used for dates picked without a time."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>" default="None">"Values outside these bounds are invalid."</ApiRow>
                        <ApiRow name="show_time" ty="bool" default="false">
                            "Whether the value includes a time, which changes when a calendar selection is committed (see "
                            <a href="#committing-values">"Committing Values"</a>")."
                        </ApiRow>
                        <ApiRow name="should_close_on_select" ty="bool" default="true">"Close the popover when a date is picked."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<Date, bool>>" default="None">
                            "Marks dates as unavailable; an unavailable value is invalid."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<OffsetDateTime>>>" default="None">"Called when the value changes."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the popover opens or closes."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Prevent opening the popover."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<OffsetDateTime>>>" default="None">"Custom validation."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"When errors are displayed."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "Matches server-side errors provided through "<Code inline=true>"FormValidationContext"</Code>"."
                        </ApiRow>
                        <ApiRow name="label" ty="Option<String>" default="None">
                            "Whether you render a label. "<Code inline=true>"Some"</Code>" labels the group, button and dialog "
                            "with "<Code inline=true>"label_props.id"</Code>"; otherwise they reference "
                            <Code inline=true>"field_props.id"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="Option<String>" default="None">
                            "Whether you render a description ("<Code inline=true>"description_props.id"</Code>")."
                        </ApiRow>
                        <ApiRow name="error_message" ty="Option<String>" default="None">
                            "Whether you render an error message ("<Code inline=true>"error_props.id"</Code>"). Also passed "
                            "on in "<Code inline=true>"calendar_props"</Code>"."
                        </ApiRow>
                        <ApiRow name="hour_cycle_24, is_required" ty="bool" default="true, false">
                            "Unused. Pass them to "<Code inline=true>"use_date_field"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-picker-return">
                    <ApiTable kind=ApiKind::Return of="UseDatePickerReturn">
                        <ApiRow name="group_props" ty="UseDatePickerGroupProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"role=\"group\""</Code>", "
                            <Code inline=true>"aria-labelledby"</Code>", "<Code inline=true>"aria-describedby"</Code>" (selected "
                            "date, description, error), "<Code inline=true>"aria-disabled"</Code>", the Alt + Arrow keydown "
                            "handler, and an element capture used by the label."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseDatePickerLabelProps">
                            <Code inline=true>"id"</Code>" and a click handler that focuses the field\u{2019}s tab stop. Spread "
                            "with "<Code inline=true>".into_attrs()"</Code>"."
                        </ApiRow>
                        <ApiRow name="field_props" ty="UseDatePickerFieldProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"role=\"presentation\""</Code>", "
                            <Code inline=true>"aria-haspopup=\"dialog\""</Code>" and "<Code inline=true>"aria-expanded"</Code>". "
                            "The date field\u{2019}s "<Code inline=true>"field_props"</Code>" also set "<Code inline=true>"id"</Code>
                            " and "<Code inline=true>"role"</Code>": spread the two on different elements, or the duplicate "
                            "attributes end up in the server-rendered HTML."
                        </ApiRow>
                        <ApiRow name="button_props" ty="UseDatePickerButtonProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"aria-label=\"Calendar\""</Code>", "
                            <Code inline=true>"aria-labelledby"</Code>" (button and label), "<Code inline=true>"aria-haspopup"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-disabled"</Code>" (also while "
                            "read-only), "<Code inline=true>"aria-describedby"</Code>", "<Code inline=true>"tabindex=\"0\""</Code>
                            " and click and Enter/Space handlers that open the popover."
                        </ApiRow>
                        <ApiRow name="dialog_props" ty="UseDatePickerDialogProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"role"</Code>" ("<Code inline=true>"dialog"</Code>"), "
                            <Code inline=true>"aria_modal"</Code>" and "<Code inline=true>"aria_labelledby"</Code>" for the "
                            "element inside the popover. Set them one by one; there is no spread."
                        </ApiRow>
                        <ApiRow name="calendar_props" ty="UseDatePickerCalendarProps">
                            "Data for the calendar: "<Code inline=true>"value"</Code>" (the picked date at midnight UTC), "
                            <Code inline=true>"on_change"</Code>" (commits through "<Code inline=true>"set_date_value"</Code>"), "
                            <Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>", "<Code inline=true>"is_disabled"</Code>", "
                            <Code inline=true>"is_read_only"</Code>", "<Code inline=true>"is_date_unavailable"</Code>", "
                            <Code inline=true>"is_invalid"</Code>", "<Code inline=true>"error_message"</Code>", "
                            <Code inline=true>"default_focused_value"</Code>" (today without a value), "
                            <Code inline=true>"auto_focus"</Code>" ("<Code inline=true>"true"</Code>") and an "<Code inline=true>"id"</Code>"."
                        </ApiRow>
                        <ApiRow name="description_props, error_props" ty="UseDatePickerDescriptionProps, UseDatePickerErrorProps">
                            "IDs (and "<Code inline=true>"role"</Code>"/"<Code inline=true>"aria_live"</Code>" for the error) "
                            "for your description and error elements."
                        </ApiRow>
                        <ApiRow name="state" ty="UseDatePickerStateReturn">"The state; configure the date field from it."</ApiRow>
                        <ApiRow name="is_open, value, open, close, set_open, is_invalid, validation_errors">
                            "Shorthands for the same fields of "<Code inline=true>"state"</Code>"."
                        </ApiRow>
                        <ApiRow name="value_description" ty="Signal<String>">
                            "\u{201C}Selected date: October 13, 2026\u{201D}, or empty without a value."
                        </ApiRow>
                        <ApiRow name="value_description_id, picker_id" ty="String">
                            "IDs of the hidden description element and of the group."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Committing Values">
                <p>
                    "The picker separates the committed "<Code inline=true>"value"</Code>" from dates and times picked in "
                    "the popover:"
                </p>

                <ul>
                    <li>
                        <b>"Date only"</b>" ("<Code inline=true>"show_time: false"</Code>"): picking a date in the calendar "
                        "commits it right away, keeping the time of the current value (or of "
                        <Code inline=true>"default_value"</Code>", or midnight)."
                    </li>
                    <li>
                        <b>"Date and time"</b>": a picked date is committed when a time has been picked too, or right away "
                        "when "<Code inline=true>"should_close_on_select"</Code>" is set. Otherwise it waits: picking a time "
                        "with "<Code inline=true>"set_time_value"</Code>" commits both, and closing the popover commits the "
                        "date with the placeholder time."
                    </li>
                    <li>
                        "Typing into the date field commits through "<Code inline=true>"set_value"</Code>" once the field is "
                        "complete."
                    </li>
                </ul>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Alt + ArrowDown / Alt + ArrowUp">"In the field: open the calendar."</KeyRow>
                    <KeyRow keys="Enter / Space">"On the button: open the calendar. In the calendar: pick the focused date and close."</KeyRow>
                    <KeyRow keys="Arrow keys">"In the calendar: move the focused date by a day or a week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"In the calendar: move to the previous or next month (with Shift, year)."</KeyRow>
                    <KeyRow keys="Home / End">"In the calendar: move to the first or last day of the month."</KeyRow>
                    <KeyRow keys="Escape">"Close the calendar without picking (handled by the popover)."</KeyRow>
                </KeyboardTable>

                <p>
                    "The segments of the field use the "
                    <Link href=routes::doc::date_time::DateFieldHooks.materialize()>"date field keys"</Link>". The button "
                    "only opens the popover; it doesn\u{2019}t toggle it."
                </p>
            </Section>

            <Section title="Internationalization">
                <p>
                    "Like the date field, the picker isn\u{2019}t localized: the button\u{2019}s label (\u{201C}Calendar\u{201D}), "
                    "the \u{201C}Selected date\u{201D} description, the formatted value (English month names) and the "
                    "validation messages are English, and right-to-left layouts are not handled. See "
                    <Link href=routes::doc::date_time::DateFieldHooks.materialize()>"the date field hooks"</Link>
                    " for the field itself."
                </p>
            </Section>

            <Section title="Forms and Validation">
                <p>
                    "The picker validates its value with "<Code inline=true>"use_form_validation_state"</Code>" ("
                    <Code inline=true>"validate"</Code>", "<Code inline=true>"is_invalid"</Code>", "<Code inline=true>"name"</Code>
                    ") and adds built-in errors for values outside "<Code inline=true>"min"</Code>"/"<Code inline=true>"max"</Code>
                    " and for unavailable dates. "<Code inline=true>"is_invalid"</Code>" and "<Code inline=true>"validation_errors"</Code>
                    " combine both; render the messages in the element with "<Code inline=true>"error_props.id"</Code>". "
                    "Give the inner date field "<Code inline=true>"is_invalid: picker.is_invalid"</Code>" if its "
                    "segments should show the state. As with the field, there is no hidden input for native forms."
                </p>
            </Section>

            <Section title="Limitations">
                <ul>
                    <li>
                        <b>"Picked parts and an existing value."</b>" The staged date and time don\u{2019}t start from the "
                        "committed value: with "<Code inline=true>"show_time"</Code>" set and a value present, "
                        <Code inline=true>"set_time_value"</Code>" only stages the time without changing the value, and "
                        <Code inline=true>"set_date_value"</Code>" combines the date with the placeholder time instead of the "
                        "value\u{2019}s time (or, with "<Code inline=true>"should_close_on_select: false"</Code>", only stages it)."
                    </li>
                    <li>
                        <b>"Focused date when reopening."</b>" "<Code inline=true>"calendar_props.default_focused_value"</Code>" is "
                        "computed once, when the picker is created. Without an initial value it stays today, so a calendar "
                        "created from it focuses today even after a date was picked."
                    </li>
                    <li>
                        <b>"Alt + Arrow Down."</b>" The focused segment also handles the arrow key, so opening the calendar "
                        "from the field also steps that segment."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link></li>
                <li><Link href=routes::doc::date_time::DateFieldHooks.materialize()>"Date field hooks"</Link></li>
                <li><Link href=routes::doc::date_time::CalendarHooks.materialize()>"Calendar hooks"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
