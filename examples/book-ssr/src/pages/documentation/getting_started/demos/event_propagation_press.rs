use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*, utils::Propagation};
use leptos::prelude::*;

#[component]
pub fn EventPropagationPressDemo() -> impl IntoView {
    let (button_bubbles, set_button_bubbles) = signal(false);
    let (card_presses, set_card_presses) = signal(0);
    let (button_presses, set_button_presses) = signal(0);

    // The card handles presses anywhere on it.
    let card = use_press(UsePressInput {
        on_press: Some(Callback::new(move |_| set_card_presses.update(|n| *n += 1))),
        ..Default::default()
    });

    // The button handles its own presses. Each callback decides for the events that trigger it, so all four continue
    // propagation, or the card sees an incomplete press.
    let continue_if_bubbling = move |e: PressEvent| {
        if button_bubbles.get_untracked() {
            e.continue_propagation();
        }
    };
    let button = use_press(UsePressInput {
        on_press_start: Some(Callback::new(continue_if_bubbling)),
        on_press_up: Some(Callback::new(continue_if_bubbling)),
        on_press_end: Some(Callback::new(continue_if_bubbling)),
        on_press: Some(Callback::new(move |e: PressEvent| {
            set_button_presses.update(|n| *n += 1);
            continue_if_bubbling(e);
        })),
        ..Default::default()
    });
    let (card_attrs, card_styles) = card.props.into_parts();
    let (button_attrs, button_styles) = button.props.into_parts();

    view! {
        <div {..card_attrs} style=card_styles class="demo-propagation-panel demo-propagation-card">
            "Card: press anywhere"
            <button {..button_attrs} style=button_styles type="button" class="demo-btn">"Button"</button>
        </div>
        <p class="demo-status">
            {move || {
                let times = |n: u32| if n == 1 { "1 time".to_owned() } else { format!("{n} times") };
                format!("Button pressed {}, card pressed {}.", times(button_presses.get()), times(card_presses.get()))
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=button_bubbles set_selected=set_button_bubbles>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Let the button\u{2019}s presses bubble"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
