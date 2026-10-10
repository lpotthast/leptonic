use indoc::indoc;
use leptos::prelude::*;

use super::demos::combobox::ComboboxDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseCombobox() -> impl IntoView {
    view! {
        <DocPage title="Combobox Hooks">
            <p>
                "Hooks for a combobox: a text input with a popover of suggestions that narrow down as you type. "
                <Code inline=true>"use_combobox_state"</Code>" holds the options, the selection and the input text; "
                <Code inline=true>"use_combobox"</Code>" configures the input, the button and the listbox you render. "
                "See the "<Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useComboBox"/>

            <Section title="Demo">
                <p>
                    "Type to filter the fruits, or open the popover with the button or "<Keys keys="ArrowDown"/>
                    ". The arrow keys move through the options while the text cursor stays in the input; "<Keys keys="Enter"/>
                    " selects, "<Keys keys="Escape"/>" restores the input. Durian is disabled. Below the combobox you see "
                    "its state: the selected key, the input text and whether the popover is open."
                </p>

                <Demo
                    description="Fruit combobox built from the hooks: filtering, a disabled option and the combobox state"
                    source=include_str!("demos/combobox.rs")
                >
                    <ComboboxDemo/>
                </Demo>
            </Section>

            <Section title="use_combobox_state">
                <p>
                    "Creates the state of a combobox from a collection of options. It filters the options by the input text, "
                    "tracks the selection, the input text and whether the popover is open, and keeps them consistent: "
                    "selecting an option puts its text into the input, and leaving the input restores the selected "
                    "option\u{2019}s text. Build the collection with "<Code inline=true>"use_collection"</Code>" or "
                    <Code inline=true>"use_list_collection"</Code>", as for a "
                    <Link href=routes::doc::listbox::Hook.materialize()>"listbox"</Link>"."
                </p>

                <Section title="Example" id="use-combobox-state-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::collections::HashSet;

                        use leptonic::{
                            hooks::{
                                collections::{Key, use_collection},
                                combobox::{
                                    ComboBoxMenuTrigger,
                                    UseComboBoxStateInput,
                                    use_combobox_state,
                                    use_contains_filter,
                                },
                                form::ValidationBehavior,
                                select::SelectMode,
                            },
                        };
                        use leptos::{logging::log, prelude::*};

                        let collection = use_collection(|b| {
                            b.item("apple", "Apple");
                            b.item("banana", "Banana");
                            b.item("durian", "Durian").disabled(true);
                        });
                        let state = use_combobox_state(UseComboBoxStateInput {
                            collection,
                            filter: Some(use_contains_filter()),
                            selection_mode: SelectMode::Single,
                            default_value: vec![Key::from("banana")],
                            value: None,
                            on_change: Some(Callback::new(|keys: Vec<Key>| log!("{keys:?}"))),
                            default_input_value: None,
                            input_value: None,
                            on_input_change: None,
                            disabled_keys: Signal::stored(HashSet::new()),
                            menu_trigger: ComboBoxMenuTrigger::Input,
                            allows_empty_collection: false,
                            allows_custom_value: false,
                            should_close_on_blur: true,
                            is_read_only: Signal::stored(false),
                            on_open_change: None,
                            is_invalid: Signal::stored(false),
                            validate: None,
                            validation_behavior: ValidationBehavior::Aria,
                            name: None,
                        });
                    "#)}
                </Code>
                </Section>

                <Section title="Input" id="use-combobox-state-input">
                    <p>
                        "Pass a "<Code inline=true>"UseComboBoxStateInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseComboBoxStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">
                            "All options. Items marked "<Code inline=true>".disabled(true)"</Code>" can\u{2019}t be focused or selected."
                        </ApiRow>
                        <ApiRow name="filter" ty="Option<ComboBoxFilter>" default="None">
                            "Prepares a matcher for the current input text, then applies it to each option text. "
                            <Code inline=true>"use_contains_filter()"</Code>" matches options containing the input, ignoring "
                            "case and accents in the current locale. "<Code inline=true>"None"</Code>" shows the collection as "
                            "is, e.g. when you filter on the server."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode" default="Single">
                            "Whether one option or several can be selected."
                        </ApiRow>
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">
                            "The initially selected keys (at most one in "<Code inline=true>"Single"</Code>" mode)."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Vec<Key>>>" default="None">
                            "The selected keys as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the selected keys, in collection order, when they change."
                        </ApiRow>
                        <ApiRow name="default_input_value" ty="Option<String>" default="None">
                            "The initial input text. "<Code inline=true>"None"</Code>" starts with the selected option\u{2019}s text."
                        </ApiRow>
                        <ApiRow name="input_value" ty="Option<ValueBinding<String>>" default="None">
                            "The input text as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_input_value"</Code>
                            ". Bound text doesn\u{2019}t follow a change of the selected option\u{2019}s own text (e.g. reloaded "
                            "options): update it yourself."
                        </ApiRow>
                        <ApiRow name="on_input_change" ty="Option<Callback<String>>" default="None">
                            "Called when the input text changes."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                            "Further options that can\u{2019}t be focused or selected."
                        </ApiRow>
                        <ApiRow name="menu_trigger" ty="Signal<ComboBoxMenuTrigger>" default="Input">
                            "When the popover opens, see "<AnchorLink href="#opening-the-popover">"Opening the Popover"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="allows_empty_collection" ty="Signal<bool>" default="false">
                            "Keep the popover open when no option matches, e.g. to show an empty state."
                        </ApiRow>
                        <ApiRow name="allows_custom_value" ty="Signal<bool>" default="false">
                            "Keep text that matches no option, see "<AnchorLink href="#custom-values">"Custom Values"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="should_close_on_blur" ty="bool" default="true">
                            "Commit the input and close the popover when focus leaves the combobox."
                        </ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">
                            "Makes the input read-only: no editing, no popover, and the button disabled ("
                            <Code inline=true>"use_combobox"</Code>" reads it from the state)."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<ComboBoxOpenChange>>" default="None">
                            "Called when the popover opens or closes, with "<Code inline=true>"is_open"</Code>" and the "
                            <Code inline=true>"trigger"</Code>" that opened it ("<Code inline=true>"None"</Code>" when it closed), a "
                            <Code inline=true>"MenuTriggerAction"</Code>": "<Code inline=true>"Input"</Code>" (typing), "<Code inline=true>"Focus"</Code>
                            " or "<Code inline=true>"Manual"</Code>" (button or arrow keys)."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the combobox invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<ComboBoxValue>>" default="None">
                            "Validates the input text and the selected keys together ("<Code inline=true>"ComboBoxValue"</Code>
                            " has the fields "<Code inline=true>"input_value"</Code>" and "<Code inline=true>"value"</Code>")."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            "Show errors while editing ("<Code inline=true>"Aria"</Code>") or on form submission ("
                            <Code inline=true>"Native"</Code>")."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The field\u{2019}s form name: what the form submits it under (see "<Code inline=true>"form_value"</Code>
                            " of "<Code inline=true>"use_combobox"</Code>"), and what server validation errors are matched by."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-combobox-state-return">
                    <p>
                        <Code inline=true>"use_combobox_state"</Code>" returns a "<Code inline=true>"ComboBoxState"</Code>
                        ". It is "<Code inline=true>"Copy"</Code>", so you can move it into any number of closures. Its "
                        "reading methods track their signals, so views and effects using them update."
                    </p>
                    <ApiTable kind=ApiKind::Return of="ComboBoxState">
                        <ApiRow name="list" ty="ListState">
                            "The options shown in the popover (the filtered collection) with their selection and focus. "
                            <Code inline=true>"use_combobox"</Code>" hands it to the listbox."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode">"The selection mode."</ApiRow>
                        <ApiRow name="validation" ty="FormValidationState">
                            "The validation state, e.g. "<Code inline=true>"validation.is_invalid"</Code>" and "
                            <Code inline=true>"validation.validation_errors"</Code>"."
                        </ApiRow>
                    </ApiTable>

                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"value() -> Vec<Key>"</Code></TableCell>
                            <TableCell>"The selected keys, in collection order."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"selected_key() -> Option<Key>"</Code></TableCell>
                            <TableCell>
                                "The selected key in "<Code inline=true>"Single"</Code>" mode ("<Code inline=true>"None"</Code>
                                " in "<Code inline=true>"Multiple"</Code>" mode)."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"selected_items() -> Vec<Node>"</Code></TableCell>
                            <TableCell>"The collection nodes of the selected options, e.g. for their text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(Vec<Key>)"</Code></TableCell>
                            <TableCell>
                                "Selects these keys (only the first one in "<Code inline=true>"Single"</Code>" mode). The input "
                                "shows the new option\u{2019}s text."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"input_value() -> String"</Code><br/>
                                <Code inline=true>"input_value_signal() -> Signal<String>"</Code>
                            </TableCell>
                            <TableCell>"The input text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_input_value(String)"</Code></TableCell>
                            <TableCell>"Replaces the input text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"default_value() -> Vec<Key>"</Code><br/>
                                <Code inline=true>"default_input_value() -> String"</Code>
                            </TableCell>
                            <TableCell>"The value and input text the combobox started with. A form reset restores the value."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_open() -> bool"</Code></TableCell>
                            <TableCell>"Whether the popover is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_focused() -> bool"</Code></TableCell>
                            <TableCell>"Whether focus is in the combobox."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_strategy() -> Option<FocusStrategy>"</Code></TableCell>
                            <TableCell>
                                "Which option gets focus when the popover opens: the first, the last, or ("<Code inline=true>"None"</Code>
                                ") the selected one."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"open(Option<FocusStrategy>, MenuTriggerAction)"</Code><br/>
                                <Code inline=true>"toggle(Option<FocusStrategy>, MenuTriggerAction)"</Code>
                            </TableCell>
                            <TableCell>
                                "Opens or toggles the popover. Opening needs options to show (or "
                                <Code inline=true>"allows_empty_collection"</Code>"). Opening with "<Code inline=true>"Manual"</Code>
                                " shows all options, not only those matching the input."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"close()"</Code></TableCell>
                            <TableCell>"Commits the input, as when focus leaves the combobox, and closes the popover."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"commit()"</Code></TableCell>
                            <TableCell>"What "<Keys keys="Enter"/>" does: selects the focused option, or commits the input when no option is focused."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"revert()"</Code></TableCell>
                            <TableCell>
                                "What "<Keys keys="Escape"/>" does: restores the selected option\u{2019}s text (or keeps custom text) "
                                "and closes the popover."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_focused(bool)"</Code></TableCell>
                            <TableCell>
                                "Tells the state that focus entered or left the combobox. "<Code inline=true>"use_combobox"</Code>
                                " calls it for you."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"allows_custom_value() -> bool"</Code></TableCell>
                            <TableCell>"The setting of the same name."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Opening the Popover">
                    <p><Code inline=true>"menu_trigger"</Code>" decides when the popover opens:"</p>
                    <DocTable headers=&["ComboBoxMenuTrigger", "Opens the popover"]>
                        <TableRow>
                            <TableCell><Code inline=true>"Input"</Code>" (default)"</TableCell>
                            <TableCell>"When the user types, showing the matching options."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Focus"</Code></TableCell>
                            <TableCell>"Also when the input gets focus, showing all options."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Manual"</Code></TableCell>
                            <TableCell>"Only with the button or "<Keys keys="ArrowDown"/>" / "<Keys keys="ArrowUp"/>"."</TableCell>
                        </TableRow>
                    </DocTable>
                    <p>
                        "The button and the arrow keys open the popover with all options, so users can browse them even "
                        "when the input holds the selected option\u{2019}s text. Typing filters again. When no option "
                        "matches the text, the popover closes, and it reopens as soon as options match again."
                    </p>
                </Section>

                <Section title="Custom Values">
                    <p>
                        "By default, the input only keeps text that belongs to an option: when focus leaves the combobox "
                        "(or on "<Keys keys="Enter"/>" without a focused option), the input shows the selected option\u{2019}s "
                        "text again. With "<Code inline=true>"allows_custom_value: true"</Code>", typed text that differs "
                        "from the selected option stays, and the selection is cleared. Read the text with "
                        <Code inline=true>"input_value()"</Code>" or "<Code inline=true>"on_input_change"</Code>"."
                    </p>
                    <p>
                        "In "<Code inline=true>"Single"</Code>" mode, clearing the input clears the selection."
                    </p>
                </Section>
            </Section>

            <Section title="use_combobox">
                <p>
                    "Configures the parts of a combobox for a "<Code inline=true>"ComboBoxState"</Code>". It doesn\u{2019}t "
                    "render anything and doesn\u{2019}t return DOM props for the whole combobox. Instead, it returns the inputs "
                    "of the hooks that make up the parts: "
                    <Link href=routes::doc::text_field::Hook.materialize()>"use_text_field"</Link>" for the input, "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" for the button and "
                    <Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link>" for the listbox in the popover."
                </p>

                <Section title="Example" id="use-combobox-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            CapturedElement,
                            hooks::{
                                button::use_button,
                                combobox::{
                                    ComboBoxFormValue,
                                    UseComboBoxInput,
                                    UseComboBoxReturn,
                                    use_combobox,
                                },
                                form::{UseTextFieldReturn, use_text_field},
                                listbox::{use_listbox, use_option},
                            },
                        };
                        use leptos::prelude::*;

                        let popover = CapturedElement::new();
                        let UseComboBoxReturn { input, input_props, button, listbox, form_values } =
                            use_combobox(UseComboBoxInput {
                                state,
                                id: None,
                                is_disabled: false.into(),
                                is_required: false.into(),
                                has_label: true.into(),
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                placeholder: MaybeProp::default(),
                                form_value: ComboBoxFormValue::Key,
                                form: None,
                                should_focus_wrap: false,
                                keyboard_delegate: None,
                                popover,
                                on_focus: None,
                                on_blur: None,
                            });

                        let UseTextFieldReturn { label_props, input_props: field_props, .. } = use_text_field(input);
                        let (button_attrs, button_styles) = use_button(button).props.into_parts();
                        let listbox = StoredValue::new(listbox);

                        view! {
                            <label {..label_props.into_attrs()}>"Fruit"</label>
                            <input {..field_props.into_attrs()} {..input_props.into_attrs()}/>
                            <button {..button_attrs} style=button_styles><span aria-hidden="true">"▼"</span></button>
                            // With a `name` in the state: the selected keys, for form submission.
                            <For each=move || form_values.get() key=Clone::clone let(value)>
                                <input type="hidden" name="fruit" value=value/>
                            </For>
                            <Show when=move || state.is_open()>
                                <div {..popover.attr()}>
                                    // `use_listbox(listbox.get_value())` and one `use_option` per item of its
                                    // collection, see the demo.
                                </div>
                            </Show>
                        }
                    "#)}
                </Code>

                <p>
                    "The listbox shows "<Code inline=true>"state.list"</Code>", the options matching the input. Render one "
                    "option per item of "<Code inline=true>"data.state.collection"</Code>" (from "<Code inline=true>"use_listbox"</Code>
                    "), not of the full collection. Options are focused "
                    <Link href=routes::doc::focus::VirtualFocus.materialize()>"virtually"</Link>
                    ": DOM focus stays in the input, which points "
                    "at the focused option with "<Code inline=true>"aria-activedescendant"</Code>". Style the focused option "
                    "with "<Code inline=true>"is_focused"</Code>" of "<Code inline=true>"use_option"</Code>"."
                </p>
                </Section>

                <Section title="Input" id="use-combobox-input">
                    <p>
                        "Pass a "<Code inline=true>"UseComboBoxInput"</Code>" with every field named; the Default column gives "
                        "the value for fields you don\u{2019}t need."

                    </p>
                    <ApiTable kind=ApiKind::Input of="UseComboBoxInput">
                        <ApiRow name="state" ty="ComboBoxState">"From "<Code inline=true>"use_combobox_state"</Code>"."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables the input and the button."
                        </ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the input as required."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>" of "
                            <Code inline=true>"use_text_field"</Code>". It then labels the input, the button and the listbox."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby, aria_describedby" ty="MaybeProp<String>, Option<String>, Option<String>" default="None">
                            "Labels or describes the input when there is no visible label, or in addition to it. "
                            <Code inline=true>"aria_labelledby"</Code>" also labels the button and the listbox."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="form_value" ty="ComboBoxFormValue" default="Key">
                            "What the form submits under the state\u{2019}s "<Code inline=true>"name"</Code>": the selected keys ("
                            <Code inline=true>"Key"</Code>", in hidden inputs from "<Code inline=true>"form_values"</Code>") or the "
                            "input\u{2019}s text ("<Code inline=true>"Text"</Code>", always with "<Code inline=true>"allows_custom_value"</Code>")."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">
                            "The id of the form the combobox belongs to, if it is outside of it."
                        </ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">
                            "Arrow keys wrap around at the ends of the list."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the list keyboard navigation."
                        </ApiRow>
                        <ApiRow name="popover" ty="CapturedElement" default="CapturedElement::new()">
                            "The popover element; capture it with "<Code inline=true>"{..popover.attr()}"</Code>". Focus "
                            "moving into it doesn\u{2019}t count as leaving the combobox, and while it is open, everything "
                            "but the input and the popover is hidden from assistive technology."
                        </ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                            "Called when focus enters or leaves the combobox. Focus moving between the input, the button and "
                            "the popover doesn\u{2019}t count."
                        </ApiRow>
                        <ApiRow name="button_element" ty="CapturedElement" default="CapturedElement::new()">"Capture of the popup button; attach it to the button element."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-combobox-return">
                    <ApiTable kind=ApiKind::Return of="UseComboBoxReturn">
                        <ApiRow name="input" ty="UseTextFieldInput">
                            "The input\u{2019}s configuration, for "<Code inline=true>"use_text_field"</Code>": the text, "
                            <Code inline=true>"aria-activedescendant"</Code>", "<Code inline=true>"aria-autocomplete=\"list\""</Code>", "
                            <Code inline=true>"aria-controls"</Code>" (while open), autocomplete and spell checking turned off, "
                            "and the keyboard interaction."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseComboBoxInputProps">
                            "Spread onto the input in addition to the text field\u{2019}s props: "<Code inline=true>"role=\"combobox\""</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", a touch handler (tapping the input\u{2019}s center, as "
                            "screen readers do, toggles the popover) and the element capture for form reset."
                        </ApiRow>
                        <ApiRow name="button" ty="UseButtonInput">
                            "The button\u{2019}s configuration, for "<Code inline=true>"use_button"</Code>": labelled "
                            "\u{201c}Show suggestions\u{201d} together with the combobox label, not in the tab order, and not "
                            "taking focus. Pressing it focuses the input and toggles the popover."
                        </ApiRow>
                        <ApiRow name="listbox" ty="UseListBoxInput">
                            "The popover\u{2019}s listbox, for "<Code inline=true>"use_listbox"</Code>": the id the input\u{2019}s "
                            <Code inline=true>"aria-controls"</Code>" points to, the label, virtual focus, selection when "
                            "the press ends, and focus following the pointer."
                        </ApiRow>
                        <ApiRow name="form_values" ty="Signal<Vec<String>>">
                            "With "<Code inline=true>"ComboBoxFormValue::Key"</Code>" and a "<Code inline=true>"name"</Code>": render "
                            "one "<Code inline=true>"<input type=\"hidden\">"</Code>" per entry, named like the field (with "
                            <Code inline=true>"form"</Code>"): the selected keys, or one empty value without a selection. Empty "
                            "otherwise (the input submits its text)."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_contains_filter">
                <p>
                    "Returns the "<Code inline=true>"ComboBoxFilter"</Code>" most comboboxes want: an option matches when "
                    "its text contains the input text, compared in the current locale, ignoring case and accents "
                    "(\u{201c}ap\u{201d} matches \u{201c}Grape\u{201d}, \u{201c}cafe\u{201d} matches "
                    "\u{201c}Caf\u{e9}\u{201d}). Pass it as "<Code inline=true>"filter"</Code>" of "
                    <Code inline=true>"use_combobox_state"</Code>" or "<Code inline=true>"ComboBox"</Code>"."
                </p>
                <p>
                    "Call it in a component: it reads the locale from the surrounding "
                    <Code inline=true>"I18nProvider"</Code>" (else "<Code inline=true>"en-US"</Code>") each time it filters. "

                    "For other matching (a prefix, a fuzzy search), pass your own function of the option text and the "
                    "input text:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use std::sync::Arc;

                        use leptonic::hooks::combobox::ComboBoxFilter;

                        let starts_with: ComboBoxFilter = Arc::new(|input: &str| {
                            let input = input.to_lowercase();
                            Box::new(move |text: &str| text.to_lowercase().starts_with(&input))
                        });
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
                <li><Link href=routes::doc::combobox::Atom.materialize()>"Combobox Atoms"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>

            </SeeAlso>
        </DocPage>
    }
}
