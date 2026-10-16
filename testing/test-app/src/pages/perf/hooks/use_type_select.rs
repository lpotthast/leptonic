//! `use_type_select` on a plain focusable `<div>` (`#perf-list`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_type_select.rs`): type-ahead through the items "Option
//! 0", "Option 1", ... of a list state ([`SMALL`] at first, [`LARGE`] after the control `large`),
//! found by `use_list_keyboard_delegate`'s delegate; the focused item's key shows in
//! `#perf-focused`. The state stays; the list is mounted at first (mounting calls the hook).
//! Between two plain buttons (`#perf-before`, `#perf-after`).

use leptonic::{
    CapturedElement, Orientation,
    hooks::collections::{
        Key, ListLayout, ListState, SelectionMode, SelectionOptions, UseListCollectionInput,
        UseListKeyboardDelegateInput, UseListStateInput, UseTypeSelectInput, use_list_collection,
        use_list_keyboard_delegate, use_list_state, use_type_select,
    },
};
use leptos::{ev, prelude::*};

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
pub const SMALL: u32 = 10;
/// The items after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let state = use_list_state(UseListStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Option {i}"),
        }),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Single),
            ..SelectionOptions::default()
        },
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
    ];
    view! {
        <h1>"use_type_select performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <TypeSelectList state />
        </Show>
        <p id="perf-focused">
            {move || state.selection.focused_key().map(|key| key.to_string())}
        </p>
        <button id="perf-after">"After"</button>
    }
}

/// The list element with type-ahead (`use_type_select`).
#[component]
fn TypeSelectList(state: ListState) -> impl IntoView {
    let element = CapturedElement::new();
    let delegate = use_list_keyboard_delegate(UseListKeyboardDelegateInput {
        state,
        element,
        orientation: Orientation::Vertical.into(),
        layout: ListLayout::Stack,
        layout_delegate: None,
    });
    let props = use_type_select(UseTypeSelectInput {
        delegate,
        selection: state.selection,
        on_type_select: None,
    })
    .props;
    let handlers = (
        props.on_keydown_capture.into_on(ev::capture(ev::keydown)),
        props.on_keydown.into_on(ev::keydown),
    );
    view! {
        <div {..handlers} {..element.attr()} id="perf-list" tabindex="0">
            "Type to find an option"
        </div>
    }
}
