use leptonic::{
    atoms::prelude::FocusScope, components::prelude::*, hooks::*, utils::CapturedElement,
};
use leptos::prelude::*;

/// A button that opens a panel: `use_overlay` dismisses it, `use_overlay_trigger` connects the button to it.
#[component]
pub fn OverlayDismissDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let trigger = CapturedElement::new();

    let UseOverlayReturn {
        props: overlay_props,
        id,
        ..
    } = use_overlay(UseOverlayInput {
        is_dismissable: Signal::stored(true),
        should_close_on_blur: Signal::stored(true),
        // The trigger toggles the panel itself, so presses on it must not count as "outside".
        should_close_on_interact_outside: Some(InteractOutsideFilter::new(
            move |el: &web_sys::Element| {
                !trigger.with_untracked(|trigger| {
                    trigger.is_some_and(|trigger| trigger.contains(Some(el.as_ref())))
                })
            },
        )),
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        group: None,
    });
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });

    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| {
            set_is_open.update(|open| *open = !*open);
        })),
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();

    view! {
        <button
            {..button_attrs}
            {..trigger_props.into_attrs()}
            {..trigger.attr()}
            style=button_styles
            class="demo-btn-primary"
        >
            "Shipping options"
        </button>

        <Show when=move || is_open.get()>
            <div
                {..overlay_attrs.get_value()}
                role="dialog"
                aria-labelledby="overlay-quick-start-title"
                class="demo-overlay-inline-panel"
            >
                // Moves focus into the panel, so that Escape reaches it, and back to the button when it closes.
                <FocusScope restore_focus=true auto_focus=true>
                    <h4 id="overlay-quick-start-title" class="demo-overlay-title">"Shipping options"</h4>
                    <p class="demo-overlay-text">"Press Escape, click outside or tab out of this panel to close it."</p>
                    <Button on_press=move |_| set_is_open.set(false)>"Done"</Button>
                </FocusScope>
            </div>
        </Show>

        <p class="demo-status">
            {move || if is_open.get() { "The panel is open." } else { "The panel is closed." }}
        </p>
    }
}
