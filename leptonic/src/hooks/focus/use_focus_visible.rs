#[cfg(not(feature = "ssr"))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock, RwLock,
};

use atomic_enum::atomic_enum;
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::{KeyboardEvent, PointerEvent};

#[cfg(not(feature = "ssr"))]
use crate::{
    utils::{
        focusability, platform::device::is_mac, virtual_click::is_virtual_click, EventAccessors,
    },
    Out,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/interactions/src/useFocusVisible.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - No `mousedown` fallback handler
//   React-aria registers a `mousedown` listener as a fallback for browsers
//   without PointerEvent support. Per CLAUDE.md, we assume PointerEvent is
//   always available.
//
// - No `currentPointerType` tracking
//   React-aria tracks `pointerType` (mouse, pen, touch) from pointer events.
//   We track only modality (Keyboard, Pointer, Virtual), not pointer sub-types.
//
// - No `ignoreFocusEvent` guard
//   React-aria uses an `ignoreFocusEvent` flag (set in `handlePointerEvent`) to
//   suppress the global focus handler for same-element refocus after pointer
//   interaction. We do not implement this guard; the focus handler may fire
//   redundantly in that case but the resulting modality is correct.
//
// - Always-notify instead of change-detection
//   React-aria always calls `triggerChangeHandlers` without dedup.
//   We match this: `set_modality_and_notify` always notifies subscribers.
//   Signal-level dedup in subscribers handles redundant updates.
//
// =============================================================================

/// Input parameters for the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusVisibleInput {
    /// Whether to auto-focus the element (affects initial visibility).
    pub auto_focus: bool,
    /// Whether the subscription is active. When `false`, the hook does not
    /// subscribe to global modality changes (saving unnecessary signal updates).
    /// Defaults to always-true for standalone use.
    pub enabled: Signal<bool>,
    /// Whether the element is a text input. When `true`, only Tab/Escape keys
    /// trigger focus-visible; other keyboard events are suppressed. This is
    /// used for compound text-input components (e.g., a date picker where focus
    /// is on a button but the component should use text-input focus rules).
    pub is_text_input: bool,
}

impl Default for UseFocusVisibleInput {
    fn default() -> Self {
        Self {
            auto_focus: false,
            enabled: Signal::stored(true),
            is_text_input: false,
        }
    }
}

/// The return value of the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusVisibleReturn {
    /// Whether the element should display a focus ring.
    /// True when the user is navigating via keyboard.
    pub focus_should_be_visible: Signal<bool>,

    /// The current interaction modality, reactively updated.
    pub modality: Signal<Modality>,
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
/// let UseFocusVisibleReturn { focus_should_be_visible, modality } = use_focus_visible(UseFocusVisibleInput::default());
///
/// view! {
///     <button class:focus-visible=move || focus_should_be_visible.get()>
///         "Button"
///     </button>
/// }
/// ```
pub fn use_focus_visible(input: UseFocusVisibleInput) -> UseFocusVisibleReturn {
    let UseFocusVisibleInput {
        auto_focus,
        enabled,
        is_text_input,
    } = input;

    #[cfg(feature = "ssr")]
    {
        let _ = enabled;
        let _ = is_text_input;
        let (is_focus_visible, _) = signal(auto_focus);
        let (modality, _) = signal(Modality::Unknown);
        return UseFocusVisibleReturn {
            focus_should_be_visible: is_focus_visible.into(),
            modality: modality.into(),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::LocalStorage;

        let state = FocusState::get();

        // Set up global listeners for the default window (idempotent).
        setup_global_focus_events(None);

        let is_focus_visible_for_modality = |modality: Modality| modality != Modality::Pointer;

        let current_modality = state.modality();

        let (is_focus_visible, set_is_focus_visible) =
            signal(auto_focus || is_focus_visible_for_modality(current_modality));

        let (modality, set_modality) = signal(current_modality);

        // Track subscriber ID for reactive registration/unregistration.
        let subscriber_id: StoredValue<Option<SubscriberId>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            if enabled.get() {
                // Sync to current global state (may have changed while unsubscribed).
                let current = state.modality();
                set_is_focus_visible.set(is_focus_visible_for_modality(current));
                set_modality.set(current);

                // Register for future updates.
                let id = state.register(is_text_input, move |new_modality: Modality| {
                    set_is_focus_visible.set(is_focus_visible_for_modality(new_modality));
                    set_modality.set(new_modality);
                });
                subscriber_id.set_value(Some(id));
            } else {
                // Unregister while disabled.
                if let Some(old_id) = subscriber_id.get_value() {
                    state.unregister(old_id);
                    subscriber_id.set_value(None);
                }
            }
        });

        on_cleanup(move || {
            if let Some(id) = subscriber_id.get_value() {
                state.unregister(id);
            }
        });

        UseFocusVisibleReturn {
            focus_should_be_visible: is_focus_visible.into(),
            modality: modality.into(),
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

    /// Whether a keyboard/pointer event occurred before the current focus event.
    /// Used to detect programmatic/virtual focus (screen readers).
    has_event_before_focus: AtomicBool,

    /// Whether the window was recently blurred. Used to avoid false positives
    /// when returning to the tab.
    has_blurred_window_recently: AtomicBool,

    /// Whether the active element was a text input during the last keyboard event.
    /// Used by per-subscriber filtering to suppress focus-visible on typing.
    active_element_is_text_input: AtomicBool,

    /// Whether the last keyboard event key was Tab or Escape (a "focus key").
    /// Used by per-subscriber filtering in combination with `active_element_is_text_input`.
    last_key_is_focus_key: AtomicBool,

    /// Next unique ID for subscriber registration.
    next_id: std::sync::atomic::AtomicU64,

    /// Registered subscribers that receive modality change notifications.
    /// Each entry is `(id, is_text_input, Out<Modality>)`. The `is_text_input`
    /// flag controls per-subscriber filtering of keyboard events.
    subscribers: RwLock<Vec<(SubscriberId, bool, Out<Modality>)>>,
}

#[cfg(not(feature = "ssr"))]
impl FocusState {
    fn new() -> Self {
        Self {
            modality: AtomicModality::new(Modality::Unknown),
            has_event_before_focus: AtomicBool::new(false),
            has_blurred_window_recently: AtomicBool::new(false),
            active_element_is_text_input: AtomicBool::new(false),
            last_key_is_focus_key: AtomicBool::new(false),
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

    /// Update the stored modality and always notify subscribers.
    /// Subscribers perform their own per-subscriber filtering (e.g., text input
    /// suppression), so we always notify and let them decide.
    fn set_modality_and_notify(&self, modality: Modality) {
        self.modality.store(modality, Ordering::Release);
        self.notify_subscribers(modality);
    }

    /// Update the stored modality without notifying subscribers.
    /// Used for `pointermove`/`pointerup` events which should update the stored
    /// modality silently — only `pointerdown`/`mousedown` should notify subscribers
    /// (matching react-aria's behavior).
    fn set_modality_silently(&self, modality: Modality) {
        self.modality.store(modality, Ordering::Release);
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

    fn active_element_is_text_input(&self) -> bool {
        self.active_element_is_text_input.load(Ordering::Acquire)
    }

    fn set_active_element_is_text_input(&self, value: bool) {
        self.active_element_is_text_input
            .store(value, Ordering::Release);
    }

    fn last_key_is_focus_key(&self) -> bool {
        self.last_key_is_focus_key.load(Ordering::Acquire)
    }

    fn set_last_key_is_focus_key(&self, value: bool) {
        self.last_key_is_focus_key.store(value, Ordering::Release);
    }

    /// Notify all registered subscribers of the given modality.
    ///
    /// Per-subscriber text input filtering (react-aria's `isKeyboardFocusEvent`):
    /// For keyboard modality, subscribers with `is_text_input = true` (or when
    /// the active element at event time was a text input) are only notified for
    /// Tab/Escape keys. Other keyboard events are suppressed for those subscribers.
    fn notify_subscribers(&self, modality: Modality) {
        let subscribers = self.subscribers.read().expect("subscriber lock poisoned");
        for (_, sub_is_text_input, subscriber) in &*subscribers {
            if modality == Modality::Keyboard {
                let is_text = *sub_is_text_input || self.active_element_is_text_input();
                if is_text && !self.last_key_is_focus_key() {
                    continue;
                }
            }
            subscriber.set(modality);
        }
    }

    /// Register a subscriber. Returns a unique ID for unregistration.
    ///
    /// `is_text_input`: when `true`, only Tab/Escape keyboard events trigger
    /// notifications for this subscriber (other keys are suppressed).
    fn register(&self, is_text_input: bool, subscriber: impl Into<Out<Modality>>) -> SubscriberId {
        // Wrapping around on overflow. Ensures endless usage.
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut subscribers = self.subscribers.write().expect("subscriber lock poisoned");
        subscribers.push((id, is_text_input, subscriber.into()));
        id
    }

    /// Unregister a subscriber by its unique ID.
    fn unregister(&self, id: SubscriberId) {
        let mut subscribers = self.subscribers.write().expect("subscriber lock poisoned");
        if let Some(pos) = subscribers.iter().position(|(sid, _, _)| *sid == id) {
            subscribers.swap_remove(pos);
        }
    }
}

// =============================================================================
// Per-window listener management
// =============================================================================

/// Stores listener data for a single window's focus tracking setup.
#[cfg(not(feature = "ssr"))]
struct WindowListenerData {
    /// The original `HTMLElement.prototype.focus` function, stored for restoration.
    original_focus: wasm_bindgen::JsValue,
    /// Registered event listener handles. Each is auto-removed on drop (via
    /// `ListenerRegistration::Drop`). Not read directly — stored for its side effect.
    _listeners: Vec<ListenerRegistration>,
}

/// An event listener registration that auto-removes on drop.
#[cfg(not(feature = "ssr"))]
struct ListenerRegistration {
    target: web_sys::EventTarget,
    event_name: &'static str,
    callback_js: js_sys::Function,
    capture: bool,
    /// Prevents the `Closure` from being dropped (which would invalidate `callback_js`).
    _closure: Box<dyn std::any::Any>,
}

#[cfg(not(feature = "ssr"))]
impl Drop for ListenerRegistration {
    fn drop(&mut self) {
        let _ = self.target.remove_event_listener_with_callback_and_bool(
            self.event_name,
            &self.callback_js,
            self.capture,
        );
    }
}

// Per-window tracking of focus event listeners.
//
// Maps window identity (as `JsValue`) to listener data. Uses a `Vec` since
// there are typically only 1-2 windows (main + possibly an iframe).
#[cfg(not(feature = "ssr"))]
thread_local! {
    static WINDOW_LISTENERS: std::cell::RefCell<Vec<(wasm_bindgen::JsValue, WindowListenerData)>>
        = const { std::cell::RefCell::new(Vec::new()) };
}

/// Helper to register an event listener and produce a `ListenerRegistration`.
#[cfg(not(feature = "ssr"))]
fn register_listener<E>(
    target: &web_sys::EventTarget,
    event_name: &'static str,
    callback: impl FnMut(E) + 'static,
    capture: bool,
) -> ListenerRegistration
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    use wasm_bindgen::closure::Closure;

    let closure: Closure<dyn FnMut(E)> = Closure::wrap(Box::new(callback));
    let callback_js: js_sys::Function =
        closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    let options = web_sys::AddEventListenerOptions::new();
    options.set_capture(capture);

    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        event_name,
        &callback_js,
        &options,
    );

    ListenerRegistration {
        target: target.clone(),
        event_name,
        callback_js,
        capture,
        _closure: Box::new(closure),
    }
}

/// Returns true for any key except modifiers.
/// Matches react-aria's `isValidKey`.
#[cfg(not(feature = "ssr"))]
fn is_valid_key(e: &KeyboardEvent, is_mac: bool) -> bool {
    !(e.meta_key()
        || (!is_mac && e.alt_key())
        || e.ctrl_key()
        || e.key() == "Control"
        || e.key() == "Shift"
        || e.key() == "Meta")
}

/// Keys that always trigger keyboard modality, even inside text inputs.
#[cfg(not(feature = "ssr"))]
const FOCUS_VISIBLE_INPUT_KEYS: &[&str] = &["Tab", "Escape"];

/// Set up global focus event listeners for a window, identified by an element within it.
///
/// Pass `None` for the default window. This is idempotent: calling it multiple times
/// for the same window is a no-op.
///
/// Based on react-aria's `setupGlobalFocusEvents`.
#[cfg(not(feature = "ssr"))]
fn setup_global_focus_events(element: Option<&web_sys::HtmlElement>) {
    let (window, document) = resolve_window_and_document(element);
    let (Some(window), Some(document)) = (window, document) else {
        tracing::warn!("use_focus_visible: window or document not available");
        return;
    };

    // Check if already set up for this window.
    let window_js: wasm_bindgen::JsValue = window.clone().into();
    let already_setup =
        WINDOW_LISTENERS.with(|wl| wl.borrow().iter().any(|(w, _)| *w == window_js));

    if already_setup {
        return;
    }

    let state = FocusState::get();

    let mut listeners = Vec::new();
    register_document_listeners(state, &document, &mut listeners);
    register_window_listeners(state, &window, &window_js, &mut listeners);

    // --- Override HTMLElement.prototype.focus ---
    let original_focus = setup_focus_override_for_window(&window, state);

    WINDOW_LISTENERS.with(|wl| {
        wl.borrow_mut().push((
            window_js,
            WindowListenerData {
                original_focus,
                _listeners: listeners,
            },
        ));
    });
}

/// Register capture-phase document listeners for keyboard, click, and pointer events.
#[cfg(not(feature = "ssr"))]
fn register_document_listeners(
    state: &'static FocusState,
    document: &web_sys::Document,
    listeners: &mut Vec<ListenerRegistration>,
) {
    let is_mac = is_mac();

    let handle_keyboard = move |e: KeyboardEvent| {
        state.set_has_event_before_focus(true);
        if !crate::utils::open_link::is_opening_link() && is_valid_key(&e, is_mac) {
            let key = e.key();
            let target_el: web_sys::Element = e.expect_target().unchecked_into();

            let is_focus_key = FOCUS_VISIBLE_INPUT_KEYS.contains(&key.as_str());
            let el_is_text_input = target_el.owner_document().is_some_and(|doc| {
                focusability::is_text_input_or_active_text_input(&target_el, &doc)
            });

            state.set_last_key_is_focus_key(is_focus_key);
            state.set_active_element_is_text_input(el_is_text_input);
            state.set_modality_and_notify(Modality::Keyboard);
        }
    };

    listeners.push(register_listener(
        document.as_ref(),
        "keydown",
        handle_keyboard,
        true,
    ));
    listeners.push(register_listener(
        document.as_ref(),
        "keyup",
        handle_keyboard,
        true,
    ));

    // `click` in capture phase: detect virtual (screen reader) clicks.
    let handle_click = move |e: web_sys::MouseEvent| {
        if !crate::utils::open_link::is_opening_link() && is_virtual_click(&e) {
            state.set_has_event_before_focus(true);
            state.set_modality_silently(Modality::Virtual);
        }
    };
    listeners.push(register_listener(
        document.as_ref(),
        "click",
        handle_click,
        true,
    ));

    // `pointerdown`: update modality AND notify subscribers.
    let handle_pointer_down = move |_: PointerEvent| {
        state.set_has_event_before_focus(true);
        state.set_modality_and_notify(Modality::Pointer);
    };
    listeners.push(register_listener(
        document.as_ref(),
        "pointerdown",
        handle_pointer_down,
        true,
    ));

    // `pointermove`/`pointerup`: silently update stored modality without notifying.
    let handle_pointer_silent = move |_: PointerEvent| {
        state.set_modality_silently(Modality::Pointer);
    };
    listeners.push(register_listener(
        document.as_ref(),
        "pointermove",
        handle_pointer_silent,
        true,
    ));
    listeners.push(register_listener(
        document.as_ref(),
        "pointerup",
        handle_pointer_silent,
        true,
    ));
}

/// Register window-level listeners for blur, focus, and beforeunload events.
#[cfg(not(feature = "ssr"))]
fn register_window_listeners(
    state: &'static FocusState,
    window: &web_sys::Window,
    window_js: &wasm_bindgen::JsValue,
    listeners: &mut Vec<ListenerRegistration>,
) {
    // Reset state when tabbing away from the page.
    listeners.push(register_listener(
        window.as_ref(),
        "blur",
        move |_: web_sys::FocusEvent| {
            state.set_has_event_before_focus(false);
            state.set_has_blurred_window_recently(true);
        },
        false,
    ));

    // Detect a return to the tab.
    listeners.push(register_listener(
        window.as_ref(),
        "focus",
        move |e: web_sys::FocusEvent| {
            // Guard: skip focus events on window or document targets (Firefox iframe workaround).
            if let Some(target) = e.target() {
                if target.dyn_ref::<web_sys::Window>().is_some()
                    || target.dyn_ref::<web_sys::Document>().is_some()
                {
                    return;
                }
            }

            // Guard: skip synthetic/programmatic focus events.
            if !e.is_trusted() {
                return;
            }

            // If a focus event occurs without a preceding keyboard or pointer event,
            // switch to virtual modality.
            if !state.has_event_before_focus() && !state.has_blurred_window_recently() {
                state.set_modality_and_notify(Modality::Virtual);
            }
            state.set_has_event_before_focus(false);
            state.set_has_blurred_window_recently(false);
        },
        true,
    ));

    // --- beforeunload: auto-cleanup when window is destroyed ---
    let window_js_for_cleanup = window_js.clone();
    listeners.push(register_listener(
        window.as_ref(),
        "beforeunload",
        move |_: web_sys::Event| {
            tear_down_window_by_key(&window_js_for_cleanup);
        },
        false,
    ));
}

/// Resolve the window and document for an optional element.
/// Falls back to `web_sys::window()` / `window.document()` when `element` is `None`.
#[cfg(not(feature = "ssr"))]
fn resolve_window_and_document(
    element: Option<&web_sys::HtmlElement>,
) -> (Option<web_sys::Window>, Option<web_sys::Document>) {
    if let Some(el) = element {
        let doc = el.owner_document();
        let win = doc.as_ref().and_then(web_sys::Document::default_view);
        (win, doc)
    } else {
        let win = web_sys::window();
        let doc = win.as_ref().and_then(web_sys::Window::document);
        (win, doc)
    }
}

/// Override `HTMLElement.prototype.focus` for a specific window to set
/// `has_event_before_focus = true`. Returns the original focus function
/// for later restoration.
#[cfg(not(feature = "ssr"))]
fn setup_focus_override_for_window(
    window: &web_sys::Window,
    state: &'static FocusState,
) -> wasm_bindgen::JsValue {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::prelude::*;

    let window_js: &wasm_bindgen::JsValue = window.as_ref();

    let html_ctor = Reflect::get(window_js, &"HTMLElement".into()).ok();
    let prototype = html_ctor.and_then(|c| Reflect::get(&c, &"prototype".into()).ok());
    let Some(prototype) = prototype else {
        return wasm_bindgen::JsValue::UNDEFINED;
    };

    let original_focus = match Reflect::get(&prototype, &"focus".into()) {
        Ok(f) if f.is_function() => f,
        _ => return wasm_bindgen::JsValue::UNDEFINED,
    };

    // Store the flag-setter as a JS-callable closure.
    let set_flag = Closure::<dyn FnMut()>::new(move || {
        state.set_has_event_before_focus(true);
    });

    // Store original + flag-setter on a hidden holder object on the prototype.
    let holder = Object::new();
    let _ = Reflect::set(&holder, &"orig".into(), &original_focus);
    let _ = Reflect::set(&holder, &"setFlag".into(), set_flag.as_ref());
    set_flag.forget();

    let _ = Reflect::set(&prototype, &"__leptonic_focus".into(), &holder);

    let wrapper = Function::new_no_args(
        "var p = this.constructor && this.constructor.prototype || \
         Object.getPrototypeOf(this); \
         var h = p.__leptonic_focus || HTMLElement.prototype.__leptonic_focus; \
         if (h) { h.setFlag(); return h.orig.apply(this, arguments); } \
         return HTMLElement.prototype.focus.apply(this, arguments);",
    );

    let _ = Reflect::set(&prototype, &"focus".into(), &wrapper);

    original_focus
}

/// Restore `HTMLElement.prototype.focus` for a window.
#[cfg(not(feature = "ssr"))]
fn restore_focus_override(
    window_js: &wasm_bindgen::JsValue,
    original_focus: &wasm_bindgen::JsValue,
) {
    use js_sys::Reflect;

    if original_focus.is_undefined() || original_focus.is_null() {
        return;
    }

    let html_ctor = Reflect::get(window_js, &"HTMLElement".into()).ok();
    let prototype = html_ctor.and_then(|c| Reflect::get(&c, &"prototype".into()).ok());
    if let Some(prototype) = prototype {
        let _ = Reflect::set(&prototype, &"focus".into(), original_focus);
        let _ = Reflect::delete_property(
            prototype.unchecked_ref::<js_sys::Object>(),
            &"__leptonic_focus".into(),
        );
    }
}

/// Tear down focus tracking for a window identified by its `JsValue`.
#[cfg(not(feature = "ssr"))]
fn tear_down_window_by_key(window_key: &wasm_bindgen::JsValue) {
    WINDOW_LISTENERS.with(|wl| {
        let mut listeners = wl.borrow_mut();
        if let Some(pos) = listeners.iter().position(|(w, _)| w == window_key) {
            let (window_js, data) = listeners.swap_remove(pos);
            // Restore the original focus method.
            restore_focus_override(&window_js, &data.original_focus);
            // Listeners are auto-removed when `data._listeners` is dropped.
            drop(data);
        }
    });
}

/// Add focus tracking for a window containing the given element.
///
/// This is the public API for multi-window/iframe support. Call this with
/// an element inside an iframe to enable focus-visible tracking for that
/// iframe's window.
///
/// Returns a cleanup function that tears down listeners for that window.
///
/// During SSR, returns a no-op cleanup function.
///
/// # Example
///
/// ```ignore
/// // Inside a component that manages an iframe:
/// let cleanup = add_window_focus_tracking(Some(&iframe_element));
/// // When done:
/// cleanup();
/// ```
pub fn add_window_focus_tracking(element: Option<&web_sys::HtmlElement>) -> Box<dyn FnOnce()> {
    #[cfg(feature = "ssr")]
    {
        let _ = element;
        Box::new(|| {})
    }

    #[cfg(not(feature = "ssr"))]
    {
        setup_global_focus_events(element);

        let window_key = element
            .and_then(|el| el.owner_document())
            .and_then(|d| d.default_view())
            .map_or_else(
                || {
                    web_sys::window().map_or(
                        wasm_bindgen::JsValue::UNDEFINED,
                        wasm_bindgen::JsValue::from,
                    )
                },
                wasm_bindgen::JsValue::from,
            );

        Box::new(move || {
            tear_down_window_by_key(&window_key);
        })
    }
}

/// Tear down focus tracking for a window containing the given element.
///
/// This is the public API for explicit teardown. In most cases, the cleanup
/// function returned by [`add_window_focus_tracking`] is preferred.
///
/// During SSR, this is a no-op.
pub fn tear_down_window_focus_tracking(element: Option<&web_sys::HtmlElement>) {
    #[cfg(feature = "ssr")]
    {
        let _ = element;
    }

    #[cfg(not(feature = "ssr"))]
    {
        let window_key = element
            .and_then(|el| el.owner_document())
            .and_then(|d| d.default_view())
            .map_or_else(
                || {
                    web_sys::window().map_or(
                        wasm_bindgen::JsValue::UNDEFINED,
                        wasm_bindgen::JsValue::from,
                    )
                },
                wasm_bindgen::JsValue::from,
            );

        tear_down_window_by_key(&window_key);
    }
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
        FocusState::get().set_modality_and_notify(modality);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    };

    use super::*;

    #[test]
    fn pointerdown_notifies_after_silent_pointer_update() {
        let state = FocusState::new();

        let count = Arc::new(AtomicU32::new(0));
        let last_modality = Arc::new(AtomicModality::new(Modality::Unknown));

        let count_clone = Arc::clone(&count);
        let modality_clone = Arc::clone(&last_modality);
        let id = state.register(false, move |m: Modality| {
            count_clone.fetch_add(1, Ordering::Relaxed);
            modality_clone.store(m, Ordering::Relaxed);
        });

        // Step 1: Keyboard event → should notify (count=1)
        state.set_modality_and_notify(Modality::Keyboard);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        assert_eq!(last_modality.load(Ordering::Relaxed), Modality::Keyboard);

        // Step 2: Silent pointer update (pointermove) → no notification
        state.set_modality_silently(Modality::Pointer);
        assert_eq!(count.load(Ordering::Relaxed), 1);

        // Step 3: Pointer down → should notify (set_modality_and_notify always notifies).
        state.set_modality_and_notify(Modality::Pointer);
        assert_eq!(count.load(Ordering::Relaxed), 2);
        assert_eq!(last_modality.load(Ordering::Relaxed), Modality::Pointer);

        state.unregister(id);
    }
}
