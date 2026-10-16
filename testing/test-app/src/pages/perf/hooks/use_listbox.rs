//! `use_listbox` on a plain `<div>` (labelled "Options"), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_listbox.rs`): a multiple selection (`use_list_state`) of
//! options "Option 0", "Option 1", ... (plain `<div>`s with `use_option`, `data-perf-option`
//! holding the number), [`SMALL`] at first, [`LARGE`] after the control `large`, in a 200px high
//! scroll view. Between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first.
//!
//! [`ListBoxFixture`] renders it for `use_option`'s performance test too: the controls `mount` and
//! `unmount` mount the listbox (calling `use_listbox`) or only its options (calling `use_option`).
//! The other controls ([`Control`]) change the options, the state (its methods: select all, clear,
//! select the first option) and the listbox's reactive input (orientation, label, disabled
//! options).

use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionMode, SelectionOptions,
            UseListCollectionInput, UseListStateInput, use_list_collection, use_list_state,
        },
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
            use_listbox, use_option,
        },
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
    /// The listbox with its options.
    ListBox,
    /// The options (in the mounted listbox).
    Options,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <ListBoxFixture title="use_listbox performance" part=Part::ListBox /> }
}

/// The listbox (see the module documentation), `part` mounted by the controls.
#[component]
pub fn ListBoxFixture(title: &'static str, part: Part) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_mounted = move |of: Part| Signal::derive(move || of != part || mounted.get());
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled_keys.into(),
            ..SelectionOptions::default()
        },
    });
    let orientation = RwSignal::new(Orientation::Vertical);
    let label = RwSignal::new("Options".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || {
            state.selection.set_selected_keys([Key::from(0_u32)]);
        }),
        Control::action("select-all", move || state.selection.select_all()),
        Control::action("clear-selection", move || state.selection.clear_selection()),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("rename", move || label.set("Choices".to_owned())),
    ];
    let options_mounted = is_mounted(Part::Options);
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || is_mounted(Part::ListBox).get()>
            <PlainListBox state items orientation label options_mounted />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The listbox element (`use_listbox`) with its options, while `options_mounted`.
#[component]
fn PlainListBox(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    orientation: RwSignal<Orientation>,
    label: RwSignal<String>,
    options_mounted: Signal<bool>,
) -> impl IntoView {
    let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: label.into(),
        aria_labelledby: Signal::stored(None),
        orientation: orientation.into(),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        layout_delegate: None,
        is_virtualized: false,
        options: CollectionOptions::default(),
        should_select_on_press_up: false,
        should_focus_on_hover: false,
        on_action: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let data = StoredValue::new(data);
    view! {
        <div
            {..props.into_attrs()}
            style="max-height: 200px; overflow: auto"
        >
            <Show when=move || options_mounted.get()>
                <For each=move || items.get() key=|i| *i let:i>
                    <PlainOption list=data.get_value() i />
                </For>
            </Show>
        </div>
    }
}

/// An option (`use_option`).
#[component]
fn PlainOption(list: ListBoxData, i: u32) -> impl IntoView {
    let UseOptionReturn {
        props,
        is_focus_visible,
        is_pressed,
        is_hovered,
        ..
    } = use_option(UseOptionInput {
        list,
        key: Key::from(i),
        on_context_menu: None,
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <div
            {..attrs}
            style=styles
            data-perf-option=i
            data-focus-visible=move || is_focus_visible.get().then_some("true")
            data-pressed=move || is_pressed.get().then_some("true")
            data-hovered=move || is_hovered.get().then_some("true")
        >
            {format!("Option {i}")}
        </div>
    }
}
