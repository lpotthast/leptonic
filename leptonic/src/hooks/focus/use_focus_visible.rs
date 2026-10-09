// Upstream: react-aria/src/interactions/useFocusVisible.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
#[cfg(not(feature = "ssr"))]
use std::sync::{
    OnceLock, RwLock,
    atomic::{AtomicBool, Ordering},
};

use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::{KeyboardEvent, PointerEvent};

use crate::utils::pointer_type::PointerType;
#[cfg(not(feature = "ssr"))]
use crate::utils::{
    dom_ext::EventAccessors,
    event_listeners::{Listener, listen, listen_to},
    focusability,
    key::{KeyboardEventKey, KeyboardKey},
    platform::device::is_mac,
    prevent_focus::is_ignoring_focus_events,
    shadow_dom::get_active_element,
    virtual_click::is_virtual_click,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The modality is `Option<Modality>`: `None` before the first interaction (react-aria: `null`).
// - No change handlers (`changeHandlers`, `useFocusVisibleListener`): the modality lives in two
//   global signals, written only when they change: the modality as of the last change
//   notification ([`use_interaction_modality`], [`is_focus_visible`]), and per kind of listener
//   (text input or not) the notifications that reached it. Hooks read them, so a key event costs
//   one signal write instead of one call per mounted listener, and an unfocused `use_focus_ring`
//   reads nothing (react-aria: `enabled: isFocused`; upstream's "does not call changeHandlers when
//   unneeded" holds by construction).
// - `get_modality`/`set_modality`/`get_pointer_type`: react-aria's `getInteractionModality`/
//   `setInteractionModality`/`getPointerType`. `is_focus_visible` is tracked when read in a
//   reactive context (react-aria's `isFocusVisible` reads a plain variable).
// - `add_window_focus_tracking` returns a guard ([`WindowFocusTracking`]) whose drop stops the
//   tracking (react-aria: a teardown function).
//
// ## DIFFERENT BEHAVIOR
// - Tracking starts when the first hook reading the modality is created
//   (`track_interaction_modality`), not on module load (`addWindowFocusTracking()` at the top
//   level): Rust has no module initialization. Without it, a grid saw no modality (treated as
//   keyboard) for mouse presses and moved focus to its old focused cell.
// - The default window's tracking is never torn down: react-aria removes it on `beforeunload`
//   and sets it up again on the next render of any hook; here hooks set it up when created, so a
//   `beforeunload` that doesn't unload (Chrome fires it for `mailto:` links and downloads) would
//   freeze the modality. Other windows (iframes) are torn down on `beforeunload` as upstream.
// - `use_interaction_modality` and `is_focus_visible` follow the modality of the last change
//   notification; `pointermove`/`pointerup` and virtual clicks change the stored modality
//   (`get_modality`) silently, as upstream, and show up there at the next notification.
// - A `use_focus_visible` created after such a silent change starts with the modality stored
//   then (react-aria: `autoFocus || isFocusVisible()`), and follows every later notification of
//   its kind.
//
// ## OMITTED FEATURES
// - The `mousedown`/`mousemove`/`mouseup` fallbacks for environments without `PointerEvent`
//   (react-aria registers them in tests only): every supported browser has pointer events.
//
// =============================================================================

/// Input parameters for the `use_focus_visible` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseFocusVisibleInput {
    /// Whether the element will be auto focused: focus counts as visible until the next change of
    /// the modality.
    pub auto_focus: bool,
    /// Whether the element is a text input. When `true`, only Tab/Escape keys
    /// trigger focus-visible; other keyboard events are suppressed. This is
    /// used for compound text-input components (e.g., a date picker where focus
    /// is on a button but the component should use text-input focus rules).
    pub is_text_input: bool,
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
/// This hook follows the current interaction modality:
/// - When the user interacts with the keyboard or virtually (though assistive technology), focus
///   is made visible.
/// - When they use a pointer (mouse, touch, ...), focus is hidden.
///
/// For the modality itself, see [`use_interaction_modality`].
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
    let UseFocusVisibleInput {
        auto_focus,
        is_text_input,
    } = input;

    #[cfg(feature = "ssr")]
    {
        // As on the client before any interaction: no modality yet, which shows focus.
        let _ = (auto_focus, is_text_input);
        UseFocusVisibleReturn {
            focus_should_be_visible: Signal::stored(true),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let state = FocusState::get();
        // Set up global listeners for the default window (idempotent).
        setup_global_focus_events(None);

        let changes = state.changes.clone();
        let kind = ListenerKind::from_is_text_input(is_text_input);
        let since = FocusVisibleSince::now(state, kind, auto_focus || state.is_focus_visible());
        UseFocusVisibleReturn {
            focus_should_be_visible: Memo::new(move |_| since.visible(&changes)).into(),
        }
    }
}

/// An input modality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modality {
    Pointer,
    Keyboard,
    Virtual,
}

#[cfg(not(feature = "ssr"))]
/// Which listeners a change notification reaches (react-aria's `isKeyboardFocusEvent`): keys
/// typed into a text input other than Tab and Escape reach no text input listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListenerKind {
    /// A listener of an element that isn't a text input.
    Other,
    /// A listener of a text input (`is_text_input`).
    TextInput,
}

#[cfg(not(feature = "ssr"))]
impl ListenerKind {
    pub(crate) fn from_is_text_input(is_text_input: bool) -> Self {
        if is_text_input {
            Self::TextInput
        } else {
            Self::Other
        }
    }
}

#[cfg(not(feature = "ssr"))]
/// The change notifications one kind of listener received.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Notifications {
    /// How many notifications reached the kind.
    pub(crate) count: u64,
    /// Whether focus was visible after the last of them.
    pub(crate) is_focus_visible: bool,
}

#[cfg(not(feature = "ssr"))]
/// The change notifications so far, per kind of listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FocusVisibleChanges {
    other: Notifications,
    text_input: Notifications,
}

#[cfg(not(feature = "ssr"))]
impl FocusVisibleChanges {
    const INITIAL: Self = Self {
        other: Notifications {
            count: 0,
            is_focus_visible: true,
        },
        text_input: Notifications {
            count: 0,
            is_focus_visible: true,
        },
    };

    pub(crate) fn of(&self, kind: ListenerKind) -> Notifications {
        match kind {
            ListenerKind::Other => self.other,
            ListenerKind::TextInput => self.text_input,
        }
    }

    fn of_mut(&mut self, kind: ListenerKind) -> &mut Notifications {
        match kind {
            ListenerKind::Other => &mut self.other,
            ListenerKind::TextInput => &mut self.text_input,
        }
    }
}

#[cfg(not(feature = "ssr"))]
/// Whether focus is visible for a listener that started at some point with a value of its own
/// (react-aria's `useState(autoFocus || isFocusVisible())` followed by its change handler): the
/// start value until the next notification of the listener's kind, then that notification's.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FocusVisibleSince {
    kind: ListenerKind,
    start: u64,
    is_focus_visible: bool,
}

#[cfg(not(feature = "ssr"))]
impl FocusVisibleSince {
    #[cfg(not(feature = "ssr"))]
    pub(crate) fn now(state: &FocusState, kind: ListenerKind, is_focus_visible: bool) -> Self {
        Self {
            kind,
            start: state
                .changes
                .with_untracked(|changes| changes.of(kind).count),
            is_focus_visible,
        }
    }

    /// Reads (and tracks) the notifications.
    pub(crate) fn visible(&self, changes: &ArcRwSignal<FocusVisibleChanges>) -> bool {
        let notifications = changes.with(|changes| changes.of(self.kind));
        if notifications.count == self.start {
            self.is_focus_visible
        } else {
            notifications.is_focus_visible
        }
    }
}

/// What a keyboard event tells the listeners (react-aria's `isKeyboardFocusEvent` inputs).
#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, Copy)]
struct KeyContext {
    /// Whether the event's target or the active element is a text input.
    is_text_input: bool,
    /// Whether the key makes focus visible even in text inputs (Tab, Escape).
    is_focus_key: bool,
}

#[cfg(not(feature = "ssr"))]
static FOCUS_STATE: OnceLock<FocusState> = OnceLock::new();

/// Global focus state: the current interaction modality and pointer type, and the signals
/// following its changes.
#[cfg(not(feature = "ssr"))]
pub(crate) struct FocusState {
    /// The current interaction modality, also changed silently (react-aria's `currentModality`).
    modality: RwLock<Option<Modality>>,

    /// The pointer type of the last interaction (react-aria's `currentPointerType`).
    pointer_type: RwLock<PointerType>,

    /// Whether a keyboard/pointer event occurred before the current focus event.
    /// Used to detect programmatic/virtual focus (screen readers).
    has_event_before_focus: AtomicBool,

    /// Whether the window was recently blurred or focused. Used to avoid false positives
    /// when returning to the tab.
    has_blurred_window_recently: AtomicBool,

    /// The modality as of the last change notification, written only when it changes
    /// (react-aria's `useInteractionModality` state).
    notified_modality: ArcRwSignal<Option<Modality>>,

    /// The change notifications per kind of listener, written only when one reaches a listener.
    pub(crate) changes: ArcRwSignal<FocusVisibleChanges>,
}

#[cfg(not(feature = "ssr"))]
impl FocusState {
    fn new() -> Self {
        Self {
            modality: RwLock::new(None),
            pointer_type: RwLock::new(PointerType::Keyboard),
            has_event_before_focus: AtomicBool::new(false),
            has_blurred_window_recently: AtomicBool::new(false),
            notified_modality: ArcRwSignal::new(None),
            changes: ArcRwSignal::new(FocusVisibleChanges::INITIAL),
        }
    }

    pub(crate) fn get() -> &'static Self {
        FOCUS_STATE.get_or_init(FocusState::new)
    }

    fn modality(&self) -> Option<Modality> {
        *self.modality.read().expect("modality lock poisoned")
    }

    /// React-aria's `isFocusVisible()`: the stored modality isn't the pointer.
    pub(crate) fn is_focus_visible(&self) -> bool {
        self.modality() != Some(Modality::Pointer)
    }

    fn pointer_type(&self) -> PointerType {
        *self
            .pointer_type
            .read()
            .expect("pointer type lock poisoned")
    }

    /// Stores the modality and pointer type without notifying (react-aria's
    /// `pointermove`/`pointerup` and virtual clicks).
    fn set_modality_silently(&self, modality: Modality, pointer_type: PointerType) {
        *self.modality.write().expect("modality lock poisoned") = Some(modality);
        *self
            .pointer_type
            .write()
            .expect("pointer type lock poisoned") = pointer_type;
    }

    /// Stores the modality and pointer type and notifies (react-aria's `triggerChangeHandlers`).
    /// `key` is the keyboard event's context, `None` for other events and programmatic changes.
    fn set_modality_and_notify(
        &self,
        modality: Modality,
        pointer_type: PointerType,
        key: Option<KeyContext>,
    ) {
        self.set_modality_silently(modality, pointer_type);
        self.notify(modality, key);
    }

    /// Updates the signals: the notified modality, and the notifications of the kinds of
    /// listeners the change reaches. A keyboard event inside a text input (the listener's own
    /// `is_text_input`, or the event's [`KeyContext`]) reaches them only for Tab and Escape.
    fn notify(&self, modality: Modality, key: Option<KeyContext>) {
        if self.notified_modality.get_untracked() != Some(modality) {
            self.notified_modality.set(Some(modality));
        }

        let reaches = |kind: ListenerKind| {
            modality != Modality::Keyboard
                || key.is_none_or(|key| {
                    key.is_focus_key || !(kind == ListenerKind::TextInput || key.is_text_input)
                })
        };
        let reached: Vec<ListenerKind> = [ListenerKind::Other, ListenerKind::TextInput]
            .into_iter()
            .filter(|kind| reaches(*kind))
            .collect();
        if reached.is_empty() {
            return;
        }
        let is_focus_visible = modality != Modality::Pointer;
        self.changes.update(|changes| {
            for kind in reached {
                let notifications = changes.of_mut(kind);
                notifications.count = notifications.count.wrapping_add(1);
                notifications.is_focus_visible = is_focus_visible;
            }
        });
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
}

/// The tracking of one window: its listeners (removed when dropped) and the replaced
/// `HTMLElement.prototype.focus`.
#[cfg(not(feature = "ssr"))]
struct WindowTracking {
    window: wasm_bindgen::JsValue,
    /// The original `HTMLElement.prototype.focus` and the closure its replacement calls, kept for
    /// restoration (`None` when the prototype couldn't be patched).
    focus_override: Option<FocusOverride>,
    _listeners: Vec<Listener>,
}

#[cfg(not(feature = "ssr"))]
impl Drop for WindowTracking {
    fn drop(&mut self) {
        if let Some(focus_override) = &self.focus_override {
            focus_override.uninstall();
        }
    }
}

// The tracked windows (react-aria's `hasSetupGlobalListeners`): the default one, plus iframes
// added with `add_window_focus_tracking`.
#[cfg(not(feature = "ssr"))]
thread_local! {
    static TRACKED_WINDOWS: std::cell::RefCell<Vec<WindowTracking>>
        = const { std::cell::RefCell::new(Vec::new()) };
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

/// Whether the window of `document` is tracked.
#[cfg(not(feature = "ssr"))]
fn is_tracked(window: &wasm_bindgen::JsValue) -> bool {
    TRACKED_WINDOWS.with_borrow(|tracked| tracked.iter().any(|t| t.window == *window))
}

/// Set up global focus event listeners for a window, identified by an element within it.
///
/// Pass `None` for the default window. This is idempotent: calling it multiple times
/// for the same window is a no-op.
///
/// Based on react-aria's `setupGlobalFocusEvents`.
#[cfg(not(feature = "ssr"))]
fn setup_global_focus_events(element: Option<&web_sys::Element>) {
    let (Some(window), Some(document)) = resolve_window_and_document(element) else {
        return;
    };
    let window_js: wasm_bindgen::JsValue = window.clone().into();
    if is_tracked(&window_js) {
        return;
    }

    let state = FocusState::get();
    let mut listeners = Vec::new();
    register_document_listeners(state, &document, &mut listeners);
    register_window_listeners(state, &window, &mut listeners);
    // Only other windows are torn down when they unload (see the deviations).
    let is_default_window = leptos_use::use_window()
        .as_ref()
        .is_some_and(|default| wasm_bindgen::JsValue::from(default.clone()) == window_js);
    if !is_default_window {
        let window_key = window_js.clone();
        listeners.push(listen(window.as_ref(), "beforeunload", false, move |_| {
            tear_down_window(&window_key);
        }));
    }

    // Programmatic `focus()` calls shouldn't change the modality, while other focus events
    // without a preceding user event (e.g. screen reader focus) switch it to virtual.
    let focus_override = FocusOverride::install(&window, state);

    TRACKED_WINDOWS.with_borrow_mut(|tracked| {
        tracked.push(WindowTracking {
            window: window_js,
            focus_override,
            _listeners: listeners,
        });
    });
}

/// Register capture-phase document listeners for keyboard, click, pointer and `invalid` events.
#[cfg(not(feature = "ssr"))]
fn register_document_listeners(
    state: &'static FocusState,
    document: &web_sys::Document,
    listeners: &mut Vec<Listener>,
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
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::keydown,
        true,
        handle_keyboard,
    ));
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::keyup,
        true,
        handle_keyboard,
    ));

    // Virtual (screen reader) clicks.
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::click,
        true,
        move |e: web_sys::MouseEvent| {
            if !crate::utils::open_link::is_opening_link() && is_virtual_click(&e) {
                state.set_has_event_before_focus(true);
                state.set_modality_silently(Modality::Virtual, PointerType::Virtual);
            }
        },
    ));

    // A focus move right after a form became invalid is a forms library (or the browser) focusing
    // the first invalid field: show the focus ring there.
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::invalid,
        true,
        |e: web_sys::Event| {
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
        },
    ));

    // `pointerdown` changes the modality and notifies, `pointermove`/`pointerup` only change it.
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::pointerdown,
        true,
        move |e: PointerEvent| {
            state.set_has_event_before_focus(true);
            state.set_modality_and_notify(Modality::Pointer, PointerType::of(&e), None);
        },
    ));
    let handle_pointer_silent = move |e: PointerEvent| {
        state.set_modality_silently(Modality::Pointer, PointerType::of(&e));
    };
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::pointermove,
        true,
        handle_pointer_silent,
    ));
    listeners.push(listen_to(
        document.as_ref(),
        leptos::ev::pointerup,
        true,
        handle_pointer_silent,
    ));
}

/// Register window-level listeners for blur and focus events.
#[cfg(not(feature = "ssr"))]
fn register_window_listeners(
    state: &'static FocusState,
    window: &web_sys::Window,
    listeners: &mut Vec<Listener>,
) {
    // Reset state when tabbing away from the page.
    listeners.push(listen_to(
        window.as_ref(),
        leptos::ev::blur,
        false,
        move |_: web_sys::FocusEvent| {
            // Focus moved back by `prevent_focus` (react-aria's `ignoreFocusEvent`).
            if is_ignoring_focus_events() {
                return;
            }
            state.set_has_event_before_focus(false);
            state.set_has_blurred_window_recently(true);
        },
    ));

    // Detect a return to the tab.
    listeners.push(listen_to(
        window.as_ref(),
        leptos::ev::focus,
        true,
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
    ));
}

/// Resolve the window and document for an optional element.
/// Falls back to the default window and its document when `element` is `None`.
#[cfg(not(feature = "ssr"))]
fn resolve_window_and_document(
    element: Option<&web_sys::Element>,
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

/// Stops tracking a window (its listeners are removed and the original `focus` restored when its
/// tracking drops).
#[cfg(not(feature = "ssr"))]
fn tear_down_window(window: &wasm_bindgen::JsValue) {
    // Dropped outside the borrow: dropping a listener may run JavaScript.
    let removed = TRACKED_WINDOWS.with_borrow_mut(|tracked| {
        tracked
            .iter()
            .position(|t| t.window == *window)
            .map(|pos| tracked.swap_remove(pos))
    });
    drop(removed);
}

/// Focus-visible tracking of another window (an iframe), started by
/// [`add_window_focus_tracking`] and stopped when dropped.
#[must_use = "dropping the guard stops tracking the window"]
pub struct WindowFocusTracking {
    #[cfg(not(feature = "ssr"))]
    window: Option<wasm_bindgen::JsValue>,
    /// Waits for `DOMContentLoaded` of a document still loading.
    #[cfg(not(feature = "ssr"))]
    _load_listener: Option<Listener>,
}

impl std::fmt::Debug for WindowFocusTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowFocusTracking")
            .finish_non_exhaustive()
    }
}

impl Drop for WindowFocusTracking {
    fn drop(&mut self) {
        #[cfg(not(feature = "ssr"))]
        if let Some(window) = &self.window {
            tear_down_window(window);
        }
    }
}

/// Tracks the interaction modality in the window containing `element` (an iframe's window) until
/// the returned guard is dropped (or the window unloads). The default window is always tracked;
/// for it, this does nothing.
///
/// Apps rendering parts of their UI into an iframe call this with an element inside it, so focus
/// rings there follow the interactions in the iframe. A document still loading is set up once its
/// `DOMContentLoaded` fired.
///
/// During SSR, this does nothing.
///
/// # Example
///
/// ```ignore
/// // Inside a component that manages an iframe, once its document has an element:
/// let tracking = StoredValue::new_local(Some(add_window_focus_tracking(&iframe_body)));
/// on_cleanup(move || tracking.set_value(None));
/// ```
pub fn add_window_focus_tracking(element: &web_sys::Element) -> WindowFocusTracking {
    #[cfg(feature = "ssr")]
    {
        let _ = element;
        WindowFocusTracking {}
    }

    #[cfg(not(feature = "ssr"))]
    {
        let (Some(window), Some(document)) = resolve_window_and_document(Some(element)) else {
            return WindowFocusTracking {
                window: None,
                _load_listener: None,
            };
        };
        let window_js: wasm_bindgen::JsValue = window.into();
        let is_default_window = leptos_use::use_window()
            .as_ref()
            .is_some_and(|default| wasm_bindgen::JsValue::from(default.clone()) == window_js);
        if is_default_window {
            setup_global_focus_events(None);
            return WindowFocusTracking {
                window: None,
                _load_listener: None,
            };
        }

        let is_loading = js_sys::Reflect::get(&document, &"readyState".into())
            .ok()
            .and_then(|state| state.as_string())
            .is_some_and(|state| state == "loading");
        let load_listener = if is_loading {
            let element = send_wrapper::SendWrapper::new(element.clone());
            Some(listen(
                document.as_ref(),
                "DOMContentLoaded",
                false,
                move |_| {
                    setup_global_focus_events(Some(&element));
                },
            ))
        } else {
            setup_global_focus_events(Some(element));
            None
        };
        WindowFocusTracking {
            window: Some(window_js),
            _load_listener: load_listener,
        }
    }
}

/// The current interaction modality (react-aria's `useInteractionModality`): `None` before the
/// first interaction and during server-side rendering. It changes with every change
/// notification (key presses, pointer presses, focus without a preceding event, [`set_modality`]).
pub fn use_interaction_modality() -> Signal<Option<Modality>> {
    #[cfg(feature = "ssr")]
    {
        Signal::stored(None)
    }
    #[cfg(not(feature = "ssr"))]
    {
        setup_global_focus_events(None);
        FocusState::get().notified_modality.clone().into()
    }
}

/// Whether keyboard focus is visible: the modality isn't the pointer (react-aria's
/// `isFocusVisible`). Tracked when read in a reactive context (changing with
/// [`use_interaction_modality`]), so per-item hooks can derive their focus ring from it without a
/// listener of their own. `true` during server-side rendering.
pub fn is_focus_visible() -> bool {
    #[cfg(feature = "ssr")]
    {
        true
    }
    #[cfg(not(feature = "ssr"))]
    {
        FocusState::get()
            .notified_modality
            .with(|modality| *modality != Some(Modality::Pointer))
    }
}

/// Starts tracking the interaction modality (idempotent, a no-op during SSR). react-aria tracks it
/// from module load; here, every hook reading the modality in its event handlers (`get_modality`)
/// calls this when it is created, so the modality is known by the first interaction.
pub(crate) fn track_interaction_modality() {
    #[cfg(not(feature = "ssr"))]
    setup_global_focus_events(None);
}

/// Returns the current input modality, without tracking (react-aria's `getInteractionModality`).
/// `None` before any interaction (or before any hook started tracking it, see
/// `track_interaction_modality`) and during SSR.
pub fn get_modality() -> Option<Modality> {
    #[cfg(feature = "ssr")]
    {
        None
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
            Modality::Keyboard => PointerType::Keyboard,
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
    use assertr::prelude::*;

    use super::*;

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

    fn notifications(state: &FocusState, kind: ListenerKind) -> Notifications {
        state.changes.with_untracked(|changes| changes.of(kind))
    }

    #[test]
    fn pointerdown_notifies_after_silent_pointer_update() {
        let state = FocusState::new();

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(notifications(&state, ListenerKind::Other).count).is_equal_to(1);

        // pointermove: stored silently.
        state.set_modality_silently(Modality::Pointer, PointerType::Touch);
        assert_that!(notifications(&state, ListenerKind::Other).count).is_equal_to(1);
        assert_that!(state.notified_modality.get_untracked()).is_equal_to(Some(Modality::Keyboard));
        assert_that!(state.modality()).is_equal_to(Some(Modality::Pointer));
        assert_that!(state.pointer_type()).is_equal_to(PointerType::Touch);

        // pointerdown: always notifies.
        state.set_modality_and_notify(Modality::Pointer, PointerType::Pen, None);
        assert_that!(notifications(&state, ListenerKind::Other)).is_equal_to(Notifications {
            count: 2,
            is_focus_visible: false,
        });
        assert_that!(state.notified_modality.get_untracked()).is_equal_to(Some(Modality::Pointer));
        assert_that!(state.pointer_type()).is_equal_to(PointerType::Pen);
    }

    /// useFocusVisible.test.js, "emits on modality change (non-text input)".
    #[test]
    fn non_text_input_listeners_see_every_key() {
        let state = FocusState::new();

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(notifications(&state, ListenerKind::Other)).is_equal_to(Notifications {
            count: 1,
            is_focus_visible: true,
        });
        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);
        assert_that!(notifications(&state, ListenerKind::Other)).is_equal_to(Notifications {
            count: 2,
            is_focus_visible: false,
        });
    }

    /// useFocusVisible.test.js, "emits on modality change (text input)": typing in a text input
    /// doesn't make focus visible, Tab and Escape do; the stored modality changes regardless.
    #[test]
    fn text_input_listeners_see_only_focus_keys() {
        let state = FocusState::new();
        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
        assert_that!(notifications(&state, ListenerKind::TextInput)).is_equal_to(Notifications {
            count: 1,
            is_focus_visible: false,
        });
        assert_that!(notifications(&state, ListenerKind::Other).count).is_equal_to(2);

        // Typing where the active element is a text input reaches no one.
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, TYPING);
        assert_that!(notifications(&state, ListenerKind::Other).count).is_equal_to(2);
        assert_that!(state.modality()).is_equal_to(Some(Modality::Keyboard));

        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, TAB_IN_TEXT_INPUT);
        assert_that!(notifications(&state, ListenerKind::TextInput)).is_equal_to(Notifications {
            count: 2,
            is_focus_visible: true,
        });
        assert_that!(notifications(&state, ListenerKind::Other).count).is_equal_to(3);
    }

    /// A programmatic keyboard modality (`set_modality`, the `invalid` handler) reaches text input
    /// listeners too (react-aria: no event, so `isKeyboardFocusEvent` is true).
    #[test]
    fn programmatic_keyboard_modality_reaches_text_inputs() {
        let state = FocusState::new();

        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, None);

        assert_that!(notifications(&state, ListenerKind::TextInput)).is_equal_to(Notifications {
            count: 2,
            is_focus_visible: true,
        });
    }

    /// A listener keeps its start value until the next notification of its kind, then follows
    /// the notifications (react-aria's `useFocusVisible` with `autoFocus`).
    #[test]
    fn a_listener_starts_with_its_own_value() {
        let state = FocusState::new();
        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);

        let auto_focused = FocusVisibleSince::now(&state, ListenerKind::Other, true);
        assert_that!(auto_focused.visible(&state.changes)).is_true();

        // Typing in a text input reaches no listener: the start value stays.
        state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, TYPING);
        assert_that!(auto_focused.visible(&state.changes)).is_true();

        state.set_modality_and_notify(Modality::Pointer, PointerType::Mouse, None);
        assert_that!(auto_focused.visible(&state.changes)).is_false();
    }

    /// Only changes write the notified modality: listeners of it (per-item hooks) don't rerun on
    /// every key.
    #[test]
    fn the_notified_modality_is_written_only_on_change() {
        crate::testing::with_owner(|| {
            let state = FocusState::new();
            let modality = state.notified_modality.clone();
            let runs = ArcRwSignal::new(0);
            let counter = runs.clone();
            Effect::new(move |_| {
                modality.track();
                *counter.write_untracked() += 1;
            });
            crate::testing::flush_effects();
            assert_that!(runs.get_untracked()).is_equal_to(1);

            for _ in 0..3 {
                state.set_modality_and_notify(Modality::Keyboard, PointerType::Keyboard, ARROW);
                crate::testing::flush_effects();
            }
            assert_that!(runs.get_untracked()).is_equal_to(2);
        });
    }
}
