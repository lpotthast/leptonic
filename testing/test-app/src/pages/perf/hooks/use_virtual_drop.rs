//! The hook `use_virtual_drop` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_virtual_drop.rs`): `#perf-described` is described by
//! the description it returns (how to drop, during keyboard and screen reader drags), mounted at
//! first. A `use_drag` source (`#perf-draggable`) starts drags, a `use_drop` target
//! (`#perf-drop`) takes their drops, after a plain button (`#perf-before`) to tab from. The page
//! announces an empty message once mounted, so that the live announcer (which keyboard drags
//! announce through) exists before the test measures. Its controls ([`Control`]) mount and
//! unmount it.

use leptonic::{
    CapturedElement, announce_polite, flag,
    hooks::dnd::{
        DragItem, UseDragInput, UseDragReturn, UseDropInput, UseDropReturn, use_drag, use_drop,
        use_virtual_drop,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    Effect::new(|| announce_polite(""));
    let UseDragReturn {
        draggable: drag_props,
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
    let element = CapturedElement::new();
    let UseDropReturn {
        drop_target: drop_props,
        ..
    } = use_drop(UseDropInput {
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        on_drop: None,
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>"use_virtual_drop performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div
            id="perf-draggable"
            role="button"
            tabindex="0"
            {..drag_props.attrs()}
            data-dragging=flag(is_dragging)
        >
            "Drag me"
        </div>
        <div id="perf-drop" role="button" tabindex="0" {..drop_props.attrs()}>
            "Drop here"
        </div>
        <Show when=move || mounted.get()>
            <Described />
        </Show>
    }
}

/// The element the description describes.
#[component]
fn Described() -> impl IntoView {
    let description = use_virtual_drop();
    view! {
        <div id="perf-described" role="button" aria-describedby=move || description.get()>
            "Described"
        </div>
    }
}
