// Upstream: react-aria/src/tooltip/useTooltip.ts @ 99e6102368
// Upstream: react-aria/test/tooltip/useTooltip.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    prelude::*,
};
use web_sys::PointerEvent;

use super::use_tooltip_trigger_state::{TooltipTiming, TooltipTriggerState};
use crate::{
    EventHandler, IntoAttrs, OnEvent,
    hooks::interactions::use_hover::{UseHoverInput, use_hover},
    utils::aria::AriaRole,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The optional state goes into the input (C8). DOM and labelling props aren't passed through:
//   set them on the element.
// - `is_disabled` (an addition) stops hovering the tooltip from keeping it open.
//
// =============================================================================

/// Input parameters for the `use_tooltip` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTooltipInput {
    /// Whether hovering the tooltip no longer keeps it open. Default: `false`.
    pub is_disabled: Signal<bool>,
    /// The tooltip trigger's state. With it, hovering the tooltip itself keeps it open (it opens
    /// right away on hover start and closes after the close delay on hover end). Default: none.
    pub state: Option<TooltipTriggerState>,
}

/// The return value of the `use_tooltip` hook.
#[derive(Debug)]
pub struct UseTooltipReturn {
    /// Props for the tooltip element. Call `.into_attrs()` for view spreading.
    pub props: UseTooltipProps,
}

/// Props for the tooltip element.
#[derive(Debug)]
pub struct UseTooltipProps {
    pub role: AriaRole,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseTooltipProps {
    type Attrs = UseTooltipAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
        )
    }
}

/// These attributes must be spread onto the tooltip element.
pub type UseTooltipAttrs = (
    Attr<attr::Role, AriaRole>,
    OnEvent<ev::pointerenter>,
    OnEvent<ev::pointerleave>,
);

/// Provides the accessibility implementation for a tooltip: `role="tooltip"`, and with the
/// trigger's state, hovering the tooltip keeps it open (it takes part in the warm-up and cooldown).
/// Give the element the id from `use_tooltip_trigger`'s `tooltip_props`, which describes the
/// trigger.
///
/// # Example
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput::default());
/// let tooltip = use_tooltip(UseTooltipInput {
///     state: Some(state),
///     ..UseTooltipInput::default()
/// });
///
/// view! {
///     <div id=tooltip_id {..tooltip.props.into_attrs()}>"Tooltip content"</div>
/// }
/// ```
pub fn use_tooltip(input: UseTooltipInput) -> UseTooltipReturn {
    let UseTooltipInput { is_disabled, state } = input;

    let hover = use_hover(UseHoverInput {
        is_disabled,
        on_hover_start: state.map(|state| {
            Callback::new(move |_| {
                state.open(TooltipTiming::Immediate);
            })
        }),
        on_hover_end: state.map(|state| {
            Callback::new(move |_| {
                state.close(TooltipTiming::Delayed);
            })
        }),
        ..UseHoverInput::default()
    });

    UseTooltipReturn {
        props: UseTooltipProps {
            role: AriaRole::Tooltip,
            on_pointerenter: hover.props.on_pointerenter,
            on_pointerleave: hover.props.on_pointerleave,
        },
    }
}
