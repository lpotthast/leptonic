// Upstream: react-aria/src/interactions/useKeyboard.ts @ 99e6102368
use std::sync::atomic::Ordering;

use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::KeyboardEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler, EventWrapper, Propagation,
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
// ## OMITTED FEATURES
// - Ignoring events from React portals: React re-dispatches events through the component tree,
//   so react-aria must skip events whose target is not a DOM descendant. Leptos uses native DOM
//   events, which only ever bubble through DOM ancestors.
//
// =============================================================================

/// A keyboard event with additional functionality.
///
/// Wraps a [`KeyboardEvent`] with propagation control following react-aria
/// semantics: events stop propagation by default; call
/// [`continue_propagation()`](Self::continue_propagation) to opt in to bubbling.
pub struct KeyboardEventWrapper {
    inner: EventWrapper<KeyboardEvent>,
}

impl Sealed for KeyboardEventWrapper {}
impl Propagation for KeyboardEventWrapper {
    fn propagation_control(&self) -> &PropagationControl {
        self.inner.propagation_control()
    }
}

impl KeyboardEventWrapper {
    /// Create a new keyboard event wrapper.
    fn new(event: KeyboardEvent) -> (Self, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let (inner, state) = EventWrapper::new(event);
        (Self { inner }, state)
    }

    /// Access the underlying [`KeyboardEvent`].
    pub fn event(&self) -> &KeyboardEvent {
        self.inner.event()
    }

    /// Prevent the browser's default action for this event.
    pub fn prevent_default(&self) {
        self.inner.prevent_default();
    }

    /// Whether `prevent_default()` has been called.
    pub fn is_default_prevented(&self) -> bool {
        self.inner.is_default_prevented()
    }

    /// Get the event target.
    pub fn target(&self) -> Option<web_sys::EventTarget> {
        self.inner.target()
    }

    /// Get the current target.
    pub fn current_target(&self) -> Option<web_sys::EventTarget> {
        self.inner.current_target()
    }

    /// The key that was pressed.
    pub fn key(&self) -> KeyboardKey {
        self.inner.event().typed_key()
    }

    /// The key's DOM value (`KeyboardEvent.key`, e.g. `"Escape"`, `"a"`), e.g. to display it.
    pub fn key_value(&self) -> String {
        self.inner.event().key()
    }

    /// Get the key code.
    pub fn code(&self) -> String {
        self.inner.event().code()
    }

    /// Whether this is a repeat event (key held down).
    pub fn repeat(&self) -> bool {
        self.inner.event().repeat()
    }

    /// Whether the shift key was held.
    pub fn shift_key(&self) -> bool {
        self.inner.event().shift_key()
    }

    /// Whether the ctrl key was held.
    pub fn ctrl_key(&self) -> bool {
        self.inner.event().ctrl_key()
    }

    /// Whether the alt key was held.
    pub fn alt_key(&self) -> bool {
        self.inner.event().alt_key()
    }

    /// Whether the meta key was held.
    pub fn meta_key(&self) -> bool {
        self.inner.event().meta_key()
    }
}

impl std::fmt::Debug for KeyboardEventWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyboardEventWrapper")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Clone for KeyboardEventWrapper {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
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
pub type UseKeyboardAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
);

/// Handles keyboard interactions for a focusable element.
///
/// This hook provides keyboard event handling with support for disabling
/// and controlling event propagation.
///
/// # Example
///
/// ```ignore
/// let keyboard = use_keyboard(UseKeyboardInput {
///     disabled: Signal::derive(|| false),
///     on_key_down: Some(Callback::new(|e| {
///         if e.key() == "Enter" {
///             // Handle enter key
///             e.prevent_default();
///         } else {
///             e.continue_propagation();
///         }
///     })),
///     on_key_up: None,
/// });
///
/// view! {
///     <div tabindex="0" {..keyboard.attrs}>
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
            let (wrapper, continue_state) = KeyboardEventWrapper::new(e.clone());
            on_key_down.run(wrapper);
            continue_propagation = continue_state.load(Ordering::Acquire);
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
            let (wrapper, continue_state) = KeyboardEventWrapper::new(e.clone());
            on_key_up.run(wrapper);

            if !continue_state.load(Ordering::Acquire) {
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
