use indoc::indoc;
use leptos::prelude::*;

use super::demos::select::SelectDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseSelectHook() -> impl IntoView {
    view! {
        <DocPage title="Select Hooks">
            <p>
                "The select hooks build an accessible dropdown select from your own markup: a trigger button showing the "
                "selected option, a listbox popover, keyboard navigation and type-ahead, and a hidden native "
                <Code inline=true>"<select>"</Code>" for forms. See the "
                <Link href=routes::doc::Select.materialize()>"Select overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useSelect"/>

            <Section title="Demo">
                <p>
                    "A select built from the hooks, with a popover from "
                    <Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>
                    ". Open it with the mouse or the keyboard, or focus the trigger and type a letter."
                </p>

                <Demo description="Fruit select with a disabled toggle" source=include_str!("demos/select.rs")>
                    <SelectDemo/>
                </Demo>
            </Section>

            <Section title="use_select_state">
                <p>
                    "Owns everything a select knows: its options (a collection, see the "
                    <Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link>
                    "), the selected keys, whether the popover is open, focus and validation. "
                    "The value is a list of keys in both modes: at most one in "<Code inline=true>"SelectMode::Single"</Code>
                    " (the default), any number in "<Code inline=true>"SelectMode::Multiple"</Code>"."
                </p>

                <Section title="Input" id="use-select-state-input">
                    <p>
                        "Pass a "<Code inline=true>"UseSelectStateInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseSelectStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The options. Required."</ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode" default="Single">
                            <Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>"."
                        </ApiRow>
                        <ApiRow name="default_value" ty="Vec<Key>" default="empty">
                            "The initially selected keys (at most one in "<Code inline=true>"Single"</Code>" mode). "
                            "Resetting the form restores it."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Vec<Key>>>" default="None">
                            "The selected keys as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_value"</Code>": the select shows them, and selecting writes them (in collection order)."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the selected keys, in collection order, when they change."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                            "Options that can\u{2019}t be focused or selected."
                        </ApiRow>
                        <ApiRow name="should_close_on_select" ty="CloseOnSelect" default="Auto">
                            "Close the popover when an option is selected: "<Code inline=true>"Always"</Code>", "
                            <Code inline=true>"Never"</Code>", or "<Code inline=true>"Auto"</Code>" (in "<Code inline=true>"Single"</Code>" mode)."
                        </ApiRow>
                        <ApiRow name="allows_empty_collection" ty="bool" default="false">
                            "Allow opening the popover without options, e.g. to show an empty state."
                        </ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Start with the popover open."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the popover opens or closes."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Vec<Key>>>" default="None">
                            "Validates the selected keys, returning error messages."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as the value changes, "<Code inline=true>"Native"</Code>
                            " defers them to form submission and uses native constraint validation."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The form field name, used to match server-side validation errors."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-select-state-return">
                    <p>
                        <Code inline=true>"SelectState"</Code>" is "<Code inline=true>"Copy"</Code>
                        ". Its queries track signals, so you can use them in views."
                    </p>

                    <ApiTable kind=ApiKind::Return of="SelectState">
                        <ApiRow name="list" ty="ListState">"The options, selection and focus."</ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode">"The selection mode."</ApiRow>
                        <ApiRow name="menu_trigger" ty="MenuTriggerState">
                            "The popover\u{2019}s open state. Open it through the select\u{2019}s "<Code inline=true>"open"</Code>
                            " and "<Code inline=true>"toggle"</Code>", which only open it with options."
                        </ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The validation state."</ApiRow>
                    </ApiTable>

                    <p>"Its methods:"</p>

                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"value() -> Vec<Key>"</Code></TableCell>
                            <TableCell>"The selected keys, in collection order."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"selected_key() -> Option<Key>"</Code></TableCell>
                            <TableCell>"The selected key (the first one in "<Code inline=true>"Multiple"</Code>" mode)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"selected_items() -> Vec<Node>"</Code></TableCell>
                            <TableCell>"The collection nodes of the selected options, e.g. for their text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(keys: Vec<Key>)"</Code></TableCell>
                            <TableCell>"Selects "<Code inline=true>"keys"</Code>" (only the first one in "<Code inline=true>"Single"</Code>" mode)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_value() -> Vec<Key>"</Code></TableCell>
                            <TableCell>"The value the select started with."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_open() -> bool"</Code></TableCell>
                            <TableCell>"Whether the popover is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"open(Option<FocusStrategy>), close(), toggle(Option<FocusStrategy>)"</Code></TableCell>
                            <TableCell>
                                "Open or close the popover. The strategy decides which option gets focus. Opening only works when "
                                "there are options, unless "<Code inline=true>"allows_empty_collection"</Code>" is set."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_strategy() -> Option<FocusStrategy>"</Code></TableCell>
                            <TableCell>"How focus enters the popover\u{2019}s listbox."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_focused() -> bool, set_focused(bool)"</Code></TableCell>
                            <TableCell>"Whether the trigger or the popover has focus."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Example" id="use-select-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use std::collections::HashSet;

                            use leptonic::hooks::{
                                Key, SelectMode, UseSelectStateInput, ValidationBehavior, collections::CloseOnSelect,
                                use_select_state,
                            };
                            use leptos::{logging::log, prelude::*};

                            let state = use_select_state(UseSelectStateInput {
                                collection,
                                selection_mode: SelectMode::Single,
                                default_value: vec![Key::from("banana")],
                                value: None,
                                on_change: Some(Callback::new(|keys: Vec<Key>| log!("{keys:?}"))),
                                disabled_keys: Signal::stored(HashSet::new()),
                                should_close_on_select: CloseOnSelect::Auto,
                                allows_empty_collection: false,
                                default_open: false,
                                on_open_change: None,
                                is_invalid: Signal::stored(false),
                                validate: None,
                                validation_behavior: ValidationBehavior::Aria,
                                name: None,
                            });

                            state.selected_key();          // Some(Key::from("banana"))
                            state.set_value(vec![Key::from("apple")]);
                            state.open(None);
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_select">
                <p>
                    "Connects the parts of a select. It doesn\u{2019}t render anything itself and, for the trigger "
                    "and the popover\u{2019}s listbox, it returns the configuration of the hooks that do: pass "
                    <Code inline=true>"trigger"</Code>" to "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                    " and "<Code inline=true>"listbox"</Code>" to "<Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link>"."
                </p>

                <Section title="Input" id="use-select-input">
                    <p>
                        "Pass a "<Code inline=true>"UseSelectInput"</Code>" with every field named; the Default column gives "
                        "the value for fields you don\u{2019}t need."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseSelectInput">
                        <ApiRow name="state" ty="SelectState">"The state from "<Code inline=true>"use_select_state"</Code>". Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The trigger\u{2019}s id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the select is disabled."</ApiRow>
                        <ApiRow name="is_required" ty="bool" default="false">
                            "Whether a value is required. Enforced by the hidden select with native validation."
                        </ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"An accessible name without a visible label."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of other elements labelling the select."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The id(s) of elements describing the select."</ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the list keyboard navigation used by type-ahead and the arrow keys on the trigger."
                        </ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                            "Called when focus moves to the select (trigger or popover) from outside, or leaves it."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called with whether the select has focus."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The form field name of the hidden select."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the select belongs to, if it is outside of it."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"Passed on to the hidden select."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-select-return">
                    <ApiTable kind=ApiKind::Return of="UseSelectReturn">
                        <ApiRow name="label_props" ty="UseSelectLabelProps">
                            "For the visible label. Clicking it focuses the trigger."
                        </ApiRow>
                        <ApiRow name="trigger" ty="UseButtonInput">
                            "The trigger button\u{2019}s configuration, for "<Code inline=true>"use_button"</Code>
                            ". Extend it with struct update syntax."
                        </ApiRow>
                        <ApiRow name="trigger_props" ty="UseSelectTriggerProps">
                            "Spread onto the trigger next to the button\u{2019}s props: lets a space continue a type-ahead search "
                            "instead of opening the popover."
                        </ApiRow>
                        <ApiRow name="value_props" ty="UseSelectValueProps">
                            "For the element showing the selected option inside the trigger. The trigger is labelled by it."
                        </ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For optional description and error message elements, referenced by "
                            <Code inline=true>"aria-describedby"</Code>" while rendered."
                        </ApiRow>
                        <ApiRow name="listbox" ty="UseListBoxInput">
                            "The popover\u{2019}s listbox, for "<Code inline=true>"use_listbox"</Code>
                            ". It focuses the selected option when it opens, selects on press up and focuses options on hover."
                        </ApiRow>
                        <ApiRow name="hidden_select" ty="UseHiddenSelectInput">
                            "The native form element mirroring the value, for "<Code inline=true>"use_hidden_select"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the value is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The current validation errors."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-select-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::{UseSelectInput, UseSelectReturn, ValidationBehavior, use_button, use_select};

                            let UseSelectReturn {
                                label_props,
                                trigger,
                                trigger_props,
                                value_props,
                                listbox,
                                hidden_select,
                                ..
                            } = use_select(UseSelectInput {
                                state,
                                id: None,
                                is_disabled: false.into(),
                                is_required: false,
                                has_label: true.into(),
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                keyboard_delegate: None,
                                on_focus: None,
                                on_blur: None,
                                on_focus_change: None,
                                name: Some("fruit".to_owned()),
                                form: None,
                                validation_behavior: ValidationBehavior::Aria,
                            });

                            let button = use_button(trigger);
                        "#)}
                    </Code>

                    <p>
                        "While closed, the trigger supports type-ahead, and in single selection mode the left and "
                        "right arrow keys change the value without opening the popover."
                    </p>
                </Section>
            </Section>

            <Section title="use_hidden_select">
                <p>
                    "Mirrors the value in a visually hidden native "<Code inline=true>"<select>"</Code>", or in hidden inputs "
                    "for more than 300 options. The hidden element makes sure that:"
                </p>

                <ul>
                    <li>"the value is submitted with the form"</li>
                    <li>"resetting the form restores the initial value"</li>
                    <li>"browsers can autofill the select (native "<Code inline=true>"<select>"</Code>" only)"</li>
                    <li>"native form validation (with "<Code inline=true>"ValidationBehavior::Native"</Code>") works"</li>
                </ul>

                <Section title="Input" id="use-hidden-select-input">
                    <p>
                        "Usually "<Code inline=true>"use_select"</Code>"\u{2019}s "<Code inline=true>"hidden_select"</Code>
                        ". Building it yourself means naming every field."

                    </p>

                    <ApiTable kind=ApiKind::Input of="UseHiddenSelectInput">
                        <ApiRow name="state" ty="SelectState">"The select\u{2019}s state. Required."</ApiRow>
                        <ApiRow name="name" ty="Option<String>">"The form field name."</ApiRow>
                        <ApiRow name="form" ty="Option<String>">"The id of the form, if the select is outside of it."</ApiRow>
                        <ApiRow name="auto_complete" ty="Option<String>">
                            "The "<Code inline=true>"autocomplete"</Code>" hint for autofill. "<Code inline=true>"use_select"</Code>
                            " leaves it at "<Code inline=true>"None"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Disables the form element."</ApiRow>
                        <ApiRow name="is_required" ty="bool">
                            "Sets "<Code inline=true>"required"</Code>" (only with native validation)."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"How validation errors are reported."</ApiRow>
                        <ApiRow name="trigger" ty="Option<CapturedElement>">
                            "The select\u{2019}s trigger, focused when the select is its form\u{2019}s first invalid field on submission."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-hidden-select-return">
                    <ApiTable kind=ApiKind::Return of="UseHiddenSelectReturn">
                        <ApiRow name="container_props" ty="UseHiddenSelectContainerProps">
                            "For a wrapper element: visually hidden and "<Code inline=true>"aria-hidden"</Code>"."
                        </ApiRow>
                        <ApiRow name="use_native_select" ty="Signal<bool>">
                            "Whether to render a native "<Code inline=true>"<select>"</Code>" (up to 300 options) or hidden inputs."
                        </ApiRow>
                        <ApiRow name="select_props" ty="UseHiddenSelectSelectProps">
                            "For the "<Code inline=true>"<select>"</Code>". Changes made by autofill update the select\u{2019}s value."
                        </ApiRow>
                        <ApiRow name="options" ty="Signal<Vec<HiddenSelectOption>>">
                            "The "<Code inline=true>"<option>"</Code>"s to render ("<Code inline=true>"value"</Code>", "
                            <Code inline=true>"text"</Code>", "<Code inline=true>"is_selected"</Code>"), starting with an empty one."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseHiddenSelectInputProps">
                            "For the hidden inputs used instead of the "<Code inline=true>"<select>"</Code>"."
                        </ApiRow>
                        <ApiRow name="input_values" ty="Signal<Vec<String>>">
                            "One value per selected key, one empty value without a selection. Render one input per value, "
                            "only if the select has a name."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-hidden-select-example">
                    <Code language=Language::Rust>
                        {indoc!(r"
                            use leptonic::hooks::{IntoAttrs, UseHiddenSelectReturn, use_hidden_select};

                            let UseHiddenSelectReturn { container_props, select_props, options, .. } =
                                use_hidden_select(hidden_select);

                            view! {
                                <div {..container_props.into_attrs()}>
                                    <select {..select_props.into_attrs()}>
                                        <For each=move || options.get() key=|o| (o.value.clone(), o.is_selected) let:o>
                                            <option value=o.value selected=o.is_selected>{o.text}</option>
                                        </For>
                                    </select>
                                </div>
                            }
                        ")}
                    </Code>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
