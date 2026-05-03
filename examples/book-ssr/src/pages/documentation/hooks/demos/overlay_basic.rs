use leptonic::hooks::*;
use leptos::{portal::Portal, prelude::*};
use leptos_classes::Classes;

/// Basic overlay demo: use_overlay only, with Escape and click-outside dismiss.
#[component]
pub fn BasicOverlayDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props: _,
        id: _,
        overlay_element: _,
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    view! {
        <div class=Classes::from("demo-flex-center")>
            <button
                on:click=move |_| set_is_open.set(!is_open.get())
                class=Classes::from("demo-btn-primary")
            >
                {move || if is_open.get() { "Close Overlay" } else { "Open Overlay" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    class=Classes::from("demo-overlay-panel")
                >
                    <h4 style="margin: 0 0 0.5em 0; color: #333;">"Overlay"</h4>
                    <p style="margin: 0; color: #666;">
                        "Press Escape or click outside to close this overlay."
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
