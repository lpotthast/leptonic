use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use web_sys::KeyboardEvent;

use crate::utils::EventHandler;
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useKeyboard.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// A keyboard event with additional functionality.
#[derive(Debug, Clone)]
pub struct KeyboardEventWrapper {
    /// The underlying keyboard event.
    pub event: KeyboardEvent,
    /// Shared state for propagation control.
    continue_propagation_state: Arc<AtomicBool>,
}

impl KeyboardEventWrapper {
    /// Create a new keyboard event wrapper.
    fn new(event: KeyboardEvent) -> (Self, Arc<AtomicBool>) {
        let state = Arc::new(AtomicBool::new(false));
        (
            Self {
                event,
                continue_propagation_state: state.clone(),
            },
            state,
        )
    }

    /// Call this to allow parent handlers to also handle this event.
    /// By default, keyboard events stop propagation.
    pub fn continue_propagation(&self) {
        self.continue_propagation_state
            .store(true, Ordering::Release);
    }

    /// Get the key that was pressed.
    pub fn key(&self) -> String {
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

    /// Prevent the default action.
    pub fn prevent_default(&self) {
        self.event.prevent_default();
    }

    /// Stop propagation of the event.
    pub fn stop_propagation(&self) {
        self.event.stop_propagation();
    }

    /// Get the event target.
    pub fn target(&self) -> Option<web_sys::EventTarget> {
        self.event.target()
    }

    /// Get the current target.
    pub fn current_target(&self) -> Option<web_sys::EventTarget> {
        self.event.current_target()
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
#[derive(Debug, Clone)]
pub struct UseKeyboardReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseKeyboardProps,
}

/// Props from `use_keyboard` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseKeyboardProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
}

impl UseKeyboardProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseKeyboardAttrs {
        (
            self.on_keydown.to_on(ev::keydown),
            self.on_keyup.to_on(ev::keyup),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseKeyboardAttrs {
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
