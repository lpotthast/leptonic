use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn PressCancelDemo() -> impl IntoView {
    let (cancel_events, set_cancel_events) = signal(HeapRb::<Oco<'static, str>>::new(10));
    let cancel_string = Memo::new(move |_| {
        cancel_events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });
    let UsePressReturn {
        props: cancel_props,
        is_pressed: cancel_is_pressed,
    } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: true,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Press completed"));
            });
        }),
        on_press_up: None,
        on_press_start: Some(Callback::new(move |_: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("PressStart"));
            });
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressEnd: pointer_type={:?}",
                    e.pointer_type,
                )));
            });
        })),
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let (cancel_press_props, cancel_press_styles) = cancel_props.into_inner();

    view! {
        <p>
            <strong>"should_cancel_on_pointer_exit"</strong>
            " \u{2014} Press the button below, then drag the pointer outside before releasing. "
            "The press is cancelled and " <code>"on_press"</code> " does not fire."
        </p>

        <button
            {..cancel_press_props.into_attrs()}
            style=cancel_press_styles
            style:background=move || if cancel_is_pressed.get() { "var(--brand-color)" } else { "" }
            style:color=move || if cancel_is_pressed.get() { "white" } else { "" }
            style:transform=move || if cancel_is_pressed.get() { "scale(0.97)" } else { "" }
            style:transition="background 0.1s, color 0.1s, transform 0.1s"
        >
            "Drag outside to cancel"
        </button>

        <pre style="
            width: 100%;
            height: 5em;
            overflow: auto;
            padding: var(--typography-code-padding);
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            { move || cancel_string.get() }
        </pre>
    }
}
