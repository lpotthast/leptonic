use std::time::Duration;

use leptonic::{
    CapturedElement,
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
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput {
        state,
        is_disabled: Signal::stored(false),
        trigger: TooltipTriggerMode::Hover,
        should_close_on_press: Signal::stored(true),
    });
    let tooltip = use_tooltip(UseTooltipInput {
        state: Some(state),
        ..Default::default()
    });
    let position = use_overlay_position(UseOverlayPositionInput {
        position: OverlayPositionOptions {
            placement: Signal::stored(placement),
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
    // `<Show>` mounts the tooltip on every opening, and each mount spreads the parts again: the
    // trigger's (the id it references), the tooltip's own and the position's. A `StoredValue` makes
    // a part `Copy` for the nested children closures.
    let trigger_tooltip = StoredValue::new(trigger.tooltip);
    let tooltip = StoredValue::new(tooltip.tooltip);
    let position = StoredValue::new(position.overlay);

    view! {
        <button {..trigger_element.attr()} {..trigger.trigger.attrs()} class="demo-btn">
            {label}
        </button>

        <Portal>
            <Show when=move || is_open.get()>
                {
                    let (position_attrs, position_styles) = position.with_value(|position| position.attrs());
                    view! {
                        <div
                            {..position_attrs}
                            {..tooltip.with_value(|tooltip| tooltip.attrs())}
                            {..trigger_tooltip.with_value(|tooltip| tooltip.attrs())}
                            style=position_styles
                            class="demo-tooltip"
                        >
                            {format!("Placed {}", label.to_lowercase())}
                        </div>
                    }
                }
            </Show>
        </Portal>
    }
}
