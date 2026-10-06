use leptonic::{atoms::prelude::FocusScope, components::prelude::*, hooks::*};
use leptos::prelude::*;

/// `use_overlay` alone: a panel that closes on Escape, on a press outside or when focus leaves it, as configured.
#[component]
pub fn BasicOverlayDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let dismissals = RwSignal::new(0_u32);
    let is_dismissable = RwSignal::new(true);
    let close_on_blur = RwSignal::new(false);
    let escape_disabled = RwSignal::new(false);

    let UseOverlayReturn { props, .. } = use_overlay(UseOverlayInput {
        is_dismissable: is_dismissable.into(),
        should_close_on_blur: close_on_blur.into(),
        is_keyboard_dismiss_disabled: escape_disabled.into(),
        ..UseOverlayInput::new(
            is_open.into(),
            Callback::new(move |()| {
                dismissals.update(|count| *count += 1);
                set_is_open.set(false);
            }),
        )
    });
    // The panel renders again on every opening, so its attributes are stored and cloned per render.
    let overlay_attrs = StoredValue::new(props.into_attrs());

    view! {
        <Button on_press=move |_| set_is_open.set(true)>"Open panel"</Button>

        <Show when=move || is_open.get()>
            <div {..overlay_attrs.get_value()} role="dialog" aria-labelledby="use-overlay-demo-title" class="demo-overlay-inline-panel">
                // Moves focus into the panel, so that Escape reaches its key handler, and back when it closes.
                <FocusScope restore_focus=true auto_focus=true>
                    <h4 id="use-overlay-demo-title" class="demo-overlay-title">"Panel"</h4>
                    <p class="demo-overlay-text">"Try Escape, a click outside or Tab."</p>
                    <Button on_press=move |_| set_is_open.set(false)>"Close"</Button>
                </FocusScope>
            </div>
        </Show>

        <p class="demo-status">
            {move || match dismissals.get() {
                1 => "Dismissed by use_overlay 1 time.".to_owned(),
                count => format!("Dismissed by use_overlay {count} times."),
            }}
        </p>

        // Settings take effect for the next opening; they can't be pressed while the panel is open, as that would
        // be a press outside.
        <div class="demo-controls">
            <Checkbox is_selected=is_dismissable set_selected=is_dismissable is_disabled=is_open>"Close on press outside"</Checkbox>
            <Checkbox is_selected=close_on_blur set_selected=close_on_blur is_disabled=is_open>"Close on blur"</Checkbox>
            <Checkbox is_selected=escape_disabled set_selected=escape_disabled is_disabled=is_open>"Ignore Escape"</Checkbox>
        </div>
    }
}
