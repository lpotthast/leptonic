use std::time::Duration;

use leptonic::{
    CapturedElement, IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        overlay::{
            OverlayPositionOptions, Placement, UseOverlayPositionInput, use_overlay_position,
        },
        tooltip::{
            TooltipTriggerMode, UseTooltipInput, UseTooltipTriggerInput,
            UseTooltipTriggerStateInput, use_tooltip, use_tooltip_trigger,
            use_tooltip_trigger_state,
        },
    },
};
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
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput {
        state,
        is_disabled: disabled.into(),
        trigger: TooltipTriggerMode::Hover,
        should_close_on_press: Signal::stored(true),
    });
    // Keeps the tooltip open while the pointer is over the tooltip itself.
    let tooltip = use_tooltip(UseTooltipInput {
        is_disabled: Signal::derive(move || disabled.get() || !keep_open.get()),
        state: Some(state),
    });
    let position = use_overlay_position(UseOverlayPositionInput {
        position: OverlayPositionOptions {
            placement: Signal::stored(Placement::Top),
            offset: Signal::stored(6.0),
            ..OverlayPositionOptions::default()
        },
        target: trigger_element,
        is_open: state.overlay.is_open,
        target_rect: Signal::stored(None),
        scroll: None,
        on_close: None,
    });

    let is_open = state.overlay.is_open;
    let tooltip_id = StoredValue::new(trigger.tooltip_props.id);
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
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=keep_open set_selected=keep_open>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Keep open while hovered"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..position_attrs.get_value()}
                    {..tooltip_attrs.get_value()}
                    style=position_styles.get_value()
                    id=tooltip_id.get_value()
                    class="demo-tooltip"
                >
                    "Make the post visible to everyone"
                </div>
            </Show>
        </Portal>
    }
}
