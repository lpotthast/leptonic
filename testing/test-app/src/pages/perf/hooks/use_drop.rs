//! The hook `use_drop` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_drop.rs`): a drop target (`#perf-drop`, "Drop here",
//! focusable), mounted at first, and a `use_drag` source to drag from with the keyboard
//! (`#perf-draggable`, dragging the text "hello world"), between two plain buttons
//! (`#perf-before`, `#perf-after`). The page announces an empty message once mounted, so that the
//! live announcer (which keyboard drags announce through) exists before the test measures.
//!
//! Its controls ([`Control`]) mount and unmount the target, disable and enable it, and read how
//! often a drag entered it (`enters`), moved over it (`moves`), left it (`exits`) and dropped
//! on it (`drops`), and the last drop's operation (`last-operation`: 0 cancel, 1 move, 2 copy,
//! 3 link).

use leptonic::{
    CapturedElement, IntoAttrs, announce_polite, flag,
    hooks::dnd::{
        DragItem, DropEvent, DropOperation, UseDragInput, UseDragReturn, UseDropInput,
        UseDropReturn, use_drag, use_drop,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// A drop operation as a number: 0 cancel, 1 move, 2 copy, 3 link.
fn operation_code(operation: DropOperation) -> f64 {
    match operation {
        DropOperation::Cancel => 0.0,
        DropOperation::Move => 1.0,
        DropOperation::Copy => 2.0,
        DropOperation::Link => 3.0,
    }
}

/// What the target counts.
#[derive(Clone, Copy)]
struct Counts {
    enters: StoredValue<u32>,
    moves: StoredValue<u32>,
    exits: StoredValue<u32>,
    drops: StoredValue<u32>,
    last_operation: StoredValue<f64>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let counts = Counts {
        enters: StoredValue::new(0),
        moves: StoredValue::new(0),
        exits: StoredValue::new(0),
        drops: StoredValue::new(0),
        last_operation: StoredValue::new(0.0),
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("enters", move || f64::from(counts.enters.get_value())),
        Control::read("moves", move || f64::from(counts.moves.get_value())),
        Control::read("exits", move || f64::from(counts.exits.get_value())),
        Control::read("drops", move || f64::from(counts.drops.get_value())),
        Control::read("last-operation", move || counts.last_operation.get_value()),
    ];
    Effect::new(|| announce_polite(""));

    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: None,
        on_drag_move: None,
        on_drag_end: None,
        has_drag_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>"use_drop performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div
            id="perf-draggable"
            role="button"
            tabindex="0"
            {..drag_props.into_attrs()}
            data-dragging=flag(is_dragging)
        >
            "Drag me"
        </div>
        <Show when=move || mounted.get()>
            <Target disabled counts />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The drop target.
#[component]
fn Target(disabled: RwSignal<bool>, counts: Counts) -> impl IntoView {
    let element = CapturedElement::new();
    let count = |counter: StoredValue<u32>| counter.update_value(|count| *count += 1);
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: Some(Callback::new(move |_| count(counts.enters))),
        on_drop_move: Some(Callback::new(move |_| count(counts.moves))),
        on_drop_activate: None,
        on_drop_exit: Some(Callback::new(move |_| count(counts.exits))),
        on_drop: Some(Callback::new(move |e: DropEvent| {
            count(counts.drops);
            counts
                .last_operation
                .set_value(operation_code(e.drop_operation));
        })),
        has_drop_button: false,
        is_disabled: disabled.into(),
    });
    view! {
        <div
            id="perf-drop"
            role="button"
            tabindex="0"
            {..drop_props.into_attrs()}
            data-drop-target=flag(is_drop_target)
        >
            "Drop here"
        </div>
    }
}
