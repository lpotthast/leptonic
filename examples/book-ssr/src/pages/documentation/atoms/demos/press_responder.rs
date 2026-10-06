use leptonic::{
    atoms::prelude::*,
    components::prelude::{Button, ButtonSize, ButtonVariant, Checkbox},
    hooks::PressEvent,
};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

/// A `PressResponder` injects an `on_press` handler and a disabled state into the `Pressable` inside it.
#[component]
pub fn PressResponderDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<&'static str>::new(50));
    let (disabled, set_disabled) = signal(false);

    let parent_on_press = Callback::new(move |_: PressEvent| {
        set_events.update(|events| {
            events.push_overwrite("Parent on_press (from the PressResponder)");
        });
    });
    let child_on_press = Callback::new(move |_: PressEvent| {
        set_events.update(|events| {
            events.push_overwrite("Child on_press (from the Pressable)");
        });
    });

    view! {
        <PressResponder on_press=parent_on_press is_disabled=disabled>
            <Pressable is_disabled=false on_press=child_on_press>
                <button class="demo-press-button">"Press me"</button>
            </Pressable>
        </PressResponder>

        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disable through the PressResponder"</Checkbox>

        <div class="demo-inline-controls">
            <span>"Event log"</span>
            <Button
                variant=ButtonVariant::Outlined
                size=ButtonSize::Small
                on_press=move |_| set_events.update(|events| { events.clear(); })
            >
                "Clear log"
            </Button>
        </div>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().copied().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
