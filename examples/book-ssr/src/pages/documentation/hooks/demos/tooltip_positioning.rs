use std::time::Duration;

use leptonic::{hooks::*, utils::CapturedElement};
use leptos::{portal::Portal, prelude::*};

/// Tooltips placed on all four sides of their trigger.
#[component]
pub fn PositioningDemo() -> impl IntoView {
    view! {
        <div class="demo-tooltip-grid">
            <PositionedTooltip label="Above" placement=Placement::Top/>
            <PositionedTooltip label="Below" placement=Placement::Bottom/>
            <PositionedTooltip label="Left" placement=Placement::Left/>
            <PositionedTooltip label="Right" placement=Placement::Right/>
        </div>
    }
}

/// A button with a tooltip at `placement`.
#[component]
fn PositionedTooltip(label: &'static str, placement: Placement) -> impl IntoView {
    let trigger_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::from_millis(500),
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        state: Some(state),
        ..Default::default()
    });
    let position = use_overlay_position(UseOverlayPositionInput {
        placement: Signal::stored(placement),
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
        <button {..trigger_element.attr()} {..trigger.trigger_props.into_attrs()} class="demo-btn">
            {label}
        </button>

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
                    {format!("Placed {}", label.to_lowercase())}
                </div>
            </Show>
        </Portal>
    }
}
