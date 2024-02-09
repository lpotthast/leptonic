use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::hooks::prelude::*;
use leptonic::hooks::r#move::{MoveEndEvent, MoveEvent, MoveStartEvent};
use leptos::*;

#[derive(Clone)]
pub enum Event {
    MoveStart(MoveStartEvent),
    Move(MoveEvent),
    MoveEnd(MoveEndEvent),
}

#[component]
pub fn PagePress() -> impl IntoView {
    let press = use_press(UsePressInput {
        on_press: Callback::new(move |_e| {}),
        on_press_up: Some(Callback::new(move |_e| {})),
        on_press_start: Some(Callback::new(move |_e| {})),
        on_press_end: Some(Callback::new(move |_e| {})),
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
    }
}
