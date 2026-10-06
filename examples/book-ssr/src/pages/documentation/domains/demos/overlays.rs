use leptonic::{
    atoms::prelude::FocusScope,
    components::prelude::{Button, ButtonVariant},
    hooks::*,
};
use leptos::prelude::*;
use leptos_element_capture::CapturedElement;

/// A button that opens a dismissable panel: `use_button` presses, `use_overlay` dismisses, `use_overlay_trigger`
/// connects the two for assistive technology.
#[component]
pub fn OverlaysDomainDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let trigger = CapturedElement::new();

    let UseOverlayReturn {
        props: overlay_props,
        id,
        ..
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: true,
        is_keyboard_dismiss_disabled: false,
        // The trigger toggles the panel itself, so presses on it must not count as "outside".
        should_close_on_interact_outside: Some(Callback::new(move |el: web_sys::Element| {
            !trigger.with_untracked(|trigger| {
                trigger.is_some_and(|trigger| trigger.contains(Some(el.as_ref())))
            })
        })),
    });
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });

    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| {
            set_is_open.update(|open| *open = !*open);
        })),
        ..Default::default()
    });
    let (button_attrs, button_styles) = props.into_parts();

    view! {
        <button
            {..button_attrs}
            {..trigger_props.into_attrs()}
            {..trigger.attr()}
            style=button_styles
            class="demo-btn-primary"
        >
            {move || if is_open.get() { "Close panel" } else { "Open panel" }}
        </button>

        <Show when=move || is_open.get()>
            <div {..overlay_attrs.get_value()} class="demo-overlay-inline-panel">
                <FocusScope restore_focus=true auto_focus=true>
                    <p class="demo-overlay-text">
                        "Press Escape, click outside or tab out of this panel to close it."
                    </p>
                    <Button variant=ButtonVariant::Outlined on_press=move |_| set_is_open.set(false)>"Close"</Button>
                </FocusScope>
            </div>
        </Show>
    }
}
