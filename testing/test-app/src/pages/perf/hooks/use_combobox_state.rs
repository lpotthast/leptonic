//! `use_combobox_state` in a combo box on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_combobox_state.rs`): a single selection combo box
//! (`use_combobox_state`, filtering by "contains") with an `<input>` (`data-perf-input`,
//! `use_text_field` with `use_combobox`'s input), a `<button>` (`data-perf-button`, `use_button`),
//! and a popover `<div>` rendered while open (`data-perf-popover`) with a listbox (`use_listbox`
//! with `use_combobox`'s listbox input) of the matching options "Option 0", "Option 1", ...
//! (`use_option`, `data-perf-option` holding the number; [`SMALL`] at first, [`LARGE`] after the
//! control `large`). Mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount everything (creating the state), change the
//! options, call the state's methods (open, close, set the input text and the value), prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions), and
//! read the value (`value`: the selected option's number, `-1` for none).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, announce,
    hooks::{
        button::use_button,
        collections::{Key, Node, UseListCollectionInput, use_list_collection},
        combobox::{
            ComboBoxFormValue, ComboBoxMenuTrigger, ComboBoxState, MenuTriggerAction,
            UseComboBoxInput, UseComboBoxReturn, UseComboBoxStateInput, use_combobox,
            use_combobox_state, use_contains_filter,
        },
        form::{UseTextFieldReturn, ValidationBehavior, use_text_field},
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
            use_listbox, use_option,
        },
        select::SelectMode,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options at first.
const SMALL: u32 = 10;
/// The options after the control `large`.
const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    // The state while mounted, for the controls.
    let state = StoredValue::new(None::<ComboBoxState>);
    let with_state = move |run: fn(&ComboBoxState)| {
        move || {
            state.with_value(|state| {
                if let Some(state) = state {
                    run(state);
                }
            });
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action(
            "open",
            with_state(|state| state.open(None, MenuTriggerAction::Manual)),
        ),
        Control::action("close", with_state(ComboBoxState::close)),
        Control::action(
            "set-text",
            with_state(|state| state.set_input_value("Option 5".to_owned())),
        ),
        Control::action(
            "clear-text",
            with_state(|state| state.set_input_value(String::new())),
        ),
        Control::action(
            "select-first",
            with_state(|state| state.set_value(vec![Key::from(0_u32)])),
        ),
        Control::action(
            "clear-value",
            with_state(|state| state.set_value(Vec::new())),
        ),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("value", move || {
            // Untracked: a read from outside the page's reactive code.
            untrack(|| {
                state.with_value(|state| {
                    state
                        .as_ref()
                        .and_then(ComboBoxState::selected_key)
                        .and_then(|key| key.as_i64())
                })
            })
            .and_then(|value| i32::try_from(value).ok())
            .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"use_combobox_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PlainComboBox items state />
        </Show>
    }
}

/// The state (`use_combobox_state`, created while mounted) and the combo box's elements.
#[component]
fn PlainComboBox(
    items: RwSignal<Vec<u32>>,
    state: StoredValue<Option<ComboBoxState>>,
) -> impl IntoView {
    let combo_box_state = use_combobox_state(UseComboBoxStateInput {
        filter: Some(use_contains_filter()),
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Option {i}"),
        }),
        selection_mode: SelectMode::Single,
        default_value: Vec::new(),
        value: None,
        on_change: None,
        default_input_value: None,
        input_value: None,
        on_input_change: None,
        disabled_keys: Signal::stored(HashSet::new()),
        menu_trigger: Signal::stored(ComboBoxMenuTrigger::Input),
        allows_empty_collection: Signal::stored(false),
        allows_custom_value: Signal::stored(false),
        should_close_on_blur: true,
        is_read_only: Signal::stored(false),
        on_open_change: None,
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: None,
    });
    state.set_value(Some(combo_box_state));
    on_cleanup(move || state.set_value(None));
    view! {
        <ComboBoxElements state=combo_box_state />
    }
}

/// The input, button and popover (`use_combobox`).
#[component]
fn ComboBoxElements(state: ComboBoxState) -> impl IntoView {
    let popover = CapturedElement::new();
    let button_element = CapturedElement::new();
    let UseComboBoxReturn {
        text_field: input,
        input: input_props,
        button,
        listbox,
        ..
    } = use_combobox(UseComboBoxInput {
        state,
        id: None,
        is_disabled: Signal::stored(false),
        is_required: Signal::stored(false),
        has_label: Signal::stored(false),
        aria_label: "Option".into(),
        aria_labelledby: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        placeholder: MaybeProp::default(),
        form_value: ComboBoxFormValue::Key,
        form: None,
        should_focus_wrap: false,
        keyboard_delegate: None,
        popover,
        button_element,
        on_focus: None,
        on_blur: None,
    });
    let UseTextFieldReturn {
        input: field_props, ..
    } = use_text_field(input);
    let (button_attrs, button_styles) = use_button(button).button.attrs();
    let listbox = StoredValue::new(listbox);
    view! {
        <input {..field_props.attrs()} {..input_props.attrs()} data-perf-input />
        <button
            {..button_attrs}
            {..button_element.attr()}
            style=button_styles
            data-perf-button
        >
            "▼"
        </button>
        <Show when=move || state.is_open()>
            <div {..popover.attr()} data-perf-popover>
                <PlainListBox input=listbox.get_value() />
            </div>
        </Show>
    }
}

/// The listbox of the open combo box (`use_listbox`), showing the matching options.
#[component]
fn PlainListBox(input: UseListBoxInput) -> impl IntoView {
    let UseListBoxReturn {
        listbox: props,
        data,
    } = use_listbox(input);
    let collection = data.state.collection;
    let options = move || collection.with(|c| c.items().cloned().collect::<Vec<Node>>());
    let data = StoredValue::new(data);
    view! {
        <div {..props.attrs()} style="max-height: 200px; overflow: auto">
            <For each=options key=|node| node.key.clone() let:node>
                <PlainOption list=data.get_value() node />
            </For>
        </div>
    }
}

/// An option (`use_option`).
#[component]
fn PlainOption(list: ListBoxData, node: Node) -> impl IntoView {
    let UseOptionReturn {
        option: props,
        is_focused,
        ..
    } = use_option(UseOptionInput {
        list,
        key: node.key.clone(),
        on_context_menu: None,
    });
    let (attrs, styles) = props.attrs();
    view! {
        <div
            {..attrs}
            style=styles
            data-perf-option=node.key.to_string()
            data-focused=move || is_focused.get().then_some("true")
        >
            {node.text_value.to_string()}
        </div>
    }
}
