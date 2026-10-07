use indoc::indoc;
use leptos::prelude::*;

use super::demos::{checkbox_basic::CheckboxBasicDemo, checkbox_group::CheckboxGroupDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseCheckboxHook() -> impl IntoView {
    view! {
        <DocPage title="Checkbox Hooks">
            <p>
                "The checkbox hooks give a native "<Code inline=true>"<input type=\"checkbox\">"</Code>" inside a "
                <Code inline=true>"<label>"</Code>" its behavior, validation and form integration: "
                <AnchorLink href="#use-toggle-state">"use_toggle_state"</AnchorLink>" holds whether it is checked, "
                <AnchorLink href="#use-checkbox">"use_checkbox"</AnchorLink>" wires up one checkbox, and "
                <AnchorLink href="#use-checkbox-group-state">"use_checkbox_group_state"</AnchorLink>", "
                <AnchorLink href="#use-checkbox-group">"use_checkbox_group"</AnchorLink>" and "
                <AnchorLink href="#use-checkbox-group-item">"use_checkbox_group_item"</AnchorLink>
                " manage a group selecting a set of values. See the "
                <Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link>" for concept guidance."
            </p>
            <ReactAria hook="useCheckbox"/>

            <Section title="use_toggle_state">
                <p>
                    "Creates the state of a toggle \u{2014} a checkbox, "<Link href=routes::doc::switch::Hook.materialize()>"switch"</Link>
                    " or "<Link href=routes::doc::toggle_button::Hook.materialize()>"toggle button"</Link>
                    ": whether it is selected. The hook owns the selection; you read it from "
                    <Code inline=true>"is_selected"</Code>" and hear about changes through "<Code inline=true>"on_change"</Code>
                    ", or keep it in your app\u{2019}s state with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-toggle-state-input">
                    <ApiTable kind=ApiKind::Input of="UseToggleStateInput">
                        <ApiRow name="default_selected" ty="bool" default="false">
                            "Whether the toggle starts selected. Also the value a form reset restores."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The selection as app state, replacing "<Code inline=true>"default_selected"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called with the new selection when it changes."
                        </ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">
                            "While "<Code inline=true>"true"</Code>", changes are ignored."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toggle-state-return">
                    <p>
                        <Code inline=true>"use_toggle_state"</Code>" returns a "<Code inline=true>"ToggleState"</Code>
                        ". It is "<Code inline=true>"Copy"</Code>", so you can hand it to the hook and keep reading it."
                    </p>
                    <ApiTable kind=ApiKind::Return of="ToggleState">
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the toggle is selected."</ApiRow>
                        <ApiRow name="default_selected" ty="bool">"The initial selection, restored on form reset."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_selected(bool)"</Code></TableCell>
                            <TableCell>"Selects or deselects the toggle."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"toggle()"</Code></TableCell>
                            <TableCell>"Flips the selection."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"with_on_change(Callback<bool>)"</Code></TableCell>
                            <TableCell>"The same state, also calling the callback with each changed selection."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"new(is_selected, default_selected, set_selected)"</Code></TableCell>
                            <TableCell>
                                "A state whose selection lives elsewhere: it reads "<Code inline=true>"is_selected"</Code>
                                " and hands changes to the "<Code inline=true>"set_selected"</Code>" callback."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                    <p>
                        "To bind a toggle to a signal of your app without "<Code inline=true>"use_toggle_state"</Code>
                        ", convert the signal: "<Code inline=true>"ToggleState::from(rw_signal)"</Code>" or "
                        <Code inline=true>"ToggleState::from((read, write))"</Code>
                        ". The toggle then reads and writes the signal, like Leptos\u{2019} "
                        <Code inline=true>"bind:checked"</Code>". The Checkbox atom and component take your state as their "
                        <Code inline=true>"is_selected"</Code>" and "<Code inline=true>"set_selected"</Code>" props instead."
                    </p>
                </Section>
            </Section>

            <Section title="use_checkbox">
                <p>
                    "Makes an "<Code inline=true>"<input type=\"checkbox\">"</Code>" inside a "<Code inline=true>"<label>"</Code>
                    " a checkbox: the input toggles natively, the label toggles it on mouse and touch presses and moves focus to it, "
                    "and the hook adds validation, form reset and focus-ring tracking."
                </p>

                <Section title="Input" id="use-checkbox-input">
                    <p>
                        "Pass a "<Code inline=true>"UseCheckboxInput"</Code>" with every field named; the Default column gives "
                        "the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseCheckboxInput">
                        <ApiRow name="state" ty="ToggleState">"The checkbox\u{2019}s state, from "<Code inline=true>"use_toggle_state"</Code>". Required."</ApiRow>
                        <ApiRow name="is_indeterminate" ty="Signal<bool>" default="false">
                            "Shows the checkbox as neither checked nor unchecked. Purely visual: the state stays as it is."
                        </ApiRow>
                        <ApiRow name="options" ty="ToggleOptions" default="ToggleOptions::default()">
                            "Everything else, shared with the other toggles: see "<AnchorLink href="#toggleoptions">"ToggleOptions"</AnchorLink>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="ToggleOptions">
                    <p>
                        "The settings of a checkbox or "<Link href=routes::doc::switch::Hook.materialize()>"switch"</Link>
                        " besides its state. "<Code inline=true>"ToggleOptions"</Code>" implements "<Code inline=true>"Default"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Input of="ToggleOptions">
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the input and its label."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">
                            "The toggle can be focused but not changed. Sets "<Code inline=true>"aria-readonly"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Sets "<Code inline=true>"aria-required"</Code>", or the native "<Code inline=true>"required"</Code>
                            " attribute with "<Code inline=true>"ValidationBehavior::Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the toggle invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<bool>>" default="None">
                            "Validates the selection: "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as the user edits, "<Code inline=true>"Native"</Code>
                            " defers them to form submission using native constraint validation."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">
                            "The id of the form the input belongs to, when it isn\u{2019}t inside it."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<String>" default="None">
                            "The value submitted while selected. Without it, the browser submits "<Code inline=true>"on"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "The accessible name, for a toggle without visible label text."
                        </ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_errormessage" ty="Option<String>" default="None">
                            "Ids of further elements labelling or describing the input, or showing its error."
                        </ApiRow>
                        <ApiRow name="aria_controls" ty="Option<String>" default="None">"The id of the element the toggle controls."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">"Removes the input from the tab order."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Focus callbacks of the input."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="on_press_start, on_press_end, on_press_up, on_press" ty="Option<Callback<PressEvent>>" default="None">
                            "Press callbacks, as in "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                            ", for presses on the input and its label."
                        </ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">"Called when the pressed state changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-checkbox-return">
                    <p>
                        <Code inline=true>"use_checkbox"</Code>" returns a "<Code inline=true>"UseCheckboxReturn"</Code>
                        ", an alias of "<Code inline=true>"UseToggleReturn"</Code>" (the return of all toggle hooks)."
                    </p>
                    <ApiTable kind=ApiKind::Return of="UseToggleReturn">
                        <ApiRow name="label_props" ty="PropsWithStyles<UseToggleLabelProps>">
                            "Press handlers for the "<Code inline=true>"<label>"</Code>" wrapping the input. Call "
                            <Code inline=true>"into_parts()"</Code>" and spread "<Code inline=true>"{..attrs}"</Code>" with "
                            <Code inline=true>"style=styles"</Code>"."
                        </ApiRow>
                        <ApiRow name="input_props" ty="PropsWithStyles<UseToggleInputProps>">
                            "Attributes and handlers for the "<Code inline=true>"<input>"</Code>", including "
                            <Code inline=true>"type=\"checkbox\""</Code>" and the "<Code inline=true>"checked"</Code>" and "
                            <Code inline=true>"indeterminate"</Code>" DOM properties."
                        </ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For a description and an error message of this toggle. The input references them while they are rendered; "
                            "render the error message only while invalid."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the toggle is selected."</ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the input or its label is pressed."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>">"The disabled and read-only settings."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the input has focus."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether to show a focus ring (keyboard focus)."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">
                            "Detailed validity, mirroring the native "<Code inline=true>"ValidityState"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-checkbox-example">
                    <p>
                        "The visible native input below is the simplest setup. To draw your own box, hide the input with "
                        <Code inline=true>"visually_hidden_styles()"</Code>" and style the label, as the "
                        <Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox atom"</Link>" does."
                    </p>
                    <Demo
                        description="Checkbox with an indeterminate toggle and a disabled toggle"
                        source=include_str!("demos/checkbox_basic.rs")
                        source_open=true
                    >
                        <CheckboxBasicDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_checkbox_group_state">
                <p>
                    "Creates the state of a checkbox group: the checked values, as "<Code inline=true>"Key"</Code>
                    "s (strings or integers). Each checkbox submits its key as its form value."
                </p>

                <Section title="Input" id="use-checkbox-group-state-input">
                    <ApiTable kind=ApiKind::Input of="UseCheckboxGroupStateInput">
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">"The initially checked values, restored on form reset."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Vec<Key>>>" default="None">
                            "The checked values as app state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the checked values when they change."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the group, or prevents changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "At least one checkbox must be checked: the checkboxes are required while none is."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid while "<Code inline=true>"true"</Code>"."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Vec<Key>>>" default="None">"Validates the checked values."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            "Shows errors as the user edits, or on form submission. Applies to all checkboxes of the group."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The checkboxes\u{2019} "<Code inline=true>"name"</Code>", for form submission and server errors."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-checkbox-group-state-return">
                    <p>
                        <Code inline=true>"use_checkbox_group_state"</Code>" returns a "<Code inline=true>"CheckboxGroupState"</Code>
                        ", which is "<Code inline=true>"Copy"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Return of="CheckboxGroupState">
                        <ApiRow name="value" ty="Signal<Vec<Key>>">"The checked values, in the order they were checked."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>">"The group\u{2019}s settings."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>">"Whether a value is still required: the group is required and nothing is checked."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation is invalid."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The group\u{2019}s validation state."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"The validation behavior of the group."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow><TableCell><Code inline=true>"is_selected(&Key)"</Code></TableCell><TableCell>"Whether the value is checked (tracked)."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"set_value(Vec<Key>)"</Code></TableCell><TableCell>"Replaces the checked values."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"add_value(Key)"</Code>", "<Code inline=true>"remove_value(&Key)"</Code>", "<Code inline=true>"toggle_value(Key)"</Code></TableCell><TableCell>"Checks or unchecks one value."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"default_value()"</Code>", "<Code inline=true>"name()"</Code></TableCell><TableCell>"The initial values and the checkboxes\u{2019} name."</TableCell></TableRow>
                    </DocTable>
                    <p>"The changing methods do nothing while the group is disabled or read-only."</p>
                </Section>
            </Section>

            <Section title="use_checkbox_group">
                <p>
                    "Gives the group element "<Code inline=true>"role=\"group\""</Code>", its label and description. "
                    "The group\u{2019}s description and error message also describe each checkbox."
                </p>
                <ReactAria hook="useCheckboxGroup"/>

                <Section title="Input" id="use-checkbox-group-input">
                    <ApiTable kind=ApiKind::Input of="UseCheckboxGroupInput">
                        <ApiRow name="state" ty="CheckboxGroupState">"From "<Code inline=true>"use_checkbox_group_state"</Code>". Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Labels the group without a visible label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">
                            "Further labelling and describing elements."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the checkboxes belong to, when they aren\u{2019}t inside it."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when focus enters or leaves the group."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the group gains or loses focus within."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-checkbox-group-return">
                    <ApiTable kind=ApiKind::Return of="UseCheckboxGroupReturn">
                        <ApiRow name="props" ty="UseCheckboxGroupProps">
                            <Code inline=true>"role=\"group\""</Code>", id, labelling and "<Code inline=true>"aria-disabled"</Code>
                            ". Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"The id of the visible label, a "<Code inline=true>"<span>"</Code>"."</ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For the group\u{2019}s description and error message. Render the error message only while invalid."
                        </ApiRow>
                        <ApiRow name="data" ty="CheckboxGroupData">
                            "What the checkboxes need from the group: pass it to "<Code inline=true>"use_checkbox_group_item"</Code>
                            ". Its "<Code inline=true>"state"</Code>" field is the group state."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the group is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_checkbox_group_item">
                <p>
                    "Makes a checkbox part of the group: it is checked while the group contains its value, and checking it adds the "
                    "value. It returns the same "<AnchorLink href="#use-checkbox-return">"UseToggleReturn"</AnchorLink>" as "
                    <Code inline=true>"use_checkbox"</Code>"."
                </p>

                <Section title="Input" id="use-checkbox-group-item-input">
                    <p>"Pass a "<Code inline=true>"UseCheckboxGroupItemInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseCheckboxGroupItemInput">
                        <ApiRow name="group" ty="CheckboxGroupData">"The "<Code inline=true>"data"</Code>" of "<Code inline=true>"use_checkbox_group"</Code>". Required."</ApiRow>
                        <ApiRow name="value" ty="Key">"The checkbox\u{2019}s value in the group, also its form value. Required."</ApiRow>
                        <ApiRow name="is_indeterminate" ty="Signal<bool>" default="false">"Shows the checkbox as partially checked."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">"Called when this checkbox is checked or unchecked."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<bool>>" default="None">"Validates this checkbox; its errors join the group\u{2019}s."</ApiRow>
                        <ApiRow name="options" ty="ToggleOptions" default="ToggleOptions::default()">
                            "Further settings. "<Code inline=true>"name"</Code>" and "<Code inline=true>"form"</Code>
                            " default to the group\u{2019}s, the form value is "<Code inline=true>"value"</Code>
                            ", and the group\u{2019}s validation behavior applies. The checkbox is disabled, read-only or required when "
                            "it or its group is."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-checkbox-group-example">
                    <p>
                        "Each checkbox is a small Leptos component calling "<Code inline=true>"use_checkbox_group_item"</Code>
                        ". Uncheck all toppings to see the error message."
                    </p>
                    <Demo
                        description="Required group of topping checkboxes with a description, an error message and a disabled toggle"
                        source=include_str!("demos/checkbox_group.rs")
                        source_open=true
                    >
                        <CheckboxGroupDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Indeterminate State">
                <p>
                    "An indeterminate checkbox is neither checked nor unchecked. Use it for a \u{201c}select all\u{201d} "
                    "checkbox summarizing child checkboxes: unchecked when none is checked, indeterminate when some are, checked "
                    "when all are. "<Code inline=true>"is_indeterminate"</Code>" only changes what the checkbox shows; set it "
                    "from your own state, and clear it when the user toggles the checkbox."
                </p>
            </Section>

            <Section title="Validation">
                <p>
                    "Validate the selection with "<Code inline=true>"validate"</Code>", or set the result with "
                    <Code inline=true>"is_invalid"</Code>". While it is invalid, the input has "<Code inline=true>"aria-invalid"</Code>
                    ", "<Code inline=true>"validation_errors"</Code>" holds the messages, and an error message rendered with "
                    <Code inline=true>"error_message_props"</Code>" describes the input."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::sync::Arc;

                        use leptonic::hooks::*;
                        use leptos::prelude::*;

                        let state = use_toggle_state(UseToggleStateInput::default());
                        let checkbox = use_checkbox(UseCheckboxInput {
                            options: ToggleOptions {
                                is_required: Signal::stored(true),
                                validate: Some(Arc::new(|accepted: &bool| {
                                    if *accepted { Ok(()) } else { Err(vec!["Please accept the terms.".to_owned()]) }
                                })),
                                ..ToggleOptions::default()
                            },
                            state,
                            is_indeterminate: false.into(),
                        });

                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox Atoms"</Link></li>
                <li><Link href=routes::doc::switch::Hook.materialize()>"Switch Hooks"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle Button Hooks"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
