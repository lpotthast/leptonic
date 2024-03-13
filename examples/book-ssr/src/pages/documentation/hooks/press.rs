use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::prelude::*;
use leptonic::hooks::r#move::{MoveEndEvent, MoveEvent, MoveStartEvent};
use leptos::*;
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
        force_prevent_default: false,
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
        <Article>
            <H1 id="use_press" class="anchor">
                "use_press"
                <AnchorLink href="#use_press" description="Direct link to article header"/>
            </H1>

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

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <P>"Is pressed: " { move || press.is_pressed.get() }</P>
            <P>"Was pressed: " { move || count.get() } { move || match count.get() {
                1 => " time",
                _ => " times",
            } }</P>

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
                Toc::Leaf { title: "use_press", link: "#use-press" },
            ]
        }/>
    }
}
