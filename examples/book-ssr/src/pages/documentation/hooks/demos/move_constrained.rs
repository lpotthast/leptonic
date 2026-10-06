use leptonic::{
    hooks::*,
    utils::{
        css::{CssDimension, LengthPercentageAuto, try_px},
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(try_px(px).unwrap_or(CssDimension::Zero))
}

#[component]
pub fn ConstrainedBasicExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let log = move |message: String| {
        set_events.update(|events| {
            events.push_overwrite(message);
        });
    };

    let UseMoveReturn {
        props,
        is_moving,
        constraint,
    } = use_move(UseMoveInput {
        is_disabled: false.into(),
        axis: None.into(),
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            log(format!("Start: pointer={}", e.pointer_type));
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            log(format!("Move: delta=({:.1}, {:.1})", e.delta_x, e.delta_y));
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            log(format!("End: pointer={}", e.pointer_type));
        })),
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: false,
        initial_position: None,
    });
    let constraint = constraint.expect("`constraint` is set, so the constraint return is present");
    let normalized_position = constraint.normalized_position;
    let pixel_position = constraint.pixel_position;

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().0)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().1)))
        .build();

    view! {
        <div {..constraint.container_props.into_attrs()} class="demo-move-area">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                class:moving=move || is_moving.get()
                style=handle_styles
            >
                "Drag me"
            </div>
        </div>

        <p>
            "Position: ("
            {move || format!("{:.2}", normalized_position.get().x)}
            ", "
            {move || format!("{:.2}", normalized_position.get().y)}
            ") | Moving: "
            {move || if is_moving.get() { "yes" } else { "no" }}
        </p>

        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
