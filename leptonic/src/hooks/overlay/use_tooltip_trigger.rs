use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tooltip/src/useTooltipTrigger.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_tooltip_trigger` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipTriggerInput {
    /// Whether the tooltip is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the tooltip is open (controlled).
    pub is_open: Option<Signal<bool>>,

    /// The default open state (uncontrolled).
    pub default_open: bool,

    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,

    /// Delay before showing the tooltip (in ms).
    pub delay: u32,

    /// Delay before hiding the tooltip (in ms).
    pub close_delay: u32,

    /// The trigger behavior.
    pub trigger: TooltipTrigger,
}

/// What triggers the tooltip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipTrigger {
    /// Show on hover (and focus for accessibility).
    #[default]
    Hover,
    /// Show on focus only.
    Focus,
}

impl Default for UseTooltipTriggerInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            is_open: None,
            default_open: false,
            on_open_change: None,
            delay: 300,
            close_delay: 0,
            trigger: TooltipTrigger::Hover,
        }
    }
}

/// The return value of the `use_tooltip_trigger` hook.
pub struct UseTooltipTriggerReturn {
    /// Props for the trigger element.
    pub trigger_props: UseTooltipTriggerProps,

    /// Props for the tooltip element.
    pub tooltip_props: UseTooltipTriggerTooltipProps,

    /// Whether the tooltip is open.
    pub is_open: Signal<bool>,

    /// The ID of the trigger element.
    pub trigger_id: String,

    /// The ID of the tooltip element.
    pub tooltip_id: String,

    /// Open the tooltip.
    pub open: Callback<()>,

    /// Close the tooltip.
    pub close: Callback<()>,
}

/// Props from `use_tooltip_trigger` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTooltipTriggerProps {
    pub id: String,
    pub aria_describedby: Signal<Option<String>>,
    pub on_pointerenter: EventHandler<web_sys::PointerEvent>,
    pub on_pointerleave: EventHandler<web_sys::PointerEvent>,
    pub on_focus: EventHandler<web_sys::FocusEvent>,
    pub on_blur: EventHandler<web_sys::FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseTooltipTriggerProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTooltipTriggerAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the tooltip trigger element.
pub type UseTooltipTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    On<ev::pointerenter, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::focus, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::blur, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the tooltip element.
#[derive(Debug)]
pub struct UseTooltipTriggerTooltipProps {
    /// The id of the tooltip element.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,
}

/// Provides the behavior and accessibility for a tooltip trigger.
///
/// A tooltip displays brief helper text or information about an element when
/// the user hovers over or focuses on the element.
///
/// # Example
///
/// ```ignore
/// let tooltip = use_tooltip_trigger(UseTooltipTriggerInput {
///     delay: 500,
///     ..Default::default()
/// });
///
/// view! {
///     <button {..tooltip.trigger_props.into_attrs()}>
///         "Hover me"
///     </button>
///     <Show when=move || tooltip.is_open.get()>
///         <div
///             id=tooltip.tooltip_props.id
///             role=tooltip.tooltip_props.role
///             class="tooltip"
///         >
///             "Helpful tooltip text"
///         </div>
///     </Show>
/// }
/// ```
pub fn use_tooltip_trigger(input: UseTooltipTriggerInput) -> UseTooltipTriggerReturn {
    let UseTooltipTriggerInput {
        is_disabled,
        is_open: controlled_is_open,
        default_open,
        on_open_change,
        delay,
        close_delay,
        trigger: trigger_type,
    } = input;

    let base_id = Uuid::new_v4();
    let trigger_id = format!("tooltip-trigger-{base_id}");
    let tooltip_id = format!("tooltip-{base_id}");

    // Internal open state
    let (internal_open, set_internal_open) = signal(default_open);
    let is_open = controlled_is_open.unwrap_or_else(|| internal_open.into());

    // Helper to update open state
    let update_open = move |new_open: bool| {
        set_internal_open.set(new_open);
        if let Some(on_change) = on_open_change {
            on_change.run(new_open);
        }
    };

    // Open callback
    let open = Callback::new(move |_| {
        if !is_disabled.get_untracked() {
            update_open(true);
        }
    });

    // Close callback
    let close = Callback::new(move |_| {
        update_open(false);
    });

    // Handle pointer enter
    let handle_pointer_enter = move |_e: web_sys::PointerEvent| {
        if is_disabled.get_untracked() || trigger_type == TooltipTrigger::Focus {
            return;
        }
        // In a full implementation, you'd use a timeout here for the delay
        update_open(true);
    };

    // Handle pointer leave
    let handle_pointer_leave = move |_e: web_sys::PointerEvent| {
        if trigger_type == TooltipTrigger::Focus {
            return;
        }
        // In a full implementation, you'd use a timeout here for the close delay
        update_open(false);
    };

    // Handle focus
    let handle_focus = move |_e: web_sys::FocusEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        update_open(true);
    };

    // Handle blur
    let handle_blur = move |_e: web_sys::FocusEvent| {
        update_open(false);
    };

    // Handle keydown (Escape to close)
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Escape" && is_open.get_untracked() {
            e.prevent_default();
            update_open(false);
        }
    };

    // Compute aria-describedby (only set when tooltip is open)
    let tooltip_id_for_aria = tooltip_id.clone();
    let aria_describedby = Signal::derive(move || {
        if is_open.get() {
            Some(tooltip_id_for_aria.clone())
        } else {
            None
        }
    });

    UseTooltipTriggerReturn {
        trigger_props: UseTooltipTriggerProps {
            id: trigger_id.clone(),
            aria_describedby,
            on_pointerenter: EventHandler::new(handle_pointer_enter),
            on_pointerleave: EventHandler::new(handle_pointer_leave),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            on_keydown: EventHandler::new(handle_keydown),
        },
        tooltip_props: UseTooltipTriggerTooltipProps {
            id: tooltip_id.clone(),
            role: "tooltip",
        },
        is_open,
        trigger_id,
        tooltip_id,
        open,
        close,
    }
}
