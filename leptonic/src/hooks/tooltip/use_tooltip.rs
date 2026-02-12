use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::PointerEvent;

use crate::{
    hooks::{
        interactions::use_hover::{use_hover, UseHoverInput},
        IntoAttrs,
    },
    utils::EventHandler,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/tooltip/src/useTooltip.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_tooltip` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipInput {
    /// Whether the tooltip is disabled.
    pub disabled: Signal<bool>,

    /// Called when the tooltip should open.
    pub on_open: Option<Callback<()>>,

    /// Called when the tooltip should close.
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
/// # Example
///
/// ```ignore
/// let tooltip = use_tooltip(UseTooltipInput {
///     disabled: Signal::derive(|| false),
///     on_open: Some(Callback::new(|_| {
///         // Show tooltip
///     })),
///     on_close: Some(Callback::new(|_| {
///         // Hide tooltip
///     })),
/// });
///
/// view! {
///     <div {..tooltip.attrs} role="tooltip">
///         "Tooltip content"
///     </div>
/// }
/// ```
pub fn use_tooltip(input: UseTooltipInput) -> UseTooltipReturn {
    let UseTooltipInput {
        disabled,
        on_open,
        on_close,
    } = input;

    let hover = use_hover(UseHoverInput {
        disabled,
        on_hover_start: on_open.map(|on_open| {
            Callback::new(move |_| {
                on_open.run(());
            })
        }),
        on_hover_end: on_close.map(|on_close| {
            Callback::new(move |_| {
                on_close.run(());
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
