use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use std::sync::atomic::AtomicBool;
#[cfg(not(feature = "ssr"))]
use std::sync::OnceLock;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/interactions/src/useFocusVisible.ts

/// Global state tracking current interaction modality.
/// This is shared across all instances to track whether the user
/// is using keyboard or pointer input.
#[cfg(not(feature = "ssr"))]
static MODALITY: OnceLock<ModalityState> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
struct ModalityState {
    /// Whether current interaction is via keyboard.
    is_keyboard: AtomicBool,
    /// Whether handlers have been set up.
    handlers_setup: AtomicBool,
}

#[cfg(not(feature = "ssr"))]
impl ModalityState {
    fn new() -> Self {
        Self {
            is_keyboard: AtomicBool::new(true),
            handlers_setup: AtomicBool::new(false),
        }
    }

    fn get() -> &'static Self {
        MODALITY.get_or_init(ModalityState::new)
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
    pub is_focus_visible: Signal<bool>,
}

/// Tracks whether the user is interacting via keyboard, which determines
/// whether focus rings should be visible.
///
/// This hook subscribes to global keyboard and pointer events to track
/// the current interaction modality. When the user interacts with the
/// keyboard (Tab, arrows, etc.), focus rings become visible. When they
/// use a pointer (mouse, touch), focus rings are hidden.
///
/// # Example
///
/// ```ignore
/// let focus_visible = use_focus_visible(UseFocusVisibleInput::default());
///
/// view! {
///     <button class:focus-visible=move || focus_visible.is_focus_visible.get()>
///         "Click or tab to me"
///     </button>
/// }
/// ```
pub fn use_focus_visible(input: UseFocusVisibleInput) -> UseFocusVisibleReturn {
    // During SSR, return a default signal - focus visibility only makes sense on the client
    #[cfg(feature = "ssr")]
    {
        let (is_focus_visible, _) = signal(input.auto_focus);
        return UseFocusVisibleReturn {
            is_focus_visible: is_focus_visible.into(),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        use std::sync::atomic::Ordering;

        let modality = ModalityState::get();

        // Set up global listeners once
        if !modality.handlers_setup.swap(true, Ordering::AcqRel) {
            setup_global_listeners();
        }

        // Create a reactive signal that reads from the global state
        let (is_focus_visible, set_is_focus_visible) =
            signal(input.auto_focus || modality.is_keyboard.load(Ordering::Acquire));

        // Set up effect to sync with global modality changes
        // We use an interval check since we can't easily subscribe to AtomicBool changes
        Effect::new(move |_| {
            // Check modality on any relevant event
            // This will be triggered by the subscription to global events
            let is_keyboard = modality.is_keyboard.load(Ordering::Acquire);
            set_is_focus_visible.set(is_keyboard);
        });

        // Subscribe to modality changes via document events
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            use leptos_use::use_event_listener;

            // Update on focus events (triggered after keydown/pointerdown)
            let _ = use_event_listener(document.clone(), leptos::ev::focus, move |_| {
                let is_keyboard = modality.is_keyboard.load(Ordering::Acquire);
                set_is_focus_visible.set(is_keyboard);
            });

            // Also update on focusin for bubbling events
            let _ = use_event_listener(document, leptos::ev::focusin, move |_| {
                let is_keyboard = modality.is_keyboard.load(Ordering::Acquire);
                set_is_focus_visible.set(is_keyboard);
            });
        }

        UseFocusVisibleReturn {
            is_focus_visible: is_focus_visible.into(),
        }
    }
}

/// Sets up global event listeners to track interaction modality.
#[cfg(not(feature = "ssr"))]
fn setup_global_listeners() {
    use std::sync::atomic::Ordering;
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let modality = ModalityState::get();

    // Track keydown events - keyboard navigation keys trigger keyboard modality
    let modality_keydown = modality;
    let keydown_closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
        move |e: web_sys::KeyboardEvent| {
            if is_keyboard_focus_key(&e.key()) {
                modality_keydown.is_keyboard.store(true, Ordering::Release);
            }
        },
    );

    let _ = document
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref());
    keydown_closure.forget();

    // Track pointer events - pointer interaction hides focus ring
    let modality_pointer = modality;
    let pointerdown_closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::PointerEvent)>::new(
        move |_: web_sys::PointerEvent| {
            modality_pointer.is_keyboard.store(false, Ordering::Release);
        },
    );

    let _ = document.add_event_listener_with_callback(
        "pointerdown",
        pointerdown_closure.as_ref().unchecked_ref(),
    );
    pointerdown_closure.forget();

    // Also track mousedown for browsers without pointer events
    let modality_mouse = modality;
    let mousedown_closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(
        move |_: web_sys::MouseEvent| {
            modality_mouse.is_keyboard.store(false, Ordering::Release);
        },
    );

    let _ = document
        .add_event_listener_with_callback("mousedown", mousedown_closure.as_ref().unchecked_ref());
    mousedown_closure.forget();
}

/// Returns whether focus is currently visible globally.
///
/// This is a convenience function for checking the current modality
/// without setting up a reactive hook.
///
/// During SSR, always returns `true` (keyboard modality is assumed).
pub fn is_focus_visible() -> bool {
    #[cfg(feature = "ssr")]
    {
        true
    }
    #[cfg(not(feature = "ssr"))]
    {
        use std::sync::atomic::Ordering;
        ModalityState::get().is_keyboard.load(Ordering::Acquire)
    }
}

/// Sets the focus visibility modality.
///
/// This can be used to programmatically set whether focus should be visible,
/// for example when opening a dialog that should show focus.
///
/// During SSR, this is a no-op.
pub fn set_focus_visible(visible: bool) {
    #[cfg(not(feature = "ssr"))]
    {
        use std::sync::atomic::Ordering;
        ModalityState::get()
            .is_keyboard
            .store(visible, Ordering::Release);
    }
    #[cfg(feature = "ssr")]
    {
        let _ = visible;
    }
}
