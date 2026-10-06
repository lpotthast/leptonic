use leptonic::{
    hooks::*,
    utils::{
        css::{CssDimension, LengthPercentageAuto, try_px},
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::{html, prelude::*};
use leptos_use::use_element_bounding;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

/// A pixel offset. Measured sizes are `NaN` before the first layout, which renders as `0px`.
fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(try_px(px).unwrap_or(CssDimension::Zero))
}

#[component]
pub fn BasicMovementExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let (left, set_left) = signal(0.0);
    let (top, set_top) = signal(0.0);

    let container: NodeRef<html::Div> = NodeRef::new();
    let container_bounding = use_element_bounding(container);

    let handle: NodeRef<html::Div> = NodeRef::new();
    let handle_bounding = use_element_bounding(handle);

    // Keep the handle inside the area. `use_move` itself is unconstrained.
    let max_left = move || (container_bounding.width.get() - handle_bounding.width.get()).max(0.0);
    let max_top = move || (container_bounding.height.get() - handle_bounding.height.get()).max(0.0);

    let UseMoveReturn {
        props, is_moving, ..
    } = use_move(UseMoveInput {
        is_disabled: false.into(),
        axis: None.into(),
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "MoveStart {{ pointer: {}, page: ({}, {}) }}",
                    e.pointer_type, e.page_x, e.page_y
                ));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_left.update(|l| *l += e.delta_x);
            set_top.update(|t| *t += e.delta_y);
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "Move {{ dx: {}, dy: {}, pointer: {} }}",
                    e.delta_x, e.delta_y, e.pointer_type
                ));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            // Drop the overshoot accumulated while dragging past the edge.
            set_left.update(|l| *l = l.clamp(0.0, max_left()));
            set_top.update(|t| *t = t.clamp(0.0, max_top()));
            set_events.update(|events| {
                events.push_overwrite(format!("MoveEnd {{ pointer: {} }}", e.pointer_type));
            });
        })),
        on_position_change: None,
        constraint: None,
        allow_container_click: false,
        initial_position: None,
    });

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(left.get().clamp(0.0, max_left()))))
        .with_reactive(move || TopProperty.declare(offset(top.get().clamp(0.0, max_top()))))
        .build();

    view! {
        <div node_ref=container class="demo-move-area">
            <div
                {..props.into_attrs()}
                node_ref=handle
                tabindex="0"
                class="demo-move-handle"
                class:moving=move || is_moving.get()
                style=handle_styles
            >
                "Drag me (or use arrow keys)"
            </div>
        </div>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
