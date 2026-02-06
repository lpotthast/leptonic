use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseFocus() -> impl IntoView {
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

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        disabled: disabled.into(),
        on_focus: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Focus: {e:?}")));
            });
        })),
        on_blur: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Blur: {e:?}")));
            });
        })),
        on_focus_change: Some(Callback::new(move |e| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Changed: {e:?}")));
            });
        })),
    });

    view! {
        <Article>
            <h1 id="use-focus" class="anchor">
                "use_focus"
                <AnchorLink href="#use-focus" description="Direct link to section: use_focus"/>
            </h1>

            <p>"Track element focus."</p>

            <Code>
                "..."
            </Code>

            <div
                tabindex=1
                {..props.into_attrs()}
                style="display: inline-flex;
                border: 0.1em solid green;
                padding: 0.5em 1em;"
            >
                "Click me or click background or tab to me and tab away"
            </div>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            //<p>"Is focused: " { move || is_focused.get() }</p>

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
                Toc::Leaf { title: "use_focus", link: "#use-focus" },
            ]
        }/>
    }
}
