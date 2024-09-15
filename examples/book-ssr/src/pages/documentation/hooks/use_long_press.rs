use std::time::Duration;

use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::*;
use ringbuf::{HeapRb, Rb};

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseLongPress() -> impl IntoView {
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

    let long_press = use_long_press(
        UseLongPressInput::builder()
            .disabled(disabled)
            .on_long_press_start(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("LongPressStart: {e:?}")));
                });
            })
            .on_long_press_end(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("LongPressEnd: {e:?}")));
                });
            })
            .on_long_press(move |e| {
                set_count.update(|c| *c += 1);
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("LongPress: {e:?}")));
                });
            })
            .threshold(Duration::from_millis(500))
            .accessibility_description(Oco::Borrowed("A button to press"))
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-long-press" class="anchor">
                "use_long_press"
                <AnchorLink href="#use-long-press" description="Direct link to article header"/>
            </H1>

            <P>"Track element long press."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
            </Code>

            <button
                {..long_press.props.attrs}
                {..long_press.props.handlers}
            >
                "Press me for 500ms"
            </button>

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

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
                Toc::Leaf { title: "use_long_press", link: "#use-long-press" },
            ]
        }/>
    }
}
