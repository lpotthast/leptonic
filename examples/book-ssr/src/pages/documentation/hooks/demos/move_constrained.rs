use leptonic::{
    components::prelude::Checkbox,
    hooks::*,
    utils::{
        css::{LengthPercentageAuto, computed_px},
        data_attributes::flag,
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

/// A pixel offset. Measured sizes are `NaN` before the first layout, which renders as `0px`.
fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(computed_px(px))
}

#[component]
pub fn ConstrainedBasicExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let disabled = RwSignal::new(false);
    let log = move |message: String| {
        set_events.update(|events| {
            events.push_overwrite(message);
        });
    };

    let UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        normalized_position,
        pixel_position,
        ..
    } = use_constrained_move(
        UseMoveInput {
            is_disabled: disabled.into(),
            on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
                log(format!("Start: pointer={}", e.pointer_type));
            })),
            on_move: Some(Callback::new(move |e: MoveEvent| {
                log(format!("Move: delta=({:.1}, {:.1})", e.delta_x, e.delta_y));
            })),
            on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
                log(format!("End: pointer={}", e.pointer_type));
            })),
            ..UseMoveInput::default()
        },
        MoveConstraintOptions::new(MoveConstraint::Bounds),
    );

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().x)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().y)))
        .build();

    view! {
        <div {..container_props.into_attrs()} class="demo-move-area">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                data-moving=flag(is_moving)
                style=handle_styles
            >
                "Drag me"
            </div>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>

        <p class="demo-status">
            {move || {
                let position = normalized_position.get();
                let state = if is_moving.get() { "moving" } else { "not moving" };
                format!("Position ({:.2}, {:.2}), {state}.", position.x, position.y)
            }}
        </p>

        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
