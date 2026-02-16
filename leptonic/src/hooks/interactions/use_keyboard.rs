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
        propagation_control::{PropagationControl, Sealed},
        EventHandler, EventWrapper, Propagation,
    },
};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useKeyboard.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
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

    // -- Keyboard-specific accessors ------------------------------------------

    /// Get the key that was pressed.
    pub fn key(&self) -> String {
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
#[derive(Debug, Clone, Copy)]
pub struct UseKeyboardInput {
    /// Whether keyboard events should be disabled.
    pub disabled: Signal<bool>,

    /// Handler called when a key is pressed down.
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,

    /// Handler called when a key is released.
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

/// The return value of the `use_keyboard` hook.
#[derive(Debug)]
pub struct UseKeyboardReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseKeyboardProps,
}

/// Props from `use_keyboard` that can be extracted and merged programmatically.
#[derive(Debug)]
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
        disabled,
        on_key_down,
        on_key_up,
    } = input;

    let handle_key_down = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        if let Some(on_key_down) = on_key_down {
            let (wrapper, continue_state) = KeyboardEventWrapper::new(e.clone());
            on_key_down.run(wrapper);

            if !continue_state.load(Ordering::Acquire) {
                e.stop_propagation();
            }
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
