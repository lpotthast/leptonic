use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::radio::RadioDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseRadioHook() -> impl IntoView {
    view! {
        <DocPage title="Radio Hooks">
            <p>
                "The radio hooks build a radio group from native "<Code inline=true>"<input type=\"radio\">"</Code>
                " elements: "<Code inline=true>"use_radio_group_state"</Code>" holds the selected value, "
                <Code inline=true>"use_radio_group"</Code>" renders the group with its label and arrow-key navigation, and "
                <Code inline=true>"use_radio"</Code>" wires up each radio. See the "
                <Link href=routes::doc::Radio.materialize()>"Radio overview"</Link>" for concept guidance."
            </p>
            <ReactAria hook="useRadioGroup"/>

            <Section title="Values">
                <p>
                    "Radios select "<Code inline=true>"Key"</Code>"s, the identifiers leptonic\u{2019}s collections use: "
                    "strings or integers ("<Code inline=true>"Key::from(\"m\")"</Code>", "<Code inline=true>"Key::from(3)"</Code>
                    "). A radio submits its key, formatted as text, as its form value."
                </p>
            </Section>

            <Section title="use_radio_group_state">
                <Section title="Input" id="use-radio-group-state-input">
                    <ApiTable kind=ApiKind::Input of="UseRadioGroupStateInput">
                        <ApiRow name="default_value" ty="Option<Key>" default="None">
                            "The initially selected value, restored on form reset."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Key>>>" default="None">
                            "Called with the selected value when it changes."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The radios\u{2019} "<Code inline=true>"name"</Code>", which makes them one group in the browser. "
                            "Generated when "<Code inline=true>"None"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all radios."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">
                            "The radios can be focused, but the selection can\u{2019}t change."
                        </ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Sets "<Code inline=true>"aria-required"</Code>" on the group (and "<Code inline=true>"required"</Code>
                            " on the radios with "<Code inline=true>"ValidationBehavior::Native"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid while "<Code inline=true>"true"</Code>"."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<Key>>>" default="None">"Validates the selected value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            "Shows errors as the user edits, or on form submission."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="RadioGroupState">
                    <ApiTable kind=ApiKind::Fields of="RadioGroupState">
                        <ApiRow name="selected_value" ty="Signal<Option<Key>>">"The selected value."</ApiRow>
                        <ApiRow name="last_focused_value" ty="Signal<Option<Key>>">
                            "The radio focused last. While nothing is selected, it is the group\u{2019}s tab stop."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_required" ty="Signal<bool>">"The group\u{2019}s settings."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation is invalid."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The group\u{2019}s validation state."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"The validation behavior of the group."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_selected_value(Option<Key>)"</Code></TableCell>
                            <TableCell>"Selects a value. Ignored while the group is disabled or read-only."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"name()"</Code>", "<Code inline=true>"default_selected_value()"</Code></TableCell>
                            <TableCell>"The radios\u{2019} name and the initial value."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_radio_group">
                <p>
                    "Gives the group element "<Code inline=true>"role=\"radiogroup\""</Code>", its label, description and "
                    "state attributes, and handles the arrow keys: they move focus to the next or previous radio and select it."
                </p>

                <Section title="Input" id="use-radio-group-input">
                    <p>"Create it with "<Code inline=true>"UseRadioGroupInput::new(state)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseRadioGroupInput">
                        <ApiRow name="state" ty="RadioGroupState">"From "<Code inline=true>"use_radio_group_state"</Code>"."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">"Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Labels the group without a visible label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_errormessage" ty="Option<String>" default="None">
                            "Further labelling, describing or error elements."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Sets "<Code inline=true>"aria-orientation"</Code>". In right-to-left locales, "<Keys keys="ArrowLeft"/>
                            " and "<Keys keys="ArrowRight"/>" swap their direction in horizontal groups."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the radios belong to, when they aren\u{2019}t inside it."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when focus enters or leaves the group."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the group gains or loses focus within."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-radio-group-return">
                    <ApiTable kind=ApiKind::Return of="UseRadioGroupReturn">
                        <ApiRow name="props" ty="UseRadioGroupProps">
                            <Code inline=true>"role=\"radiogroup\""</Code>", labelling, "<Code inline=true>"aria-orientation"</Code>", "
                            <Code inline=true>"aria-required"</Code>", "<Code inline=true>"aria-readonly"</Code>", "
                            <Code inline=true>"aria-invalid"</Code>", "<Code inline=true>"aria-disabled"</Code>" and the arrow-key handler. "
                            "Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"The id of the visible label, a "<Code inline=true>"<span>"</Code>"."</ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For the group\u{2019}s description and error message. They also describe each radio; render the error "
                            "message only while invalid."
                        </ApiRow>
                        <ApiRow name="data" ty="RadioGroupData">
                            "What the radios need from the group: pass it to "<Code inline=true>"use_radio"</Code>". It is "
                            <Code inline=true>"Copy"</Code>"; its "<Code inline=true>"state"</Code>" field is the group state."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the group is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_radio">
                <p>
                    "Makes an "<Code inline=true>"<input type=\"radio\">"</Code>" inside a "<Code inline=true>"<label>"</Code>
                    " a radio of the group. Only one radio of the group is a tab stop: the selected one, or while none is "
                    "selected, the one focused last (or every radio, before any was focused)."
                </p>

                <Section title="Input" id="use-radio-input">
                    <p>"Create it with "<Code inline=true>"UseRadioInput::new(group.data, value)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseRadioInput">
                        <ApiRow name="group" ty="RadioGroupData">"The "<Code inline=true>"data"</Code>" of "<Code inline=true>"use_radio_group"</Code>"."</ApiRow>
                        <ApiRow name="value" ty="Key">"The value the radio selects."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this radio. It is also disabled while the group is."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a radio without visible label text."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling or describing elements."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the radio when it mounts."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Focus callbacks."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the radio gains or loses focus."</ApiRow>
                        <ApiRow name="on_press_start, on_press_end, on_press_up, on_press" ty="Option<Callback<PressEvent>>" default="None">
                            "Press callbacks for the input and its label."
                        </ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">"Called when the pressed state changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-radio-return">
                    <ApiTable kind=ApiKind::Return of="UseRadioReturn">
                        <ApiRow name="label_props" ty="PropsWithStyles<UseToggleLabelProps>">
                            "Press handlers for the "<Code inline=true>"<label>"</Code>": pressing it selects the radio and focuses it."
                        </ApiRow>
                        <ApiRow name="input_props" ty="PropsWithStyles<UseRadioInputProps>">
                            "Attributes and handlers for the "<Code inline=true>"<input>"</Code>": "<Code inline=true>"type=\"radio\""</Code>
                            ", the group\u{2019}s "<Code inline=true>"name"</Code>", the value, "<Code inline=true>"checked"</Code>
                            " and the roving "<Code inline=true>"tabindex"</Code>"."
                        </ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">"For a description of this radio."</ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether this radio is selected."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether this radio or its group is disabled."</ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the input or its label is pressed."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the radio has focus."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether to show a focus ring (keyboard focus)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let state = use_radio_group_state(UseRadioGroupStateInput {
                            default_value: Some(Key::from("m")),
                            ..UseRadioGroupStateInput::default()
                        });
                        let group = use_radio_group(UseRadioGroupInput {
                            has_label: true,
                            ..UseRadioGroupInput::new(state)
                        });
                        let small = use_radio(UseRadioInput::new(group.data, "s"));
                        let (label_attrs, label_styles) = small.label_props.into_parts();
                        let (input_attrs, input_styles) = small.input_props.into_parts();

                        view! {
                            <div {..group.props.into_attrs()}>
                                <span {..group.label_props.into_attrs()}>"Size"</span>
                                <label {..label_attrs} style=label_styles>
                                    <input {..input_attrs} style=input_styles/>
                                    "Small"
                                </label>
                                // ... one `use_radio` per option
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Each option is a small component calling "<Code inline=true>"use_radio"</Code>
                    ". Tab into the group and use the arrow keys."
                </p>
                <Demo
                    description="Radio group of sizes with the selected value, a disabled toggle and a read-only toggle"
                    source=include_str!("demos/radio.rs")
                >
                    <RadioDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into the group (to the selected radio) and out of it."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowRight">
                        "Focuses and selects the next radio, wrapping around at the end. Disabled radios are skipped."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowLeft">"Focuses and selects the previous radio, wrapping around at the start."</KeyRow>
                    <KeyRow keys="Space">"Selects the focused radio."</KeyRow>
                </KeyboardTable>
                <p>
                    "Both arrow key pairs work in either orientation. In right-to-left locales, "<Keys keys="ArrowRight"/>
                    " moves backwards in horizontal groups."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Radio.materialize()>"Radio overview"</Link></li>
                <li><Link href=routes::doc::radio::Atom.materialize()>"Radio atoms"</Link></li>
                <li><Link href=routes::doc::radio::Component.materialize()>"Radio component"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
