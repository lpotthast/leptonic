use leptonic::{
    atoms::prelude::{Button, DismissButton, FocusScope},
    hooks::*,
};
use leptos::prelude::*;

/// A panel built with `use_overlay`, with a dismiss button at its start and end for screen reader users.
#[component]
pub fn DismissButtonDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (closed_by, set_closed_by) = signal(None::<&'static str>);

    let UseOverlayReturn { props, .. } = use_overlay(UseOverlayInput {
        is_dismissable: Signal::stored(true),
        is_open: is_open.into(),
        on_close: Callback::new(move |()| {
            set_closed_by.set(Some("Escape or a press outside"));
            set_is_open.set(false);
        }),
        should_close_on_blur: Signal::stored(false),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
    });
    let overlay_attrs = StoredValue::new(props.into_attrs());
    let dismiss = Callback::new(move |()| {
        set_closed_by.set(Some("a dismiss button"));
        set_is_open.set(false);
    });

    view! {
        <Button on_press=move |_| set_is_open.set(true) classes="demo-btn">"Open notifications"</Button>

        <Show when=move || is_open.get()>
            <div {..overlay_attrs.get_value()} role="dialog" aria-label="Notifications" class="demo-overlay-inline-panel">
                <FocusScope restore_focus=true auto_focus=true>
                    <DismissButton on_dismiss=dismiss/>
                    <p class="demo-overlay-text">"Your export is ready."</p>
                    <Button
                        on_press=move |_| {
                            set_closed_by.set(Some("the Download button"));
                            set_is_open.set(false);
                        }
                        classes="demo-btn-primary"
                    >
                        "Download"
                    </Button>
                    <DismissButton on_dismiss=dismiss/>
                </FocusScope>
            </div>
        </Show>

        <p class="demo-status">
            {move || match closed_by.get() {
                Some(by) => format!("Last closed by {by}."),
                None => "Not closed yet.".to_owned(),
            }}
        </p>
    }
}
