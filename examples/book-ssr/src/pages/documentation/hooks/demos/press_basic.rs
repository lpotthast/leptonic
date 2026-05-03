use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn PressBasicDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (dbl_count, set_dbl_count) = signal(0);
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = signal(false);
    let (press_state, set_press_state) = signal(false);

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
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |e: PressEvent| {
            set_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Press: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        }),
        on_press_up: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressUp: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_change: Some(Callback::new(move |pressed: bool| {
            set_press_state.set(pressed);
        })),
        on_double_press: Some(Callback::new(move |e: PressEvent| {
            set_dbl_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "DoublePress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let (press_props, press_styles) = props.into_inner();

    view! {
        <p>"Try interacting with the button below using mouse, touch, or keyboard (Tab to focus, Enter/Space to press)."</p>

        <button
            {..press_props.into_attrs()}
            style=press_styles
            style:background=move || if is_pressed.get() { "var(--brand-color)" } else { "" }
            style:color=move || if is_pressed.get() { "white" } else { "" }
            style:transform=move || if is_pressed.get() { "scale(0.97)" } else { "" }
            style:transition="background 0.1s, color 0.1s, transform 0.1s"
        >
            "Press me"
        </button>

        <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>

        <p>"Is pressed: " { move || is_pressed.get() }</p>
        <p>"on_press_change: " { move || press_state.get() }</p>
        <p>"Was pressed: " { move || count.get() } { move || match count.get() {
            1 => " time",
            _ => " times",
        } }</p>
        <p>"Was double-pressed: " { move || dbl_count.get() } { move || match dbl_count.get() {
            1 => " time",
            _ => " times",
        } }</p>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events: "</p>

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
    }
}
