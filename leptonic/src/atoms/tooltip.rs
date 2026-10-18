// Upstream: react-aria-components/src/Tooltip.tsx @ 99e6102368
// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
use std::time::Duration;

use leptos::{context::Provider, portal::Portal, prelude::*};
use leptos_classes::Classes;

use super::{overlay_arrow::OverlayArrowContext, press::ClearTriggerContexts};
use crate::{
    CapturedElement, IntoAttrs, Out, Part, ValueBinding,
    hooks::{
        animation::{
            UseEnterAnimationInput, UseEnterAnimationReturn, UseExitAnimationInput,
            use_enter_animation, use_exit_animation,
        },
        focus::{FocusableContext, FocusableContextAttrs},
        overlay::{
            OverlayPositionOptions, Placement, PlacementAxis, UseOverlayPositionInput,
            UseOverlayPositionProps, UseOverlayPositionReturn, use_overlay_position,
        },
        tooltip::{
            TooltipTiming, TooltipTriggerMode, TooltipTriggerState, UseTooltipInput,
            UseTooltipProps, UseTooltipTriggerInput, UseTooltipTriggerReturn,
            UseTooltipTriggerStateInput, UseTooltipTriggerTooltipProps, use_tooltip,
            use_tooltip_trigger, use_tooltip_trigger_state,
        },
    },
    utils::{data_attributes::data_flags, default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger props reach the trigger (any focusable atom: `Button`, `Link`, ...) through
//   `FocusableContext`, as react-aria-components' `FocusableProvider`.
// - Open state (C4): `default_open` + `on_open_change`, or `is_open` + `set_open` (react-aria:
//   `isOpen` + `onOpenChange`).
// - `placement` is the typed `Placement` enum; render props become `data-placement`,
//   `data-entering` and `data-exiting` plus plain children.
// - A `Tooltip` must be inside a `TooltipTrigger` (react-aria-components: a standalone tooltip
//   can have its own `isOpen`/`defaultOpen`).
//
// ## OMITTED FEATURES
// - `isEntering`/`isExiting` props.
// - `UNSTABLE_portalContainer`: leptonic ports no `UNSTABLE_` APIs (no legacy or unstable
//   surface).
//
// =============================================================================

/// Context from a [`TooltipTrigger`] to its [`Tooltip`].
#[derive(Debug, Clone)]
struct TooltipTriggerContext {
    state: TooltipTriggerState,
    /// The tooltip element (its id), one tooltip element at a time.
    tooltip: Part<UseTooltipTriggerTooltipProps>,
    trigger: CapturedElement,
}

/// Shows its [`Tooltip`] while its trigger (the focusable atom inside, e.g. a `Button`) is hovered
/// or focused: after `delay` for the first tooltip, right away once one was open (react-aria's
/// warm-up). Escape and pressing the trigger close it.
#[component]
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
    #[prop(into, default = Signal::stored(true))]
    should_close_on_press: Signal<bool>,
    #[prop(optional)] default_open: bool,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// Whether the tooltip is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay,
        close_delay,
        default_open,
        value: is_open,
        on_open_change,
    });
    let UseTooltipTriggerReturn {
        trigger: trigger_props,
        tooltip,
        ..
    } = use_tooltip_trigger(UseTooltipTriggerInput {
        state,
        is_disabled,
        trigger,
        should_close_on_press,
    });
    let trigger_props = trigger_props.into_props();
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
        attrs: Some(FocusableContextAttrs::new(move |is_disabled| {
            use leptos::{attr::any_attribute::IntoAnyAttribute, ev};
            let (enter, leave, down) = pointer_handlers.get_value();
            (
                crate::EventHandler::new(move |event| {
                    if !is_disabled.get_untracked() {
                        enter.call(event);
                    }
                })
                .into_on(ev::pointerenter),
                crate::EventHandler::new(move |event| {
                    if !is_disabled.get_untracked() {
                        leave.call(event);
                    }
                })
                .into_on(ev::pointerleave),
                crate::EventHandler::new(move |event| {
                    if !is_disabled.get_untracked() {
                        down.call(event);
                    }
                })
                .into_on(ev::pointerdown),
            )
                .into_any_attr()
        })),
        element: Some(trigger_element),
    };
    let context = TooltipTriggerContext {
        state,
        tooltip,
        trigger: trigger_element,
    };

    view! {
        <Provider value=context>
            <Provider value=focusable>{children()}</Provider>
        </Provider>
    }
}

/// The tooltip of the [`TooltipTrigger`] around it: `role="tooltip"`, describing the trigger,
/// positioned next to it (above by default) while open. Hovering it keeps it open; scrolling closes
/// it. Put an [`OverlayArrow`](super::overlay_arrow::OverlayArrow) in it for an arrow pointing at
/// the trigger.
///
/// Data attributes: `data-placement` (`top`, `bottom`, `left` or `right`, after flipping),
/// `data-entering`, `data-exiting`. CSS variable: `--trigger-anchor-point` (the point closest to
/// the trigger).
///
/// Default class: `leptonic-Tooltip`.
#[component]
pub fn Tooltip(
    /// Where the tooltip goes relative to the trigger.
    #[prop(into, default = Signal::stored(Placement::Top))]
    placement: Signal<Placement>,
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
    /// The minimum distance between an `OverlayArrow` and the tooltip's edges.
    #[prop(into, optional)]
    arrow_boundary_offset: Signal<f64>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-Tooltip", classes);
    let Some(TooltipTriggerContext {
        state,
        tooltip: tooltip_trigger,
        trigger,
    }) = use_context::<TooltipTriggerContext>()
    else {
        crate::utils::dev_warn!("A <Tooltip> must be inside a <TooltipTrigger>.");
        return None;
    };
    let is_open = state.overlay.is_open;

    let UseOverlayPositionReturn {
        overlay: position_props,
        arrow: arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_overlay_position(UseOverlayPositionInput {
        position: OverlayPositionOptions {
            placement,
            offset,
            cross_offset,
            container_padding,
            should_flip,
            arrow_boundary_offset,
            ..OverlayPositionOptions::default()
        },
        // As react-aria: scrolling closes the tooltip right away.
        on_close: Some(Callback::new(move |()| {
            state.close(TooltipTiming::Immediate);
        })),
        target: trigger,
        is_open,
        target_rect: Signal::stored(None),
        scroll: None,
    });
    let position_props = position_props.into_props();
    let arrow_props = arrow_props.into_props();
    let tooltip = use_tooltip(UseTooltipInput {
        state: Some(state),
        ..UseTooltipInput::default()
    });
    let tooltip_props = tooltip.tooltip.into_props();

    let (UseOverlayPositionProps { element_capture }, position_styles) =
        position_props.into_inner();
    let anchor_point = Styles::builder()
        .with_optional_unchecked("--trigger-anchor-point", move || {
            trigger_anchor_point
                .get()
                .map(|point| format!("{}px {}px", point.x, point.y))
        })
        .build();
    // The tooltip element, built for each opening (one element at a time).
    let UseTooltipProps {
        role,
        on_pointerenter,
        on_pointerleave,
    } = tooltip_props;
    // What each opening renders: one `StoredValue` (`performance.md` R7).
    let content = StoredValue::new(TooltipContent {
        arrow_context: OverlayArrowContext::new(arrow_props, resolved_placement),
        element: Part::new(move || {
            (
                UseOverlayPositionProps {
                    element_capture: element_capture.clone(),
                },
                UseTooltipProps {
                    role,
                    on_pointerenter: on_pointerenter.clone(),
                    on_pointerleave: on_pointerleave.clone(),
                },
            )
        }),
        classes,
        styles: position_styles.merge(anchor_point).merge(styles),
        children,
        trigger: tooltip_trigger,
    });
    // The tooltip stays rendered while its exit animations run (`data-exiting`), unless it closes
    // instantly (another tooltip replaces it during the warm-up).
    let element = CapturedElement::new();
    let exit_animation = use_exit_animation(UseExitAnimationInput {
        element,
        is_open,
        on_exit: None,
    })
    .is_exiting;
    let skip_animation = state.should_skip_animation;
    let is_exiting = Signal::derive(move || !skip_animation.get() && exit_animation.get());

    Some(view! {
        // No portal container while closed: a modal would make it inert.
        <Show when=move || is_open.get() || is_exiting.get()>
            {
                // Entering (per opening) once the placement is known (react-aria-components).
                let entering = CapturedElement::new();
                let UseEnterAnimationReturn { is_entering: enter_animation, styles: hiding } =
                    use_enter_animation(UseEnterAnimationInput {
                        is_ready: Signal::derive(move || resolved_placement.get().is_some()),
                        element: entering,
                        on_enter: None,
                    });
                // Hidden until placed; the only writer of the tooltip's `style` (lessons.md). Cloned
                // once more when the portal renders its content.
                let (classes, styles) = content.with_value(|content| {
                    (content.classes.clone(), hiding.merge(content.styles.clone()))
                });
                // Not when opening instantly (replacing another tooltip during the warm-up).
                let is_entering =
                    Signal::derive(move || !skip_animation.get() && enter_animation.get());
                view! {
                    <Portal>
                        <div
                            {..content.with_value(|content| {
                                match content.element.props() {
                                    Some((position, tooltip)) => leptos::either::Either::Left((
                                        position.into_attrs(),
                                        tooltip.into_attrs(),
                                    )),
                                    None => leptos::either::Either::Right(()),
                                }
                            })}
                            {..element.attr().chain(entering.attr())}
                            {..content.with_value(|content| content.trigger.attrs())}
                            {..data_flags([
                                ("data-entering", is_entering),
                                ("data-exiting", is_exiting),
                            ])}
                            class=classes.clone()
                            style=styles.clone()
                            data-placement=move || {
                                resolved_placement.get().map(PlacementAxis::as_str)
                            }
                        >
                            // The tooltip's own content is no trigger.
                            <ClearTriggerContexts>
                                <Provider value=content
                                    .with_value(|content| content.arrow_context.clone())>
                                    {content.with_value(|content| content.children.clone())()}
                                </Provider>
                            </ClearTriggerContexts>
                        </div>
                    </Portal>
                }
            }
        </Show>
    })
}

/// What each opening of a [`Tooltip`] renders.
struct TooltipContent {
    arrow_context: OverlayArrowContext,
    /// The tooltip element's own props (one element at a time).
    element: Part<(UseOverlayPositionProps, UseTooltipProps)>,
    classes: Classes,
    styles: Styles,
    children: ChildrenFn,
    /// The trigger's reference to the tooltip (its id).
    trigger: Part<UseTooltipTriggerTooltipProps>,
}
