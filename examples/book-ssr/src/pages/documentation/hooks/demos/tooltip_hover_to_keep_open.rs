use std::time::Duration;

use leptonic::hooks::{PlacementX, PlacementY, *};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// Two tooltips side by side. `use_tooltip` keeps the second one open while the pointer is over it.
#[component]
pub fn HoverToKeepOpenDemo() -> impl IntoView {
    view! {
        <div class="demo-overlays-stage">
            <div class="demo-overlays-centered">
                <DemoTooltip keep_open_on_hover=false/>
                <p class="demo-overlays-caption">"use_tooltip disabled"</p>
            </div>
            <div class="demo-overlays-centered">
                <DemoTooltip keep_open_on_hover=true/>
                <p class="demo-overlays-caption">"use_tooltip enabled"</p>
            </div>
        </div>
    }
}

#[component]
fn DemoTooltip(keep_open_on_hover: bool) -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: Duration::from_millis(200),
        close_delay: Duration::from_millis(50),
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        is_disabled: Signal::stored(!keep_open_on_hover),
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
        <button
            {..target_element.attr()}
            {..trigger.trigger_props.into_attrs()}
            class=if keep_open_on_hover { "demo-btn-primary" } else { "demo-btn-dark" }
        >
            "Hover me"
        </button>

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
                    {if keep_open_on_hover { "Move the pointer here: stays open" } else { "Move the pointer here: closes" }}
                </div>
            </Show>
        </Portal>
    }
}
