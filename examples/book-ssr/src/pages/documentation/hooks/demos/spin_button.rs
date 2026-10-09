use leptonic::{
    IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        button::{UseButtonInput, use_button},
        spinbutton::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
};
use leptos::prelude::*;

const MIN: i32 = 0;
const MAX: i32 = 10;

#[component]
pub fn SpinButtonDemo() -> impl IntoView {
    let (cups, set_cups) = signal(2_i32);
    let disabled = RwSignal::new(false);
    let step = move |delta: i32| set_cups.update(|v| *v = (*v + delta).clamp(MIN, MAX));
    let cups_text = move || match cups.get() {
        1 => "1 cup".to_owned(),
        n => format!("{n} cups"),
    };

    let UseSpinButtonReturn {
        props,
        increment_button,
        decrement_button,
    } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || Some(cups.get())),
        text_value: Signal::derive(move || Some(cups_text())),
        min_value: Signal::stored(Some(MIN)),
        max_value: Signal::stored(Some(MAX)),
        is_disabled: disabled.into(),
        on_increment: Some(Callback::new(move |()| step(1))),
        on_decrement: Some(Callback::new(move |()| step(-1))),
        on_increment_page: Some(Callback::new(move |()| step(5))),
        on_decrement_page: Some(Callback::new(move |()| step(-5))),
        on_increment_to_max: Some(Callback::new(move |()| set_cups.set(MAX))),
        on_decrement_to_min: Some(Callback::new(move |()| set_cups.set(MIN))),
        ..Default::default()
    });

    // The stepper buttons come as `UseButtonInput`s, disabled with the spin button. Add a label and disable them at
    // the limits too, then render them with `use_button`. They stay focusable while disabled, so holding "+" until the
    // maximum doesn't lose focus.
    let decrement_disabled = decrement_button.is_disabled;
    let (decrement_attrs, decrement_styles) = use_button(UseButtonInput {
        aria_label: "Fewer cups".into(),
        is_disabled: Signal::derive(move || decrement_disabled.get() || cups.get() <= MIN),
        allow_focus_when_disabled: true.into(),
        ..decrement_button
    })
    .props
    .into_parts();
    let increment_disabled = increment_button.is_disabled;
    let (increment_attrs, increment_styles) = use_button(UseButtonInput {
        aria_label: "More cups".into(),
        is_disabled: Signal::derive(move || increment_disabled.get() || cups.get() >= MAX),
        allow_focus_when_disabled: true.into(),
        ..increment_button
    })
    .props
    .into_parts();

    view! {
        <div class="demo-flex-center-row">
            <button {..decrement_attrs} style=decrement_styles class="demo-btn demo-spin-stepper">
                <span aria-hidden="true">"\u{2212}"</span>
            </button>
            <div
                {..props.into_attrs()}
                tabindex=move || (!disabled.get()).then_some("0")
                aria-label="Cups of coffee"
                class="demo-spinbutton"
            >
                {move || cups.get()}
            </div>
            <button {..increment_attrs} style=increment_styles class="demo-btn demo-spin-stepper">
                <span aria-hidden="true">"+"</span>
            </button>
        </div>
        <p class="demo-status">"Value: "{cups_text}"."</p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
