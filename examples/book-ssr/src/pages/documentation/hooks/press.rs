use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

#[derive(Clone)]
pub enum Event {
    MoveStart(MoveStartEvent),
    Move(MoveEvent),
    MoveEnd(MoveEndEvent),
}

#[component]
pub fn PageUsePress() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = signal(false);

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push_str("\n");
            }
            result
        })
    });

    let UsePressReturn { attrs, is_pressed } = use_press(UsePressInput {
        disabled: disabled.into(),
        force_prevent_default: false,
        allow_propagation: false,
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
            <h1 id="use_press" class="anchor">
                "use_press"
                <AnchorLink href="#use_press" description="Direct link to article header"/>
            </h1>

            <p>"Track element press."</p>

            <Code>
                "..."
            </Code>

            <button {..attrs}>
                "Press me"
            </button>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <p>"Is pressed: " { move || is_pressed.get() }</p>
            <p>"Was pressed: " { move || count.get() } { move || match count.get() {
                1 => " time",
                _ => " times",
            } }</p>

            <p>"Last " { move || events.with(|events| events.occupied_len()) } " events: "</p>

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
