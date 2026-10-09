// Upstream: react-aria/src/tooltip/useTooltipTrigger.ts @ 99e6102368
// Upstream: react-aria/test/tooltip/useTooltip.test.js @ 99e6102368
use leptos::{attr, attr::Attr, ev, prelude::*};
use web_sys::KeyboardEvent;

use super::use_tooltip_trigger_state::{TooltipTiming, TooltipTriggerState};
use crate::{
    EventHandler, IntoAttrs, OnEvent,
    hooks::focus::use_focus_visible::{Modality, get_modality},
    utils::{id::use_id, pointer_type::PointerType},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The state goes into the input (C8); `trigger` is the `TooltipTriggerMode` enum.
// - The trigger element isn't passed: the props carry the handlers react-aria's `useFocusable`
//   and `useHover` would add (pointer enter/leave, focus/blur, pointer and key down).
//
// ## DIFFERENT BEHAVIOR
// - Focus opens the tooltip unless the modality is the pointer's (react-aria: `isFocusVisible()`,
//   the same test).
//
// =============================================================================

/// Input parameters for the `use_tooltip_trigger` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseTooltipTriggerInput {
    /// The tooltip's state (from `use_tooltip_trigger_state`).
    pub state: TooltipTriggerState,

    /// Whether the tooltip is disabled. Default: `false`.
    pub is_disabled: Signal<bool>,

    /// The trigger behavior. Default: [`TooltipTriggerMode::Hover`].
    pub trigger: TooltipTriggerMode,

    /// Whether pressing the trigger should close the tooltip. Default: `true`.
    pub should_close_on_press: Signal<bool>,
}

/// What triggers the tooltip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipTriggerMode {
    /// Show on hover (and focus for accessibility).
    #[default]
    Hover,
    /// Show on focus only.
    Focus,
}

/// The return value of the `use_tooltip_trigger` hook.
#[derive(Debug)]
pub struct UseTooltipTriggerReturn {
    /// Props for the trigger element.
    pub trigger_props: UseTooltipTriggerProps,

    /// Props for the tooltip element (its id; `use_tooltip` adds the rest).
    pub tooltip_props: UseTooltipTriggerTooltipProps,
}

/// Props from `use_tooltip_trigger` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTooltipTriggerProps {
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
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    OnEvent<ev::pointerenter>,
    OnEvent<ev::pointerleave>,
    OnEvent<ev::focus>,
    OnEvent<ev::blur>,
    OnEvent<ev::keydown>,
    OnEvent<ev::pointerdown>,
);

/// Props for the tooltip element.
#[derive(Debug)]
pub struct UseTooltipTriggerTooltipProps {
    /// The id of the tooltip element, which describes the trigger while it is open.
    pub id: String,
}

/// Provides the behavior and accessibility for a tooltip trigger: the tooltip opens when the
/// trigger is hovered (after the state's delay; right away once another tooltip was open) or
/// focused by keyboard, and closes when it is left, blurred or pressed, or on Escape. While open,
/// the tooltip describes the trigger.
///
/// # Example
///
/// ```ignore
/// let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput::default());
/// let trigger = use_tooltip_trigger(UseTooltipTriggerInput {
///     state,
///     is_disabled: Signal::stored(false),
///     trigger: TooltipTriggerMode::Hover,
///     should_close_on_press: Signal::stored(true),
/// });
/// let tooltip = use_tooltip(UseTooltipInput {
///     state: Some(state),
///     ..UseTooltipInput::default()
/// });
///
/// view! {
///     <button {..trigger.trigger_props.into_attrs()}>"Hover me"</button>
///     <Show when=move || state.is_open()>
///         <div id=trigger.tooltip_props.id.clone() {..tooltip.props.into_attrs()}>
///             "Helpful tooltip text"
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_tooltip_trigger(input: UseTooltipTriggerInput) -> UseTooltipTriggerReturn {
    crate::hooks::focus::use_focus_visible::track_interaction_modality();
    let UseTooltipTriggerInput {
        state,
        is_disabled,
        trigger: trigger_type,
        should_close_on_press,
    } = input;

    let tooltip_id = use_id("tooltip");

    let is_open = state.overlay.is_open;

    // Track hover/focus state to coordinate show/hide.
    let is_hovered: StoredValue<bool> = StoredValue::new(false);
    let is_focused: StoredValue<bool> = StoredValue::new(false);

    // handle_show: open the tooltip if hovered or focused (immediately when focused).
    let handle_show = move || {
        if is_hovered.get_value() || is_focused.get_value() {
            state.open(if is_focused.get_value() {
                TooltipTiming::Immediate
            } else {
                TooltipTiming::Delayed
            });
        }
    };

    // handle_hide: close the tooltip if neither hovered nor focused.
    let handle_hide = move |immediate: bool| {
        if !is_hovered.get_value() && !is_focused.get_value() {
            if immediate {
                state.close(TooltipTiming::Immediate);
            } else {
                state.close(TooltipTiming::Delayed);
            }
        }
    };

    // --- Pointer handlers ---

    let handle_pointer_enter = move |e: web_sys::PointerEvent| {
        // Touch never hovers (as `useHover`).
        if is_disabled.get_untracked()
            || trigger_type == TooltipTriggerMode::Focus
            || PointerType::of(&e) == PointerType::Touch
        {
            return;
        }
        // Only count as hovered when the user is using a pointer. This prevents Chrome's phantom
        // hover events after keyboard interactions.
        is_hovered.set_value(get_modality() == Some(Modality::Pointer));
        handle_show();
    };

    let handle_pointer_leave = move |_e: web_sys::PointerEvent| {
        if trigger_type == TooltipTriggerMode::Focus {
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
        if get_modality() != Some(Modality::Pointer) {
            is_focused.set_value(true);
            handle_show();
        }
    };

    let handle_blur = move |_e: web_sys::FocusEvent| {
        is_focused.set_value(false);
        is_hovered.set_value(false);
        handle_hide(true);
    };

    // --- Press start (pointer down or key down on the trigger) ---
    let press_start = move || {
        if !should_close_on_press.get_untracked() {
            return;
        }
        is_focused.set_value(false);
        is_hovered.set_value(false);
        handle_hide(true);
    };
    let handle_keydown = move |_e: KeyboardEvent| press_start();
    let handle_pointer_down = move |_e: web_sys::PointerEvent| press_start();

    // --- Global Escape handler ---
    // Register a document-level Escape handler in capture phase when the tooltip
    // is open. This ensures Escape closes the tooltip even when focus is not on
    // the trigger element.
    #[cfg(not(feature = "ssr"))]
    {
        use leptos_use::{UseEventListenerOptions, use_document, use_event_listener_with_options};

        use crate::utils::key::{KeyboardEventKey, KeyboardKey};

        let document = use_document();
        Effect::new(move |_| {
            if is_open.get()
                && let Some(doc) = document.as_ref()
            {
                let _cleanup = use_event_listener_with_options(
                    doc.clone(),
                    ev::keydown,
                    move |e: KeyboardEvent| {
                        // Escape closes the tooltip only, not an enclosing overlay.
                        if e.typed_key() == KeyboardKey::Escape {
                            e.stop_propagation();
                            state.close(TooltipTiming::Immediate);
                        }
                    },
                    UseEventListenerOptions::default().capture(true),
                );
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
            aria_describedby,
            on_pointerenter: EventHandler::new(handle_pointer_enter),
            on_pointerleave: EventHandler::new(handle_pointer_leave),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            on_keydown: EventHandler::new(handle_keydown),
            on_pointerdown: EventHandler::new(handle_pointer_down),
        },
        tooltip_props: UseTooltipTriggerTooltipProps { id: tooltip_id },
    }
}
