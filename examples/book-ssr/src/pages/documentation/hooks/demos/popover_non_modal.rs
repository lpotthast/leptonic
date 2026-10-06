use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
};
use leptos::{portal::Portal, prelude::*};

#[component]
pub fn NonModalPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (counter, set_counter) = signal(0);

    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        ..
    } = use_popover(UsePopoverInput {
        placement_x: Signal::stored(PlacementX::OuterRight),
        placement_y: Signal::stored(PlacementY::Top),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        // The page stays interactive, so no underlay is needed. Scrolling the page closes the popover.
        modality: PopoverModality::NonModal,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
        ..UsePopoverInput::new(OverlayTriggerState::from((is_open, set_is_open)))
    });

    // The trigger: a button toggling the popover, announcing it with `aria-haspopup`, `aria-expanded` and
    // `aria-controls`.
    let UseOverlayTriggerReturn {
        props: overlay_trigger,
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
        aria_haspopup: Signal::stored(overlay_trigger.aria_haspopup),
        aria_expanded: overlay_trigger.aria_expanded,
        aria_controls: overlay_trigger.aria_controls,
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();

    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);

    view! {
        <div class="demo-button-row-centered">
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn-primary">
                {move || if is_open.get() { "Close" } else { "Open Non-Modal" }}
            </button>

            <Button on_press=move |_| set_counter.update(|c| *c += 1) variant=ButtonVariant::Outlined>
                {move || format!("Counter: {}", counter.get())}
            </Button>
        </div>

        <p class="demo-overlays-caption demo-overlays-centered">
            "The counter stays clickable while the popover is open."
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..popover_attrs.get_value()}
                    style=popover_styles.get_value()
                    class="demo-overlays-popover demo-overlays-popover-narrow"
                >
                    <p class="demo-overlays-text">"A non-modal popover. The rest of the page stays interactive."</p>
                </div>
            </Show>
        </Portal>
    }
}
