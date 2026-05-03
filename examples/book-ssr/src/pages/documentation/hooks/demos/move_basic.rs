use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::{html, prelude::*};
use leptos_classes::Classes;
use leptos_use::use_element_bounding;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn BasicMovementExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (left, set_left) = signal(0.0);
    let (top, set_top) = signal(0.0);

    let container: NodeRef<html::Div> = NodeRef::new();
    let container_bounding = use_element_bounding(container);

    let draggable: NodeRef<html::Div> = NodeRef::new();
    let draggable_bounding = use_element_bounding(draggable);

    let UseMoveReturn { props, .. } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: None.into(),
        is_rtl: false,
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "MoveStart {{ pointer: {}, page: ({}, {}) }}",
                    e.pointer_type, e.page_x, e.page_y
                )));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_left.update(move |l| *l += e.delta_x);
            set_top.update(move |l| *l += e.delta_y);
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move {{ dx: {}, dy: {}, pointer: {} }}",
                    e.delta_x, e.delta_y, e.pointer_type
                )));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            set_left.update(move |l| {
                *l = (*l).clamp(
                    0.0,
                    container_bounding.width.get_untracked()
                        - draggable_bounding.width.get_untracked(),
                );
            });
            set_top.update(move |t| {
                *t = (*t).clamp(
                    0.0,
                    container_bounding.height.get_untracked()
                        - draggable_bounding.height.get_untracked(),
                );
            });
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "MoveEnd {{ pointer: {} }}",
                    e.pointer_type
                )));
            });
        })),
        on_position_change: None,
        constraint: None,
        allow_container_click: false,
        initial_position: None,
    });

    let string = Memo::new(move |_| {
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
        <Code language=Language::Rust>
            {indoc!(r#"
                let UseMoveReturn { props, is_moving, .. } = use_move(UseMoveInput {
                    disabled: false.into(),
                    axis: None.into(),
                    is_rtl: false,
                    on_move_start: Some(Callback::new(|e: MoveStartEvent| { .. })),
                    on_move: Some(Callback::new(|e: MoveEvent| {
                        // e.delta_x, e.delta_y for position updates
                    })),
                    on_move_end: Some(Callback::new(|e: MoveEndEvent| { .. })),
                    on_position_change: None,
                    constraint: None,
                    allow_container_click: false,
                    initial_position: None,
                });

                view! {
                    <div {..props.into_attrs()} tabindex="0">
                        "Drag me"
                    </div>
                }
            "#)}
        </Code>

        // The `touch-action: none` is important. Browsers would otherwise interrupt touchmove events after a small delay!
        <div node_ref=container style="
            width: 100%;
            height: 10em;
            touch-action: none;
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            <div
                {..props.into_attrs()}
                node_ref=draggable
                tabindex="0"
                style=move || "
                    border: 0.1em solid green;
                    padding: 0.5em 1em;
                    transition: none;
                    position: relative;
                    width: fit-content;
                    cursor: pointer;
                ".to_string()
                style:left=move || format!("{}px", left.get().clamp(
                    0.0,
                    container_bounding.width.get_untracked()
                        - draggable_bounding.width.get_untracked(),
                ))
                style:top=move || format!("{}px", top.get().clamp(
                    0.0,
                    container_bounding.height.get_untracked()
                        - draggable_bounding.height.get_untracked(),
                ))
            >
                "Drag me (or use arrow keys)"
            </div>
        </div>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events: "</p>

        <pre class=Classes::from("demo-event-log")>
            { move || string.get() }
        </pre>
    }
}
