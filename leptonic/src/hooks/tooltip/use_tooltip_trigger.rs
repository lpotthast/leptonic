// Upstream: react-aria/src/tooltip/useTooltipTrigger.ts @ 99e6102368
// Upstream: react-aria/test/tooltip/useTooltip.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::KeyboardEvent;

use super::use_tooltip_trigger_state::{TooltipTiming, TooltipTriggerState};
use crate::{
    EventHandler, Id, IntoAttrs, OnEvent, Part,
    hooks::focus::use_focus_visible::{Modality, get_modality},
    utils::{
        id::use_id,
        part::Remount,
        pointer_type::PointerType,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        slot_id::referenced_id,
        stored_field::StoredField,
    },
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
// - The tooltip describes the trigger (`aria-describedby`) while it is open and its element is
//   rendered (after mount, as `use_slot`'s references; react-aria: while open), so an open
//   tooltip without its element leaves no reference to a missing element.
// - Blurring the trigger leaves the tooltip open while the pointer is on it (react-aria closes it
//   right away): Pressing into the tooltip, e.g. to select its text, takes the focus from the
//   trigger. It closes once the pointer leaves it (`use_tooltip`).
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

/// The return value of [`use_tooltip_trigger`].
#[derive(Debug)]
pub struct UseTooltipTriggerReturn {
    /// The trigger element.
    pub trigger: Part<UseTooltipTriggerProps>,
    /// The tooltip element (its id; `use_tooltip` adds the rest): it describes the trigger while
    /// it is rendered.
    pub tooltip: Part<UseTooltipTriggerTooltipProps>,
}

/// Props from `use_tooltip_trigger` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTooltipTriggerProps {
    pub aria_describedby: Signal<Option<Id>>,
    pub on_pointerenter: EventHandler<web_sys::PointerEvent>,
    pub on_pointerleave: EventHandler<web_sys::PointerEvent>,
    pub on_focus: EventHandler<web_sys::FocusEvent>,
    pub on_blur: EventHandler<web_sys::FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_pointerdown: EventHandler<web_sys::PointerEvent>,
}

impl Remount for UseTooltipTriggerProps {
    fn remount(&self) -> Self {
        Self {
            aria_describedby: self.aria_describedby,
            on_pointerenter: self.on_pointerenter.clone(),
            on_pointerleave: self.on_pointerleave.clone(),
            on_focus: self.on_focus.clone(),
            on_blur: self.on_blur.clone(),
            on_keydown: self.on_keydown.clone(),
            on_pointerdown: self.on_pointerdown.clone(),
        }
    }
}

impl IntoAttrs for UseTooltipTriggerProps {
    type Attrs = UseTooltipTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            signal_attr("aria-describedby", self.aria_describedby),
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
    SignalAttr<Option<Id>>,
    OnEvent<ev::pointerenter>,
    OnEvent<ev::pointerleave>,
    OnEvent<ev::focus>,
    OnEvent<ev::blur>,
    OnEvent<ev::keydown>,
    OnEvent<ev::pointerdown>,
);

/// Props for the tooltip element: it describes the trigger while it is open and rendered.
#[derive(Debug)]
pub struct UseTooltipTriggerTooltipProps {
    /// The id of the tooltip element.
    pub id: Id,
}

pub type UseTooltipTriggerTooltipAttrs = FixedAttr;

impl IntoAttrs for UseTooltipTriggerTooltipProps {
    type Attrs = UseTooltipTriggerTooltipAttrs;

    fn into_attrs(self) -> Self::Attrs {
        fixed_attr("id", self.id)
    }
}

/// Provides the behavior and accessibility for a tooltip trigger: the tooltip opens when the
/// trigger is hovered (after the state's delay; right away once another tooltip was open) or
/// focused by keyboard, and closes when it is left, blurred (unless the pointer is on the tooltip)
/// or pressed, or on Escape. While open, the tooltip describes the trigger.
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
/// // Parts: the tooltip renders again each time it opens.
/// let (trigger_tooltip, tooltip) = (trigger.tooltip, tooltip.tooltip);
/// view! {
///     <button {..trigger.trigger.attrs()}>"Hover me"</button>
///     <Show when=move || state.is_open()>
///         <div {..trigger_tooltip.attrs()} {..tooltip.attrs()}>
///             "Helpful tooltip text"
///         </div>
///     </Show>
/// }
/// ```
pub fn use_tooltip_trigger(input: UseTooltipTriggerInput) -> UseTooltipTriggerReturn {
    crate::metrics::subject!();
    let UseTooltipTriggerInput {
        state,
        is_disabled,
        trigger: trigger_type,
        should_close_on_press,
    } = input;
    crate::hooks::focus::use_focus_visible::track_interaction_modality();

    let tooltip_id = use_id("tooltip");

    let is_open = state.overlay.is_open;

    // Track hover/focus state to coordinate show/hide.
    // One `StoredValue` for both (`performance.md` R7).
    let interaction = StoredValue::new_local((false, false));
    let is_hovered = StoredField::new(interaction, |s| &s.0, |s| &mut s.0);
    let is_focused = StoredField::new(interaction, |s| &s.1, |s| &mut s.1);

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
    let handle_hide = move |timing: TooltipTiming| {
        if !is_hovered.get_value() && !is_focused.get_value() {
            state.close(timing);
        }
    };

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
        handle_hide(TooltipTiming::Delayed);
    };

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

    // Whether the pointer is on the open tooltip (whose hovering keeps it open, see `use_tooltip`).
    let hovered_tooltip_id = tooltip_id.clone();
    let is_tooltip_hovered = move || {
        leptos_use::use_document()
            .as_ref()
            .and_then(|document| document.get_element_by_id(&hovered_tooltip_id))
            .is_some_and(|tooltip| tooltip.matches(":hover").unwrap_or(false))
    };

    let handle_blur = move |_e: web_sys::FocusEvent| {
        is_focused.set_value(false);
        is_hovered.set_value(false);
        // Pressing into the tooltip, e.g. to select its text, takes the focus from the trigger.
        if !is_tooltip_hovered() {
            handle_hide(TooltipTiming::Immediate);
        }
    };

    // Pressing the trigger (pointer down or key down) closes the tooltip.
    let press_start = move || {
        if !should_close_on_press.get_untracked() {
            return;
        }
        is_focused.set_value(false);
        is_hovered.set_value(false);
        handle_hide(TooltipTiming::Immediate);
    };
    let handle_keydown = move |_e: KeyboardEvent| press_start();
    let handle_pointer_down = move |_e: web_sys::PointerEvent| press_start();

    // Register a document-level Escape handler in capture phase when the tooltip
    // is open. This ensures Escape closes the tooltip even when focus is not on
    // the trigger element.
    #[cfg(not(feature = "ssr"))]
    {
        use leptos_use::use_document;
        use wasm_bindgen::JsCast;

        use crate::utils::{
            event_listeners::listen,
            key::{KeyboardEventKey, KeyboardKey},
        };

        let document = use_document();
        Effect::new(move |_| {
            if is_open.get()
                && let Some(doc) = document.as_ref()
            {
                // Removed when the tooltip closes (the effect's next run) or unmounts.
                let listener = listen(doc.as_ref(), "keydown", true, move |e: web_sys::Event| {
                    let Some(e) = e.dyn_ref::<KeyboardEvent>() else {
                        return;
                    };
                    // Escape closes the tooltip only, not an enclosing overlay.
                    if e.typed_key() == KeyboardKey::Escape {
                        e.stop_propagation();
                        state.close(TooltipTiming::Immediate);
                    }
                });
                let listener = send_wrapper::SendWrapper::new(listener);
                on_cleanup(move || drop(listener));
            }
        });
    }

    // The tooltip describes the trigger while it is open, and rendered.
    let tooltip = Part::observed({
        let id = tooltip_id.clone();
        move || UseTooltipTriggerTooltipProps { id: id.clone() }
    });
    let tooltip_rendered = referenced_id(&tooltip, tooltip_id);
    let aria_describedby =
        Signal::derive(move || is_open.get().then(|| tooltip_rendered.get()).flatten());

    UseTooltipTriggerReturn {
        trigger: Part::stored(UseTooltipTriggerProps {
            aria_describedby,
            on_pointerenter: EventHandler::new(handle_pointer_enter),
            on_pointerleave: EventHandler::new(handle_pointer_leave),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            on_keydown: EventHandler::new(handle_keydown),
            on_pointerdown: EventHandler::new(handle_pointer_down),
        }),
        tooltip,
    }
}
