use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::PointerEvent;

use crate::hooks::interactions::use_hover::{use_hover, UseHoverInput};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/tooltip/src/useTooltip.ts

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
#[derive(Debug, Clone)]
pub struct UseTooltipReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseTooltipProps,
}

/// Props from `use_tooltip` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTooltipProps {
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl UseTooltipProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTooltipAttrs {
        (
            self.on_pointerenter.to_on(ev::pointerenter),
            self.on_pointerleave.to_on(ev::pointerleave),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTooltipAttrs {
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
    let hover = use_hover(UseHoverInput {
        disabled: input.disabled,
        on_hover_start: input.on_open.map(|on_open| {
            Callback::new(move |_| {
                on_open.run(());
            })
        }),
        on_hover_end: input.on_close.map(|on_close| {
            Callback::new(move |_| {
                on_close.run(());
            })
        }),
    });

    UseTooltipReturn {
        props: UseTooltipProps {
            on_pointerenter: hover.props.on_pointerenter,
            on_pointerleave: hover.props.on_pointerleave,
        },
    }
}
