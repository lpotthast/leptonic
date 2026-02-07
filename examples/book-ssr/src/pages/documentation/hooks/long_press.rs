use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

#[component]
pub fn PageUseLongPress() -> impl IntoView {
    let (long_press_count, set_long_press_count) = signal(0);
    let (events, set_events) = signal(HeapRb::<String>::new(20));
    let (disabled, set_disabled) = signal(false);
    let (threshold, set_threshold) = signal(500u64);

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

    let UseLongPressReturn { props } = use_long_press(UseLongPressInput {
        disabled: disabled.into(),
        threshold: Some(threshold.get_untracked()),
        on_long_press_start: Some(Callback::new(move |e: LongPressEvent| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        on_long_press: Some(Callback::new(move |e: LongPressEvent| {
            set_long_press_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        on_long_press_end: Some(Callback::new(move |e: LongPressEvent| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        accessibility_description: Some("Long press to increment counter"),
    });

    view! {
        <Article>
            <h1 id="use_long_press" class="anchor">
                "use_long_press"
                <AnchorLink href="#use_long_press" description="Direct link to article header"/>
            </h1>

            <p>"Detect long press interactions across mouse and touch devices with customizable threshold."</p>

            <Code>
                {r#"let UseLongPressReturn { props } = use_long_press(UseLongPressInput {
    disabled: Signal::derive(|| false),
    threshold: Some(500), // ms
    on_long_press_start: Some(Callback::new(|e| { /* ... */ })),
    on_long_press: Some(Callback::new(|e| { /* ... */ })),
    on_long_press_end: Some(Callback::new(|e| { /* ... */ })),
    accessibility_description: Some("Long press to open menu"),
});

view! {
    <button {..props.into_attrs()}>
        "Long press me"
    </button>
}"#}
            </Code>

            <p>"Press and hold the button below for " { move || threshold.get() } "ms to trigger a long press:"</p>

            <button
                {..props.into_attrs()}
                style="
                    display: inline-flex;
                    border: 2px solid var(--brand-color);
                    padding: 1em 2em;
                    cursor: pointer;
                    border-radius: 4px;
                    background: var(--brand-color);
                    color: white;
                    font-size: 1em;
                "
            >
                "Long press me"
            </button>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 1em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 0.5em;">
                <Label>"Threshold: " { move || threshold.get() } "ms"</Label>
                <input
                    type="range"
                    min="100"
                    max="2000"
                    step="100"
                    prop:value=move || threshold.get().to_string()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<u64>() {
                            set_threshold.set(val);
                        }
                    }
                    style="width: 200px;"
                />
            </FormControl>

            <p>"Long press count: " { move || long_press_count.get() }</p>

            <p>"Last " { move || events.with(|events| events.occupied_len()) } " events:"</p>

            <pre style="
                width: 100%;
                height: 10em;
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
                <li>"Customizable time threshold (default 500ms)"</li>
                <li>"Handles both mouse and touch events"</li>
                <li>"Prevents context menu on touch devices during long press"</li>
                <li>"Three event callbacks: start, threshold met, and end"</li>
                <li>"Accessibility via " <code>"aria-describedby"</code></li>
                <li>"Automatic focus management when threshold is met"</li>
                <li>"Cancels sibling press handlers via synthetic " <code>"pointercancel"</code></li>
            </ul>

            <h2 id="how-it-works" class="anchor">
                "How it works"
                <AnchorLink href="#how-it-works" description="Direct link to how it works"/>
            </h2>

            <p>
                "When the user presses down on the target element, a timer starts. If the user "
                "holds the press for the configured threshold duration (default 500ms), the following "
                "sequence occurs:"
            </p>

            <ol>
                <li>"A synthetic " <code>"pointercancel"</code> " event is dispatched on the target, "
                    "which cancels any sibling " <code>"use_press"</code> " handlers."</li>
                <li>"The " <code>"on_long_press_end"</code> " callback fires (triggered by the cancellation)."</li>
                <li>"The target element is focused without scrolling."</li>
                <li>"The " <code>"on_long_press"</code> " callback fires."</li>
            </ol>

            <p>
                "If the user releases before the threshold, the timer is cancelled and only "
                <code>"on_long_press_start"</code> " and " <code>"on_long_press_end"</code> " fire. "
                "On touch devices, the native context menu is automatically prevented during the interaction."
            </p>

            <h2 id="relationship-to-use-press" class="anchor">
                "Relationship to use_press"
                <AnchorLink href="#relationship-to-use-press" description="Direct link to relationship to use_press"/>
            </h2>

            <p>
                <code>"use_long_press"</code> " wraps " <code>"use_press"</code> " internally. It listens for "
                "press start and end events to manage the long press timer. When the threshold is met, "
                "it dispatches a synthetic " <code>"pointercancel"</code> " event that cancels the underlying "
                "press interaction. This ensures that sibling " <code>"use_press"</code> " handlers on the "
                "same element do not fire their " <code>"on_press"</code> " callback for an interaction "
                "that was actually a long press."
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_long_press", link: "#use_long_press" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "How it works", link: "#how-it-works" },
                Toc::Leaf { title: "Relationship to use_press", link: "#relationship-to-use-press" },
            ]
        }/>
    }
}
