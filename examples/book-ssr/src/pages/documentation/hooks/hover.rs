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
pub fn PageHover() -> impl IntoView {
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

    let press = use_hover(UseHoverInput {
        disabled: disabled.into(),
        on_hover_start: Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("HoverStart: {e:?}")));
            });
        }),
        on_hover_end: Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("HoverEnd: {e:?}")));
            });
        }),
    });

    view! {
        <H1>"use_hover"</H1>

        <P>"Track element hover."</P>

        <Code>
            {indoc!(r"
                ...
            ")}
        </Code>

        <div
            {..press.props.attrs}
            on:pointerenter=press.props.on_pointer_enter
            on:pointerleave=press.props.on_pointer_leave
            style="display: inline-flex;
            border: 0.1em solid green;
            padding: 0.5em 1em;"
        >
            "Hover me"
        </div>

        <P>"Is disabled: " { move || disabled.get() } <Checkbox checked=disabled set_checked=set_disabled /></P>
        <P>"Is hovered: " { move || press.is_hovered.get() }</P>

        <P>"Last " { move || events.with(|events| events.len()) } " events: "</P>

        <pre style="background-color: grey; width: 100%; height: 15em; border: 0.1em solid darkgrey; overflow: auto;">
            { move || string.get() }
        </pre>
    }
}
