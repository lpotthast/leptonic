use std::collections::VecDeque;

use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::*,
    utils::{
        css::{LengthPercentageAuto, computed_px},
        data_attributes::flag,
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::{html, prelude::*};
use leptos_use::use_element_bounding;

/// A pixel offset. Measured sizes are `NaN` before the first layout, which renders as `0px`.
fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(computed_px(px))
}

#[component]
pub fn BasicMovementExample() -> impl IntoView {
    let (events, set_events) = signal(VecDeque::<String>::new());
    let (left, set_left) = signal(0.0);
    let (top, set_top) = signal(0.0);
    // Shown to CSS as `data-moving`.
    let is_moving = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    let container: NodeRef<html::Div> = NodeRef::new();
    let container_bounding = use_element_bounding(container);

    let handle: NodeRef<html::Div> = NodeRef::new();
    let handle_bounding = use_element_bounding(handle);

    // Keep the handle inside the area. `use_move` itself is unconstrained.
    let max_left = move || (container_bounding.width.get() - handle_bounding.width.get()).max(0.0);
    let max_top = move || (container_bounding.height.get() - handle_bounding.height.get()).max(0.0);

    let UseMoveReturn { props } = use_move(UseMoveInput {
        is_disabled: disabled.into(),
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            is_moving.set(true);
            set_events.update(|events| {
                events.push_front(format!("MoveStart {{ pointer: {} }}", e.pointer_type));
                events.truncate(50);
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_left.update(|l| *l += e.delta_x);
            set_top.update(|t| *t += e.delta_y);
            set_events.update(|events| {
                events.push_front(format!(
                    "Move {{ dx: {}, dy: {}, pointer: {} }}",
                    e.delta_x, e.delta_y, e.pointer_type
                ));
                events.truncate(50);
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            is_moving.set(false);
            // Drop the overshoot accumulated while dragging past the edge.
            set_left.update(|l| *l = l.clamp(0.0, max_left()));
            set_top.update(|t| *t = t.clamp(0.0, max_top()));
            set_events.update(|events| {
                events.push_front(format!("MoveEnd {{ pointer: {} }}", e.pointer_type));
                events.truncate(50);
            });
        })),
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
                data-moving=flag(is_moving.into())
                style=handle_styles
            >
                "Drag me (or use arrow keys)"
            </div>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>

        <p>"Last " {move || events.with(VecDeque::len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
