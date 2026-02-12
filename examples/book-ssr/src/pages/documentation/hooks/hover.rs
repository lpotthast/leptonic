use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};

use crate::pages::documentation::{article::Article, toc::Toc};

#[derive(Clone)]
pub enum Event {
    MoveStart(MoveStartEvent),
    Move(MoveEvent),
    MoveEnd(MoveEndEvent),
}

#[component]
pub fn PageUseHover() -> impl IntoView {
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

    let UseHoverReturn { props, is_hovered } = use_hover(UseHoverInput {
        disabled: disabled.into(),
        on_hover_start: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("HoverStart: {e:?}")));
            });
        })),
        on_hover_end: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("HoverEnd: {e:?}")));
            });
        })),
        on_hover_change: None,
    });

    view! {
        <Article>
            <h1 id="use-hover" class="anchor">
                "use_hover"
                <AnchorLink href="#use-hover" description="Direct link to section: use_hover"/>
            </h1>

            <p>"Track element hover."</p>

            <Code>
                "..."
            </Code>

            <div
                {..props.into_attrs()}
                style="display: inline-flex;
                border: 0.1em solid green;
                padding: 0.5em 1em;"
            >
                "Hover me"
            </div>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <p>"Is hovered: " { move || is_hovered.get() }</p>

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
                Toc::Leaf { title: "use_hover", link: "#use-hover" },
            ]
        }/>
    }
}
