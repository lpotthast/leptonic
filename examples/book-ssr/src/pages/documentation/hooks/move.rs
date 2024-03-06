use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::contexts::global_pointer_event::{
    GlobalPointerCancelEvent, GlobalPointerDownEvent, GlobalPointerMoveEvent, GlobalPointerUpEvent,
};
use leptonic::hooks::prelude::*;
use leptonic::hooks::r#move::{MoveEndEvent, MoveEvent, MoveStartEvent};
use leptos::*;
use leptos_use::use_element_bounding;
use ringbuf::{HeapRb, Rb};

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[derive(Clone)]
pub enum Event {
    MoveStart(MoveStartEvent),
    Move(MoveEvent),
    MoveEnd(MoveEndEvent),
}

#[component]
pub fn PageMove() -> impl IntoView {
    let (events, set_events) = create_signal(HeapRb::<Oco<'static, str>>::new(50));
    let (left, set_left) = create_signal(0.0);
    let (top, set_top) = create_signal(0.0);

    let global_pointer_up = expect_context::<GlobalPointerUpEvent>().read_signal;
    let global_pointer_down = expect_context::<GlobalPointerDownEvent>().read_signal;
    let global_pointer_cancel = expect_context::<GlobalPointerCancelEvent>().read_signal;
    let global_pointer_move = expect_context::<GlobalPointerMoveEvent>().read_signal;

    let container: NodeRef<html::Div> = create_node_ref();
    let container_bounding = use_element_bounding(container);

    let draggable: NodeRef<html::Div> = create_node_ref();
    let draggable_bounding = use_element_bounding(draggable);

    let mov: UseMoveReturn = use_move(UseMoveInput {
        on_move_start: Callback::new(move |_e| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Borrowed("MoveStart"));
            });
        }),
        on_move: Callback::new(move |e: MoveEvent| {
            set_left.update(move |l| *l += e.delta_x);
            set_top.update(move |l| *l += e.delta_y);
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move {{ dx: {}, dy: {} }}",
                    e.delta_x, e.delta_y
                )));
            });
        }),
        on_move_end: Callback::new(move |_e| {
            set_left.update(move |l| {
                *l = (*l).clamp(
                    0.0,
                    container_bounding.width.get_untracked()
                        - draggable_bounding.width.get_untracked(),
                )
            });
            set_top.update(move |t| {
                *t = (*t).clamp(
                    0.0,
                    container_bounding.height.get_untracked()
                        - draggable_bounding.height.get_untracked(),
                )
            });
            set_events.update(move |events| {
                events.push_overwrite(Oco::Borrowed("MoveEnd"));
            });
        }),
        global_pointer_up: global_pointer_up.into(),
        global_pointer_down: global_pointer_down.into(),
        global_pointer_cancel: global_pointer_cancel.into(),
        global_pointer_move: global_pointer_move.into(),
    });

    let string = create_memo(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push_str("\n");
            }
            result
        })
    });

    view! {
        <Article>
            <H1>"use_move"</H1>

            <P>"Track movement."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
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
                    {..mov.props.attrs}
                    node_ref=draggable
                    on:pointerdown=mov.props.on_pointer_down
                    style=move || format!("
                        border: 0.1em solid green;
                        padding: 0.5em 1em;
                        transition: none;
                        position: relative;
                        width: fit-content;
                        cursor: pointer;
                    ")
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
                    "Drag me"
                </div>
            </div>

            <P>"Last " { move || events.with(|events| events.len()) } " events: "</P>

            <pre style="
                width: 100%;
                height: 15em;
                overflow: auto;
                padding: var(--typography-code-padding);
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
            ">
                { move || string.get() }
            </pre>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Need help?", link: "#help" },
                Toc::Leaf { title: "Contribute", link: "#contribute" },
            ]
        }/>
    }
}
