use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::*;
use ringbuf::{HeapRb, Rb};

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseHover() -> impl IntoView {
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

    let hover = use_hover(
        UseHoverInput::builder()
            .disabled(disabled)
            .on_hover_start(move |e: HoverStartEvent| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("{e:?}")));
                });
            })
            .on_hover_end(move |e: HoverEndEvent| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("{e:?}")));
                });
            })
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-hover" class="anchor">
                "use_hover"
                <AnchorLink href="#use-hover" description="Direct link to section: use_hover"/>
            </H1>

            <P>"Track element hover."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
            </Code>

            <div
                {..hover.props.attrs}
                {..hover.props.handlers}
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

            <P>"Is hovered: " { move || hover.is_hovered.get() }</P>

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
                Toc::Leaf { title: "use_hover", link: "#use-hover" },
            ]
        }/>
    }
}
