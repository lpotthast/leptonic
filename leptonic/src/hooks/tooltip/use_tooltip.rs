use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::PointerEvent;

use super::use_tooltip_trigger_state::UseTooltipTriggerStateReturn;
use crate::{
    hooks::{
        IntoAttrs,
        interactions::use_hover::{UseHoverInput, use_hover},
    },
    utils::EventHandler,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/tooltip/src/useTooltip.ts

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Accepts `UseTooltipTriggerStateReturn` directly instead of separate
//   `on_open`/`on_close` callbacks, enabling the tooltip to participate in
//   the warmup/cooldown system when hovered.
//

/// Input parameters for the `use_tooltip` hook.
#[derive(Clone, Copy)]
pub struct UseTooltipInput {
    /// Whether the tooltip is disabled.
    pub disabled: Signal<bool>,

    /// Tooltip trigger state. When provided, hovering the tooltip itself
    /// keeps it open (calls `state.open(true)` on hover start and
    /// `state.close(false)` on hover end).
    pub state: Option<UseTooltipTriggerStateReturn>,

    /// Called when the tooltip should open. Used when `state` is `None`.
    pub on_open: Option<Callback<()>>,

    /// Called when the tooltip should close. Used when `state` is `None`.
    pub on_close: Option<Callback<()>>,
}

/// The return value of the `use_tooltip` hook.
#[derive(Debug)]
pub struct UseTooltipReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseTooltipProps,
}

/// Props from `use_tooltip` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTooltipProps {
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseTooltipProps {
    type Attrs = UseTooltipAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
        )
    }
}

/// These attributes must be spread onto the tooltip element.
pub type UseTooltipAttrs = (
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
);

/// Provides the accessibility implementation for a Tooltip component.
///
/// Tooltips display contextual help or information about an element when it is hovered.
/// The tooltip should have `role="tooltip"` set on the actual tooltip content element.
///
/// When `state` is provided, hovering over the tooltip itself keeps it open
/// (participates in the warmup/cooldown system).
///
/// # Example
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput::default());
/// let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
/// let tooltip = use_tooltip(UseTooltipInput {
///     disabled: Signal::derive(|| false),
///     state: Some(state),
///     on_open: None,
///     on_close: None,
/// });
///
/// view! {
///     <div {..tooltip.props.into_attrs()} role="tooltip">
///         "Tooltip content"
///     </div>
/// }
/// ```
pub fn use_tooltip(input: UseTooltipInput) -> UseTooltipReturn {
    let UseTooltipInput {
        disabled,
        state,
        on_open,
        on_close,
    } = input;

    // Determine hover callbacks: prefer state-based, fall back to direct callbacks.
    let on_hover_start: Option<Callback<()>> = if let Some(st) = state {
        Some(Callback::new(move |_| {
            st.open.run(true);
        }))
    } else {
        on_open
    };

    let on_hover_end: Option<Callback<()>> = if let Some(st) = state {
        Some(Callback::new(move |_| {
            st.close.run(false);
        }))
    } else {
        on_close
    };

    let hover = use_hover(UseHoverInput {
        disabled,
        on_hover_start: on_hover_start.map(|cb| {
            Callback::new(move |_| {
                cb.run(());
            })
        }),
        on_hover_end: on_hover_end.map(|cb| {
            Callback::new(move |_| {
                cb.run(());
            })
        }),
        on_hover_change: None,
    });

    UseTooltipReturn {
        props: UseTooltipProps {
            on_pointerenter: hover.props.on_pointerenter,
            on_pointerleave: hover.props.on_pointerleave,
        },
    }
}
