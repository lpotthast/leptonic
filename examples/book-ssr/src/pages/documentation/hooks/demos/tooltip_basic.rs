use leptonic::hooks::{PlacementX, PlacementY, *};
use leptonic::utils::{classes::Classes, locale::WritingDirection};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// Simple tooltip demo using all three hooks
#[component]
pub fn TooltipDemo() -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 300,
        close_delay: 100,
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
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Above),
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
        <div style="margin: 3em 0; display: flex; justify-content: center;">
            <button
                {..target_capture}
                class=Classes::from("demo-btn-primary")
                {..trigger.trigger_props.into_attrs()}
            >
                "Hover me"
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
                                class="tooltip-demo"
                            >
                                "This is a tooltip!"
                            </div>
                        </Show>
                    }
                }
            </Portal>
        </div>
    }
}
