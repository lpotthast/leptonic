use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn FocusDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let (is_focused, set_is_focused) = signal(false);
    let (events, set_events) = signal(HeapRb::<String>::new(50));

    let log = move |entry: String| {
        set_events.update(|events| {
            events.push_overwrite(entry);
        });
    };

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        is_disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| log("on_focus".to_string()))),
        on_blur: Some(Callback::new(move |_| log("on_blur".to_string()))),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
            log(format!("on_focus_change: {focused}"));
        })),
    });

    view! {
        <label class="demo-focus-label">
            "Name "
            <input type="text" class="demo-focus-item" {..props.into_attrs()}/>
        </label>

        <p class="demo-status">
            {move || if is_focused.get() { "The field has focus." } else { "The field doesn\u{2019}t have focus." }}
        </p>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
