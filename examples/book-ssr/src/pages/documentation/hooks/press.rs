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
    let (count, set_count) = create_signal(0);
    let (events, set_events) = create_signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = create_signal(false);

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

    let press = use_press(UsePressInput {
        disabled: disabled.into(),
        on_press: Callback::new(move |e| {
            set_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Press: {e:?}")));
            });
        }),
        on_press_up: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("PressUp: {e:?}")));
            });
        })),
        on_press_start: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("PressStart: {e:?}")));
            });
        })),
        on_press_end: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("PressEnd: {e:?}")));
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
            on:pointerdown=press.props.on_pointer_down
        >
            "Press me"
        </button>

        <P>"Is disabled: " { move || disabled.get() } <Checkbox checked=disabled set_checked=set_disabled /></P>
        <P>"Is pressed: " { move || press.is_pressed.get() }</P>
        <P>"Was pressed: " { move || count.get() } { move || match count.get() {
            1 => " time",
            _ => " times",
        } }</P>

        <P>"Last " { move || events.with(|events| events.len()) } " events: "</P>

        <pre style="background-color: grey; width: 100%; height: 15em; border: 0.1em solid darkgrey; overflow: auto;">
            { move || string.get() }
        </pre>
    }
}
