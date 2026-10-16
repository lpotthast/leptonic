//! `use_combobox` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_combobox.rs`): a single selection combo box
//! (`use_combobox_state`, filtering by "contains") with an `<input>` (`data-perf-input`,
//! `use_text_field` with `use_combobox`'s input), a `<button>` (`data-perf-button`, `use_button`),
//! and a popover `<div>` rendered while open (`data-perf-popover`) with a listbox (`use_listbox`
//! with `use_combobox`'s listbox input) of the matching options "Option 0", "Option 1", ...
//! (`use_option`, `data-perf-option` holding the number; [`SMALL`] at first, [`LARGE`] after the
//! control `large`). Between two plain buttons (`#perf-before`, `#perf-after`), mounted at first.
//!
//! [`ComboBoxFixture`] renders it for `use_combobox_state`'s performance test too: the controls
//! `mount` and `unmount` mount everything (creating the state, [`Part::State`]) or the combo box's
//! elements (calling `use_combobox`, [`Part::ComboBox`]). The other controls ([`Control`]) call the
//! state's methods (open, close, set the input text and the value), change `use_combobox`'s
//! disabled input, prepare the live announcer (`prepare-announcer`: its first announcement waits
//! for its live regions), and read the value (`value`: the selected option's number, `-1` for
//! none).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce,
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
pub const SMALL: u32 = 10;
/// The options after the control `large`.
pub const LARGE: u32 = 1_000;

/// What the controls `mount` and `unmount` mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// Everything, with the state.
    State,
    /// The combo box's elements (`use_combobox`), the state staying.
    ComboBox,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <ComboBoxFixture title="use_combobox performance" part=Part::ComboBox /> }
}

/// The combo box (see the module documentation), `part` mounted by the controls.
#[component]
pub fn ComboBoxFixture(title: &'static str, part: Part) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_mounted = move |of: Part| Signal::derive(move || of != part || mounted.get());
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let disabled = RwSignal::new(false);
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
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
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
    let combo_box_mounted = is_mounted(Part::ComboBox);
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || is_mounted(Part::State).get()>
            <PlainComboBox items disabled state combo_box_mounted />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The state (`use_combobox_state`, created while mounted) and the combo box's elements.
#[component]
fn PlainComboBox(
    items: RwSignal<Vec<u32>>,
    disabled: RwSignal<bool>,
    state: StoredValue<Option<ComboBoxState>>,
    combo_box_mounted: Signal<bool>,
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
        <Show when=move || combo_box_mounted.get()>
            <ComboBoxElements state=combo_box_state disabled />
        </Show>
    }
}

/// The input, button and popover (`use_combobox`).
#[component]
fn ComboBoxElements(state: ComboBoxState, disabled: RwSignal<bool>) -> impl IntoView {
    let popover = CapturedElement::new();
    let button_element = CapturedElement::new();
    let UseComboBoxReturn {
        input,
        input_props,
        button,
        listbox,
        ..
    } = use_combobox(UseComboBoxInput {
        state,
        id: None,
        is_disabled: disabled.into(),
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
        input_props: field_props,
        ..
    } = use_text_field(input);
    let (button_attrs, button_styles) = use_button(button).props.into_parts();
    let listbox = StoredValue::new(listbox);
    view! {
        <input {..field_props.into_attrs()} {..input_props.into_attrs()} data-perf-input />
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
    let UseListBoxReturn { props, data } = use_listbox(input);
    let collection = data.state.collection;
    let options = move || collection.with(|c| c.items().cloned().collect::<Vec<Node>>());
    let data = StoredValue::new(data);
    view! {
        <div {..props.into_attrs()} style="max-height: 200px; overflow: auto">
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
        props, is_focused, ..
    } = use_option(UseOptionInput {
        list,
        key: node.key.clone(),
        on_context_menu: None,
    });
    let (attrs, styles) = props.into_parts();
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
