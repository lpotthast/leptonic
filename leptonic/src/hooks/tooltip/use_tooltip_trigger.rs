use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::use_tooltip_trigger_state::UseTooltipTriggerStateReturn;
use crate::{
    hooks::{
        focus::use_focus_visible::{get_modality, Modality},
        IntoAttrs,
    },
    utils::{aria::AriaRole, EventHandler},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tooltip/src/useTooltipTrigger.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - State is passed as a separate parameter (`UseTooltipTriggerStateReturn`)
//   instead of being embedded in the hook, matching react-aria's
//   `useTooltipTrigger(props, state, ref)` pattern.
//
// - Uses `StoredValue<bool, LocalStorage>` for `is_hovered` and `is_focused`
//   tracking instead of React refs.
//
// - Global Escape handler uses `Effect` + `leptos_use::use_event_listener`
//   on the document, active only when `state.is_open` is true.
//
// - `get_modality()` from `use_focus_visible` is used instead of react-aria's
//   `getInteractionModality()` for focus-visible checks and hover modality
//   filtering.
//
// ## DIFFERENT BEHAVIOR
//
// - No `mousedown` fallback handler: We assume PointerEvent is always available
//   (per CLAUDE.md).
//
// =============================================================================

/// Input parameters for the `use_tooltip_trigger` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipTriggerInput {
    /// Whether the tooltip is disabled.
    pub is_disabled: Signal<bool>,

    /// The trigger behavior.
    pub trigger: TooltipTrigger,

    /// Whether pressing the trigger should close the tooltip. Default: `true`.
    pub should_close_on_press: bool,
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
            trigger: TooltipTrigger::Hover,
            should_close_on_press: true,
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
    pub on_pointerdown: EventHandler<web_sys::PointerEvent>,
}

impl IntoAttrs for UseTooltipTriggerProps {
    type Attrs = UseTooltipTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_keydown.into_on(ev::keydown),
            self.on_pointerdown.into_on(ev::pointerdown),
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
    On<ev::pointerdown, SharedEventCallback<web_sys::PointerEvent>>,
);

/// Props for the tooltip element.
#[derive(Debug)]
pub struct UseTooltipTriggerTooltipProps {
    /// The id of the tooltip element.
    pub id: String,

    /// The role attribute.
    pub role: AriaRole,
}

/// Provides the behavior and accessibility for a tooltip trigger.
///
/// A tooltip displays brief helper text or information about an element when
/// the user hovers over or focuses on the element.
///
/// This hook delegates open/close behavior to the `state` parameter, which
/// implements the warmup/cooldown system via `use_tooltip_trigger_state`.
///
/// # Example
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput::default());
/// let tooltip = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
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
#[allow(clippy::too_many_lines)]
pub fn use_tooltip_trigger(
    input: UseTooltipTriggerInput,
    state: UseTooltipTriggerStateReturn,
) -> UseTooltipTriggerReturn {
    let UseTooltipTriggerInput {
        is_disabled,
        trigger: trigger_type,
        should_close_on_press,
    } = input;

    let base_id = Uuid::new_v4();
    let trigger_id = format!("tooltip-trigger-{base_id}");
    let tooltip_id = format!("tooltip-{base_id}");

    let is_open = state.is_open;

    // Track hover/focus state to coordinate show/hide.
    let is_hovered: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);
    let is_focused: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

    // handle_show: open the tooltip if hovered or focused.
    let handle_show = move || {
        if is_hovered.get_value() || is_focused.get_value() {
            state.open.run(false);
        }
    };

    // handle_hide: close the tooltip if neither hovered nor focused.
    let handle_hide = move |immediate: bool| {
        if !is_hovered.get_value() && !is_focused.get_value() {
            if immediate {
                state.close.run(true);
            } else {
                state.close.run(false);
            }
        }
    };

    // --- Pointer handlers ---

    let handle_pointer_enter = move |_e: web_sys::PointerEvent| {
        if is_disabled.get_untracked() || trigger_type == TooltipTrigger::Focus {
            return;
        }
        // Only set hovered when the user is using a pointer (mouse/pen/touch).
        // This prevents Chrome's phantom hover events after keyboard interactions.
        if get_modality() == Modality::Pointer {
            is_hovered.set_value(true);
            handle_show();
        }
    };

    let handle_pointer_leave = move |_e: web_sys::PointerEvent| {
        if trigger_type == TooltipTrigger::Focus {
            return;
        }
        is_hovered.set_value(false);
        // Also reset focus tracking on hover end (matches react-aria).
        is_focused.set_value(false);
        handle_hide(false);
    };

    // --- Focus handlers ---

    let handle_focus = move |_e: web_sys::FocusEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        // Only show tooltip on focus when it's keyboard/virtual focus,
        // not pointer focus (clicking to focus should not show tooltip).
        if get_modality() != Modality::Pointer {
            is_focused.set_value(true);
            handle_show();
        }
    };

    let handle_blur = move |_e: web_sys::FocusEvent| {
        is_focused.set_value(false);
        is_hovered.set_value(false);
        handle_hide(true);
    };

    // --- Keydown handler (local, for Escape) ---
    // This is a local handler as a fallback. The global Escape handler
    // on the document (below) handles the primary case, but this ensures
    // the tooltip closes even if the global handler is somehow bypassed.
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Escape" && is_open.get_untracked() {
            e.prevent_default();
            state.close.run(true);
        }
    };

    // --- Press handler (for should_close_on_press) ---
    let handle_pointer_down = move |_e: web_sys::PointerEvent| {
        if should_close_on_press && is_open.get_untracked() {
            state.close.run(true);
        }
    };

    // --- Global Escape handler ---
    // Register a document-level Escape handler in capture phase when the tooltip
    // is open. This ensures Escape closes the tooltip even when focus is not on
    // the trigger element.
    #[cfg(not(feature = "ssr"))]
    {
        use leptos_use::{use_document, use_event_listener_with_options, UseEventListenerOptions};

        let document = use_document();
        Effect::new(move |_| {
            if is_open.get() {
                if let Some(doc) = document.as_ref() {
                    let _cleanup = use_event_listener_with_options(
                        doc.clone(),
                        ev::keydown,
                        move |e: KeyboardEvent| {
                            if e.key() == "Escape" {
                                e.prevent_default();
                                state.close.run(true);
                            }
                        },
                        UseEventListenerOptions::default().capture(true),
                    );
                }
            }
        });
    }

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
            on_pointerdown: EventHandler::new(handle_pointer_down),
        },
        tooltip_props: UseTooltipTriggerTooltipProps {
            id: tooltip_id.clone(),
            role: AriaRole::Tooltip,
        },
        is_open,
        trigger_id,
        tooltip_id,
    }
}
