use leptonic::hooks::{PlacementX, PlacementY, *};
use leptonic::utils::locale::WritingDirection;
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// Demo showing different tooltip placements
#[component]
pub fn PositioningDemo() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 2em; max-width: 400px; margin: 2em auto;">
            <PositionedTooltip
                label="Above"
                placement_x=PlacementX::Center
                placement_y=PlacementY::Above
            />
            <PositionedTooltip
                label="Below"
                placement_x=PlacementX::Center
                placement_y=PlacementY::Below
            />
            <PositionedTooltip
                label="Left"
                placement_x=PlacementX::OuterLeft
                placement_y=PlacementY::Center
            />
            <PositionedTooltip
                label="Right"
                placement_x=PlacementX::OuterRight
                placement_y=PlacementY::Center
            />
        </div>
    }
}

/// Individual positioned tooltip button
#[component]
fn PositionedTooltip(
    label: &'static str,
    placement_x: PlacementX,
    placement_y: PlacementY,
) -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        disabled: Signal::derive(|| false),
        state: Some(state),
        on_open: None,
        on_close: None,
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::derive(move || placement_x),
        placement_y: Signal::derive(move || placement_y),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = trigger.tooltip_props.id.clone();
    let target_capture = target_element.attr();
    let (position_attrs, position_styles) = position.props.into_parts();
    let tooltip_attrs = tooltip.props.into_attrs();

    view! {
        <div style="display: flex; justify-content: center;">
            <button
                {..target_capture}
                style="padding: 0.5em 1em; border-radius: 6px; cursor: pointer; background: #555; color: white; border: none; font-size: 0.9em; min-width: 80px;"
                {..trigger.trigger_props.into_attrs()}
            >
                {label}
            </button>

            <Portal>
                {
                    let tooltip_id = tooltip_id.clone();
                    let position_attrs = position_attrs.clone();
                    let position_styles = position_styles.clone();
                    let tooltip_attrs = tooltip_attrs.clone();
                    view! {
                        <Show when=move || is_open.get()>
                            <div
                                {..position_attrs.clone()}
                                style=position_styles.clone()
                                {..tooltip_attrs.clone()}
                                id=tooltip_id.clone()
                                role="tooltip"
                                class="positioned-tooltip"
                            >
                                {format!("Tooltip: {label}")}
                            </div>
                        </Show>
                    }
                }
            </Portal>
        </div>
    }
}
