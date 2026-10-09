// Upstream: react-aria/src/interactions/useKeyboard.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useKeyboard.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::KeyboardEvent;

use crate::{
    EventHandler, IntoAttrs, OnEvent, Propagation,
    utils::{
        key::{KeyboardEventKey, KeyboardKey},
        keyboard_shortcut::KeyboardShortcuts,
        propagation_control::{PropagationControl, Sealed},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `shortcuts`: typed `KeyboardShortcuts` built from `Shortcut` values instead of a record of
//   shortcut strings. `Shortcut::parse` still accepts react-aria's string syntax.
//
// ## DIFFERENT BEHAVIOR
// - With `shortcuts`, `on_key_down` and `on_key_up` still get a `KeyboardEventWrapper` and stop
//   their events unless they call `continue_propagation()`, as without shortcuts. react-aria then
//   chains them unwrapped (they get the raw event, which has no `continuePropagation`), so a key
//   no shortcut handles, and every key up, propagates. One propagation model for every handler,
//   with or without shortcuts.
//
// ## OMITTED FEATURES
// - Ignoring events from React portals: React re-dispatches events through the component tree,
//   so react-aria must skip events whose target is not a DOM descendant. Leptos uses native DOM
//   events, which only ever bubble through DOM ancestors.
//
// =============================================================================

/// A keyboard event whose propagation the handler controls (react-aria's
/// `BaseEvent<KeyboardEvent>`): it stops propagating by default; call
/// [`continue_propagation()`](Propagation::continue_propagation) to let it bubble.
#[derive(Debug, Clone)]
pub struct KeyboardEventWrapper {
    event: KeyboardEvent,
    propagation: PropagationControl,
}

impl Sealed for KeyboardEventWrapper {}
impl Propagation for KeyboardEventWrapper {
    fn propagation_control(&self) -> &PropagationControl {
        &self.propagation
    }
}

impl KeyboardEventWrapper {
    /// Wraps `event`, sharing `propagation` with the caller, who stops the event afterwards unless
    /// the handler continued it.
    pub(crate) fn new(event: KeyboardEvent, propagation: &PropagationControl) -> Self {
        Self {
            event,
            propagation: propagation.clone(),
        }
    }

    /// Access the underlying [`KeyboardEvent`].
    pub fn event(&self) -> &KeyboardEvent {
        &self.event
    }

    /// Prevent the browser's default action for this event.
    pub fn prevent_default(&self) {
        self.event.prevent_default();
    }

    /// Whether `prevent_default()` has been called.
    pub fn is_default_prevented(&self) -> bool {
        self.event.default_prevented()
    }

    /// Get the event target.
    pub fn target(&self) -> Option<web_sys::EventTarget> {
        self.event.target()
    }

    /// Get the current target.
    pub fn current_target(&self) -> Option<web_sys::EventTarget> {
        self.event.current_target()
    }

    /// The key that was pressed.
    pub fn key(&self) -> KeyboardKey {
        self.event.typed_key()
    }

    /// The key's DOM value (`KeyboardEvent.key`, e.g. `"Escape"`, `"a"`), e.g. to display it.
    pub fn key_value(&self) -> String {
        self.event.key()
    }

    /// Get the key code.
    pub fn code(&self) -> String {
        self.event.code()
    }

    /// Whether this is a repeat event (key held down).
    pub fn repeat(&self) -> bool {
        self.event.repeat()
    }

    /// Whether the shift key was held.
    pub fn shift_key(&self) -> bool {
        self.event.shift_key()
    }

    /// Whether the ctrl key was held.
    pub fn ctrl_key(&self) -> bool {
        self.event.ctrl_key()
    }

    /// Whether the alt key was held.
    pub fn alt_key(&self) -> bool {
        self.event.alt_key()
    }

    /// Whether the meta key was held.
    pub fn meta_key(&self) -> bool {
        self.event.meta_key()
    }
}

/// Input parameters for the `use_keyboard` hook.
#[derive(Debug, Clone, Default)]
pub struct UseKeyboardInput {
    /// Whether keyboard events should be disabled.
    pub is_disabled: Signal<bool>,

    /// Handler called when a key is pressed down.
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,

    /// Handler called when a key is released.
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,

    /// Shortcuts handled on key down, after `on_key_down`. A handled shortcut prevents the
    /// browser default and stops propagation; key presses that match no shortcut bubble on.
    pub shortcuts: Option<KeyboardShortcuts>,

    /// Whether shortcuts also fire for auto-repeated key presses (a key held down). Enable this
    /// for navigation keys, so that holding an arrow key keeps moving.
    pub allow_repeats: bool,

    /// Whether shortcuts also fire while an input method editor is composing text.
    pub allow_composing: bool,
}

/// The return value of the `use_keyboard` hook.
#[derive(Debug)]
pub struct UseKeyboardReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseKeyboardProps,
}

/// Props from `use_keyboard` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseKeyboardProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseKeyboardProps {
    type Attrs = UseKeyboardAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
        )
    }
}

/// These attributes must be spread onto the target element.
pub type UseKeyboardAttrs = (OnEvent<ev::keydown>, OnEvent<ev::keyup>);

/// Handles keyboard interactions for a focusable element.
///
/// This hook provides keyboard event handling with support for disabling
/// and controlling event propagation.
///
/// # Example
///
/// ```ignore
/// let keyboard = use_keyboard(UseKeyboardInput {
///     on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
///         if e.key() == KeyboardKey::Enter {
///             // Handle enter key
///             e.prevent_default();
///         } else {
///             e.continue_propagation();
///         }
///     })),
///     ..UseKeyboardInput::default()
/// });
///
/// view! {
///     <div tabindex="0" {..keyboard.props.into_attrs()}>
///         "Press a key"
///     </div>
/// }
/// ```
pub fn use_keyboard(input: UseKeyboardInput) -> UseKeyboardReturn {
    let UseKeyboardInput {
        is_disabled: disabled,
        on_key_down,
        on_key_up,
        shortcuts,
        allow_repeats,
        allow_composing,
    } = input;

    let handle_key_down = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        // Without a handler, events bubble. A handler stops propagation unless it opts in.
        let mut continue_propagation = true;

        if let Some(on_key_down) = on_key_down {
            let propagation = PropagationControl::new();
            on_key_down.run(KeyboardEventWrapper::new(e.clone(), &propagation));
            continue_propagation = !propagation.is_propagation_stopped();
        }

        if let Some(shortcuts) = &shortcuts
            && (allow_repeats || !e.repeat())
            && (allow_composing || !e.is_composing())
            && let Some(outcome) = shortcuts.handle(&e)
        {
            if outcome.prevent_default() {
                e.prevent_default();
            }
            continue_propagation &= outcome.continue_propagation();
        }

        if !continue_propagation {
            e.stop_propagation();
        }
    };

    let handle_key_up = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        if let Some(on_key_up) = on_key_up {
            let propagation = PropagationControl::new();
            on_key_up.run(KeyboardEventWrapper::new(e.clone(), &propagation));
            if propagation.is_propagation_stopped() {
                e.stop_propagation();
            }
        }
    };

    UseKeyboardReturn {
        props: UseKeyboardProps {
            on_keydown: EventHandler::new(handle_key_down),
            on_keyup: EventHandler::new(handle_key_up),
        },
    }
}
