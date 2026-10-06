use std::time::Duration;

use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// A tooltip built from all three tooltip hooks, positioned with `use_overlay_position`.
#[component]
pub fn TooltipDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::from_millis(300),
        close_delay: Duration::from_millis(100),
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(
        UseTooltipTriggerInput {
            is_disabled: disabled.into(),
            ..Default::default()
        },
        state,
    );
    // Keeps the tooltip open while the pointer is over it.
    let tooltip = use_tooltip(UseTooltipInput {
        is_disabled: disabled.into(),
        state: Some(state),
        on_open: None,
        on_close: None,
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::stored(PlacementX::Center),
        placement_y: Signal::stored(PlacementY::Above),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = StoredValue::new(trigger.tooltip_props.id);
    let tooltip_role = trigger.tooltip_props.role;
    let (position_attrs, position_styles) = position.props.into_parts();
    let position_attrs = StoredValue::new(position_attrs);
    let position_styles = StoredValue::new(position_styles);
    let tooltip_attrs = StoredValue::new(tooltip.props.into_attrs());

    view! {
        <div class="demo-overlays-stage">
            <button {..target_element.attr()} {..trigger.trigger_props.into_attrs()} class="demo-btn-primary">
                "Hover me"
            </button>
        </div>

        <Checkbox state=disabled>"Disable tooltip"</Checkbox>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..position_attrs.get_value()}
                    {..tooltip_attrs.get_value()}
                    style=position_styles.get_value()
                    id=tooltip_id.get_value()
                    role=tooltip_role
                    class="demo-overlays-tooltip"
                >
                    "This is a tooltip!"
                </div>
            </Show>
        </Portal>
    }
}
