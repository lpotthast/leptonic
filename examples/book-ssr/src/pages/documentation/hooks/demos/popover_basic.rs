use leptonic::utils::CapturedElement;
use leptonic::{atoms::prelude::FocusScope, hooks::*, utils::id::use_id};
use leptos::{portal::Portal, prelude::*};

#[component]
pub fn BasicPopoverDemo() -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let is_open = state.is_open;
    let title_id = use_id("account-title");

    // Modal and below the trigger (the defaults), 8px away from it.
    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        ..
    } = use_popover(UsePopoverInput {
        offset: Signal::stored(8.0),
        state,
        trigger: CapturedElement::new(),
        placement: Signal::stored(Placement::Bottom),
        cross_offset: Signal::stored(0.0),
        container_padding: Signal::stored(12.0),
        should_flip: Signal::stored(true),
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });

    // The trigger: a button toggling the popover, with `aria-expanded` and `aria-controls` from
    // `use_overlay_trigger` (a dialog gets no `aria-haspopup`).
    let UseOverlayTriggerReturn {
        props: overlay_trigger,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open,
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });
    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| state.toggle())),
        aria_expanded: overlay_trigger.aria_expanded,
        aria_controls: overlay_trigger.aria_controls,
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();

    // `<Show>` renders its children on every opening, so keep the attributes in stored values.
    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);
    let title_id = StoredValue::new(title_id);

    view! {
        <div class="demo-flex-center">
            // `trigger_props` capture the button, which the popover is positioned at.
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn">
                "Account"
            </button>
        </div>
        <p class="demo-status">
            {move || if is_open.get() { "The popover is open." } else { "The popover is closed." }}
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                // The underlay covers the page behind a modal popover: a press on it closes the popover.
                <div class="demo-popover-underlay"/>
                // Moves focus into the popover, so that Escape reaches it, keeps it inside and returns
                // it to the button when the popover closes.
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..popover_attrs.get_value()}
                        style=popover_styles.get_value()
                        class="demo-popover"
                        role="dialog"
                        aria-labelledby=title_id.get_value()
                        tabindex="-1"
                    >
                        <h2 id=title_id.get_value() class="demo-overlay-title">"Account"</h2>
                        <p class="demo-overlay-text">"Signed in as ada@example.com."</p>
                    </div>
                </FocusScope>
            </Show>
        </Portal>
    }
}
