use leptonic::{hooks::*, utils::classes::Classes};
use leptos::prelude::*;

const MIN: f64 = 0.0;
const MAX: f64 = 10.0;

#[component]
pub fn SpinButtonDemo() -> impl IntoView {
    let (cups, set_cups) = signal(2.0_f64);
    let step = move |delta: f64| set_cups.update(|v| *v = (*v + delta).clamp(MIN, MAX));

    let UseSpinButtonReturn {
        props,
        increment_button,
        decrement_button,
    } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || Some(cups.get())),
        text_value: Signal::derive(move || Some(format!("{} cups", cups.get()))),
        min_value: Signal::stored(Some(MIN)),
        max_value: Signal::stored(Some(MAX)),
        on_increment: Some(Callback::new(move |()| step(1.0))),
        on_decrement: Some(Callback::new(move |()| step(-1.0))),
        on_increment_page: Some(Callback::new(move |()| step(5.0))),
        on_decrement_page: Some(Callback::new(move |()| step(-5.0))),
        on_increment_to_max: Some(Callback::new(move |()| set_cups.set(MAX))),
        on_decrement_to_min: Some(Callback::new(move |()| set_cups.set(MIN))),
        ..Default::default()
    });

    // The stepper buttons come as `UseButtonInput`s. Add a label and a disabled state, then render
    // them with `use_button`. They stay focusable while disabled, so holding "+" until the
    // maximum doesn't lose focus.
    let (decrement_attrs, decrement_styles) = use_button(UseButtonInput {
        aria_label: Some("Fewer cups".into()),
        disabled: Signal::derive(move || cups.get() <= MIN),
        allow_focus_when_disabled: true,
        ..decrement_button
    })
    .props
    .into_parts();
    let (increment_attrs, increment_styles) = use_button(UseButtonInput {
        aria_label: Some("More cups".into()),
        disabled: Signal::derive(move || cups.get() >= MAX),
        allow_focus_when_disabled: true,
        ..increment_button
    })
    .props
    .into_parts();

    view! {
        <div class=Classes::from("demo-flex-center-row")>
            <button {..decrement_attrs} style=decrement_styles class="demo-stepper-btn">
                "\u{2212}"
            </button>
            <div
                {..props.into_attrs()}
                tabindex="0"
                aria-label="Cups of coffee"
                class=Classes::from("demo-spinbutton")
            >
                {move || cups.get()}
            </div>
            <button {..increment_attrs} style=increment_styles class="demo-stepper-btn">
                "+"
            </button>
        </div>
        <p class=Classes::from("demo-caption")>
            "Focus the number and use \u{2191} / \u{2193}, Page Up / Page Down, Home and End, or hold a button."
        </p>
    }
}
