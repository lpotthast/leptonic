use leptonic::hooks::{PlacementX, PlacementY, *};
use leptonic::utils::{classes::Classes, locale::WritingDirection};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// Demo comparing tooltips with and without use_tooltip
#[component]
pub fn HoverToKeepOpenDemo() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 3em; justify-content: center; margin: 2em 0;">
            <div style="text-align: center;">
                <WithoutUseTooltipDemo />
                <p style="margin-top: 0.5em; font-size: 0.85em; color: #888;">"Without use_tooltip"</p>
            </div>
            <div style="text-align: center;">
                <WithUseTooltipDemo />
                <p style="margin-top: 0.5em; font-size: 0.85em; color: #888;">"With use_tooltip"</p>
            </div>
        </div>
    }
}

/// Tooltip without use_tooltip -- closes when moving cursor to tooltip content
#[component]
fn WithoutUseTooltipDemo() -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);

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

    view! {
        <button
            {..target_capture}
            class=Classes::from("demo-btn-dark")
            {..trigger.trigger_props.into_attrs()}
        >
            "Hover me"
        </button>

        <Portal>
            {
                let tooltip_id = tooltip_id.clone();
                let position_attrs = position_attrs.clone();
                let position_styles = position_styles.clone();
                view! {
                    <Show when=move || is_open.get()>
                        <div
                            {..position_attrs.clone()}
                            style=position_styles.clone()
                            id=tooltip_id.clone()
                            role="tooltip"
                            class="tooltip-demo"
                        >
                            "Move cursor here \u{2014} closes!"
                        </div>
                    </Show>
                }
            }
        </Portal>
    }
}

/// Tooltip with use_tooltip -- stays open when moving cursor to tooltip content
#[component]
fn WithUseTooltipDemo() -> impl IntoView {
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
                            "Move cursor here \u{2014} stays open!"
                        </div>
                    </Show>
                }
            }
        </Portal>
    }
}
