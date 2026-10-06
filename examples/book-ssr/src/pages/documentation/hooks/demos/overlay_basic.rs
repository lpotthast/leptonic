use leptonic::{atoms::prelude::FocusScope, components::prelude::*, hooks::*};
use leptos::{portal::Portal, prelude::*};

/// `use_overlay` alone: a centered panel that closes on Escape and on a press outside of it.
#[component]
pub fn BasicOverlayDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (dismiss_count, set_dismiss_count) = signal(0_u32);

    let UseOverlayReturn {
        props: overlay_props,
        ..
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| {
            set_dismiss_count.update(|count| *count += 1);
            set_is_open.set(false);
        }),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    // The overlay is rendered whenever it opens, so its attributes are stored and cloned per render.
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    view! {
        <div class="demo-flex-center-row">
            <Button on_press=move |_| set_is_open.set(true)>"Open overlay"</Button>
            <span>"Dismissed by use_overlay: " {move || dismiss_count.get()} " times"</span>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                // The underlay covers the page, so a press anywhere outside the panel lands on it.
                <div class="demo-overlay-underlay"></div>
                <div {..overlay_attrs.get_value()} class="demo-overlay-dialog">
                    // Moves focus into the panel, so that Escape reaches the overlay's key handler.
                    <FocusScope contain=true restore_focus=true auto_focus=true>
                        <h4 class="demo-overlay-title">"Overlay"</h4>
                        <p class="demo-overlay-text">"Press Escape or click outside to dismiss this overlay."</p>
                        <Button on_press=move |_| set_is_open.set(false)>"Close"</Button>
                    </FocusScope>
                </div>
            </Show>
        </Portal>
    }
}
