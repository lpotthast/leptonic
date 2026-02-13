use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use educe::Educe;
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::style::Style,
};
use leptos_use::use_event_listener;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, EventTarget, HtmlElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent,
    MouseEvent, PointerEvent,
};

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::AriaDescribedby,
        focus::focus_element,
        is_over, node_contains,
        open_link::open_link,
        platform::device,
        pointer_type::PointerType,
        style::TouchActionStyle,
        use_continue_propagation,
        use_description::use_description,
        virtual_click::{is_virtual_click, is_virtual_pointer_event},
        ContainsTarget, ElementExt, EventAccessors, EventHandler, EventModifiers, EventTargetExt,
        Modifiers,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/usePress.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - React-aria's `usePress` does not handle double-click. Double-click behavior
//   lives in `useSelectableItem` (where double-click triggers an action). We add
//   `on_double_press` here as a convenience so that any pressable element can opt
//   into double-press handling without requiring a full selection model.
//
// - React-aria has a separate `useLongPress` hook that wraps `usePress`. We merged
//   long press detection directly into `usePress` to avoid double-hook overhead
//   when both press and long press are needed on the same element (e.g. menu triggers).
//
// - React-aria sets `touch-action: manipulation` via a global `<style>` element
//   injected at runtime targeting `[data-pressable]` attributes. We use an inline
//   `style="touch-action: pan-x pan-y pinch-zoom"` instead. This avoids
//   programmatic DOM manipulation, is SSR-safe (inline styles serialize naturally
//   without hydration concerns), and eliminates the need for a `data_pressable`
//   field threaded through every press-based hook.
//
// ## INTENTIONAL OMISSIONS
//
// - React-aria's `onClick` compatibility alias is intentionally not provided.
//   Leptonic uses `on_press` as the primary interaction callback. The `onClick`
//   alias exists in react-aria for third-party library compatibility which is
//   not relevant in the Rust/Leptos ecosystem.
//
// ## FUTURE WORK
//
// - `PressResponderContext` is not yet implemented. This would allow parent
//   components to intercept and augment press behavior. Tracked in
//   `leptonic/src/atoms/press.rs`.
//
// =============================================================================

/// The default long press threshold.
pub const DEFAULT_LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(500);

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
    pub target: SendWrapper<EventTarget>,

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
    pub target: SendWrapper<EventTarget>,

    /// States which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer relative to the target element.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer relative to the target element.
    /// `None` for keyboard events.
    pub y: Option<f64>,

    /// The keyboard key that triggered this press event.
    /// `None` for pointer/mouse/virtual events.
    pub key: Option<String>,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continue_propagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Arc<dyn Fn() + Send + Sync + 'static>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct UsePressInput {
    /// Whether the targeted element is currently disabled.
    pub disabled: Signal<bool>,

    /// Set this to true if you want controlled press behavior
    /// with the guarantee of no browser-specific behavior happening on user interactions.
    pub force_prevent_default: bool,

    /// Unconditionally allow event propagation, bypassing per-event `continue_propagation()`
    /// control in press callbacks. By default (`false`), propagation is stopped unless a callback
    /// explicitly calls `continue_propagation()`.
    ///
    /// **Deviation from react-aria**: react-aria has no equivalent prop — it only provides
    /// `continuePropagation()` on the event object. This field is a leptonic addition for cases
    /// where propagation must always be allowed regardless of callback behavior.
    pub force_propagation: bool,

    /// When `true`, text selection is not disabled during press interactions.
    /// By default (`false`), text selection is disabled to prevent accidental selection
    /// while pressing.
    pub allow_text_selection_on_press: bool,

    /// When `true`, the press is cancelled entirely when the pointer exits the target.
    /// By default (`false`), the user can press, drag outside, drag back in, and still
    /// complete the press.
    pub should_cancel_on_pointer_exit: bool,

    /// When `true`, prevents the browser from moving focus to the pressed element.
    /// Useful for toolbar buttons near text editors where focus should remain in the editor.
    pub prevent_focus_on_press: bool,

    /// When provided, the returned `is_pressed` signal will be `true` when either the
    /// internal pressed state or this signal is `true`, forcing the pressed visual state.
    /// This allows parent components (e.g., overlays) to force the pressed appearance.
    ///
    /// **Deviation from react-aria**: react-aria calls this `isPressed: boolean`. Renamed to
    /// `force_is_pressed` to avoid ambiguity with the returned `is_pressed` signal.
    pub force_is_pressed: Option<Signal<bool>>,

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
    pub long_press_threshold: Option<Signal<Duration>>,

    /// A description for assistive technology users indicating that a long press
    /// action is available, e.g. "Long press to open menu".
    /// Only applied when `on_long_press` is `Some`.
    pub long_press_accessibility_description: Option<Oco<'static, str>>,
}

#[derive(Debug)]
pub struct UsePressReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
}

/// Props from `use_press` that can be extracted and merged programmatically.
///
/// Use [`UsePressProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
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
#[derive(Debug)]
pub struct UsePressProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    /// Set when `on_long_press` and `long_press_accessibility_description` are provided.
    /// `None` otherwise.
    pub aria_describedby: Option<AriaDescribedby>,
}

impl IntoAttrs for UsePressProps {
    type Attrs = UsePressAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_dblclick.into_on(ev::dblclick),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            TouchActionStyle::with_value("pan-x pan-y pinch-zoom"),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UsePressAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    Attr<attr::AriaDescribedby, Option<AriaDescribedby>>,
    Style<(TouchActionStyle, &'static str)>,
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

    /// The element this press hook was bound to.
    current_target: EventTarget,

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
            if let Some(element) = self.current_target.to_element() {
                element.restore_text_selection();
            }
        }
    }

    fn is_pointer_over_target(&self, e: &PointerEvent) -> bool {
        is_over(e, self.current_target.as_element().expect("element"))
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

    fn current_target(&self) -> EventTarget {
        match self {
            EventRef::Pointer(e) => e.expect_current_target(),
            EventRef::Keyboard(e) => e.expect_current_target(),
            EventRef::Mouse(e) => e.expect_current_target(),
        }
    }

    /// Returns the keyboard key that triggered this event, if any.
    fn key(&self) -> Option<String> {
        match self {
            EventRef::Keyboard(e) => Some(e.key()),
            EventRef::Pointer(_) | EventRef::Mouse(_) => None,
        }
    }

    /// Returns coordinates relative to the target element's bounding rect.
    /// For pointer/mouse events, returns the event position relative to the element.
    /// For keyboard events, returns the element's center point (`width/2`, `height/2`),
    /// matching react-aria behavior. This is useful for consumers that position UI relative
    /// to the press point (e.g., ripple effects).
    /// Returns `None` if the target has no client bounding rect.
    fn coordinates(&self) -> (Option<f64>, Option<f64>) {
        match self {
            EventRef::Pointer(e) => {
                let (client_x, client_y) = (f64::from(e.client_x()), f64::from(e.client_y()));
                self.current_target()
                    .to_element()
                    .map(|el| el.get_bounding_client_rect())
                    .map_or((None, None), |rect| {
                        (Some(client_x - rect.left()), Some(client_y - rect.top()))
                    })
            }
            EventRef::Mouse(e) => {
                let (client_x, client_y) = (f64::from(e.client_x()), f64::from(e.client_y()));
                self.current_target()
                    .to_element()
                    .map(|el| el.get_bounding_client_rect())
                    .map_or((None, None), |rect| {
                        (Some(client_x - rect.left()), Some(client_y - rect.top()))
                    })
            }
            EventRef::Keyboard(_) => self
                .current_target()
                .to_element()
                .map(|el| el.get_bounding_client_rect())
                .map_or((None, None), |rect| {
                    (Some(rect.width() / 2.0), Some(rect.height() / 2.0))
                }),
        }
    }
}

fn fire_press_callback(
    callback: Callback<PressEvent>,
    state: &PressState,
    event: &EventRef<'_>,
    force_propagation: bool,
    is_triggering_event: StoredValue<bool, LocalStorage>,
) {
    let (continue_propagation_state, continue_propagation) = use_continue_propagation();
    let (x, y) = event.coordinates();
    let key = event.key();
    is_triggering_event.set_value(true);
    callback.run(PressEvent {
        pointer_type: state.pointer_type.clone(),
        target: SendWrapper::new(state.current_target.clone()),
        modifiers: event.modifiers(),
        x,
        y,
        key,
        continue_propagation,
    });
    is_triggering_event.set_value(false);
    if !force_propagation && !continue_propagation_state.load(Ordering::Acquire) {
        event.stop_propagation();
    }
}

/// # Panics
///
/// Panics if the press state is initialized while already active (debug assertion),
/// or if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines)]
pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let UsePressInput {
        disabled,
        force_prevent_default,
        force_propagation,
        allow_text_selection_on_press,
        should_cancel_on_pointer_exit,
        prevent_focus_on_press,
        force_is_pressed,
        on_press,
        on_press_up,
        on_press_start,
        on_press_end,
        on_press_change,
        on_double_press,
        on_long_press_start,
        on_long_press,
        on_long_press_end,
        long_press_threshold,
        long_press_accessibility_description,
    } = input;

    let (is_pressed, set_is_pressed) = signal(false);

    let supports_long_press =
        on_long_press.is_some() || on_long_press_start.is_some() || on_long_press_end.is_some();
    let long_press_threshold =
        long_press_threshold.unwrap_or_else(|| Signal::stored(DEFAULT_LONG_PRESS_THRESHOLD));

    let state: StoredValue<Option<PressState>, LocalStorage> = StoredValue::new_local(None);

    // Tracks the pointer type from the most recently completed press,
    // so the dblclick handler can construct a PressEvent after state has been cleared.
    let last_pointer_type: StoredValue<Option<PointerType>, LocalStorage> =
        StoredValue::new_local(None);

    // Re-entrancy guard: prevents infinite loops when a press callback
    // synchronously triggers a click event on the same element.
    let is_triggering_event: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

    // Tracks whether a virtual pointer event (e.g. VoiceOver on iOS) was seen in pointerdown,
    // so that the click handler can detect it and fire a full virtual press cycle.
    let saw_virtual_pointer_event: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

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
            current_target: e.current_target(),
            is_over_target: match e {
                EventRef::Pointer(e) => {
                    is_over(e, e.expect_current_target().as_element().expect("element"))
                }
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

        if let Some(on_press_start) = on_press_start {
            fire_press_callback(
                on_press_start,
                s,
                &e,
                force_propagation,
                is_triggering_event,
            );
        }

        if let Some(on_press_change) = on_press_change {
            is_triggering_event.set_value(true);
            on_press_change.run(true);
            is_triggering_event.set_value(false);
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

        if let Some(on_press_end) = on_press_end {
            fire_press_callback(on_press_end, s, &e, force_propagation, is_triggering_event);
        }

        // Fire long press end for mouse/touch when long press is configured.
        if supports_long_press
            && (s.pointer_type == PointerType::Mouse || s.pointer_type == PointerType::Touch)
        {
            if let Some(on_long_press_end) = on_long_press_end {
                let (x, y) = e.coordinates();
                is_triggering_event.set_value(true);
                on_long_press_end.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressEnd,
                    pointer_type: s.pointer_type.clone(),
                    target: SendWrapper::new(s.current_target.clone()),
                    modifiers: e.modifiers(),
                    x,
                    y,
                });
                is_triggering_event.set_value(false);
            }
        }

        if let Some(on_press_change) = on_press_change {
            is_triggering_event.set_value(true);
            on_press_change.run(false);
            is_triggering_event.set_value(false);
        }

        set_is_pressed.set(false);

        // Do NOT fire on_press if the long press threshold was met.
        // The short press was consumed by the long press interaction.
        if was_pressed && !s.long_press_triggered {
            fire_press_callback(on_press, s, &e, force_propagation, is_triggering_event);
        }
    };

    let trigger_press_up = move |s: &PressState, e: EventRef<'_>| {
        if let Some(on_press_up) = on_press_up {
            fire_press_callback(on_press_up, s, &e, force_propagation, is_triggering_event);
        }
    };

    let cancel_active_press = move |e: EventRef<'_>| {
        state.update_value(|s| {
            if let Some(s) = s {
                s.clear_click_timeout();
                s.clear_long_press_timeout();
                trigger_press_end(s, e, false);
                s.restore_text_selection_if_needed(allow_text_selection_on_press);
                s.cleanup_event_handlers();
            }
        });
        state.set_value(None);
    };

    let handle_key_up = move |e: KeyboardEvent| {
        // First check if we should handle this event (immutable check).
        // Use the stored press target, not e.current_target(), because the keyup listener
        // is registered on the document — e.current_target() would be the Document, not the
        // pressed element, causing is_valid_keyboard_event to bypass validation.
        let should_handle = state.with_value(|s| {
            s.as_ref().is_some_and(|s| {
                !disabled.get_untracked() && is_valid_keyboard_event(&e, s.current_target.clone())
            })
        });
        if !should_handle {
            return;
        }

        let key = e.key();
        if e.expect_target()
            .to_element()
            .is_some_and(|el| should_prevent_default_keyboard(&el, key.as_str()))
        {
            e.prevent_default();
        }

        // If a link was triggered with a key other than Enter, open the URL ourselves.
        // This means the link has a role override, and the default browser behavior
        // only applies when using the Enter key.
        if key != "Enter" {
            if let Some(current_target) =
                state.with_value(|s| s.as_ref().map(|s| s.current_target.clone()))
            {
                if let Some(true) = node_contains(
                    current_target.as_node().as_ref(),
                    e.expect_target()
                        .to_element()
                        .and_then(|el| el.as_node())
                        .as_ref(),
                ) {
                    if let Some(el) = current_target.as_element() {
                        if el.is_anchor_link() {
                            open_link(el, e.modifiers(), true);
                        }
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

        // Check if the keyup target is still inside the original press target.
        // If focus moved away during the keypress, we should not fire on_press.
        let was_pressed = state.with_value(|s| {
            s.as_ref().is_some_and(|s| {
                node_contains(
                    s.current_target.as_node().as_ref(),
                    e.expect_target().as_node().as_ref(),
                )
                .unwrap_or(false)
            })
        });

        // Now perform mutable operations: fire press_up and press_end.
        state.update_value(move |s| {
            if let Some(s) = s.as_mut() {
                trigger_press_up(s, EventRef::Keyboard(&e));
                trigger_press_end(s, EventRef::Keyboard(&e), was_pressed);
                s.cleanup_event_handlers();
            }
        });
        state.set_value(None);
    };

    let handle_key_down = move |e: KeyboardEvent| {
        if !node_contains(
            e.expect_current_target().as_node().as_ref(),
            e.expect_target().as_node().as_ref(),
        )
        .unwrap_or(true)
        {
            tracing::debug!("Aborting handle_key_down, as current_target did not contain target.");
            return;
        }

        let key = e.key();

        if is_valid_keyboard_event(&e, e.expect_current_target()) {
            if e.expect_target()
                .as_element()
                .is_some_and(|el| should_prevent_default_keyboard(el, &key))
            {
                e.prevent_default();
            }

            // Read meta_key before the closure moves `e`.
            let is_meta_held = device::is_mac() && e.meta_key() && key != "Meta";
            let e_for_meta = if is_meta_held { Some(e.clone()) } else { None };

            // Only initialize press on the first keydown, not on repeats.
            if state.with_value(Option::is_none) && !disabled.get_untracked() && !e.repeat() {
                initialize_press_state(
                    EventRef::Keyboard(&e),
                    EventHandlers::KeyboardEvents {
                        global_on_key_up_cleanup: Box::new(use_event_listener(
                            e.expect_current_target().get_owner_document(),
                            ev::keyup,
                            handle_key_up,
                        )),
                    },
                );

                state.update_value(move |s| {
                    if let Some(s) = s {
                        trigger_press_start(s, EventRef::Keyboard(&e));
                    }
                });
            }

            // macOS Meta key workaround: store events pressed while Meta is held
            // because macOS doesn't fire keyup for non-Meta keys while Meta is down.
            // This must be OUTSIDE the state.is_none() check so it captures keys
            // pressed while Meta is held even during an active press.
            if let Some(e) = e_for_meta {
                meta_key_events.update_value(|map| {
                    let map = map.get_or_insert_with(std::collections::HashMap::new);
                    map.insert(key.clone(), e.clone());
                });
            }
        } else if key == "Meta" {
            // Initialize the map when Meta key itself is pressed.
            meta_key_events.set_value(Some(std::collections::HashMap::new()));
        }
    };

    let handle_click = move |e: MouseEvent| {
        // Re-entrancy guard: if we are currently inside a press callback that
        // synchronously triggered a click, skip this handler to prevent infinite loops.
        if is_triggering_event.get_value() {
            return;
        }

        if !node_contains(
            e.expect_current_target().as_node().as_ref(),
            e.expect_target().as_node().as_ref(),
        )
        .unwrap_or(true)
        {
            tracing::debug!("Aborting handle_click, as current_target did not contain target.");
            return;
        }

        if disabled.get_untracked() {
            e.prevent_default();
            return;
        }

        if force_prevent_default {
            e.prevent_default();
        }
        if !force_propagation {
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
                    last_pointer_type.set_value(Some(s.pointer_type.clone()));
                    s.clear_click_timeout();
                    trigger_press_up(s, EventRef::Mouse(&e));
                    trigger_press_end(s, EventRef::Mouse(&e), true);
                    s.restore_text_selection_if_needed(allow_text_selection_on_press);
                    s.cleanup_event_handlers();
                }
            });
            state.set_value(None);
            return;
        }

        // Handle virtual click (screen reader / assistive technology).
        // Also handle deferred virtual pointer events from VoiceOver on iOS.
        if !is_pressed.get_untracked()
            && (saw_virtual_pointer_event.get_value() || is_virtual_click(&e))
        {
            saw_virtual_pointer_event.set_value(false);
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
    let handle_pointer_move = move |e: PointerEvent| {
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                if e.pointer_id() != s.pointer_id {
                    return;
                }
                let is_over_target = s.is_pointer_over_target(&e);

                if should_cancel_on_pointer_exit && s.is_over_target && !is_over_target {
                    // Cancel the entire press when configured to do so.
                    trigger_press_end(s, EventRef::Pointer(&e), false);
                    s.cleanup_event_handlers();
                    s.restore_text_selection_if_needed(allow_text_selection_on_press);
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
        if should_cancel_on_pointer_exit {
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
    let handle_pointer_up = move |e: PointerEvent| {
        // Only handle primary button releases.
        if e.button() != 0 {
            return;
        }

        // Only handle the pointer that started the press.
        let should_handle =
            state.with_value(|s| s.as_ref().is_some_and(|s| e.pointer_id() == s.pointer_id));
        if !should_handle {
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

                if force_prevent_default {
                    e.prevent_default();
                }
                if !force_propagation {
                    e.stop_propagation();
                }

                // Set up 80ms fallback to programmatically click the target.
                let current_target = s.current_target.clone();
                s.click_timeout_handle = set_timeout_with_handle(
                    move || {
                        // Focus the element without scrolling before clicking,
                        // matching react-aria's focusWithoutScrolling behavior.
                        if let Some(el) = current_target.as_element() {
                            focus_element(el, true);
                        }
                        if let Some(html_el) = current_target.as_html_element() {
                            html_el.click();
                        }
                    },
                    Duration::from_millis(80),
                )
                .ok();
            }
        });
    };

    // Cancel the ongoing press.
    let handle_pointer_cancel = move |e: PointerEvent| {
        cancel_active_press(EventRef::Pointer(&e));
    };

    // Start a press.
    let handle_pointer_down = move |e: PointerEvent| {
        if e.button() != 0 {
            return;
        }

        if !e.current_target_contains_target() {
            tracing::trace!(
                "Aborting handle_pointer_down, as current_target did not contain target."
            );
            return;
        }

        // Handle virtual pointer events (e.g., VoiceOver on iOS).
        // These are deferred to the onClick handler.
        if is_virtual_pointer_event(&e) {
            // Store that we saw a virtual event; onClick will handle the full press cycle.
            saw_virtual_pointer_event.set_value(true);
            return;
        }

        let target = e.expect_target();

        if !allow_text_selection_on_press {
            if let Some(target) = target.as_element() {
                target.disable_text_selection();
            }
        }

        if !disabled.get_untracked() {
            // Release pointer capture to enable pointerleave/pointerenter on touch.
            // By default, the browser captures pointer events to the original target,
            // which prevents these events from firing correctly.
            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                if element.has_pointer_capture(e.pointer_id()) {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }
            }

            let doc = e.expect_current_target().get_owner_document();
            initialize_press_state(
                EventRef::Pointer(&e),
                EventHandlers::PointerEvents {
                    global_on_pointer_move_cleanup: Box::new(use_event_listener(
                        doc.clone(),
                        ev::pointermove,
                        handle_pointer_move,
                    )),
                    global_on_pointer_up_cleanup: Box::new(use_event_listener(
                        doc.clone(),
                        ev::pointerup,
                        handle_pointer_up,
                    )),
                    global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                        doc,
                        ev::pointercancel,
                        handle_pointer_cancel,
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
                        if let Some(on_long_press_start) = on_long_press_start {
                            let (x, y) = EventRef::Pointer(&e).coordinates();
                            is_triggering_event.set_value(true);
                            on_long_press_start.run(LongPressEvent {
                                event_type: LongPressEventType::LongPressStart,
                                pointer_type: s.pointer_type.clone(),
                                target: SendWrapper::new(s.current_target.clone()),
                                modifiers: EventRef::Pointer(&e).modifiers(),
                                x,
                                y,
                            });
                            is_triggering_event.set_value(false);
                        }

                        // Capture values for the timeout closure
                        let pointer_type = s.pointer_type.clone();
                        let modifiers = EventRef::Pointer(&e).modifiers();
                        let current_target = s.current_target.clone();
                        let (x, y) = EventRef::Pointer(&e).coordinates();

                        s.long_press_timeout_handle = set_timeout_with_handle(
                            move || {
                                // Dispatch pointercancel on the target to cancel the press
                                // interaction. This is synchronous — the pointercancel handler
                                // (and thus trigger_press_end / on_long_press_end) will fire
                                // before the code after dispatch_event.
                                if let Some(el) = current_target.as_element() {
                                    let cancel_event = PointerEvent::new("pointercancel")
                                        .expect("should create pointercancel event");
                                    let _ = el.dispatch_event(&cancel_event);

                                    // Focus the element without scrolling.
                                    focus_element(el, true);
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
                                        target: SendWrapper::new(current_target),
                                        modifiers,
                                        x,
                                        y,
                                    });
                                }
                            },
                            long_press_threshold.get(),
                        )
                        .ok();

                        // For touch, prevent the context menu on the event target.
                        if s.pointer_type == PointerType::Touch {
                            s.current_target.prevent_default_once("contextmenu");
                        }
                    }
                }
            });
        }
    };

    // Safari doesn't fire pointercancel on drag. Handle dragstart to cancel the press.
    let handle_dragstart = move |_e: DragEvent| {
        cancel_active_press(EventRef::Pointer(
            &PointerEvent::new("pointercancel").expect("should create pointercancel event"),
        ));
    };

    // Handle native dblclick for on_double_press.
    // By the time dblclick fires, the press state has already been cleared by the second click
    // handler. We use `last_pointer_type` (saved before clearing state) to construct the event.
    let handle_dblclick = move |e: MouseEvent| {
        let Some(on_double_press) = on_double_press else {
            return;
        };
        if disabled.get_untracked() {
            return;
        }

        let Some(pointer_type) = last_pointer_type.get_value() else {
            tracing::warn!("no pointer type saved for dblclick");
            return;
        };

        let e = EventRef::Mouse(&e);
        let (continue_propagation_state, continue_propagation) = use_continue_propagation();
        let (x, y) = e.coordinates();
        let key = e.key();
        is_triggering_event.set_value(true);
        on_double_press.run(PressEvent {
            pointer_type,
            target: SendWrapper::new(e.current_target()),
            modifiers: e.modifiers(),
            x,
            y,
            key,
            continue_propagation,
        });
        is_triggering_event.set_value(false);
        if !force_propagation && !continue_propagation_state.load(Ordering::Acquire) {
            e.stop_propagation();
        }
    };

    // Prevent focus on mousedown when prevent_focus_on_press is enabled.
    let handle_mousedown = move |e: MouseEvent| {
        if prevent_focus_on_press {
            e.prevent_default();
        }
        if !force_propagation && e.button() == 0 {
            e.stop_propagation();
        }
    };

    // Element-level pointerup handler: fires on_press_up for pointer-up events
    // over the element that didn't have a corresponding press-down (no active press state).
    let handle_element_pointer_up = move |e: PointerEvent| {
        let ev = e;
        let e = EventRef::Pointer(&ev);

        if !ev.current_target_contains_target() || saw_virtual_pointer_event.get_value() {
            return;
        }
        if ev.button() != 0 {
            return;
        }
        if disabled.get_untracked() {
            return;
        }
        // Only fire when there is no active press (the global pointerup handles active presses).
        if state.with_value(Option::is_some) {
            return;
        }
        if let Some(on_press_up) = on_press_up {
            let (x, y) = e.coordinates();
            let (continue_propagation_state, continue_propagation) = use_continue_propagation();
            is_triggering_event.set_value(true);
            on_press_up.run(PressEvent {
                pointer_type: PointerType::from(ev.pointer_type()),
                target: SendWrapper::new(e.current_target()),
                modifiers: e.modifiers(),
                x,
                y,
                key: None,
                continue_propagation,
            });
            is_triggering_event.set_value(false);
            if !force_propagation && !continue_propagation_state.load(Ordering::Acquire) {
                e.stop_propagation();
            }
        }
    };

    // Only set aria-describedby when on_long_press is provided and a description is given.
    // Creates a hidden <span> element and references it by ID, per WAI-ARIA spec.
    let aria_describedby = if on_long_press.is_some() {
        long_press_accessibility_description.map(use_description)
    } else {
        None
    };

    // Cleanup on unmount: restore text selection, remove global listeners, and clear timeouts.
    on_cleanup(move || {
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                s.restore_text_selection_if_needed(allow_text_selection_on_press);
                s.cleanup_event_handlers();
                s.clear_click_timeout();
                s.clear_long_press_timeout();
            }
        });
        state.set_value(None);
    });

    UsePressReturn {
        props: UsePressProps {
            on_keydown: EventHandler::new(handle_key_down),
            on_click: EventHandler::new(handle_click),
            on_pointerdown: EventHandler::new(handle_pointer_down),
            on_pointerup: EventHandler::new(handle_element_pointer_up),
            on_mousedown: EventHandler::new(handle_mousedown),
            on_dragstart: EventHandler::new(handle_dragstart),
            on_dblclick: EventHandler::new(handle_dblclick),
            aria_describedby,
        },
        is_pressed: match force_is_pressed {
            Some(prop) => Signal::derive(move || is_pressed.get() || prop.get()),
            None => is_pressed.into(),
        },
    }
}

/// Tests whether a keyboard event's default action should be presented when the given `key` was pressed.
fn should_prevent_default_keyboard(element: &web_sys::Element, key: &str) -> bool {
    if element.is_instance_of::<HtmlInputElement>() {
        return !is_valid_input_key(element.unchecked_ref::<HtmlInputElement>(), key);
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
fn is_valid_input_key(element: &HtmlInputElement, key: &str) -> bool {
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
fn is_valid_keyboard_event(e: &KeyboardEvent, current_target: EventTarget) -> bool {
    let key = e.key();
    let code = e.code();
    let resembles_press =
        matches!(key.as_str(), "Enter" | " " | "Spacebar" | "Space") || code == "Space";

    if !resembles_press {
        return false;
    }

    match current_target.as_element() {
        Some(element) => {
            let is_input = element.is_instance_of::<HtmlInputElement>();
            let is_text_area = element.is_instance_of::<HtmlTextAreaElement>();
            let is_content_editable = element
                .dyn_ref::<HtmlElement>()
                .is_some_and(HtmlElement::is_content_editable);
            // Role-aware link detection: respect role overrides on anchors.
            // An `<a href role="button">` should be treated as a button, not a link.
            // React-aria checks: role === 'link' || (!role && isHTMLAnchorLink(element)).
            // We do NOT use element.is_link() here because it ignores role overrides.
            let role = element.get_attribute("role");
            let is_link =
                role.as_deref() == Some("link") || (role.is_none() && element.is_anchor_link());

            // Links should only trigger with Enter key
            !(is_text_area
                || is_content_editable
                || is_input
                    && !is_valid_input_key(
                        element.unchecked_ref::<HtmlInputElement>(),
                        key.as_str(),
                    )
                || is_link && key.as_str() != "Enter")
        }
        None => true,
    }
}
