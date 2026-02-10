use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use crate::{
    utils::{EventListenerOptions, ListenExt},
    Out,
};

#[cfg(not(feature = "ssr"))]
use leptos::ev;

use atomic_enum::atomic_enum;

#[cfg(not(feature = "ssr"))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock, RwLock,
};
#[cfg(not(feature = "ssr"))]
use web_sys::{KeyboardEvent, PointerEvent};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/interactions/src/useFocusVisible.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - Key filtering: Positive filter vs react-aria's negative filter
//   React-aria: `isValidKey` accepts all keys except modifiers (Shift, Control,
//   Alt, Meta). Any non-modifier key (including typing characters) sets keyboard
//   modality. Compensated by `isKeyboardFocusEvent()` which restricts text inputs
//   to only respond to Tab/Escape.
//   Leptonic: `is_keyboard_focus_key` only accepts navigation keys (Tab, Escape,
//   arrows, Home, End, PageUp, PageDown, Enter, Space). Typing characters never
//   trigger keyboard modality. This avoids the need for `isTextInput` special-
//   casing.
//   Rationale: More conservative approach that works correctly without needing
//   to identify text input elements.
//
// - No `isTextInput` support
//   Rationale: Unnecessary given our positive key filter approach above.
//
// - No `HTMLElement.prototype.focus()` override for virtual/programmatic focus
//   React-aria overrides `HTMLElement.prototype.focus` to set a
//   `hasEventBeforeFocus` flag, enabling detection of programmatic focus (e.g.
//   screen readers) that should count as focus-visible. We do not implement this
//   override. Screen reader users may not always get focus rings as a result.
//   This is a known limitation and a potential future enhancement.
//
// =============================================================================

/// Input parameters for the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseFocusVisibleInput {
    /// Whether to auto-focus the element (affects initial visibility).
    pub auto_focus: bool,
}

/// The return value of the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusVisibleReturn {
    /// Whether the element should display a focus ring.
    /// True when the user is navigating via keyboard.
    pub focus_should_be_visible: Signal<bool>,
}

/// Tracks whether focus (of the currently focused element) should be made visible.
///
/// This hook subscribes to the current interaction modality:
/// - When the user interacts with the keyboard or virtually (though assistive technology), focus
///   is made visible.
/// - When they use a pointer (mouse, touch, ...), focus is hidden.
///
/// # Example
///
/// ```ignore
/// let UseFocusVisibleReturn { focus_should_be_visible } = use_focus_visible(UseFocusVisibleInput::default());
///
/// view! {
///     <button class:focus-visible=move || focus_should_be_visible.get()>
///         "Button"
///     </button>
/// }
/// ```
pub fn use_focus_visible(input: UseFocusVisibleInput) -> UseFocusVisibleReturn {
    let UseFocusVisibleInput { auto_focus } = input;

    #[cfg(feature = "ssr")]
    {
        let (is_focus_visible, _) = signal(auto_focus);
        return UseFocusVisibleReturn {
            focus_should_be_visible: is_focus_visible.into(),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        let state = FocusState::get();

        if !state.handlers_setup.swap(true, Ordering::AcqRel) {
            setup_global_listeners(state);
        }

        let (is_focus_visible, set_is_focus_visible) =
            signal(auto_focus || state.modality() == Modality::Keyboard);

        let id = state.register(move |modality| {
            set_is_focus_visible.set(modality == Modality::Keyboard);
        });

        on_cleanup(move || {
            state.unregister(id);
        });

        UseFocusVisibleReturn {
            focus_should_be_visible: is_focus_visible.into(),
        }
    }
}

#[cfg(not(feature = "ssr"))]
static FOCUS_STATE: OnceLock<FocusState> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
type SubscriberId = u64;

/// An input modality.
#[derive(PartialEq, Eq)]
#[atomic_enum]
pub enum Modality {
    Unknown = 0,
    Pointer,
    Keyboard,
    Virtual,
}

/// Global focus state management, tracking:
///
/// - The current interaction modality (keyboard, pointer, or virtual).
/// - Registered subscribers that are notified when the modality changes.
#[cfg(not(feature = "ssr"))]
struct FocusState {
    /// The current interaction modality (Unknown, Pointer, Keyboard, or Virtual).
    modality: AtomicModality,

    /// Whether global event handler were already set up.
    handlers_setup: AtomicBool,

    /// Whether a keyboard/pointer event occurred before the current focus event.
    /// Used to detect programmatic/virtual focus (screen readers).
    has_event_before_focus: AtomicBool,

    /// Whether the window was recently blurred. Used to avoid false positives
    /// when returning to the tab.
    has_blurred_window_recently: AtomicBool,

    /// Next unique ID for subscriber registration.
    next_id: std::sync::atomic::AtomicU64,

    /// Registered subscribers that receive modality change notifications.
    /// Each subscriber is an `(id, Out<Modality>)` pair. The ID is used for unregistration.
    subscribers: RwLock<Vec<(SubscriberId, Out<Modality>)>>,
}

#[cfg(not(feature = "ssr"))]
impl FocusState {
    fn new() -> Self {
        Self {
            modality: AtomicModality::new(Modality::Unknown),
            handlers_setup: AtomicBool::new(false),
            has_event_before_focus: AtomicBool::new(false),
            has_blurred_window_recently: AtomicBool::new(false),
            next_id: std::sync::atomic::AtomicU64::new(0),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    fn get() -> &'static Self {
        FOCUS_STATE.get_or_init(FocusState::new)
    }

    fn modality(&self) -> Modality {
        self.modality.load(Ordering::Acquire)
    }

    /// Note: Subscribers are only notified if the value CHANGED.
    /// Frequent calls with the same value will be ignored.
    fn set_modality_and_notify_subscribers(&self, modality: Modality) {
        let prev_modality = self.modality.swap(modality, Ordering::Release);
        if modality != prev_modality {
            self.notify_subscribers();
        }
    }

    fn has_event_before_focus(&self) -> bool {
        self.has_event_before_focus.load(Ordering::Acquire)
    }

    fn set_has_event_before_focus(&self, value: bool) {
        self.has_event_before_focus.store(value, Ordering::Release);
    }

    fn has_blurred_window_recently(&self) -> bool {
        self.has_blurred_window_recently.load(Ordering::Acquire)
    }

    fn set_has_blurred_window_recently(&self, value: bool) {
        self.has_blurred_window_recently
            .store(value, Ordering::Release);
    }

    /// Notify all registered subscribers of the current modality.
    fn notify_subscribers(&self) {
        let modality = self.modality();
        let subscribers = self.subscribers.read().expect("subscriber lock poisoned");
        for (_, subscriber) in &*subscribers {
            subscriber.set(modality);
        }
    }

    /// Register a subscriber signal. Returns a unique ID for unregistration.
    fn register(&self, subscriber: impl Into<Out<Modality>>) -> SubscriberId {
        // Wrapping around on overflow. Ensures endless usage.
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut subscribers = self.subscribers.write().expect("subscriber lock poisoned");
        subscribers.push((id, subscriber.into()));
        id
    }

    /// Unregister a subscriber by its unique ID.
    fn unregister(&self, id: SubscriberId) {
        let mut subscribers = self.subscribers.write().expect("subscriber lock poisoned");
        if let Some(pos) = subscribers.iter().position(|(sid, _)| *sid == id) {
            subscribers.swap_remove(pos);
        }
    }
}

/// Keys that indicate keyboard navigation.
#[cfg(not(feature = "ssr"))]
fn is_keyboard_focus_key(key: &str) -> bool {
    matches!(
        key,
        "Tab"
            | "Escape"
            | "ArrowUp"
            | "ArrowDown"
            | "ArrowLeft"
            | "ArrowRight"
            | "Home"
            | "End"
            | "PageUp"
            | "PageDown"
            | "Enter"
            | " "
    )
}

/// Sets up global event listeners to track interaction modality.
///
/// All listeners use capture phase so they fire before any element-level handlers.
/// When modality changes, all registered subscriber signals are notified directly,
/// eliminating the need for per-instance bubble-phase listeners.
#[cfg(not(feature = "ssr"))]
fn setup_global_listeners(state: &'static FocusState) {
    let Some(window) = web_sys::window() else {
        tracing::warn!("use_focus_visible: window not available");
        return;
    };
    let Some(document) = window.document() else {
        tracing::warn!("use_focus_visible: document not available");
        return;
    };

    setup_global_document_listeners(&document, state);
    setup_global_window_listeners(&window, state);
}

/// Sets up capture-phase document event listeners for modality tracking.
#[cfg(not(feature = "ssr"))]
fn setup_global_document_listeners(document: &web_sys::Document, state: &'static FocusState) {
    let handle_keyboard = move |e: KeyboardEvent| {
        if is_keyboard_focus_key(&e.key()) {
            state.set_has_event_before_focus(true);
            state.set_modality_and_notify_subscribers(Modality::Keyboard);
        }
    };

    let handle_pointer_event = move |_: PointerEvent| {
        state.set_has_event_before_focus(true);
        state.set_modality_and_notify_subscribers(Modality::Pointer);
    };

    let capturing = EventListenerOptions::capturing();

    document
        .listen(ev::keydown, handle_keyboard, capturing)
        .forget();

    document
        .listen(ev::keyup, handle_keyboard, capturing)
        .forget();

    document
        .listen(ev::pointerdown, handle_pointer_event, capturing)
        .forget();

    document
        .listen(ev::pointermove, handle_pointer_event, capturing)
        .forget();

    document
        .listen(ev::pointerup, handle_pointer_event, capturing)
        .forget();

    // Fallback for browsers without pointer events.
    document
        .listen(ev::mousedown, handle_pointer_event, capturing)
        .forget();
}

/// Sets up window-level listeners for blur/focus (tab-away/return detection).
#[cfg(not(feature = "ssr"))]
fn setup_global_window_listeners(window: &web_sys::Window, state: &'static FocusState) {
    // Reset state when tabbing away from the page.
    window
        .listen(
            ev::blur,
            move |_: web_sys::FocusEvent| {
                state.set_has_event_before_focus(false);
                state.set_has_blurred_window_recently(true);
            },
            EventListenerOptions::default(), // Run after everything else.
        )
        .forget();

    // Detect a return to the tab.
    window
        .listen(
            ev::focus,
            move |_: web_sys::FocusEvent| {
                // If a focus event occurs without a preceding keyboard or pointer event,
                // switch to virtual modality.
                // This occurs, for example, when navigating a form with the next/previous buttons
                // on iOS.
                if !state.has_event_before_focus() && !state.has_blurred_window_recently() {
                    state.set_modality_and_notify_subscribers(Modality::Virtual);
                }
                state.set_has_event_before_focus(false);
                state.set_has_blurred_window_recently(false);
            },
            EventListenerOptions::capturing(), // Run before everything else.
        )
        .forget();
}

/// Returns the current input modality.
///
/// This is a convenience function for checking the current modality
/// without setting up a reactive hook.
///
/// During SSR, always returns `Modality::Unknown`.
pub fn get_modality() -> Modality {
    #[cfg(feature = "ssr")]
    {
        Modality::Unknown
    }
    #[cfg(not(feature = "ssr"))]
    {
        FocusState::get().modality()
    }
}

/// Programmatically set the current input modality.
///
/// Note: This does not forcefully override the input modality derived from global event listeners.
/// As soon as the user uses any input device, the modality might be overwritten again.
///
/// During SSR, this is a no-op.
pub fn set_modality(modality: Modality) {
    #[cfg(feature = "ssr")]
    {
        let _ = modality;
    }
    #[cfg(not(feature = "ssr"))]
    {
        FocusState::get().set_modality_and_notify_subscribers(modality);
    }
}
