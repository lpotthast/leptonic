//! `use_hidden_select` in a select on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_hidden_select.rs`): a single select (`use_select_state`,
//! field `option`) with a trigger `<button>` (`use_button` with `use_select`'s trigger input), a
//! listbox `<div>` rendered while open, and its hidden select (`use_hidden_select`: a native
//! `<select>` up to 300 options, else hidden inputs) of the options "Option 0", "Option 1", ...
//! ([`SMALL`] at first, [`LARGE`] after the control `large`), mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the hidden select (the rest stays), change the
//! options, set the value (the first option, none), change `use_select`'s reactive input
//! (disabled, required), and read the value (`value`: the selected option's number, `-1` for
//! none).

use std::collections::HashSet;

use leptonic::hooks::{
    button::use_button,
    collections::{CloseOnSelect, Key, UseListCollectionInput, use_list_collection},
    form::ValidationBehavior,
    listbox::{
        ListBoxData, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
        use_listbox, use_option,
    },
    select::{
        SelectMode, SelectState, UseHiddenSelectInput, UseHiddenSelectReturn, UseSelectInput,
        UseSelectReturn, UseSelectStateInput, use_hidden_select, use_select, use_select_state,
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
    let disabled = RwSignal::new(false);
    let required = RwSignal::new(false);
    // The state while mounted, for the controls.
    let state = StoredValue::new(None::<SelectState>);
    let with_state = move |run: fn(&SelectState)| {
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
            "select-first",
            with_state(|state| state.set_value(vec![Key::from(0_u32)])),
        ),
        Control::action(
            "clear-value",
            with_state(|state| state.set_value(Vec::new())),
        ),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::read("value", move || {
            // Untracked: a read from outside the page's reactive code.
            untrack(|| {
                state.with_value(|state| {
                    state
                        .as_ref()
                        .and_then(SelectState::selected_key)
                        .and_then(|key| key.as_i64())
                })
            })
            .and_then(|value| i32::try_from(value).ok())
            .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"use_hidden_select performance"</h1>
        <PerfControls controls />
        <PlainSelect items disabled required state hidden_select=mounted />
    }
}

/// The state (`use_select_state`) and the select's elements, its hidden select while
/// `hidden_select`.
#[component]
fn PlainSelect(
    items: RwSignal<Vec<u32>>,
    disabled: RwSignal<bool>,
    required: RwSignal<bool>,
    state: StoredValue<Option<SelectState>>,
    hidden_select: RwSignal<bool>,
) -> impl IntoView {
    let select_state = use_select_state(UseSelectStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Option {i}"),
        }),
        selection_mode: SelectMode::Single,
        default_value: Vec::new(),
        value: None,
        on_change: None,
        disabled_keys: Signal::stored(HashSet::new()),
        should_close_on_select: CloseOnSelect::Auto,
        allows_empty_collection: false,
        default_open: false,
        is_open: None,
        on_open_change: None,
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::Native,
        name: Some("option".to_owned()),
    });
    state.set_value(Some(select_state));
    on_cleanup(move || state.set_value(None));
    view! {
        <SelectElements state=select_state items disabled required hidden_select />
    }
}

/// The trigger, value, listbox and hidden select (`use_select`).
#[component]
fn SelectElements(
    state: SelectState,
    items: RwSignal<Vec<u32>>,
    disabled: RwSignal<bool>,
    required: RwSignal<bool>,
    hidden_select: RwSignal<bool>,
) -> impl IntoView {
    let UseSelectReturn {
        trigger_button: trigger,
        trigger: trigger_props,
        value,
        listbox,
        hidden_select: hidden_select_input,
        ..
    } = use_select(UseSelectInput {
        state,
        id: None,
        is_disabled: disabled.into(),
        is_required: required.into(),
        has_label: Signal::stored(false),
        aria_label: "Option".into(),
        aria_labelledby: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        keyboard_delegate: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        form: None,
    });
    let button = use_button(trigger);
    let is_pressed = button.is_pressed;
    let is_hovered = button.is_hovered;
    let (attrs, styles) = button.button.attrs();
    let listbox = StoredValue::new(listbox);
    let hidden_select_input = StoredValue::new(hidden_select_input);
    let text = move || {
        state.selected_items().first().map_or_else(
            || "Pick an option".to_owned(),
            |node| node.text_value.to_string(),
        )
    };
    view! {
        <button
            {..attrs}
            {..trigger_props.attrs()}
            style=styles
            data-perf-trigger
            data-pressed=move || is_pressed.get().then_some("true")
            data-hovered=move || is_hovered.get().then_some("true")
            data-open=move || state.is_open().then_some("true")
            data-disabled=move || disabled.get().then_some("true")
        >
            <span {..value.attrs()} data-perf-value>
                {text}
            </span>
        </button>
        <Show when=move || state.is_open()>
            <PlainListBox input=listbox.get_value() items />
        </Show>
        <Show when=move || hidden_select.get()>
            <PlainHiddenSelect input=hidden_select_input.get_value() />
        </Show>
    }
}

/// The listbox of the open select (`use_listbox`).
#[component]
fn PlainListBox(input: UseListBoxInput, items: RwSignal<Vec<u32>>) -> impl IntoView {
    let UseListBoxReturn {
        listbox: props,
        data,
    } = use_listbox(input);
    let data = StoredValue::new(data);
    view! {
        <div {..props.attrs()} style="max-height: 200px; overflow: auto">
            <For each=move || items.get() key=|i| *i let:i>
                <PlainOption list=data.get_value() i />
            </For>
        </div>
    }
}

/// An option (`use_option`).
#[component]
fn PlainOption(list: ListBoxData, i: u32) -> impl IntoView {
    let UseOptionReturn { option: props, .. } = use_option(UseOptionInput {
        list,
        key: Key::from(i),
        on_context_menu: None,
    });
    let (attrs, styles) = props.attrs();
    view! {
        <div {..attrs} style=styles data-perf-option=i>
            {format!("Option {i}")}
        </div>
    }
}

/// The hidden select (`use_hidden_select`): a native `<select>` (`name="option"`) or hidden
/// inputs.
#[component]
fn PlainHiddenSelect(input: UseHiddenSelectInput) -> impl IntoView {
    let state = input.state;
    let UseHiddenSelectReturn {
        container: container_props,
        use_native_select,
        select: select_props,
        options,
        input: input_props,
        input_values,
        ..
    } = use_hidden_select(input);
    let select = select_props;
    let input_props = StoredValue::new(input_props.props().expect("a new part's props"));
    let inputs = move || {
        input_values
            .get()
            .into_iter()
            .map(|value| {
                input_props.with_value(|props| {
                    view! {
                        <input
                            type=props.r#type
                            name=props.name.clone()
                            disabled=props.disabled
                            value=value
                        />
                    }
                })
            })
            .collect_view()
    };
    view! {
        <div {..container_props.attrs()}>
            <Show when=move || use_native_select.get() fallback=inputs>
                <select {..select.attrs()}>
                    <For each=move || options.get() key=|option| option.key.clone() let:option>
                        {
                            let value = option.value.clone();
                            let text = option.text.clone();
                            let selected = move || option.is_selected(&state);
                            view! {
                                <option value=value selected=selected.clone() prop:selected=selected>
                                    {text}
                                </option>
                            }
                        }
                    </For>
                </select>
            </Show>
        </div>
    }
}
