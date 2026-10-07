use std::collections::VecDeque;

use leptonic::{
    atoms::prelude::*,
    hooks::PressEvent,
};
use leptos::prelude::*;

/// A `PressResponder` injects an `on_press` handler and a disabled state into the `Pressable` inside it.
#[component]
pub fn PressResponderDemo() -> impl IntoView {
    let (events, set_events) = signal(VecDeque::<&'static str>::new());
    let disabled = RwSignal::new(false);

    let parent_on_press = Callback::new(move |_: PressEvent| {
        set_events.update(|events| {
            events.push_front("Parent on_press (from the PressResponder)");
            events.truncate(50);
        });
    });
    let child_on_press = Callback::new(move |_: PressEvent| {
        set_events.update(|events| {
            events.push_front("Child on_press (from the Pressable)");
            events.truncate(50);
        });
    });

    view! {
        <PressResponder on_press=parent_on_press is_disabled=disabled>
            <Pressable on_press=child_on_press>
                // The responder only stops the presses: tell assistive technology (and CSS) yourself.
                <button class="demo-press-button" aria-disabled=move || disabled.get().then_some("true")>
                    "Press me"
                </button>
            </Pressable>
        </PressResponder>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
            <Button on_press=move |_| set_events.update(|events| { events.clear(); }) classes="demo-btn">
                "Clear log"
            </Button>
        </div>

        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().copied().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
