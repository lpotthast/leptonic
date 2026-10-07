use leptonic::utils::CapturedElement;
use leptonic::{
    atoms::prelude::FocusScope,
    components::prelude::{Button, ButtonVariant},
    hooks::*,
    utils::id::use_id,
};
use leptos::{portal::Portal, prelude::*};

#[component]
pub fn NonModalPopoverDemo() -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let is_open = state.is_open;
    let items = RwSignal::new(0_u32);
    let title_id = use_id("cart-title");

    // Non-modal: the page stays usable, so there is no underlay. Moving focus out of the popover
    // or scrolling the page closes it.
    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        ..
    } = use_popover(UsePopoverInput {
        placement: Signal::stored(Placement::Top),
        offset: Signal::stored(8.0),
        modality: PopoverModality::NonModal,
        state,
        trigger: CapturedElement::new(),
        cross_offset: Signal::stored(0.0),
        container_padding: Signal::stored(12.0),
        should_flip: Signal::stored(true),
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });

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

    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);
    let title_id = StoredValue::new(title_id);

    view! {
        <div class="demo-button-row-centered">
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn">
                "Cart"
            </button>
            // A press on the page reaches it. Here it also moves focus out of the popover, which
            // closes it.
            <Button variant=ButtonVariant::Outlined on_press=move |_| items.update(|n| *n += 1)>
                "Add item"
            </Button>
        </div>
        <p class="demo-status">
            {move || {
                let count = items.get();
                format!(
                    "{count} {} in the cart. The popover is {}.",
                    if count == 1 { "item" } else { "items" },
                    if is_open.get() { "open" } else { "closed" },
                )
            }}
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                // Moves focus into the popover and back to the trigger, without containing it.
                <FocusScope restore_focus=true auto_focus=true>
                    <div
                        {..popover_attrs.get_value()}
                        style=popover_styles.get_value()
                        class="demo-popover demo-popover-narrow"
                        role="dialog"
                        aria-labelledby=title_id.get_value()
                        tabindex="-1"
                    >
                        <h2 id=title_id.get_value() class="demo-overlay-title">"Cart"</h2>
                        <p class="demo-overlay-text">"Free shipping from three items."</p>
                    </div>
                </FocusScope>
            </Show>
        </Portal>
    }
}
