use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

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
                result.push('\n');
            }
            result
        })
    });

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        disabled: disabled.into(),
        force_prevent_default: false,
        allow_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
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
        on_press_change: None,
    });

    view! {
        <Article>
            <h1 id="use_press" class="anchor">
                "use_press"
                <AnchorLink href="#use_press" description="Direct link to article header"/>
            </h1>

            <p>
                "Normalizes press interactions across mouse, touch, keyboard, and screen readers. "
                "Provides consistent press event handling with support for press start, end, up, "
                "and completion events, along with pointer type detection."
            </p>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to example"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
                        disabled: disabled.into(),
                        force_prevent_default: false,
                        allow_propagation: false,
                        allow_text_selection_on_press: false,
                        should_cancel_on_pointer_exit: false,
                        on_press: Callback::new(move |e| { /* ... */ }),
                        on_press_up: None,
                        on_press_start: None,
                        on_press_end: None,
                        on_press_change: None,
                    });

                    view! {
                        <button {..props.into_attrs()}>
                            "Press me"
                        </button>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Try interacting with the button below using mouse, touch, or keyboard (Tab to focus, Enter/Space to press)."</p>

            <button {..props.into_attrs()}>
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

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Normalizes press events across mouse, touch, keyboard, and screen readers"</li>
                <li>"Handles press start/end/up lifecycle with pointer type detection"</li>
                <li>"Supports drag-out and drag-back-in behavior during press"</li>
                <li>"Virtual click detection for screen reader accessibility"</li>
                <li>"Text selection prevention during press interactions"</li>
                <li>"Safari drag cancellation workaround"</li>
                <li>"iOS pointer capture release for correct touch events"</li>
                <li>"macOS Meta key workaround for stuck key states"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_press", link: "#use_press" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
