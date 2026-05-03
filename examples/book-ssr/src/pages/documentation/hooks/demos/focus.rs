use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use leptos_classes::Classes;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn FocusDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = signal(false);
    let (is_focused, set_is_focused) = signal(false);

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
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!("Changed: {focused}")));
            });
        })),
    });

    view! {
        <div
            tabindex=0
            {..props.into_attrs()}
            class=Classes::builder().with_toggle(is_focused.get(), "demo-container-active", "demo-container-inactive").build()
        >
            <strong class=Classes::builder().with_toggle(is_focused.get(), "demo-state-active", "demo-state-inactive").build()>
                { move || if is_focused.get() { "Focused" } else { "Not focused" } }
            </strong>
            " \u{2014} click here or press Tab"
        </div>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events: "</p>

        <pre class=Classes::from("demo-event-log")>
            { move || string.get() }
        </pre>

        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>
    }
}
