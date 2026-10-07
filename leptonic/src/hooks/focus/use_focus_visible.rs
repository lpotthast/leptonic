// Upstream: react-aria/src/interactions/useFocusVisible.ts @ 99e6102368
#[cfg(not(feature = "ssr"))]
use std::sync::{
    OnceLock, RwLock,
    atomic::{AtomicBool, Ordering},
};

use atomic_enum::atomic_enum;
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::{KeyboardEvent, PointerEvent};

use crate::utils::pointer_type::PointerType;
#[cfg(not(feature = "ssr"))]
use crate::{
    Out,
    utils::{
        EventAccessors, focusability,
        key::{KeyboardEventKey, KeyboardKey},
        platform::device::is_mac,
        prevent_focus::is_ignoring_focus_events,
        shadow_dom::get_active_element,
        virtual_click::is_virtual_click,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `Modality::Unknown` before the first interaction (react-aria: `null`).
// - `use_focus_visible` also returns the modality its subscriber saw, and takes `is_disabled`
//   (react-aria's `useFocusVisibleListener` option `enabled`) to pause the subscription.
// - `get_modality`/`set_modality`/`get_pointer_type`: react-aria's `getInteractionModality`/
//   `setInteractionModality`/`getPointerType`.
//
// ## DIFFERENT BEHAVIOR
// - Tracking starts when the first hook reading the modality is created
//   (`track_interaction_modality`), not on module load (`addWindowFocusTracking()` at the top
//   level): Rust has no module initialization. Without it, a grid saw `Unknown` (treated as
//   keyboard) for mouse presses and moved focus to its old focused cell.
//
// ## OMITTED FEATURES
// - The `mousedown`/`mousemove`/`mouseup` fallbacks for environments without `PointerEvent`
//   (react-aria registers them in tests only): every supported browser has pointer events.
//
// =============================================================================

/// Input parameters for the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusVisibleInput {
    /// Whether to auto-focus the element (affects initial visibility).
    pub auto_focus: bool,
    /// Stops tracking modality changes while `true` (e.g. while the element isn't focused),
    /// saving signal updates. Tracking resumes with the current modality.
    pub is_disabled: Signal<bool>,
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
            is_disabled: Signal::stored(false),
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
        is_disabled,
        is_text_input,
    } = input;

    #[cfg(feature = "ssr")]
    {
        let _ = is_disabled;
        let _ = is_text_input;
        let (is_focus_visible, _) = signal(auto_focus);
        let (modality, _) = signal(Modality::Unknown);
        UseFocusVisibleReturn {
            focus_should_be_visible: is_focus_visible.into(),
            modality: modality.into(),
        }
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

        // Whether the hook was disabled since it was created: then the modality may have changed
        // unseen. (On creation the state is current, including `auto_focus`.)
        let was_disabled = StoredValue::new_local(false);
        Effect::new(move |_| {
            if is_disabled.get() {
                // Unregister while disabled.
                if let Some(old_id) = subscriber_id.get_value() {
                    state.unregister(old_id);
                    subscriber_id.set_value(None);
                }
                was_disabled.set_value(true);
            } else {
                if was_disabled.get_value() {
                    let current = state.modality();
                    set_is_focus_visible.set(is_focus_visible_for_modality(current));
                    set_modality.set(current);
                }

                // Register for future updates.
                let id = state.register(is_text_input, move |new_modality: Modality| {
                    set_is_focus_visible.set(is_focus_visible_for_modality(new_modality));
                    set_modality.set(new_modality);
                });
                subscriber_id.set_value(Some(id));
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

/// What a keyboard event tells the subscribers (react-aria's `isKeyboardFocusEvent` inputs).
#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, Copy)]
struct KeyContext {
    /// Whether the event's target or the active element is a text input.
    is_text_input: bool,
    /// Whether the key makes focus visible even in text inputs (Tab, Escape).
    is_focus_key: bool,
}

/// Global focus state management, tracking:
///
/// - The current interaction modality (keyboard, pointer, or virtual) and pointer type.
/// - Registered subscribers that are notified when the modality changes.
#[cfg(not(feature = "ssr"))]
struct FocusState {
    /// The current interaction modality (Unknown, Pointer, Keyboard, or Virtual).
    modality: AtomicModality,

    /// The pointer type of the last interaction (react-aria's `currentPointerType`).
    pointer_type: RwLock<PointerType>,

    /// Whether a keyboard/pointer event occurred before the current focus event.
    /// Used to detect programmatic/virtual focus (screen readers).
    has_event_before_focus: AtomicBool,

    /// Whether the window was recently blurred or focused. Used to avoid false positives
    /// when returning to the tab.
    has_blurred_window_recently: AtomicBool,

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
            pointer_type: RwLock::new(PointerType::Keyboard),
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

    fn pointer_type(&self) -> PointerType {
        self.pointer_type
            .read()
            .expect("pointer type lock poisoned")
            .clone()
    }

    /// Stores the modality and pointer type without notifying subscribers (react-aria's
    /// `pointermove`/`pointerup` and virtual clicks).
    fn set_modality_silently(&self, modality: Modality, pointer_type: PointerType) {
        self.modality.store(modality, Ordering::Release);
        *self
            .pointer_type
            .write()
            .expect("pointer type lock poisoned") = pointer_type;
    }

    /// Stores the modality and pointer type and notifies the subscribers (react-aria's
    /// `triggerChangeHandlers`, which always notifies; subscribers' signals dedupe). `key` is the
    /// keyboard event's context, `None` for other events and programmatic changes.
    fn set_modality_and_notify(
        &self,
        modality: Modality,
        pointer_type: PointerType,
        key: Option<KeyContext>,
    ) {
        self.set_modality_silently(modality, pointer_type);
        self.notify_subscribers(modality, key);
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

    /// Notify the registered subscribers of the given modality.
    ///
    /// Per-subscriber text input filtering (react-aria's `isKeyboardFocusEvent`): a keyboard
    /// event inside a text input (the subscriber's own `is_text_input`, or the event's
    /// [`KeyContext`]) notifies only for Tab and Escape.
    fn notify_subscribers(&self, modality: Modality, key: Option<KeyContext>) {
        // Collect first and notify without holding the lock: a subscriber may (un)register
        // synchronously, which would deadlock (and panic on wasm).
        let to_notify: Vec<Out<Modality>> = {
            let subscribers = self.subscribers.read().expect("subscriber lock poisoned");
            subscribers
                .iter()
                .filter(|(_, sub_is_text_input, _)| {
                    modality != Modality::Keyboard
                        || key.is_none_or(|key| {
                            key.is_focus_key || !(*sub_is_text_input || key.is_text_input)
                        })
                })
                .map(|(_, _, subscriber)| *subscriber)
                .collect()
        };
        for subscriber in to_notify {
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

/// Stores listener data for a single window's focus tracking setup.
#[cfg(not(feature = "ssr"))]
struct WindowListenerData {
    /// The original `HTMLElement.prototype.focus` function and the closure its replacement calls,
    /// kept for restoration (`None` when the prototype couldn't be patched).
    focus_override: Option<FocusOverride>,
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
        || matches!(
            e.typed_key(),
            KeyboardKey::Control | KeyboardKey::Shift | KeyboardKey::Meta
        ))
}

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

    // Programmatic `focus()` calls shouldn't change the modality, while other focus events
    // without a preceding user event (e.g. screen reader focus) switch it to virtual.
    let focus_override = FocusOverride::install(&window, state);

    WINDOW_LISTENERS.with(|wl| {
        wl.borrow_mut().push((
            window_js,
            WindowListenerData {
                focus_override,
                _listeners: listeners,
            },
        ));
    });
}

/// Register capture-phase document listeners for keyboard, click, pointer and `invalid` events.
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
            let target: web_sys::Element = e.expect_target().unchecked_into();
            let key = KeyContext {
                is_text_input: target.owner_document().is_some_and(|doc| {
                    focusability::is_text_input_or_active_text_input(&target, &doc)
                }),
                is_focus_key: matches!(e.typed_key(), KeyboardKey::Tab | KeyboardKey::Escape),
            };
            state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, Some(key));
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

    // Virtual (screen reader) clicks.
    let handle_click = move |e: web_sys::MouseEvent| {
        if !crate::utils::open_link::is_opening_link() && is_virtual_click(&e) {
            state.set_has_event_before_focus(true);
            state.set_modality_silently(Modality::Virtual, PointerType::Virtual);
        }
    };
    listeners.push(register_listener(
        document.as_ref(),
        "click",
        handle_click,
        true,
    ));

    // A focus move right after a form became invalid is a forms library (or the browser) focusing
    // the first invalid field: show the focus ring there.
    let handle_invalid = move |e: web_sys::Event| {
        let Some(document) = e
            .expect_target()
            .dyn_into::<web_sys::Node>()
            .ok()
            .and_then(|node| node.owner_document())
        else {
            return;
        };
        let starting_active_element = get_active_element(&document);
        let document = send_wrapper::SendWrapper::new(document);
        let starting_active_element = send_wrapper::SendWrapper::new(starting_active_element);
        queue_microtask(move || {
            if get_active_element(&document) != *starting_active_element {
                set_modality(Modality::Keyboard);
            }
        });
    };
    listeners.push(register_listener(
        document.as_ref(),
        "invalid",
        handle_invalid,
        true,
    ));

    // `pointerdown` changes the modality and notifies, `pointermove`/`pointerup` only change it.
    let handle_pointer_down = move |e: PointerEvent| {
        state.set_has_event_before_focus(true);
        state.set_modality_and_notify(Modality::Pointer, PointerType::from(e.pointer_type()), None);
    };
    listeners.push(register_listener(
        document.as_ref(),
        "pointerdown",
        handle_pointer_down,
        true,
    ));
    let handle_pointer_silent = move |e: PointerEvent| {
        state.set_modality_silently(Modality::Pointer, PointerType::from(e.pointer_type()));
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
            // Focus moved back by `prevent_focus` (react-aria's `ignoreFocusEvent`).
            if is_ignoring_focus_events() {
                return;
            }
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
            // Focus moved back by `prevent_focus` (react-aria's `ignoreFocusEvent`).
            if is_ignoring_focus_events() {
                return;
            }
            let target = e.expect_target();
            // The window regaining focus: the browser then restores focus to the element focused
            // before, which the user didn't initiate. Safari fires the window/element focus pair
            // twice when returning to a tab (the first element focus clears the flag), so re-arm
            // the flag on every window focus. Like the blur handler, no `isTrusted` check.
            if target.dyn_ref::<web_sys::Window>().is_some() {
                state.set_has_blurred_window_recently(true);
                return;
            }
            // Firefox fires focus on the window and then the document when the user first clicks
            // into an iframe; synthetic focus events don't count either.
            if target.dyn_ref::<web_sys::Document>().is_some() || !e.is_trusted() {
                return;
            }

            // If a focus event occurs without a preceding keyboard or pointer event,
            // switch to virtual modality.
            if !state.has_event_before_focus() && !state.has_blurred_window_recently() {
                state.set_modality_and_notify(Modality::Virtual, PointerType::Virtual, None);
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
        let win = leptos_use::use_window().as_ref().cloned();
        let doc = win.as_ref().and_then(web_sys::Window::document);
        (win, doc)
    }
}

/// The replaced `HTMLElement.prototype.focus` of a window: its replacement marks the next focus
/// event as programmatic (`has_event_before_focus`) and calls the original.
#[cfg(not(feature = "ssr"))]
struct FocusOverride {
    prototype: js_sys::Object,
    original_focus: wasm_bindgen::JsValue,
    /// Called by the replacement; must live as long as the replacement is installed.
    _set_flag: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

#[cfg(not(feature = "ssr"))]
impl FocusOverride {
    /// Replaces `HTMLElement.prototype.focus` of `window`, with `Reflect.defineProperty` (not an
    /// assignment), so it works even when `focus` is an accessor without a setter.
    fn install(window: &web_sys::Window, state: &'static FocusState) -> Option<Self> {
        use js_sys::{Function, Object, Reflect};
        use wasm_bindgen::{JsValue, closure::Closure};

        let prototype: Object = Reflect::get(window, &"HTMLElement".into())
            .and_then(|ctor| Reflect::get(&ctor, &"prototype".into()))
            .ok()?
            .dyn_into()
            .ok()?;
        let original_focus = Reflect::get(&prototype, &"focus".into())
            .ok()
            .filter(JsValue::is_function)?;
        let set_flag = Closure::<dyn FnMut()>::new(move || {
            state.set_has_event_before_focus(true);
        });
        let replacement = Function::new_with_args(
            "original, setFlag",
            "return function focus() { setFlag(); return original.apply(this, arguments); };",
        )
        .call2(&JsValue::NULL, &original_focus, set_flag.as_ref())
        .ok()?;
        define_focus(&prototype, &replacement).then_some(Self {
            prototype,
            original_focus,
            _set_flag: set_flag,
        })
    }

    /// Puts the original `focus` back.
    fn uninstall(&self) {
        define_focus(&self.prototype, &self.original_focus);
    }
}

/// Defines `prototype.focus` as a writable, configurable data property holding `value`.
#[cfg(not(feature = "ssr"))]
fn define_focus(prototype: &js_sys::Object, value: &wasm_bindgen::JsValue) -> bool {
    use js_sys::{Object, Reflect};

    let descriptor = Object::new();
    let _ = Reflect::set(&descriptor, &"configurable".into(), &true.into());
    let _ = Reflect::set(&descriptor, &"writable".into(), &true.into());
    let _ = Reflect::set(&descriptor, &"value".into(), value);
    Reflect::define_property(prototype, &"focus".into(), &descriptor).unwrap_or(false)
}

/// Tear down focus tracking for a window identified by its `JsValue`.
#[cfg(not(feature = "ssr"))]
fn tear_down_window_by_key(window_key: &wasm_bindgen::JsValue) {
    WINDOW_LISTENERS.with(|wl| {
        let mut listeners = wl.borrow_mut();
        if let Some(pos) = listeners.iter().position(|(w, _)| w == window_key) {
            let (_, data) = listeners.swap_remove(pos);
            if let Some(focus_override) = &data.focus_override {
                focus_override.uninstall();
            }
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
                    leptos_use::use_window()
                        .as_ref()
                        .map_or(wasm_bindgen::JsValue::UNDEFINED, |w| w.clone().into())
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
                    leptos_use::use_window()
                        .as_ref()
                        .map_or(wasm_bindgen::JsValue::UNDEFINED, |w| w.clone().into())
                },
                wasm_bindgen::JsValue::from,
            );

        tear_down_window_by_key(&window_key);
    }
}

/// The current interaction modality, updated as it changes (react-aria's
/// `useInteractionModality`). `None` during server-side rendering.
pub fn use_interaction_modality() -> Signal<Option<Modality>> {
    #[cfg(feature = "ssr")]
    {
        Signal::stored(None)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let modality = use_focus_visible(UseFocusVisibleInput {
            auto_focus: false,
            is_disabled: Signal::stored(false),
            is_text_input: false,
        })
        .modality;
        Signal::derive(move || Some(modality.get()))
    }
}

/// Starts tracking the interaction modality (idempotent, a no-op during SSR). react-aria tracks it
/// from module load; here, every hook reading the modality in its event handlers (`get_modality`)
/// calls this when it is created, so the modality is known by the first interaction.
pub(crate) fn track_interaction_modality() {
    #[cfg(not(feature = "ssr"))]
    setup_global_focus_events(None);
}

/// Returns the current input modality.
///
/// This is a convenience function for checking the current modality
/// without setting up a reactive hook. Before any hook started tracking it (see
/// `track_interaction_modality`), it is `Modality::Unknown`.
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
        let pointer_type = match modality {
            Modality::Pointer => PointerType::Mouse,
            Modality::Virtual => PointerType::Virtual,
            Modality::Keyboard | Modality::Unknown => PointerType::Keyboard,
        };
        FocusState::get().set_modality_and_notify(modality, pointer_type, None);
    }
}

/// The pointer type of the last interaction (react-aria's `getPointerType`): `Keyboard` and
/// `Virtual` for those modalities, else the type of the last pointer event (`Mouse`, `Pen`,
/// `Touch`). `Keyboard` before any interaction and during SSR.
pub fn get_pointer_type() -> PointerType {
    #[cfg(feature = "ssr")]
    {
        PointerType::Keyboard
    }
    #[cfg(not(feature = "ssr"))]
    {
        FocusState::get().pointer_type()
    }
}

#[cfg(all(test, not(feature = "ssr")))]
mod tests {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    };

    use assertr::prelude::*;

    use super::*;

    /// A subscriber recording the modalities it was notified of.
    fn record(state: &FocusState, is_text_input: bool) -> Arc<Mutex<Vec<Modality>>> {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        state.register(is_text_input, move |m: Modality| {
            sink.lock().unwrap().push(m);
        });
        seen
    }

    const TYPING: Option<KeyContext> = Some(KeyContext {
        is_text_input: true,
        is_focus_key: false,
    });
    const TAB_IN_TEXT_INPUT: Option<KeyContext> = Some(KeyContext {
        is_text_input: true,
        is_focus_key: true,
    });
    const ARROW: Option<KeyContext> = Some(KeyContext {
        is_text_input: false,
        is_focus_key: false,
    });

    #[test]
    fn pointerdown_notifies_after_silent_pointer_update() {
        let state = FocusState::new();

        let count = Arc::new(AtomicU32::new(0));
        let count_clone = Arc::clone(&count);
        let id = state.register(false, move |_: Modality| {
            count_clone.fetch_add(1, Ordering::Relaxed);
        });

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(count.load(Ordering::Relaxed)).is_equal_to(1);

        // pointermove: stored silently.
        state.set_modality_silently(Modality::Pointer, PointerType::Touch);
        assert_that!(count.load(Ordering::Relaxed)).is_equal_to(1);
        assert_that!(state.modality()).is_equal_to(Modality::Pointer);
        assert_that!(state.pointer_type()).is_equal_to(PointerType::Touch);

        // pointerdown: always notifies.
        state.set_modality_and_notify(Modality::Pointer, PointerType::Pen, None);
        assert_that!(count.load(Ordering::Relaxed)).is_equal_to(2);
        assert_that!(state.pointer_type()).is_equal_to(PointerType::Pen);

        state.unregister(id);
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(count.load(Ordering::Relaxed)).is_equal_to(2);
    }

    /// useFocusVisible.test.js, "emits on modality change (non-text input)".
    #[test]
    fn non_text_input_subscribers_see_every_key() {
        let state = FocusState::new();
        let seen = record(&state, false);

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);

        assert_that!(seen.lock().unwrap().clone())
            .is_equal_to(vec![Modality::Keyboard, Modality::Pointer]);
    }

    /// useFocusVisible.test.js, "emits on modality change (text input)": typing in a text input
    /// doesn't make focus visible, Tab and Escape do; the stored modality changes regardless.
    #[test]
    fn text_input_subscribers_see_only_focus_keys() {
        let state = FocusState::new();
        let text_input = record(&state, true);
        let other = record(&state, false);

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(text_input.lock().unwrap().len()).is_equal_to(0);
        assert_that!(other.lock().unwrap().len()).is_equal_to(1);

        // Typing where the active element is a text input reaches no one.
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, TYPING);
        assert_that!(other.lock().unwrap().len()).is_equal_to(1);
        assert_that!(state.modality()).is_equal_to(Modality::Keyboard);

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, TAB_IN_TEXT_INPUT);
        assert_that!(text_input.lock().unwrap().clone()).is_equal_to(vec![Modality::Keyboard]);
        assert_that!(other.lock().unwrap().len()).is_equal_to(2);
    }

    /// A programmatic keyboard modality (`set_modality`, the `invalid` handler) reaches text input
    /// subscribers too (react-aria: no event, so `isKeyboardFocusEvent` is true).
    #[test]
    fn programmatic_keyboard_modality_reaches_text_inputs() {
        let state = FocusState::new();
        let text_input = record(&state, true);

        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, None);

        assert_that!(text_input.lock().unwrap().clone())
            .is_equal_to(vec![Modality::Pointer, Modality::Keyboard]);
    }
}
