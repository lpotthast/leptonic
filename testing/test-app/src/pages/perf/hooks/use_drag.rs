//! The hook `use_drag` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_drag.rs`): a draggable element (`#perf-draggable`,
//! focusable, dragging the text "hello world"), mounted at first, and a `use_drop` target to drop
//! on (`#perf-drop`, "Drop here"), between two plain buttons (`#perf-before`, `#perf-after`). The
//! page announces an empty message once mounted, so that the live announcer (which keyboard drags
//! announce through) exists before the test measures.
//!
//! Its controls ([`Control`]) mount and unmount the draggable, disable and enable it, and read
//! how often a drag started (`starts`), moved (`moves`) and ended (`ends`), how often the target
//! took a drop (`drops`), and the last drag's drop operation (`last-operation`: 0 none or
//! cancel, 1 move, 2 copy, 3 link).

use leptonic::{
    CapturedElement, IntoAttrs, announce_polite, flag,
    hooks::dnd::{
        DragEndEvent, DragItem, DropOperation, UseDragInput, UseDragReturn, UseDropInput,
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

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let starts = StoredValue::new(0_u32);
    let moves = StoredValue::new(0_u32);
    let ends = StoredValue::new(0_u32);
    let drops = StoredValue::new(0_u32);
    let last_operation = StoredValue::new(0.0_f64);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("starts", move || f64::from(starts.get_value())),
        Control::read("moves", move || f64::from(moves.get_value())),
        Control::read("ends", move || f64::from(ends.get_value())),
        Control::read("drops", move || f64::from(drops.get_value())),
        Control::read("last-operation", move || last_operation.get_value()),
    ];
    Effect::new(|| announce_polite(""));

    let element = CapturedElement::new();
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        on_drop: Some(Callback::new(move |_| {
            drops.update_value(|drops| *drops += 1);
        })),
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>"use_drag performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Draggable disabled starts moves ends last_operation />
        </Show>
        <div
            id="perf-drop"
            role="button"
            tabindex="0"
            {..drop_props.into_attrs()}
            data-drop-target=flag(is_drop_target)
        >
            "Drop here"
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// The draggable element.
#[component]
fn Draggable(
    disabled: RwSignal<bool>,
    starts: StoredValue<u32>,
    moves: StoredValue<u32>,
    ends: StoredValue<u32>,
    last_operation: StoredValue<f64>,
) -> impl IntoView {
    let UseDragReturn {
        drag_props,
        is_dragging,
        ..
    } = use_drag(UseDragInput {
        items: Signal::stored(vec![DragItem::text("hello world")]),
        allowed_drop_operations: None,
        preview: None,
        on_drag_start: Some(Callback::new(move |_| {
            starts.update_value(|starts| *starts += 1);
        })),
        on_drag_move: Some(Callback::new(move |_| {
            moves.update_value(|moves| *moves += 1);
        })),
        on_drag_end: Some(Callback::new(move |e: DragEndEvent| {
            ends.update_value(|ends| *ends += 1);
            last_operation.set_value(operation_code(e.drop_operation));
        })),
        has_drag_button: false,
        is_disabled: disabled.into(),
    });
    view! {
        <div
            id="perf-draggable"
            role="button"
            tabindex="0"
            {..drag_props.into_attrs()}
            data-dragging=flag(is_dragging)
        >
            "Drag me"
        </div>
    }
}
