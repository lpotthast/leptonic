use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn ConstrainedBasicExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));

    let UseMoveReturn {
        props,
        is_moving,
        constraint,
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        is_rtl: false,
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!("Start: pointer={}", e.pointer_type)));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move: delta=({:.1}, {:.1})",
                    e.delta_x, e.delta_y
                )));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!("End: pointer={}", e.pointer_type)));
            });
        })),
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: false,
        initial_position: None,
    });
    let c = constraint.unwrap();
    let normalized_position = c.normalized_position;
    let pixel_position = c.pixel_position;
    let container_attrs = c.container_props.into_attrs();
    let movable_attrs = props.into_attrs();

    let event_string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    view! {
        <div
            {..container_attrs}
            style="
                width: 100%;
                height: 12em;
                touch-action: none;
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
                position: relative;
            "
        >
            <div
                {..movable_attrs}
                tabindex="0"
                style=move || format!("
                    border: 0.15em solid {};
                    padding: 0.5em 1em;
                    position: absolute;
                    width: fit-content;
                    cursor: grab;
                    user-select: none;
                    left: {}px;
                    top: {}px;
                ",
                    if is_moving.get() { "var(--brand-color)" } else { "green" },
                    pixel_position.get().0,
                    pixel_position.get().1
                )
            >
                "Drag me"
            </div>
        </div>

        <p style="font-size: 0.9em;">
            "Position: ("
            { move || format!("{:.2}", normalized_position.get().x) }
            ", "
            { move || format!("{:.2}", normalized_position.get().y) }
            ") | Moving: "
            { move || if is_moving.get() { "Yes" } else { "No" } }
        </p>

        <pre class=Classes::from("demo-event-log")>
            { move || event_string.get() }
        </pre>
    }
}
