use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tooltip/src/useTooltipTrigger.ts

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
    pub trigger_props: UseTooltipTriggerAttrs,

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
#[derive(Debug, Clone)]
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
///     <button {..tooltip.trigger_props}>
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
    let base_id = Uuid::new_v4();
    let trigger_id = format!("tooltip-trigger-{base_id}");
    let tooltip_id = format!("tooltip-{base_id}");

    let is_disabled = input.is_disabled;
    let on_open_change = input.on_open_change;
    let trigger_type = input.trigger;

    // Internal open state
    let (internal_open, set_internal_open) = signal(input.default_open);
    let is_open = input.is_open.unwrap_or_else(|| internal_open.into());

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
        trigger_props: (
            Attr(attr::Id, trigger_id.clone()),
            Attr(attr::AriaDescribedby, aria_describedby),
            on(ev::pointerenter, handle_pointer_enter).into_cloneable(),
            on(ev::pointerleave, handle_pointer_leave).into_cloneable(),
            on(ev::focus, handle_focus).into_cloneable(),
            on(ev::blur, handle_blur).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
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

/// State for managing tooltip visibility.
#[derive(Clone, Copy)]
pub struct UseTooltipTriggerStateReturn {
    /// Whether the tooltip is open.
    pub is_open: Signal<bool>,

    /// Open the tooltip.
    pub open: Callback<()>,

    /// Close the tooltip.
    pub close: Callback<()>,
}

/// Creates internal state for a tooltip trigger.
pub fn use_tooltip_trigger_state(default_open: bool) -> UseTooltipTriggerStateReturn {
    let (is_open, set_is_open) = signal(default_open);

    UseTooltipTriggerStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
    }
}
