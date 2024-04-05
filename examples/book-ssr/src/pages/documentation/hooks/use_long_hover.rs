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
pub fn PageUseLongHover() -> impl IntoView {
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

    let long_hover = use_long_hover(
        UseLongHoverInput::builder()
            .disabled(disabled)
            .on_long_hover_start(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("HoverStart: {e:?}")));
                });
            })
            .on_long_hover_end(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("HoverEnd: {e:?}")));
                });
            })
            .threshold(Duration::from_millis(500))
            .accessibility_description(Oco::Borrowed("Long hover to show tooltip."))
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-long-hover" class="anchor">
                "use_long_hover"
                <AnchorLink href="#use-hover" description="Direct link to section: use_long_hover"/>
            </H1>

            <P>"Track element long hover."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
            </Code>

            <div
                {..long_hover.props.attrs}
                {..long_hover.props.handlers}
                style="display: inline-flex;
                border: 0.1em solid green;
                padding: 0.5em 1em;"
            >
                "Hover me"
            </div>

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <P>"Is hovered: " { move || long_hover.is_hovered.get() }</P>

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
                Toc::Leaf { title: "use_long_hover", link: "#use-long-hover" },
            ]
        }/>
    }
}
