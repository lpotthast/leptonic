use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use leptos_classes::Classes;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn HoverDemo() -> impl IntoView {
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
        <div
            {..props.into_attrs()}
            class=Classes::from("demo-btn")
        >
            "Hover me"
        </div>

        <p>
            "Is hovered: "
            <strong class=Classes::builder().with_toggle(is_hovered, "demo-state-active", "demo-state-inactive").build()>
                { move || is_hovered.get() }
            </strong>
        </p>

        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events:"</p>

        <pre class=Classes::from("demo-event-log")>
            { move || string.get() }
        </pre>
    }
}
