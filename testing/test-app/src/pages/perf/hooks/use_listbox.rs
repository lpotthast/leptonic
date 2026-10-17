//! `use_listbox` on a plain `<div>` (labelled "Options"), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_listbox.rs`): a multiple selection (`use_list_state`) of
//! options "Option 0", "Option 1", ... (plain `<div>`s with `use_option`, `data-perf-option`
//! holding the number), [`SMALL`] at first, [`LARGE`] after the control `large`, in a 200px high
//! scroll view. Between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount the listbox, change the options and the listbox's
//! reactive input (orientation, label).

use leptonic::{
    CapturedElement, Orientation,
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
const SMALL: u32 = 10;
/// The options after the control `large`.
const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let orientation = RwSignal::new(Orientation::Vertical);
    let label = RwSignal::new("Options".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("rename", move || label.set("Choices".to_owned())),
    ];
    view! {
        <h1>"use_listbox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PlainListBox state items orientation label />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The listbox element (`use_listbox`) with its options.
#[component]
fn PlainListBox(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    orientation: RwSignal<Orientation>,
    label: RwSignal<String>,
) -> impl IntoView {
    let UseListBoxReturn {
        listbox: props,
        data,
    } = use_listbox(UseListBoxInput {
        state,
        element: CapturedElement::new(),
        presence: None,
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
            {..props.attrs()}
            style="max-height: 200px; overflow: auto"
        >
            <For each=move || items.get() key=|i| *i let:i>
                <PlainOption list=data.get_value() i />
            </For>
        </div>
    }
}

/// An option (`use_option`).
#[component]
fn PlainOption(list: ListBoxData, i: u32) -> impl IntoView {
    let UseOptionReturn {
        option: props,
        is_focus_visible,
        is_pressed,
        is_hovered,
        ..
    } = use_option(UseOptionInput {
        list,
        key: Key::from(i),
        on_context_menu: None,
    });
    let (attrs, styles) = props.attrs();
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
