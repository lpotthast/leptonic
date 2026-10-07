use std::time::Duration;

use leptonic::{components::prelude::Checkbox, hooks::*, utils::CapturedElement};
use leptos::{portal::Portal, prelude::*};

/// A tooltip built from the three tooltip hooks, positioned with `use_overlay_position`.
#[component]
pub fn TooltipDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let keep_open = RwSignal::new(true);
    let trigger_element = CapturedElement::new();

    // Hovering opens the tooltip after half a second (default: 1.5 s).
    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::from_millis(500),
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(
        UseTooltipTriggerInput {
            is_disabled: disabled.into(),
            ..Default::default()
        },
        state,
    );
    // Keeps the tooltip open while the pointer is over the tooltip itself.
    let tooltip = use_tooltip(UseTooltipInput {
        is_disabled: Signal::derive(move || disabled.get() || !keep_open.get()),
        state: Some(state),
        ..Default::default()
    });
    let position = use_overlay_position(UseOverlayPositionInput {
        placement: Signal::stored(Placement::Top),
        offset: Signal::stored(6.0),
        target: trigger_element,
        is_open: trigger.is_open,
        container_padding: Signal::stored(12.0),
        cross_offset: Signal::stored(0.0),
        should_flip: Signal::stored(true),
        boundary: None,
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        should_update_position: Signal::stored(true),
        target_rect: Signal::stored(None),
        scroll: None,
        on_close: None,
    });

    let is_open = trigger.is_open;
    let tooltip_id = StoredValue::new(trigger.tooltip_props.id);
    let tooltip_role = trigger.tooltip_props.role;
    let (position_attrs, position_styles) = position.props.into_parts();
    let position_attrs = StoredValue::new(position_attrs);
    let position_styles = StoredValue::new(position_styles);
    let tooltip_attrs = StoredValue::new(tooltip.props.into_attrs());

    view! {
        <div class="demo-tooltip-stage">
            <button {..trigger_element.attr()} {..trigger.trigger_props.into_attrs()} class="demo-btn">
                "Publish"
            </button>
        </div>
        <p class="demo-status">
            {move || if is_open.get() { "The tooltip is open." } else { "The tooltip is closed." }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=keep_open set_selected=keep_open>"Keep open while hovered"</Checkbox>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..position_attrs.get_value()}
                    {..tooltip_attrs.get_value()}
                    style=position_styles.get_value()
                    id=tooltip_id.get_value()
                    role=tooltip_role
                    class="demo-tooltip"
                >
                    "Make the post visible to everyone"
                </div>
            </Show>
        </Portal>
    }
}
