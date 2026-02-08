use educe::Educe;
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos_use::use_event_listener;
use send_wrapper::SendWrapper;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::focus::focus_element;
use crate::utils::{
    current_target_contains_target,
    pointer_type::PointerType,
    use_continue_propagation,
    virtual_click::{is_virtual_click, is_virtual_pointer_event},
    ElementExt, EventExt, EventHandler, EventModifiers, EventTargetExt, Modifiers,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/usePress.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// React-aria's `usePress` does not handle double-click. Double-click behavior
// lives in `useSelectableItem` (where double-click triggers an action). We add
// `on_double_press` here as a convenience so that any pressable element can opt
// into double-press handling without requiring a full selection model.
//
// React-aria has a separate `useLongPress` hook that wraps `usePress`. We merged
// long press detection directly into `usePress` to avoid double-hook overhead
// when both press and long press are needed on the same element (e.g. menu triggers).

/// The default long press threshold in milliseconds.
pub const DEFAULT_LONG_PRESS_THRESHOLD: u64 = 500;

/// The type of long press event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongPressEventType {
    /// The long press interaction has started.
    LongPressStart,
    /// The long press threshold time was met.
    LongPress,
    /// The long press interaction has ended.
    LongPressEnd,
}

/// Event fired during long press interactions.
#[derive(Debug)]
pub struct LongPressEvent {
    /// The type of long press event.
    pub event_type: LongPressEventType,

    /// The pointer type that triggered the long press event.
    pub pointer_type: PointerType,

    /// The target element of the long press event.
    pub target: Option<SendWrapper<web_sys::EventTarget>>,

    /// States which modifier keys were held during the long press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub y: Option<f64>,
}

#[derive(Debug)]
pub enum PressEvents {
    PressStart(PressEvent),
    PressEnd(PressEvent),
    PressUp(PressEvent),
    Press(PressEvent),
}

#[derive(Educe)]
#[educe(Debug)]
pub struct PressEvent {
    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The target element of the press event.
    pub target: Option<send_wrapper::SendWrapper<web_sys::EventTarget>>,

    /// States which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer at the time of the press event.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer at the time of the press event.
    /// `None` for keyboard events.
    pub y: Option<f64>,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continue_propagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Arc<dyn Fn() + Send + Sync + 'static>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy)]
pub struct UsePressInput {
    /// Whether the targeted element is currently disabled.
    pub disabled: Signal<bool>,

    /// Set this to true if you want controlled press behavior
    /// with the guarantee of no browser-specific behavior happening on user interactions.
    pub force_prevent_default: bool,

    pub allow_propagation: bool,

    /// When `true`, text selection is not disabled during press interactions.
    /// By default (`false`), text selection is disabled to prevent accidental selection
    /// while pressing.
    pub allow_text_selection_on_press: bool,

    /// When `true`, the press is cancelled entirely when the pointer exits the target.
    /// By default (`false`), the user can press, drag outside, drag back in, and still
    /// complete the press.
    pub should_cancel_on_pointer_exit: bool,

    pub on_press: Callback<PressEvent>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,

    /// Called when the press state changes. Receives `true` when press starts,
    /// `false` when press ends.
    pub on_press_change: Option<Callback<bool>>,

    /// Called when the element receives a native `dblclick` event.
    pub on_double_press: Option<Callback<PressEvent>>,

    // Long press fields (all optional — when all are None, behavior is identical to press-only).
    /// Handler called when a long press interaction starts (mouse/touch only).
    pub on_long_press_start: Option<Callback<LongPressEvent>>,

    /// Handler called when the long press threshold time is met.
    pub on_long_press: Option<Callback<LongPressEvent>>,

    /// Handler called when a long press interaction ends.
    pub on_long_press_end: Option<Callback<LongPressEvent>>,

    /// The amount of time in milliseconds to wait before triggering a long press.
    /// Default is 500ms. Only used when at least one long press callback is set.
    pub long_press_threshold: Option<u64>,

    /// A description for assistive technology users indicating that a long press
    /// action is available, e.g. "Long press to open menu".
    /// Only applied when `on_long_press` is `Some`.
    pub long_press_accessibility_description: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct UsePressReturn {
    /// Props for programmatic merging. Call `.to_attrs()` for view spreading.
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
}

/// Props from `use_press` that can be extracted and merged programmatically.
///
/// Use [`UsePressProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
///
/// Use [`UsePressProps::to_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.to_attrs()}>`) (without takin ownership, requiring internal
/// cloning).
///
/// # Example
///
/// ```ignore
/// // Basic usage
/// let press = use_press(input);
/// view! { <button {..press.props.into_attrs()}>"Click"</button> }
///
/// // Merging multiple press hooks
/// let press1 = use_press(input1);
/// let press2 = use_press(input2);
/// let merged = press1.props.merge(press2.props);
/// view! { <button {..merged.into_attrs()}>"Both handlers fire"</button> }
/// ```
#[derive(Debug, Clone)]
pub struct UsePressProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<web_sys::DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    /// Accessibility description for long press action.
    /// Set when `on_long_press` is provided; `None` otherwise.
    pub aria_describedby: Option<&'static str>,
}

impl UsePressProps {
    /// Convert to spreadable attributes for Leptos views.
    #[must_use]
    pub fn to_attrs(&self) -> UsePressAttrs {
        (
            self.on_keydown.to_on(ev::keydown),
            self.on_click.to_on(ev::click),
            self.on_pointerdown.to_on(ev::pointerdown),
            self.on_dragstart.to_on(ev::dragstart),
            self.on_dblclick.to_on(ev::dblclick),
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }

    /// Convert to spreadable attributes for Leptos views.
    #[must_use]
    pub fn into_attrs(self) -> UsePressAttrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_dblclick.into_on(ev::dblclick),
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UsePressAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<web_sys::DragEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
);

enum EventHandlers {
    PointerEvents {
        global_on_pointer_move_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
        global_on_pointer_up_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
        global_on_pointer_cancel_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    },
    KeyboardEvents {
        global_on_key_up_cleanup: Box<dyn Fn() + Send + Sync + 'static>,
    },
}

struct PressState {
    pointer_id: i32,
    pointer_type: PointerType,
    target: Option<web_sys::EventTarget>,
    is_over_target: bool,

    /// Tracks whether `trigger_press_start` was actually fired, to prevent
    /// firing `press_end` without a corresponding `press_start`.
    did_fire_press_start: bool,

    /// Handle for the 80ms fallback timeout that triggers `target.click()`
    /// when iOS long press doesn't naturally fire a click event.
    click_timeout_handle: Option<TimeoutHandle>,

    event_handlers: EventHandlers,

    // Long press tracking
    /// Timeout handle for the long press threshold timer.
    long_press_timeout_handle: Option<TimeoutHandle>,
    /// Whether the long press threshold was met during this interaction.
    long_press_triggered: bool,
}

impl PressState {
    fn cleanup_event_handlers(&self) {
        match &self.event_handlers {
            EventHandlers::PointerEvents {
                global_on_pointer_move_cleanup,
                global_on_pointer_up_cleanup,
                global_on_pointer_cancel_cleanup,
            } => {
                global_on_pointer_move_cleanup();
                global_on_pointer_up_cleanup();
                global_on_pointer_cancel_cleanup();
            }
            EventHandlers::KeyboardEvents {
                global_on_key_up_cleanup,
            } => {
                global_on_key_up_cleanup();
            }
        }
    }

    fn clear_click_timeout(&mut self) {
        if let Some(handle) = self.click_timeout_handle.take() {
            handle.clear();
        }
    }

    fn clear_long_press_timeout(&mut self) {
        if let Some(handle) = self.long_press_timeout_handle.take() {
            handle.clear();
        }
    }

    fn restore_text_selection_if_needed(&self, allow_text_selection_on_press: bool) {
        if !allow_text_selection_on_press {
            if let Some(target) = self.target.as_ref() {
                if let Some(target) = target.as_element() {
                    target.restore_text_selection();
                }
            }
        }
    }

    fn is_pointer_over_target(&self, e: &PointerEvent) -> bool {
        e.current_target().unwrap().is_over(
            e,
            self.target
                .as_ref()
                .and_then(EventTargetExt::as_element)
                .unwrap(),
        )
    }
}

enum EventRef<'a> {
    Pointer(&'a PointerEvent),
    Keyboard(&'a KeyboardEvent),
    Mouse(&'a MouseEvent),
}

impl EventRef<'_> {
    fn modifiers(&self) -> Modifiers {
        match self {
            EventRef::Pointer(e) => e.modifiers(),
            EventRef::Keyboard(e) => e.modifiers(),
            EventRef::Mouse(e) => e.modifiers(),
        }
    }

    fn stop_propagation(&self) {
        match self {
            EventRef::Pointer(e) => e.stop_propagation(),
            EventRef::Keyboard(e) => e.stop_propagation(),
            EventRef::Mouse(e) => e.stop_propagation(),
        }
    }

    fn target(&self) -> Option<web_sys::EventTarget> {
        match self {
            EventRef::Pointer(e) => e.target(),
            EventRef::Keyboard(e) => e.target(),
            EventRef::Mouse(e) => e.target(),
        }
    }

    fn coordinates(&self) -> (Option<f64>, Option<f64>) {
        match self {
            EventRef::Pointer(e) => (Some(f64::from(e.client_x())), Some(f64::from(e.client_y()))),
            EventRef::Mouse(e) => (Some(f64::from(e.client_x())), Some(f64::from(e.client_y()))),
            EventRef::Keyboard(_) => (None, None),
        }
    }
}

fn fire_press_callback(
    callback: Callback<PressEvent>,
    state: &PressState,
    event: &EventRef<'_>,
    allow_propagation: bool,
) {
    let (continue_propagation_state, continue_propagation) = use_continue_propagation();
    let (x, y) = event.coordinates();
    callback.run(PressEvent {
        pointer_type: state.pointer_type.clone(),
        target: state.target.clone().map(send_wrapper::SendWrapper::new),
        modifiers: event.modifiers(),
        x,
        y,
        continue_propagation,
    });
    if !allow_propagation && !continue_propagation_state.load(Ordering::Acquire) {
        event.stop_propagation();
    }
}

fn is_mac() -> bool {
    web_sys::window()
        .and_then(|w| w.navigator().platform().ok())
        .is_some_and(|p| p.contains("Mac"))
}

/// # Panics
///
/// Panics if the press state is initialized while already active (debug assertion),
/// or if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines)]
pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let (is_pressed, set_is_pressed) = signal(false);

    let supports_long_press = input.on_long_press.is_some()
        || input.on_long_press_start.is_some()
        || input.on_long_press_end.is_some();
    let long_press_threshold = input
        .long_press_threshold
        .unwrap_or(DEFAULT_LONG_PRESS_THRESHOLD);

    let state: StoredValue<Option<PressState>, LocalStorage> = StoredValue::new_local(None);

    // Tracks meta key events for macOS workaround
    let meta_key_events: StoredValue<
        Option<std::collections::HashMap<String, KeyboardEvent>>,
        LocalStorage,
    > = StoredValue::new_local(None);

    let initialize_press_state = move |e: EventRef<'_>, event_handlers: EventHandlers| {
        // If a press is already active, ignore this initialization request.
        // This can happen when a second pointerdown fires before the first press's
        // click event completes the cycle (the window between pointerup and click).
        // We follow React Aria's approach of ignoring the second press rather than
        // cancelling + re-initializing, because cancel+re-init causes click event
        // cross-pollination: click#1 (from interaction 1) would complete press#2
        // (from interaction 2), since the click handler cannot distinguish which
        // press a click belongs to.
        if state.with_value(Option::is_some) {
            return;
        }

        state.set_value(Some(PressState {
            pointer_id: match e {
                EventRef::Pointer(e) => e.pointer_id(),
                EventRef::Keyboard(_) | EventRef::Mouse(_) => 0,
            },
            pointer_type: match e {
                EventRef::Pointer(e) => PointerType::from(e.pointer_type()),
                EventRef::Keyboard(_e) => PointerType::Keyboard,
                EventRef::Mouse(_e) => PointerType::Virtual,
            },
            target: e.target(),
            is_over_target: match e {
                EventRef::Pointer(e) => e
                    .current_target()
                    .unwrap()
                    .is_over(e, e.target().as_ref().unwrap().as_element().unwrap()),
                EventRef::Keyboard(_) | EventRef::Mouse(_) => false,
            },
            did_fire_press_start: false,
            click_timeout_handle: None,
            event_handlers,
            long_press_timeout_handle: None,
            long_press_triggered: false,
        }));
    };

    // Has no effect if press is already started. Calling this multiple times only executes the effect once.
    let trigger_press_start = move |s: &mut PressState, e: EventRef<'_>| {
        if s.did_fire_press_start {
            return;
        }
        s.did_fire_press_start = true;

        if let Some(on_press_start) = input.on_press_start {
            fire_press_callback(on_press_start, s, &e, input.allow_propagation);
        }

        if let Some(on_press_change) = input.on_press_change {
            on_press_change.run(true);
        }

        set_is_pressed.set(true);
    };

    // Has no effect if press was not started. Calling this multiple times only executes the effect once.
    // When `was_pressed` is true, also fires the `on_press` callback.
    let trigger_press_end = move |s: &mut PressState, e: EventRef<'_>, was_pressed: bool| {
        if !s.did_fire_press_start {
            return;
        }
        s.did_fire_press_start = false;

        // Clear long press timeout on press end.
        s.clear_long_press_timeout();

        if let Some(on_press_end) = input.on_press_end {
            fire_press_callback(on_press_end, s, &e, input.allow_propagation);
        }

        // Fire long press end for mouse/touch when long press is configured.
        if supports_long_press
            && (s.pointer_type == PointerType::Mouse || s.pointer_type == PointerType::Touch)
        {
            if let Some(on_long_press_end) = input.on_long_press_end {
                let (x, y) = e.coordinates();
                on_long_press_end.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressEnd,
                    pointer_type: s.pointer_type.clone(),
                    target: s.target.clone().map(SendWrapper::new),
                    modifiers: e.modifiers(),
                    x,
                    y,
                });
            }
        }

        if let Some(on_press_change) = input.on_press_change {
            on_press_change.run(false);
        }

        set_is_pressed.set(false);

        // Do NOT fire on_press if the long press threshold was met.
        // The short press was consumed by the long press interaction.
        if was_pressed && !s.long_press_triggered {
            fire_press_callback(input.on_press, s, &e, input.allow_propagation);
        }
    };

    let trigger_press_up = move |s: &PressState, e: EventRef<'_>| {
        if let Some(on_press_up) = input.on_press_up {
            fire_press_callback(on_press_up, s, &e, input.allow_propagation);
        }
    };

    let cancel_active_press = move |e: EventRef<'_>| {
        state.update_value(|s| {
            if let Some(s) = s {
                s.clear_click_timeout();
                s.clear_long_press_timeout();
                trigger_press_end(s, e, false);
                s.restore_text_selection_if_needed(input.allow_text_selection_on_press);
                s.cleanup_event_handlers();
            }
        });
        state.set_value(None);
    };

    let on_key_up = move |e: KeyboardEvent| {
        // First check if we should handle this event (immutable check).
        let should_handle = state.with_value(|s| {
            s.is_some()
                && !input.disabled.get_untracked()
                && is_valid_keyboard_event(&e, e.current_target().unwrap())
        });

        if !should_handle {
            return;
        }

        let key = e.key();
        if e.target()
            .and_then(|t| t.as_element())
            .is_some_and(|t| should_prevent_default_keyboard(t, key.as_str()))
        {
            e.prevent_default();
        }

        // Handle Space key on link elements: programmatically click the link
        // since browsers don't natively trigger navigation on Space for links.
        if key != "Enter" {
            if let Some(target) = e.target().and_then(|t| t.as_element()) {
                if target.is_anchor_link() {
                    if let Ok(html_el) = target.dyn_into::<web_sys::HtmlElement>() {
                        html_el.click();
                    }
                }
            }
        }

        // macOS Meta key workaround: if Meta key is up, dispatch synthetic
        // keyup events for any keys that were pressed while Meta was held.
        if key == "Meta" {
            meta_key_events.update_value(|map| {
                if let Some(events) = map.take() {
                    for (_key, stored_e) in events {
                        if let Some(ct) = stored_e.current_target() {
                            let _ = ct.dispatch_event(&stored_e);
                        }
                    }
                }
            });
        }

        // Now perform mutable operations: fire press_up and press_end.
        state.update_value(move |s| {
            if let Some(s) = s.as_mut() {
                trigger_press_up(s, EventRef::Keyboard(&e));
                trigger_press_end(s, EventRef::Keyboard(&e), true);
                s.cleanup_event_handlers();
            }
        });
        state.set_value(None);
    };

    let on_key_down_handler = move |e: KeyboardEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_key_down, as current_target did not contain target.");
            return;
        }

        // Ignore key repeats to prevent duplicate press starts when holding a key.
        if e.repeat() {
            return;
        }

        let key = e.key();

        if e.target()
            .and_then(|t| t.as_element())
            .is_some_and(|t| should_prevent_default_keyboard(t, &key))
        {
            e.prevent_default();
        }

        if state.with_value(Option::is_none)
            && !input.disabled.get_untracked()
            && is_valid_keyboard_event(&e, e.current_target().unwrap())
        {
            // macOS Meta key workaround: store events pressed while Meta is held
            // because macOS doesn't fire keyup for non-Meta keys while Meta is down.
            if is_mac() && e.meta_key() && key != "Meta" {
                meta_key_events.update_value(|map| {
                    let map = map.get_or_insert_with(std::collections::HashMap::new);
                    map.insert(key.clone(), e.clone());
                });
            }

            initialize_press_state(
                EventRef::Keyboard(&e),
                EventHandlers::KeyboardEvents {
                    global_on_key_up_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        ev::keyup,
                        on_key_up,
                    )),
                },
            );

            state.update_value(move |s| {
                if let Some(s) = s {
                    trigger_press_start(s, EventRef::Keyboard(&e));
                }
            });
        }
    };

    let on_click_handler = move |e: MouseEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_click, as current_target did not contain target.");
            return;
        }

        if input.disabled.get_untracked() {
            e.prevent_default();
            return;
        }

        if input.force_prevent_default {
            e.prevent_default();
        }
        if !input.allow_propagation {
            e.stop_propagation();
        }

        // Check if this is the completion of a pointer-initiated press.
        // After pointerup, we defer press completion to onClick for DOM mutation safety.
        let was_pointer_press = state.with_value(|s| {
            s.as_ref().is_some_and(|s| {
                s.pointer_type != PointerType::Keyboard
                    && s.pointer_type != PointerType::Virtual
                    && is_pressed.get_untracked()
            })
        });

        if was_pointer_press {
            state.update_value(|s| {
                if let Some(s) = s {
                    s.clear_click_timeout();
                    trigger_press_up(s, EventRef::Mouse(&e));
                    trigger_press_end(s, EventRef::Mouse(&e), true);
                    s.restore_text_selection_if_needed(input.allow_text_selection_on_press);
                    s.cleanup_event_handlers();
                }
            });
            state.set_value(None);
            return;
        }

        // Handle virtual click (screen reader / assistive technology)
        if !is_pressed.get_untracked() && is_virtual_click(&e) {
            // Fire full virtual press cycle
            initialize_press_state(
                EventRef::Mouse(&e),
                EventHandlers::KeyboardEvents {
                    // No global listener needed for virtual clicks — they complete immediately.
                    global_on_key_up_cleanup: Box::new(|| {}),
                },
            );

            state.update_value(|s| {
                if let Some(s) = s {
                    trigger_press_start(s, EventRef::Mouse(&e));
                    trigger_press_up(s, EventRef::Mouse(&e));
                    trigger_press_end(s, EventRef::Mouse(&e), true);
                }
            });
            state.set_value(None);
        }
    };

    // Pointer move handler for drag-in / drag-out behavior.
    let on_pointer_move = move |e: PointerEvent| {
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                if e.pointer_id() != s.pointer_id {
                    return;
                }
                let is_over_target = s.is_pointer_over_target(&e);

                if input.should_cancel_on_pointer_exit && s.is_over_target && !is_over_target {
                    // Cancel the entire press when configured to do so.
                    trigger_press_end(s, EventRef::Pointer(&e), false);
                    s.cleanup_event_handlers();
                    s.restore_text_selection_if_needed(input.allow_text_selection_on_press);
                } else {
                    match (s.is_over_target, is_over_target) {
                        (true, false) => trigger_press_end(s, EventRef::Pointer(&e), false),
                        (false, true) => trigger_press_start(s, EventRef::Pointer(&e)),
                        _ => {}
                    }
                }
                s.is_over_target = is_over_target;
            }
        });

        // If should_cancel_on_pointer_exit caused a full cancel, clear state.
        if input.should_cancel_on_pointer_exit {
            let should_clear = state.with_value(|s| {
                s.as_ref()
                    .is_some_and(|s| !s.did_fire_press_start && !s.is_over_target)
            });
            if should_clear {
                state.set_value(None);
            }
        }
    };

    // Pointer up: defer press completion to onClick for DOM mutation safety.
    let on_pointer_up = move |e: PointerEvent| {
        if !e.current_target_contains_target() {
            return;
        }

        let should_clear = state.with_value(|s| {
            let Some(s) = s.as_ref() else {
                return false;
            };

            // Pointer is not over the target — cancel the press.
            !s.is_pointer_over_target(&e)
        });

        if should_clear {
            cancel_active_press(EventRef::Pointer(&e));
            return;
        }

        // Pointer is over the target. Keep state.is_pressed=true and defer
        // actual completion to the onClick handler (Phase 3).
        // Set up an 80ms timeout fallback: on iOS, long press interactions
        // may not naturally fire a click event, so we programmatically trigger one.
        state.update_value(|s| {
            if let Some(s) = s {
                // Prevent duplicate pointerleave handling
                s.is_over_target = false;

                if input.force_prevent_default {
                    e.prevent_default();
                }
                if !input.allow_propagation {
                    e.stop_propagation();
                }

                // Set up 80ms fallback to programmatically click the target.
                let target = s.target.clone();
                s.click_timeout_handle = set_timeout_with_handle(
                    move || {
                        if let Some(target) = target {
                            if let Some(html_el) = target.dyn_ref::<web_sys::HtmlElement>() {
                                html_el.click();
                            }
                        }
                    },
                    Duration::from_millis(80),
                )
                .ok();
            }
        });
    };

    // Cancel the ongoing press.
    let on_pointer_cancel = move |e: PointerEvent| {
        cancel_active_press(EventRef::Pointer(&e));
    };

    // Start a press.
    let on_pointer_down_handler = move |e: PointerEvent| {
        if e.button() != 0 {
            return;
        }

        if !e.current_target_contains_target() {
            tracing::trace!("Aborting on_pointer_down, as current_target did not contain target.");
            return;
        }

        // Handle virtual pointer events (e.g., VoiceOver on iOS).
        // These are deferred to the onClick handler.
        if is_virtual_pointer_event(&e) {
            // Store that we saw a virtual event; onClick will handle the full press cycle.
            return;
        }

        let target = e.target();

        if !input.allow_text_selection_on_press {
            if let Some(target) = target.as_ref() {
                if let Some(target) = target.as_element() {
                    target.disable_text_selection();
                }
            }
        }

        if !input.disabled.get_untracked() {
            // Release pointer capture to enable pointerleave/pointerenter on touch.
            // By default, the browser captures pointer events to the original target,
            // which prevents these events from firing correctly.
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    if element.has_pointer_capture(e.pointer_id()) {
                        let _ = element.release_pointer_capture(e.pointer_id());
                    }
                }
            }

            initialize_press_state(
                EventRef::Pointer(&e),
                EventHandlers::PointerEvents {
                    global_on_pointer_move_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        ev::pointermove,
                        on_pointer_move,
                    )),
                    global_on_pointer_up_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        ev::pointerup,
                        on_pointer_up,
                    )),
                    global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        ev::pointercancel,
                        on_pointer_cancel,
                    )),
                },
            );

            state.update_value(move |s| {
                if let Some(s) = s {
                    trigger_press_start(s, EventRef::Pointer(&e));

                    // Start long press timer for mouse/touch when long press is configured.
                    if supports_long_press
                        && (s.pointer_type == PointerType::Mouse
                            || s.pointer_type == PointerType::Touch)
                    {
                        // Fire on_long_press_start
                        if let Some(on_long_press_start) = input.on_long_press_start {
                            let (x, y) = EventRef::Pointer(&e).coordinates();
                            on_long_press_start.run(LongPressEvent {
                                event_type: LongPressEventType::LongPressStart,
                                pointer_type: s.pointer_type.clone(),
                                target: s.target.clone().map(SendWrapper::new),
                                modifiers: EventRef::Pointer(&e).modifiers(),
                                x,
                                y,
                            });
                        }

                        // Capture values for the timeout closure
                        let pointer_type = s.pointer_type.clone();
                        let modifiers = EventRef::Pointer(&e).modifiers();
                        let target = s.target.clone();
                        let (x, y) = EventRef::Pointer(&e).coordinates();
                        let on_long_press = input.on_long_press;

                        s.long_press_timeout_handle = set_timeout_with_handle(
                            move || {
                                // Dispatch pointercancel on the target to cancel the press
                                // interaction. This is synchronous — the pointercancel handler
                                // (and thus trigger_press_end / on_long_press_end) will fire
                                // before the code after dispatch_event.
                                if let Some(ref target) = target {
                                    if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                                        let cancel_event = PointerEvent::new("pointercancel")
                                            .expect("should create pointercancel event");
                                        let _ = el.dispatch_event(&cancel_event);

                                        // Focus the element without scrolling.
                                        focus_element(el, true);
                                    }
                                }

                                // Mark long press as triggered so on_press is suppressed.
                                state.update_value(|s| {
                                    if let Some(s) = s.as_mut() {
                                        s.long_press_triggered = true;
                                        s.long_press_timeout_handle = None;
                                    }
                                });

                                // Fire the long press callback.
                                if let Some(on_long_press) = on_long_press {
                                    on_long_press.run(LongPressEvent {
                                        event_type: LongPressEventType::LongPress,
                                        pointer_type: pointer_type.clone(),
                                        target: target.map(SendWrapper::new),
                                        modifiers,
                                        x,
                                        y,
                                    });
                                }
                            },
                            Duration::from_millis(long_press_threshold),
                        )
                        .ok();

                        // For touch, prevent the context menu on the event target.
                        if s.pointer_type == PointerType::Touch {
                            if let Some(target) = s.target.as_ref() {
                                target.prevent_default_once("contextmenu");
                            }
                        }
                    }
                }
            });
        }
    };

    // Safari doesn't fire pointercancel on drag. Handle dragstart to cancel the press.
    let on_dragstart_handler = move |_e: web_sys::DragEvent| {
        cancel_active_press(EventRef::Pointer(
            &PointerEvent::new("pointercancel").unwrap(),
        ));
    };

    // Handle native dblclick for on_double_press.
    let on_dblclick_handler = move |e: MouseEvent| {
        let Some(on_double_press) = input.on_double_press else {
            return;
        };
        if input.disabled.get_untracked() {
            return;
        }

        let (continue_propagation_state, continue_propagation) = use_continue_propagation();
        on_double_press.run(PressEvent {
            pointer_type: PointerType::Mouse,
            target: e.target().map(send_wrapper::SendWrapper::new),
            modifiers: e.modifiers(),
            x: Some(f64::from(e.client_x())),
            y: Some(f64::from(e.client_y())),
            continue_propagation,
        });
        if !input.allow_propagation && !continue_propagation_state.load(Ordering::Acquire) {
            e.stop_propagation();
        }
    };

    // Only set aria-describedby when on_long_press is provided.
    let aria_describedby = if input.on_long_press.is_some() {
        input.long_press_accessibility_description
    } else {
        None
    };

    UsePressReturn {
        props: UsePressProps {
            on_keydown: EventHandler::new(on_key_down_handler),
            on_click: EventHandler::new(on_click_handler),
            on_pointerdown: EventHandler::new(on_pointer_down_handler),
            on_dragstart: EventHandler::new(on_dragstart_handler),
            on_dblclick: EventHandler::new(on_dblclick_handler),
            aria_describedby,
        },
        is_pressed: is_pressed.into(),
    }
}

/// Tests whether a keyboard event's default action should be presented when the given `key` was pressed.
fn should_prevent_default_keyboard(element: web_sys::Element, key: &str) -> bool {
    if element.is_instance_of::<web_sys::HtmlInputElement>() {
        return !is_valid_input_key(element.unchecked_into::<web_sys::HtmlInputElement>(), key);
    }

    if element.is_instance_of::<web_sys::HtmlButtonElement>() {
        return match element.get_attribute("type") {
            Some(ty) => ty != "submit" && ty != "reset",
            None => false,
        };
    }

    !element.is_anchor_link()
}

const NON_TEXT_INPUT_TYPES: [&str; 9] = [
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

#[allow(clippy::needless_pass_by_value)]
fn is_valid_input_key(element: web_sys::HtmlInputElement, key: &str) -> bool {
    // Checkboxes and radio-buttons should only toggle with space, not enter.
    match element.get_attribute("type") {
        Some(ty) => match ty.as_str() {
            "checkbox" | "radio" => key == " " || key == "Spacebar",
            other => NON_TEXT_INPUT_TYPES.contains(&other),
        },
        None => true,
    }
}

/// Accessibility for keyboards. Space and Enter only.
#[allow(clippy::needless_pass_by_value)]
fn is_valid_keyboard_event(e: &KeyboardEvent, current_target: web_sys::EventTarget) -> bool {
    let key = e.key();
    let code = e.code();
    let resembles_press =
        matches!(key.as_str(), "Enter" | " " | "Spacebar" | "Space") || code == "Space";

    if !resembles_press {
        return false;
    }

    match current_target.as_element() {
        Some(element) => {
            let is_input = element.is_instance_of::<web_sys::HtmlInputElement>();
            let is_text_area = element.is_instance_of::<web_sys::HtmlTextAreaElement>();
            let is_content_editable = element
                .dyn_ref::<web_sys::HtmlElement>()
                .is_some_and(web_sys::HtmlElement::is_content_editable);
            let is_link = element.is_link();

            // Links should only trigger with Enter key
            !(is_text_area
                || is_content_editable
                || is_input
                    && !is_valid_input_key(
                        element.unchecked_into::<web_sys::HtmlInputElement>(),
                        key.as_str(),
                    )
                || is_link && key.as_str() != "Enter")
        }
        None => true,
    }
}
