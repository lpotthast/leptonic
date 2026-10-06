use leptonic::hooks::{PlacementX, PlacementY, *};
use leptos::{portal::Portal, prelude::*};

#[component]
pub fn BasicPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        ..
    } = use_popover(UsePopoverInput {
        placement_x: Signal::stored(PlacementX::Center),
        placement_y: Signal::stored(PlacementY::Below),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        modality: PopoverModality::Modal,
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

    // `<Show>` may render its children more than once, so keep the attributes in stored values.
    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);

    view! {
        <div class="demo-flex-center">
            // `trigger_props` capture the button, which the popover is positioned against.
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn-primary">
                {move || if is_open.get() { "Close Popover" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                // The underlay covers the page behind a modal popover.
                <div class="demo-popover-underlay"/>
                <div
                    {..popover_attrs.get_value()}
                    style=popover_styles.get_value()
                    class="demo-overlays-popover"
                >
                    <h4 class="demo-overlays-popover-title">"Popover Title"</h4>
                    <p class="demo-overlays-text">"Press Escape or click outside to close."</p>
                </div>
            </Show>
        </Portal>
    }
}
