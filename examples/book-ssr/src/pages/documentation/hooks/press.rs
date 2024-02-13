use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::hooks::prelude::*;
use leptonic::hooks::r#move::{MoveEndEvent, MoveEvent, MoveStartEvent};
use leptos::*;
use ringbuf::{HeapRb, Rb};

#[derive(Clone)]
pub enum Event {
    MoveStart(MoveStartEvent),
    Move(MoveEvent),
    MoveEnd(MoveEndEvent),
}

#[component]
pub fn PagePress() -> impl IntoView {
    let (events, set_events) = create_signal(HeapRb::<Oco<'static, str>>::new(50));

    let press = use_press(UsePressInput {
        on_press: Callback::new(move |_e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Press"));
            });
        }),
        on_press_up: Some(Callback::new(move |_e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("PressUp"));
            });
        })),
        on_press_start: Some(Callback::new(move |_e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("PressStart"));
            });
        })),
        on_press_end: Some(Callback::new(move |_e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("PressEnd"));
            });
        })),
    });

    view! {
        <H1>"useMove"</H1>

        <P>"Track element press."</P>

        <Code>
            {indoc!(r"
                ...
            ")}
        </Code>

        <button
            {..press.props.attrs}
            on:keydown=press.props.on_key_down
            on:click=press.props.on_click
            on:touchstart=press.props.on_touch_start
            on:touchmove=press.props.on_touch_move
            on:touchend=press.props.on_touch_end
        >
            "Press me"
        </button>

        <P>"Is pressed: " { move || press.is_pressed.get() }</P>

        <P>"Last " { move || events.with(|events| events.len()) } " events: "</P>

        <pre style="background-color: grey; width: 100%; height: 15em; border: 0.1em solid darkgrey; overflow: auto;">
            { move || events.with(|events| events.iter().rev().map(|e| view! { <div>{e.clone()}</div> }).collect_view()) }
        </pre>
    }
}
