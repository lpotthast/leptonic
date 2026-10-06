// Upstream: react-aria-components/src/Tooltip.tsx @ 99e6102368
use std::time::Duration;

use leptos::{context::Provider, portal::Portal, prelude::*};

use crate::{
    hooks::{
        FocusableContext, FocusableContextAttrs, IntoAttrs, PhysicalPlacementX, PlacementX,
        PlacementY, TooltipTiming, TooltipTriggerMode, TooltipTriggerState, UseCloseOnScrollInput,
        UseOverlayPositionInput, UseOverlayPositionReturn, UseTooltipInput, UseTooltipTriggerInput,
        UseTooltipTriggerReturn, UseTooltipTriggerStateInput, use_close_on_scroll,
        use_overlay_position, use_tooltip, use_tooltip_trigger, use_tooltip_trigger_state,
    },
    utils::{CapturedElement, ValueBinding, classes::Classes, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger props reach the trigger (any focusable atom: `Button`, `Link`, ...) through
//   `FocusableContext`, as react-aria-components' `FocusableProvider`.
// - The open state is hook-owned (`default_open` + `on_open_change`, or `state` bound to app
//   state) instead of a controlled `isOpen`.
// - Placement is two typed axes; render props become `data-placement` plus plain children.
//
// ## OMITTED FEATURES
// - `OverlayArrow`, entry/exit animations (`should_skip_animation` is exposed on the state),
//   `UNSTABLE_portalContainer`.
//
// =============================================================================

/// Context from a [`TooltipTrigger`] to its [`Tooltip`].
#[derive(Debug, Clone)]
struct TooltipTriggerContext {
    state: TooltipTriggerState,
    tooltip_id: String,
    trigger: CapturedElement,
}

/// Shows its [`Tooltip`] while its trigger (the focusable atom inside, e.g. a `Button`) is hovered
/// or focused: after `delay` for the first tooltip, right away once one was open (react-aria's
/// warm-up). Escape and pressing the trigger close it.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn TooltipTrigger(
    /// How long hovering takes to open the first tooltip.
    #[prop(default = Duration::from_millis(1500))]
    delay: Duration,
    /// How long the tooltip stays open after the pointer leaves.
    #[prop(default = Duration::from_millis(500))]
    close_delay: Duration,
    /// Whether hovering or only focusing the trigger opens the tooltip.
    #[prop(optional)]
    trigger: TooltipTriggerMode,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Whether pressing the trigger closes the tooltip.
    #[prop(default = true)]
    should_close_on_press: bool,
    #[prop(optional)] default_open: bool,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// The open state as app state (e.g. an `RwSignal<bool>`), replacing `default_open`.
    #[prop(into, optional)]
    state: Option<ValueBinding<bool>>,
    children: Children,
) -> impl IntoView {
    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay,
        close_delay,
        default_open,
        value: state,
        on_open_change,
    });
    let UseTooltipTriggerReturn {
        trigger_props,
        tooltip_props,
        ..
    } = use_tooltip_trigger(
        UseTooltipTriggerInput {
            is_disabled,
            trigger,
            should_close_on_press,
        },
        state,
    );
    let trigger_element = CapturedElement::new();
    let pointer_handlers = StoredValue::new((
        trigger_props.on_pointerenter,
        trigger_props.on_pointerleave,
        trigger_props.on_pointerdown,
    ));
    let focusable = FocusableContext {
        on_focus: Some(trigger_props.on_focus),
        on_blur: Some(trigger_props.on_blur),
        on_keydown: Some(trigger_props.on_keydown),
        on_keyup: None,
        aria_describedby: Some(trigger_props.aria_describedby),
        attrs: Some(FocusableContextAttrs::new(move || {
            use leptos::{attr::any_attribute::IntoAnyAttribute, ev};
            let (enter, leave, down) = pointer_handlers.get_value();
            (
                enter.into_on(ev::pointerenter),
                leave.into_on(ev::pointerleave),
                down.into_on(ev::pointerdown),
            )
                .into_any_attr()
        })),
        element: Some(trigger_element),
    };
    let context = TooltipTriggerContext {
        state,
        tooltip_id: tooltip_props.id,
        trigger: trigger_element,
    };

    view! {
        <Provider value=context>
            <Provider value=focusable>{children()}</Provider>
        </Provider>
    }
}

/// The tooltip of the [`TooltipTrigger`] around it: `role="tooltip"`, describing the trigger,
/// positioned next to it (above by default) while open. Hovering it keeps it open.
///
/// Data attributes: `data-placement` (`top`, `bottom`, `left` or `right`, after flipping).
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Tooltip(
    #[prop(into, default = Signal::stored(PlacementX::Center))] placement_x: Signal<PlacementX>,
    #[prop(into, default = Signal::stored(PlacementY::Above))] placement_y: Signal<PlacementY>,
    /// The distance from the trigger, in pixels.
    #[prop(into, optional)]
    offset: Signal<f64>,
    #[prop(into, optional)] cross_offset: Signal<f64>,
    /// The minimum distance from the viewport edges, in pixels.
    #[prop(into, default = Signal::stored(12.0))]
    container_padding: Signal<f64>,
    /// Whether the tooltip flips to the other side when there is no room.
    #[prop(into, default = Signal::stored(true))]
    should_flip: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let TooltipTriggerContext {
        state,
        tooltip_id,
        trigger,
    } = expect_context::<TooltipTriggerContext>();
    let is_open = state.overlay.is_open;

    let UseOverlayPositionReturn {
        props: position_props,
        resolved_placement_x,
        resolved_placement_y,
    } = use_overlay_position(UseOverlayPositionInput {
        target: trigger,
        placement_x,
        placement_y,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height: None,
        is_open,
    });
    // As react-aria: scrolling closes the tooltip right away.
    use_close_on_scroll(UseCloseOnScrollInput {
        is_open,
        trigger_element: trigger,
        on_close: Callback::new(move |()| state.close(TooltipTiming::Immediate)),
    });
    let tooltip = use_tooltip(UseTooltipInput {
        state: Some(state),
        ..UseTooltipInput::default()
    });

    let placement = Memo::new(move |_| match resolved_placement_y.get() {
        PlacementY::Above | PlacementY::Top => "top",
        PlacementY::Bottom | PlacementY::Below => "bottom",
        PlacementY::Center => match resolved_placement_x.get() {
            PhysicalPlacementX::OuterLeft | PhysicalPlacementX::Left => "left",
            PhysicalPlacementX::Center
            | PhysicalPlacementX::Right
            | PhysicalPlacementX::OuterRight => "right",
        },
    });
    let (position_attrs, position_styles) = position_props.into_parts();
    let position_attrs = StoredValue::new(position_attrs);
    let tooltip_attrs = StoredValue::new(tooltip.props.into_attrs());
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(position_styles.merge(styles));
    let children = StoredValue::new(children);
    let tooltip_id = StoredValue::new(tooltip_id);

    view! {
        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..position_attrs.get_value()}
                    {..tooltip_attrs.get_value()}
                    id=tooltip_id.get_value()
                    role="tooltip"
                    class=classes.get_value()
                    style=styles.get_value()
                    data-placement=move || placement.get()
                >
                    // The tooltip's own content is no trigger.
                    <Provider value=FocusableContext::default()>{(children.get_value())()}</Provider>
                </div>
            </Show>
        </Portal>
    }
}
