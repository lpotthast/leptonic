//! Virtual drag-and-drop manager for keyboard and screen-reader support.
//!
//! This module provides keyboard-based drag-and-drop that works alongside
//! native drag events. When a user activates a drag via keyboard (Enter key
//! on a draggable element), a virtual drag session is started that allows:
//!
//! - **Tab / Shift+Tab** to navigate between valid drop targets
//! - **Enter** to drop on the currently focused target
//! - **Escape** to cancel the drag
//!
//! The manager also:
//! - Uses [`aria_hide_outside`] to hide non-target elements from assistive tech
//! - Blocks all pointer/mouse/touch/focus events during the session
//! - Announces drag state changes via the [`LiveAnnouncer`]
//! - Manages focus during the drag session, including guard focus restoration
//! - Supports virtual click (screen reader) cancel/drop
//! - Cycles through source element when Tab-navigating past the last target
//!
//! Based on react-aria's `DragManager` from
//! `@react-aria/dnd/src/DragManager.ts`.

#[cfg(not(feature = "ssr"))]
use leptos::prelude::Callable;

use crate::hooks::dnd::{
    AllowedDropOperations, DragEndEvent, DragItem, DragTypes, DropActivateEvent, DropEffect,
    DropEnterEvent, DropEvent, DropExitEvent, DropOperationEvent,
};
#[cfg(not(feature = "ssr"))]
use crate::utils::live_announcer;

use super::types::DropTarget;

/// A drag source for a virtual (keyboard) drag session.
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct DragSource {
    /// The element being dragged.
    pub element: web_sys::Element,
    /// The items being dragged.
    pub items: Vec<DragItem>,
    /// The allowed drop operations.
    pub allowed_operations: AllowedDropOperations,
    /// Callback fired when the drag session ends (drop or cancel).
    pub on_end: Box<dyn FnOnce(DragEndEvent)>,
}

/// A drop target registered with the drag manager.
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) struct RegisteredDropTarget {
    /// Unique ID for this target.
    pub id: String,
    /// The DOM element.
    pub element: web_sys::Element,
    /// Accepted MIME types (empty = accept all).
    pub accepted_types: Vec<String>,
    /// Callback to determine the drop operation.
    pub get_drop_operation: Option<leptos::prelude::Callback<DropOperationEvent, DropEffect>>,
    /// Callback when drag enters.
    pub on_drop_enter: Option<leptos::prelude::Callback<DropEnterEvent>>,
    /// Callback when drag exits.
    pub on_drop_exit: Option<leptos::prelude::Callback<DropExitEvent>>,
    /// Callback when drop occurs.
    pub on_drop: Option<leptos::prelude::Callback<DropEvent>>,
    /// Callback when drag hovers for 800ms.
    pub on_drop_activate: Option<leptos::prelude::Callback<DropActivateEvent>>,
    /// Optional keyboard handler for collection-level navigation.
    /// When present, arrow/Home/End/PageUp/PageDown keys are forwarded here
    /// instead of being treated as Tab navigation.
    pub on_key_down: Option<leptos::prelude::Callback<KeyDownForwardEvent, bool>>,
    /// When true, the drag manager won't focus the source element after drop.
    /// Used by collections that manage their own post-drop focus.
    pub prevent_focus_on_drop: bool,
    /// Optional "activate" button element within this target.
    /// When present, Enter on this button triggers `on_drop_activate` instead of drop.
    pub activate_button: Option<web_sys::Element>,
}

/// Event forwarded to a drop target's `on_key_down` handler.
#[derive(Debug, Clone)]
#[allow(dead_code, clippy::struct_excessive_bools)]
pub struct KeyDownForwardEvent {
    /// The key that was pressed.
    pub key: String,
    /// Whether Shift was held.
    pub shift_key: bool,
    /// Whether Alt was held.
    pub alt_key: bool,
    /// Whether Ctrl was held.
    pub ctrl_key: bool,
    /// Whether Meta was held.
    pub meta_key: bool,
}

/// A drop item registered with the drag manager for per-item ARIA hiding
/// during keyboard drag sessions.
#[allow(dead_code)]
pub(crate) struct RegisteredDropItem {
    /// The DOM element for this item.
    pub element: web_sys::Element,
    /// The drop target this item represents.
    pub target: DropTarget,
    /// Callback to determine the drop operation for this item.
    pub get_drop_operation:
        Option<leptos::prelude::Callback<(DragTypes, AllowedDropOperations), DropEffect>>,
    /// Optional "activate" button element within this item.
    pub activate_button: Option<web_sys::Element>,
}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn register_drop_target(_target: RegisteredDropTarget) {}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn unregister_drop_target(_id: &str) {}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn begin_dragging(_source: DragSource) {
    // No-op on server.
}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn register_drop_item(_item: RegisteredDropItem) {}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn unregister_drop_item(_element: &web_sys::Element) {}

/// Returns a reactive signal indicating whether a virtual (keyboard)
/// drag session is currently active.
#[cfg(feature = "ssr")]
pub fn use_drag_session_active() -> leptos::prelude::Signal<bool> {
    leptos::prelude::Signal::derive(|| false)
}

/// Returns the items being dragged in the current session, if any.
#[cfg(feature = "ssr")]
pub fn get_session_drag_items() -> Option<Vec<DragItem>> {
    None
}

/// Returns the allowed drop operations for the current session, if any.
#[cfg(feature = "ssr")]
pub fn get_session_allowed_operations() -> Option<AllowedDropOperations> {
    None
}

#[cfg(not(feature = "ssr"))]
use std::cell::{Cell, RefCell};
#[cfg(not(feature = "ssr"))]
use std::collections::HashMap;
#[cfg(not(feature = "ssr"))]
use std::rc::Rc;

#[cfg(not(feature = "ssr"))]
use leptos::prelude::{Notify, Track, Trigger};
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::*;

/// Events blocked during a virtual drag session. Matches react-aria's
/// `CANCELED_EVENTS` list. Registered in capture phase on `document`.
#[cfg(not(feature = "ssr"))]
const CANCELED_EVENTS: &[&str] = &[
    "pointerdown",
    "pointermove",
    "pointerenter",
    "pointerleave",
    "pointerover",
    "pointerout",
    "pointerup",
    "mousedown",
    "mousemove",
    "mouseenter",
    "mouseleave",
    "mouseover",
    "mouseout",
    "mouseup",
    "touchstart",
    "touchmove",
    "touchend",
    "focusin",
    "focusout",
];

/// Events whose `preventDefault` must NOT be called because doing so
/// would break click synthesis in some browsers.
#[cfg(not(feature = "ssr"))]
const CLICK_EVENTS: &[&str] = &["pointerup", "mouseup", "touchend"];

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// All registered drop targets (collection-level).
    static DROP_TARGETS: RefCell<HashMap<String, RegisteredDropTarget>> =
        RefCell::new(HashMap::new());

    /// All registered drop items (individual items within collections).
    static DROP_ITEMS: RefCell<Vec<RegisteredDropItem>> =
        const { RefCell::new(Vec::new()) };

    /// The currently active virtual drag session, if any.
    static ACTIVE_SESSION: RefCell<Option<DragSession>> = const { RefCell::new(None) };

    /// Trigger that notifies subscribers when a session starts or ends.
    static SESSION_TRIGGER: Cell<Option<Trigger>> = const { Cell::new(None) };
}

#[cfg(not(feature = "ssr"))]
struct SessionState {
    source_element: web_sys::Element,
    items: Vec<DragItem>,
    allowed_operations: AllowedDropOperations,
    valid_target_ids: Vec<String>,
    current_target_index: Option<usize>,
    /// Index into the `DROP_ITEMS` vec for the currently focused drop item.
    current_drop_item: Option<usize>,
    /// Whether the first item has been announced (for initial-focus announcement).
    initial_focused: bool,
    on_end: Option<Box<dyn FnOnce(DragEndEvent)>>,
    /// Set by `pointerdown` handler to detect Android `TalkBack` virtual clicks.
    is_virtual_click: bool,
}

#[cfg(not(feature = "ssr"))]
struct DragSession {
    state: Rc<RefCell<SessionState>>,
    /// Keyboard event listener closure (capture phase on `document`).
    keyboard_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
    /// Keyup event listener closure (capture phase on `document`).
    keyup_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
    /// Focus guard listener closure (capture phase on `window`).
    focus_closure: Closure<dyn FnMut(web_sys::FocusEvent)>,
    /// Blur guard listener closure (capture phase on `window`).
    blur_closure: Closure<dyn FnMut(web_sys::FocusEvent)>,
    /// Click handler for virtual click cancel/drop (capture phase on `document`).
    click_closure: Closure<dyn FnMut(web_sys::MouseEvent)>,
    /// Pointerdown handler for virtual pointer detection (capture phase on `document`).
    pointerdown_closure: Closure<dyn FnMut(web_sys::PointerEvent)>,
    /// Generic event canceller registered for all [`CANCELED_EVENTS`].
    cancel_event_closure: Closure<dyn FnMut(web_sys::Event)>,
    /// ARIA hide cleanup function. Uses `RefCell` so `update_valid_drop_targets`
    /// can swap it while the session is borrowed.
    aria_hide_cleanup: RefCell<Option<Box<dyn FnOnce()>>>,
    /// `MutationObserver` watching for `aria-hidden`/`inert` attribute changes.
    mutation_observer: Option<web_sys::MutationObserver>,
    /// Closure backing the `MutationObserver` callback (must stay alive).
    #[allow(clippy::type_complexity)]
    mutation_callback: Option<Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>>,
    /// Pending `requestAnimationFrame` handle and closure for deferred first navigation.
    pending_raf: RefCell<Option<(i32, Closure<dyn FnMut()>)>>,
}

/// Registers a drop target with the drag manager so it can participate in
/// virtual (keyboard) drag sessions.
///
/// Call [`unregister_drop_target`] when the target is removed.
#[cfg(not(feature = "ssr"))]
pub fn register_drop_target(target: RegisteredDropTarget) {
    DROP_TARGETS.with(|targets| {
        targets.borrow_mut().insert(target.id.clone(), target);
    });
    // Re-validate targets if a session is active.
    if is_virtual_dragging() {
        update_valid_drop_targets();
    }
}

/// Removes a previously registered drop target.
#[cfg(not(feature = "ssr"))]
pub fn unregister_drop_target(id: &str) {
    DROP_TARGETS.with(|targets| {
        targets.borrow_mut().remove(id);
    });
    // Re-validate targets if a session is active.
    if is_virtual_dragging() {
        update_valid_drop_targets();
    }
}

/// Registers an individual drop item with the drag manager.
///
/// Items registered here are included in ARIA hiding during keyboard drag
/// sessions — valid items remain visible while invalid ones are hidden.
#[cfg(not(feature = "ssr"))]
pub fn register_drop_item(item: RegisteredDropItem) {
    DROP_ITEMS.with(|items| {
        let mut items = items.borrow_mut();
        // Remove any existing registration for the same element.
        items.retain(|i| i.element != item.element);
        items.push(item);
    });
}

/// Removes a previously registered drop item.
#[cfg(not(feature = "ssr"))]
pub fn unregister_drop_item(element: &web_sys::Element) {
    DROP_ITEMS.with(|items| {
        items.borrow_mut().retain(|i| &i.element != element);
    });
}

/// Returns (or lazily creates) the session-change `Trigger`.
#[cfg(not(feature = "ssr"))]
fn get_or_init_session_trigger() -> Trigger {
    SESSION_TRIGGER.with(|cell| {
        if let Some(t) = cell.get() {
            t
        } else {
            let t = Trigger::new();
            cell.set(Some(t));
            t
        }
    })
}

/// Notifies all subscribers that the session state changed.
#[cfg(not(feature = "ssr"))]
fn notify_session_change() {
    SESSION_TRIGGER.with(|cell| {
        if let Some(t) = cell.get() {
            t.notify();
        }
    });
}

/// Returns a reactive signal that is `true` while a virtual (keyboard)
/// drag session is active. Components can use this to show/hide UI such
/// as drop indicators.
#[cfg(not(feature = "ssr"))]
pub fn use_drag_session_active() -> leptos::prelude::Signal<bool> {
    let trigger = get_or_init_session_trigger();
    leptos::prelude::Signal::derive(move || {
        trigger.track();
        is_virtual_dragging()
    })
}

/// Returns the items being dragged in the current session, if any.
#[cfg(not(feature = "ssr"))]
pub fn get_session_drag_items() -> Option<Vec<DragItem>> {
    ACTIVE_SESSION.with(|session| {
        session
            .borrow()
            .as_ref()
            .map(|s| s.state.borrow().items.clone())
    })
}

/// Returns the allowed drop operations for the current session, if any.
#[cfg(not(feature = "ssr"))]
pub fn get_session_allowed_operations() -> Option<AllowedDropOperations> {
    ACTIVE_SESSION.with(|session| {
        session
            .borrow()
            .as_ref()
            .map(|s| s.state.borrow().allowed_operations)
    })
}

/// Begins a virtual drag session. If a session is already active, it is
/// cancelled first.
#[cfg(not(feature = "ssr"))]
#[allow(clippy::too_many_lines)]
pub fn begin_dragging(source: DragSource) {
    // Cancel any existing session.
    end_session(DropEffect::None);

    let item_types: Vec<String> = source
        .items
        .iter()
        .flat_map(DragItem::types)
        .map(String::from)
        .collect();

    // Find valid drop targets and sort by proximity to the source.
    let mut valid_target_ids = find_valid_targets(&item_types, source.allowed_operations);
    sort_targets_by_proximity(&source.element, &mut valid_target_ids);

    if valid_target_ids.is_empty() {
        let num_registered = DROP_TARGETS.with(|t| t.borrow().len());
        tracing::warn!(
            "DragManager: No valid drop targets found. \
             {} target(s) registered, item_types={item_types:?}, \
             allowed_operations={:?}",
            num_registered,
            source.allowed_operations
        );
        // Log each registered target's accepted types for debugging.
        DROP_TARGETS.with(|targets| {
            for (id, target) in targets.borrow().iter() {
                tracing::warn!(
                    "  target id={id}, accepted_types={:?}, has_get_drop_operation={}",
                    target.accepted_types,
                    target.get_drop_operation.is_some()
                );
            }
        });
        live_announcer::announce_assertive("No drop targets available. Drag cancelled.");
        let (x, y) = get_element_center(&source.element);
        (source.on_end)(DragEndEvent {
            x,
            y,
            drop_effect: DropEffect::None,
        });
        return;
    }

    live_announcer::announce_assertive(format!(
        "Started dragging. {} drop target{} available. \
         Press Tab to navigate between targets, Enter to drop, or Escape to cancel.",
        valid_target_ids.len(),
        if valid_target_ids.len() == 1 { "" } else { "s" }
    ));

    let state = Rc::new(RefCell::new(SessionState {
        source_element: source.element.clone(),
        items: source.items,
        allowed_operations: source.allowed_operations,
        valid_target_ids: valid_target_ids.clone(),
        current_target_index: None,
        current_drop_item: None,
        initial_focused: false,
        on_end: Some(source.on_end),
        is_virtual_click: false,
    }));

    // --- Set up event listeners ---
    let window = web_sys::window().expect("window should be available");
    let document = window.document().expect("document should be available");

    // 1. Generic cancel-event closure for all CANCELED_EVENTS.
    let source_el_for_cancel = source.element.clone();
    let state_for_cancel = Rc::clone(&state);
    let cancel_event_closure: Closure<dyn FnMut(web_sys::Event)> =
        Closure::new(move |e: web_sys::Event| {
            cancel_event(&e, &source_el_for_cancel, &state_for_cancel);
        });
    for event_name in CANCELED_EVENTS {
        let _ = document.add_event_listener_with_callback_and_bool(
            event_name,
            cancel_event_closure.as_ref().unchecked_ref(),
            true,
        );
    }

    // 2. Keyboard listener (capture phase on document).
    let state_for_kb = Rc::clone(&state);
    let keyboard_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)> =
        Closure::new(move |e: web_sys::KeyboardEvent| {
            handle_keyboard_event(&state_for_kb, &e);
        });
    let _ = document.add_event_listener_with_callback_and_bool(
        "keydown",
        keyboard_closure.as_ref().unchecked_ref(),
        true,
    );

    // 2b. Keyup listener (capture phase on document).
    let state_for_keyup = Rc::clone(&state);
    let keyup_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)> =
        Closure::new(move |e: web_sys::KeyboardEvent| {
            handle_keyup_event(&state_for_keyup, &e);
        });
    let _ = document.add_event_listener_with_callback_and_bool(
        "keyup",
        keyup_closure.as_ref().unchecked_ref(),
        true,
    );

    // 3. Focus guard (capture phase on window).
    let state_for_focus = Rc::clone(&state);
    let focus_closure: Closure<dyn FnMut(web_sys::FocusEvent)> =
        Closure::new(move |e: web_sys::FocusEvent| {
            handle_focus_event(&state_for_focus, &e);
        });
    let _ = window.add_event_listener_with_callback_and_bool(
        "focus",
        focus_closure.as_ref().unchecked_ref(),
        true,
    );

    // 4. Blur guard (capture phase on window).
    let state_for_blur = Rc::clone(&state);
    let blur_closure: Closure<dyn FnMut(web_sys::FocusEvent)> =
        Closure::new(move |e: web_sys::FocusEvent| {
            handle_blur_event(&state_for_blur, &e);
        });
    let _ = window.add_event_listener_with_callback_and_bool(
        "blur",
        blur_closure.as_ref().unchecked_ref(),
        true,
    );

    // 5. Click handler for virtual click cancel/drop (capture phase on document).
    let state_for_click = Rc::clone(&state);
    let source_el_for_click = source.element.clone();
    let click_closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
        Closure::new(move |e: web_sys::MouseEvent| {
            handle_click_event(&state_for_click, &e, &source_el_for_click);
        });
    let _ = document.add_event_listener_with_callback_and_bool(
        "click",
        click_closure.as_ref().unchecked_ref(),
        true,
    );

    // 6. Pointerdown handler for virtual pointer detection (capture phase on document).
    let state_for_pd = Rc::clone(&state);
    let source_el_for_pd = source.element.clone();
    let pointerdown_closure: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::new(move |e: web_sys::PointerEvent| {
            handle_pointerdown_event(&state_for_pd, &e, &source_el_for_pd);
        });
    let _ = document.add_event_listener_with_callback_and_bool(
        "pointerdown",
        pointerdown_closure.as_ref().unchecked_ref(),
        true,
    );

    // Keep a clone of state for auto-navigation after storing.
    let state_for_navigate = Rc::clone(&state);

    // Store session.
    ACTIVE_SESSION.with(|session| {
        *session.borrow_mut() = Some(DragSession {
            state,
            keyboard_closure,
            keyup_closure,
            focus_closure,
            blur_closure,
            click_closure,
            pointerdown_closure,
            cancel_event_closure,
            aria_hide_cleanup: RefCell::new(None),
            mutation_observer: None,
            mutation_callback: None,
            pending_raf: RefCell::new(None),
        });
    });

    // Notify subscribers that a session has started.
    notify_session_change();

    // Set up ARIA hiding, MutationObserver, and validate targets.
    update_valid_drop_targets();
    setup_mutation_observer();

    // Step 5: Auto-navigate to first drop target via requestAnimationFrame.
    // Deferred so ARIA hiding has time to take effect before focus moves.
    let raf_closure: Closure<dyn FnMut()> = Closure::new(move || {
        if is_virtual_dragging() {
            navigate(&state_for_navigate, false);
        }
        // Clear pending_raf.
        ACTIVE_SESSION.with(|s| {
            if let Some(ref session) = *s.borrow() {
                session.pending_raf.borrow_mut().take();
            }
        });
    });
    let raf_handle = window
        .request_animation_frame(raf_closure.as_ref().unchecked_ref())
        .unwrap_or(0);
    // Store in the session's pending_raf.
    ACTIVE_SESSION.with(|s| {
        if let Some(ref session) = *s.borrow() {
            *session.pending_raf.borrow_mut() = Some((raf_handle, raf_closure));
        }
    });
}

/// Cancels an event during a drag session. Matches react-aria's `cancelEvent`.
///
/// - Allows `focusin`/`focusout` on the drag source and activate button so focus ring works.
/// - Skips `preventDefault` for click-related events (`pointerup`/`mouseup`/`touchend`)
///   because suppressing them can break click synthesis.
/// - Calls `preventDefault`, `stopPropagation`, and `stopImmediatePropagation` on everything else.
#[cfg(not(feature = "ssr"))]
fn cancel_event(
    e: &web_sys::Event,
    source_element: &web_sys::Element,
    state: &Rc<RefCell<SessionState>>,
) {
    let event_type = e.type_();

    // Allow focusin/focusout on the drag source and the current activate button.
    if event_type == "focusin" || event_type == "focusout" {
        if let Some(target) = e.target() {
            if let Some(target_el) = target.dyn_ref::<web_sys::Element>() {
                if target_el == source_element {
                    return;
                }
                // Also allow focus on the activate button.
                if let Some(ref activate_btn) = get_current_activate_button(state) {
                    if crate::utils::dom_ext::node_contains(
                        Some(activate_btn.as_ref()),
                        Some(target_el.as_ref()),
                    )
                    .unwrap_or(false)
                    {
                        return;
                    }
                }
            }
        }
    }

    // Don't prevent default for events that might cancel a click event.
    if !CLICK_EVENTS.contains(&event_type.as_str()) {
        e.prevent_default();
    }

    e.stop_propagation();
    e.stop_immediate_propagation();
}

#[cfg(not(feature = "ssr"))]
fn handle_keyboard_event(state: &Rc<RefCell<SessionState>>, e: &web_sys::KeyboardEvent) {
    // Step 4: Unconditional cancellation — ALL keyboard events during a drag
    // session are consumed, preventing arrows from scrolling, etc.
    e.prevent_default();
    e.stop_propagation();
    e.stop_immediate_propagation();

    match e.key().as_str() {
        "Escape" => {
            cancel_session(state);
            return;
        }
        // Only handle unmodified Tab (ignore with meta/alt/ctrl).
        "Tab" if !(e.meta_key() || e.alt_key() || e.ctrl_key()) => {
            navigate(state, e.shift_key());
        }
        _ => {}
    }

    // Forward ALL keys to the current target's on_key_down handler.
    // Enter is handled on keyup (see handle_keyup_event).
    forward_key_to_target(state, e);
}

/// Forwards a keyboard event to the current target's `on_key_down` handler.
#[cfg(not(feature = "ssr"))]
fn forward_key_to_target(state: &Rc<RefCell<SessionState>>, e: &web_sys::KeyboardEvent) {
    let current_target_id = {
        let s = state.borrow();
        s.current_target_index
            .map(|i| s.valid_target_ids[i].clone())
    };

    let Some(target_id) = current_target_id else {
        return;
    };

    let on_key_down =
        DROP_TARGETS.with(|targets| targets.borrow().get(&target_id).and_then(|t| t.on_key_down));

    let Some(on_key_down) = on_key_down else {
        return;
    };

    on_key_down.run(KeyDownForwardEvent {
        key: e.key(),
        shift_key: e.shift_key(),
        alt_key: e.alt_key(),
        ctrl_key: e.ctrl_key(),
        meta_key: e.meta_key(),
    });
}

/// Handles `keyup` events during a drag session (capture phase on `document`).
///
/// Enter on keyup triggers drop or activate, matching react-aria behavior.
/// Using keyup allows the activate button to receive the keydown first.
#[cfg(not(feature = "ssr"))]
fn handle_keyup_event(state: &Rc<RefCell<SessionState>>, e: &web_sys::KeyboardEvent) {
    e.prevent_default();
    e.stop_propagation();
    e.stop_immediate_propagation();

    if e.key() == "Enter" {
        let activate_btn = get_current_activate_button(state);
        let on_activate_btn = activate_btn.as_ref().is_some_and(|btn| {
            e.target()
                .and_then(|t| t.dyn_ref::<web_sys::Node>().cloned())
                .is_some_and(|node| {
                    crate::utils::dom_ext::node_contains(Some(btn.as_ref()), Some(&node))
                        .unwrap_or(false)
                })
        });

        if e.alt_key() || on_activate_btn {
            activate(state);
        } else {
            drop_on_current_target(state);
        }
    }
}

/// Handles `focus` events on `window` (capture phase) during a drag session.
///
/// Guards focus: if focus escapes to a non-target element, force-restores it
/// to the current drop target or source element.
#[cfg(not(feature = "ssr"))]
fn handle_focus_event(state: &Rc<RefCell<SessionState>>, e: &web_sys::FocusEvent) {
    let Some(event_target) = e.target() else {
        return;
    };

    let source_element = state.borrow().source_element.clone();

    // Prevent focus events except to the original drag target.
    if event_target.dyn_ref::<web_sys::Element>() != Some(&source_element) {
        e.stop_propagation();
        e.stop_immediate_propagation();
    }

    // Ignore non-element targets (e.g. window/document in JSDOM). Handled in blur.
    let Some(target_html) = event_target.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    let target_el: &web_sys::Element = target_html.as_ref();

    // Allow focus on the drag source element.
    if target_el == &source_element {
        return;
    }

    // Check if the focused element is (or is within) a valid drop target.
    let valid_target_ids = state.borrow().valid_target_ids.clone();
    let found = find_target_for_element(&valid_target_ids, target_el);

    if let Some((index, ref id)) = found {
        // Focus landed on a valid drop target — update session state.
        set_current_drop_target(state, Some((index, id)), false);
    } else {
        // Focus escaped — force it back.
        let current_idx = state.borrow().current_target_index;
        if let Some(idx) = current_idx {
            let target_id = state.borrow().valid_target_ids[idx].clone();
            focus_element_by_id(&target_id);
        } else if let Some(html_el) = source_element.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    }
}

/// Handles `blur` events on `window` (capture phase) during a drag session.
///
/// If nothing is gaining focus (or the related target is non-element),
/// force-restores focus to the current drop target or source.
#[cfg(not(feature = "ssr"))]
fn handle_blur_event(state: &Rc<RefCell<SessionState>>, e: &web_sys::FocusEvent) {
    let source_element = state.borrow().source_element.clone();

    // Prevent blur events except from the original drag target.
    if let Some(event_target) = e.target() {
        if event_target.dyn_ref::<web_sys::Element>() != Some(&source_element) {
            e.stop_propagation();
            e.stop_immediate_propagation();
        }
    }

    // If nothing is gaining focus (or related_target is not an HTMLElement),
    // restore focus to current target or source.
    let related = e.related_target();
    let has_html_related = related
        .as_ref()
        .and_then(|rt| rt.dyn_ref::<web_sys::HtmlElement>())
        .is_some();

    if !has_html_related {
        let current_idx = state.borrow().current_target_index;
        if let Some(idx) = current_idx {
            let target_id = state.borrow().valid_target_ids[idx].clone();
            focus_element_by_id(&target_id);
        } else {
            let source = state.borrow().source_element.clone();
            if let Some(html_el) = source.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    }
}

/// Handles `click` events during a drag session (capture phase on `document`).
///
/// - Virtual click on source element → cancel session.
/// - Virtual click on a valid drop target → drop on that target.
/// - All other clicks are silently consumed.
#[cfg(not(feature = "ssr"))]
fn handle_click_event(
    state: &Rc<RefCell<SessionState>>,
    e: &web_sys::MouseEvent,
    source_element: &web_sys::Element,
) {
    // Cancel the event regardless of virtual/real.
    cancel_event(e.as_ref(), source_element, state);

    let is_virtual =
        crate::utils::virtual_click::is_virtual_click(e) || state.borrow().is_virtual_click;

    if !is_virtual {
        return;
    }

    // Reset the flag.
    state.borrow_mut().is_virtual_click = false;

    let Some(event_target) = e.target() else {
        return;
    };

    // Virtual click on source element → cancel.
    if event_target.dyn_ref::<web_sys::Element>() == Some(source_element) {
        cancel_session(state);
        return;
    }

    // Virtual click on the activate button → activate, not drop.
    if let Some(target_node) = event_target.dyn_ref::<web_sys::Node>() {
        let activate_btn = get_current_activate_button(state);
        if activate_btn.is_some_and(|btn| {
            crate::utils::dom_ext::node_contains(Some(btn.as_ref()), Some(target_node))
                .unwrap_or(false)
        }) {
            activate(state);
            return;
        }
    }

    // Virtual click on a valid drop target → drop on that target.
    if let Some(target_el) = event_target.dyn_ref::<web_sys::Element>() {
        let valid_target_ids = state.borrow().valid_target_ids.clone();
        if let Some((index, ref id)) = find_target_for_element(&valid_target_ids, target_el) {
            set_current_drop_target(state, Some((index, id)), false);
            drop_on_current_target(state);
        }
    }
}

/// Handles `pointerdown` events during a drag session (capture phase on `document`).
///
/// Detects Android `TalkBack` virtual pointer events so the subsequent `click`
/// event can be correctly identified as virtual.
#[cfg(not(feature = "ssr"))]
fn handle_pointerdown_event(
    state: &Rc<RefCell<SessionState>>,
    e: &web_sys::PointerEvent,
    source_element: &web_sys::Element,
) {
    cancel_event(e.as_ref(), source_element, state);
    state.borrow_mut().is_virtual_click = crate::utils::virtual_click::is_virtual_pointer_event(e);
}

/// Navigates to the next or previous drop target. When reaching the end/start
/// of the target list and the source element is not hidden, cycles back to the
/// source element as an escape hatch (e.g. for iPad keyboards without Escape).
#[cfg(not(feature = "ssr"))]
fn navigate(state: &Rc<RefCell<SessionState>>, reverse: bool) {
    let (current_index, num_targets) = {
        let s = state.borrow();
        let num = s.valid_target_ids.len();
        if num == 0 {
            return;
        }
        (s.current_target_index, num)
    };

    let source_hidden = {
        let s = state.borrow();
        is_element_hidden(&s.source_element)
    };

    // Determine the new target. `None` means "focus the source element".
    let new_target: Option<(usize, String)> = if reverse {
        match current_index {
            None => {
                // On source → go to last target.
                let s = state.borrow();
                let i = num_targets - 1;
                Some((i, s.valid_target_ids[i].clone()))
            }
            Some(0) => {
                // At first target → go to source (or wrap to last).
                if source_hidden {
                    let s = state.borrow();
                    let i = num_targets - 1;
                    Some((i, s.valid_target_ids[i].clone()))
                } else {
                    None
                }
            }
            Some(i) => {
                let s = state.borrow();
                Some((i - 1, s.valid_target_ids[i - 1].clone()))
            }
        }
    } else {
        match current_index {
            None => {
                // On source (or initial) → go to first target.
                let s = state.borrow();
                Some((0, s.valid_target_ids[0].clone()))
            }
            Some(i) if i + 1 >= num_targets => {
                // At last target → go to source (or wrap to first).
                if source_hidden {
                    let s = state.borrow();
                    Some((0, s.valid_target_ids[0].clone()))
                } else {
                    None
                }
            }
            Some(i) => {
                let s = state.borrow();
                Some((i + 1, s.valid_target_ids[i + 1].clone()))
            }
        }
    };

    // Fire exit/enter and update state.
    set_current_drop_target(
        state,
        new_target.as_ref().map(|(i, s)| (*i, s.as_str())),
        true,
    );

    // Announce the new position.
    if let Some((new_index, _)) = new_target {
        live_announcer::announce_polite(format!(
            "Over drop target {} of {}. Press Enter to drop, or press Escape to cancel.",
            new_index + 1,
            num_targets,
        ));
    }
}

/// Sets the current drop target, firing exit/enter events as needed.
/// When `should_focus` is true, focuses the new target or source element.
#[cfg(not(feature = "ssr"))]
fn set_current_drop_target(
    state: &Rc<RefCell<SessionState>>,
    new_target: Option<(usize, &str)>,
    should_focus: bool,
) {
    let old_target_id = {
        let s = state.borrow();
        s.current_target_index
            .map(|i| s.valid_target_ids[i].clone())
    };
    let new_target_id: Option<&str> = new_target.map(|(_, id)| id);

    // Only fire events if the target actually changed.
    if old_target_id.as_deref() != new_target_id {
        // Update index.
        state.borrow_mut().current_target_index = new_target.map(|(i, _)| i);

        let item_types = collect_drag_types(state);

        // Fire exit on old target.
        if let Some(ref old_id) = old_target_id {
            fire_drop_exit(old_id, &item_types);
        }

        // Fire enter on new target.
        if let Some(new_id) = new_target_id {
            fire_drop_enter(new_id, &item_types);
        }

        // Announce the first item's aria-label on initial focus.
        if !state.borrow().initial_focused {
            state.borrow_mut().initial_focused = true;
            if let Some(new_id) = new_target_id {
                let label = DROP_TARGETS.with(|targets| {
                    targets
                        .borrow()
                        .get(new_id)
                        .and_then(|t| t.element.get_attribute("aria-label"))
                });
                if let Some(label) = label {
                    live_announcer::announce_polite(label);
                }
            }
        }
    }

    // Focus.
    if should_focus {
        if let Some((_, id)) = new_target {
            focus_element_by_id(id);
        } else {
            // Navigate to source element.
            let source = state.borrow().source_element.clone();
            if let Some(html_el) = source.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    }
}

/// Fires `on_drop_activate` on the current drop target without performing a drop.
/// Called on Alt+Enter or Enter on an activate button.
#[cfg(not(feature = "ssr"))]
fn activate(state: &Rc<RefCell<SessionState>>) {
    let target_id = {
        let s = state.borrow();
        s.current_target_index
            .map(|i| s.valid_target_ids[i].clone())
    };
    let Some(target_id) = target_id else {
        return;
    };

    let on_activate =
        DROP_TARGETS.with(|t| t.borrow().get(&target_id).and_then(|t| t.on_drop_activate));
    if let Some(on_activate) = on_activate {
        let (x, y) = get_target_center_coords(&target_id);
        on_activate.run(DropActivateEvent { x, y });
    }
}

#[cfg(not(feature = "ssr"))]
fn drop_on_current_target(state: &Rc<RefCell<SessionState>>) {
    let (target_id, items, allowed_operations) = {
        let s = state.borrow();
        let Some(idx) = s.current_target_index else {
            cancel_session(state);
            return;
        };
        let target_id = s.valid_target_ids[idx].clone();
        (target_id, s.items.clone(), s.allowed_operations)
    };

    // Determine drop effect: check per-item callback first, then fall back to target-level.
    let current_drop_item = state.borrow().current_drop_item;
    let item_drop_effect = current_drop_item.and_then(|item_idx| {
        DROP_ITEMS.with(|drop_items| {
            let items_vec = drop_items.borrow();
            items_vec.get(item_idx).and_then(|drop_item| {
                drop_item.get_drop_operation.map(|get_op| {
                    let types = DragTypes::Known(
                        items
                            .iter()
                            .flat_map(DragItem::types)
                            .map(String::from)
                            .collect(),
                    );
                    get_op.run((types, allowed_operations))
                })
            })
        })
    });
    let drop_effect = item_drop_effect
        .unwrap_or_else(|| get_target_drop_operation(&target_id, &items, allowed_operations));

    // Check if the target wants to prevent focus restoration.
    let prevent_focus = DROP_TARGETS.with(|targets| {
        targets
            .borrow()
            .get(&target_id)
            .is_some_and(|t| t.prevent_focus_on_drop)
    });

    // Fire the drop callback on the target.
    let (tx, ty) = get_target_center_coords(&target_id);
    let on_drop =
        DROP_TARGETS.with(|targets| targets.borrow().get(&target_id).and_then(|t| t.on_drop));
    if let Some(on_drop) = on_drop {
        on_drop.run(DropEvent {
            items: items.clone(),
            drop_effect,
            x: tx,
            y: ty,
        });
    }

    live_announcer::announce_assertive("Drop complete.");

    // End the session.
    let on_end = state.borrow_mut().on_end.take();
    let source_element = state.borrow().source_element.clone();

    end_session(drop_effect);

    // Fire on_end callback.
    if let Some(on_end) = on_end {
        let (sx, sy) = get_element_center(&source_element);
        on_end(DragEndEvent {
            x: sx,
            y: sy,
            drop_effect,
        });
    }

    // Return focus to source element (unless the target manages its own focus).
    if !prevent_focus {
        if let Some(html_el) = source_element.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    }

    // Step 8: Re-trigger focusin on active element to fix focus ring state,
    // since focusin was suppressed during the drag session.
    if !prevent_focus {
        dispatch_focusin_on_active_element();
    }
}

#[cfg(not(feature = "ssr"))]
fn cancel_session(state: &Rc<RefCell<SessionState>>) {
    let on_end = state.borrow_mut().on_end.take();
    let source_element = state.borrow().source_element.clone();

    // Fire exit on current target.
    let (current_target_id, drag_types) = {
        let s = state.borrow();
        let target_id = s
            .current_target_index
            .map(|i| s.valid_target_ids[i].clone());
        let types = DragTypes::Known(
            s.items
                .iter()
                .flat_map(DragItem::types)
                .map(String::from)
                .collect(),
        );
        (target_id, types)
    };
    if let Some(target_id) = current_target_id {
        fire_drop_exit(&target_id, &drag_types);
    }

    live_announcer::announce_assertive("Drop cancelled.");

    end_session(DropEffect::None);

    // Fire on_end callback.
    if let Some(on_end) = on_end {
        let (sx, sy) = get_element_center(&source_element);
        on_end(DragEndEvent {
            x: sx,
            y: sy,
            drop_effect: DropEffect::None,
        });
    }

    // Return focus to source element (if not hidden).
    if !is_element_hidden(&source_element) {
        if let Some(html_el) = source_element.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    }

    // Step 8: Re-trigger focusin on active element to fix focus ring state.
    dispatch_focusin_on_active_element();
}

/// Tears down the active session (removes listeners, restores ARIA state).
#[cfg(not(feature = "ssr"))]
fn end_session(drop_effect: DropEffect) {
    let had_session = ACTIVE_SESSION.with(|session| {
        if let Some(session) = session.borrow_mut().take() {
            let window = web_sys::window();
            let document = window.as_ref().and_then(web_sys::Window::document);

            if let Some(document) = &document {
                // Remove keyboard listeners.
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "keydown",
                    session.keyboard_closure.as_ref().unchecked_ref(),
                    true,
                );
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "keyup",
                    session.keyup_closure.as_ref().unchecked_ref(),
                    true,
                );
                // Remove click listener.
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "click",
                    session.click_closure.as_ref().unchecked_ref(),
                    true,
                );
                // Remove pointerdown listener.
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "pointerdown",
                    session.pointerdown_closure.as_ref().unchecked_ref(),
                    true,
                );
                // Remove all CANCELED_EVENTS listeners.
                for event_name in CANCELED_EVENTS {
                    let _ = document.remove_event_listener_with_callback_and_bool(
                        event_name,
                        session.cancel_event_closure.as_ref().unchecked_ref(),
                        true,
                    );
                }
            }

            if let Some(window) = &window {
                // Remove focus/blur listeners from window.
                let _ = window.remove_event_listener_with_callback_and_bool(
                    "focus",
                    session.focus_closure.as_ref().unchecked_ref(),
                    true,
                );
                let _ = window.remove_event_listener_with_callback_and_bool(
                    "blur",
                    session.blur_closure.as_ref().unchecked_ref(),
                    true,
                );

                // Cancel pending requestAnimationFrame.
                if let Some((handle, _)) = session.pending_raf.borrow_mut().take() {
                    window.cancel_animation_frame(handle).ok();
                }
            }

            // Disconnect MutationObserver.
            if let Some(ref observer) = session.mutation_observer {
                observer.disconnect();
            }

            // Restore ARIA visibility.
            if let Some(cleanup) = session.aria_hide_cleanup.borrow_mut().take() {
                cleanup();
            }

            // Fire on_end if not already taken.
            let on_end = session.state.borrow_mut().on_end.take();
            if let Some(on_end) = on_end {
                let (sx, sy) = get_element_center(&session.state.borrow().source_element);
                on_end(DragEndEvent {
                    x: sx,
                    y: sy,
                    drop_effect,
                });
            }

            true
        } else {
            false
        }
    });

    if had_session {
        notify_session_change();
    }
}

/// Re-validates drop targets, updates ARIA hiding, and sets up a `MutationObserver`.
///
/// Called when:
/// - A drag session begins (initial setup)
/// - A drop target is registered or unregistered during an active session
/// - The `MutationObserver` detects `aria-hidden` or `inert` attribute changes
#[cfg(not(feature = "ssr"))]
fn update_valid_drop_targets() {
    ACTIVE_SESSION.with(|session_cell| {
        let session_borrow = session_cell.borrow();
        let Some(session) = session_borrow.as_ref() else {
            return;
        };

        let (item_types, allowed_operations, source_element) = {
            let s = session.state.borrow();
            let types: Vec<String> = s
                .items
                .iter()
                .flat_map(DragItem::types)
                .map(String::from)
                .collect();
            (types, s.allowed_operations, s.source_element.clone())
        };

        // Re-find valid targets and sort by proximity.
        let mut valid_target_ids = find_valid_targets(&item_types, allowed_operations);
        sort_targets_by_proximity(&source_element, &mut valid_target_ids);

        // If current target is no longer valid, find nearest valid target.
        let current_target_id = {
            let s = session.state.borrow();
            s.current_target_index
                .map(|i| s.valid_target_ids[i].clone())
        };
        let new_current_index = if let Some(ref current_id) = current_target_id {
            valid_target_ids.iter().position(|id| id == current_id)
        } else {
            None
        };

        // Update state with new valid targets.
        {
            let mut s = session.state.borrow_mut();
            s.valid_target_ids.clone_from(&valid_target_ids);
            s.current_target_index = new_current_index;
        }

        // --- Swap ARIA hiding ---
        // Clean up old ARIA hiding.
        if let Some(old_cleanup) = session.aria_hide_cleanup.borrow_mut().take() {
            old_cleanup();
        }

        // Set up new ARIA hiding.
        let mut visible_elements = vec![source_element];

        let valid_item_elements: Vec<web_sys::Element> = DROP_ITEMS.with(|items| {
            let items = items.borrow();
            items
                .iter()
                .filter(|item| {
                    item.get_drop_operation.is_none_or(|get_op| {
                        let types = DragTypes::Known(item_types.iter().cloned().collect());
                        get_op.run((types, allowed_operations)) != DropEffect::None
                    })
                })
                .map(|item| item.element.clone())
                .collect()
        });

        DROP_TARGETS.with(|targets| {
            let targets = targets.borrow();
            for id in &valid_target_ids {
                if let Some(target) = targets.get(id) {
                    let contains_valid_item = valid_item_elements.iter().any(|item_el| {
                        crate::utils::dom_ext::node_contains(
                            Some(target.element.as_ref()),
                            Some(item_el.as_ref()),
                        )
                        .unwrap_or(false)
                    });
                    if !contains_valid_item {
                        visible_elements.push(target.element.clone());
                    }
                }
            }
        });

        visible_elements.extend(valid_item_elements);

        let new_cleanup = crate::utils::aria_hide_outside(
            &visible_elements,
            crate::utils::AriaHideOutsideOptions::default(),
        );
        *session.aria_hide_cleanup.borrow_mut() = Some(new_cleanup);

        // --- Set up MutationObserver ---
        // Disconnect any existing observer first.
        if let Some(ref observer) = session.mutation_observer {
            observer.disconnect();
        }

        // Note: We cannot mutably modify session.mutation_observer/mutation_callback
        // while session_cell is borrowed. Instead, the observer is set up once in
        // begin_dragging after the session is stored. Re-calls from register/unregister
        // reuse the existing observer — it watches document.body with subtree: true,
        // so it already covers dynamically added targets.
    });
}

/// Sets up the `MutationObserver` for the active session.
/// Must be called after the session is stored in `ACTIVE_SESSION`.
#[cfg(not(feature = "ssr"))]
fn setup_mutation_observer() {
    let callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)> = Closure::new(
        move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
            update_valid_drop_targets();
        },
    );

    let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()) else {
        return;
    };

    let document = web_sys::window().and_then(|w| w.document());
    if let Some(body) = document.and_then(|d| d.body()) {
        let options = web_sys::MutationObserverInit::new();
        options.set_subtree(true);
        options.set_attributes(true);

        // Set attributeFilter via js_sys::Reflect (not available on MutationObserverInit).
        let filter = js_sys::Array::of2(
            &wasm_bindgen::JsValue::from_str("aria-hidden"),
            &wasm_bindgen::JsValue::from_str("inert"),
        );
        let _ = js_sys::Reflect::set(
            options.as_ref(),
            &wasm_bindgen::JsValue::from_str("attributeFilter"),
            &filter,
        );

        let _ = observer.observe_with_options(body.as_ref(), &options);
    }

    // Store observer and callback in the session.
    ACTIVE_SESSION.with(|session_cell| {
        let mut session_borrow = session_cell.borrow_mut();
        if let Some(ref mut session) = *session_borrow {
            session.mutation_observer = Some(observer);
            session.mutation_callback = Some(callback);
        }
    });
}

/// Returns the `activate_button` for the current drop item or drop target.
/// Checks `current_drop_item` first, then falls back to the current target.
#[cfg(not(feature = "ssr"))]
fn get_current_activate_button(state: &Rc<RefCell<SessionState>>) -> Option<web_sys::Element> {
    let s = state.borrow();

    // Check current drop item first.
    if let Some(item_index) = s.current_drop_item {
        let btn = DROP_ITEMS.with(|items| {
            items
                .borrow()
                .get(item_index)
                .and_then(|item| item.activate_button.clone())
        });
        if btn.is_some() {
            return btn;
        }
    }

    // Fall back to current target.
    if let Some(target_idx) = s.current_target_index {
        let target_id = &s.valid_target_ids[target_idx];
        return DROP_TARGETS.with(|targets| {
            targets
                .borrow()
                .get(target_id)
                .and_then(|t| t.activate_button.clone())
        });
    }

    None
}

/// Returns the center coordinates of an element's bounding client rect.
#[cfg(not(feature = "ssr"))]
fn get_element_center(element: &web_sys::Element) -> (f64, f64) {
    let rect = element.get_bounding_client_rect();
    (
        rect.left() + rect.width() / 2.0,
        rect.top() + rect.height() / 2.0,
    )
}

/// Returns the center coordinates of a registered drop target's element.
/// Falls back to `(0.0, 0.0)` if the target is not found.
#[cfg(not(feature = "ssr"))]
fn get_target_center_coords(target_id: &str) -> (f64, f64) {
    DROP_TARGETS.with(|t| {
        t.borrow()
            .get(target_id)
            .map_or((0.0, 0.0), |t| get_element_center(&t.element))
    })
}

/// Find all drop targets that accept the given item types and allowed operations.
#[cfg(not(feature = "ssr"))]
fn find_valid_targets(
    item_types: &[String],
    allowed_operations: AllowedDropOperations,
) -> Vec<String> {
    let drag_types = DragTypes::Known(item_types.iter().cloned().collect());
    DROP_TARGETS.with(|targets| {
        let targets = targets.borrow();
        targets
            .iter()
            .filter(|(_, target)| {
                // Check type acceptance.
                let types_accepted = target.accepted_types.is_empty()
                    || item_types.iter().any(|t| target.accepted_types.contains(t));

                if !types_accepted {
                    return false;
                }

                // Check drop operation.
                if let Some(get_op) = target.get_drop_operation {
                    let effect = get_op.run(DropOperationEvent {
                        types: drag_types.clone(),
                        allowed_operations,
                    });
                    effect != DropEffect::None
                } else {
                    true
                }
            })
            .map(|(id, _)| id.clone())
            .collect()
    })
}

/// Get the drop operation for a specific target.
#[cfg(not(feature = "ssr"))]
fn get_target_drop_operation(
    target_id: &str,
    items: &[DragItem],
    allowed_operations: AllowedDropOperations,
) -> DropEffect {
    DROP_TARGETS.with(|targets| {
        let targets = targets.borrow();
        let Some(target) = targets.get(target_id) else {
            return DropEffect::None;
        };

        if let Some(get_op) = target.get_drop_operation {
            let types = DragTypes::Known(
                items
                    .iter()
                    .flat_map(DragItem::types)
                    .map(String::from)
                    .collect(),
            );
            get_op.run(DropOperationEvent {
                types,
                allowed_operations,
            })
        } else {
            // Default to Move if allowed, else Copy, else Link.
            if allowed_operations.contains_effect(DropEffect::Move) {
                DropEffect::Move
            } else if allowed_operations.contains_effect(DropEffect::Copy) {
                DropEffect::Copy
            } else if allowed_operations.contains_effect(DropEffect::Link) {
                DropEffect::Link
            } else {
                DropEffect::None
            }
        }
    })
}

/// Collect all MIME types from the session's items as a `DragTypes`.
#[cfg(not(feature = "ssr"))]
fn collect_drag_types(state: &Rc<RefCell<SessionState>>) -> DragTypes {
    let types = state
        .borrow()
        .items
        .iter()
        .flat_map(DragItem::types)
        .map(String::from)
        .collect();
    DragTypes::Known(types)
}

/// Fire `on_drop_enter` on a registered target.
#[cfg(not(feature = "ssr"))]
fn fire_drop_enter(target_id: &str, drag_types: &DragTypes) {
    let on_enter = DROP_TARGETS.with(|targets| {
        targets
            .borrow()
            .get(target_id)
            .and_then(|t| t.on_drop_enter)
    });
    if let Some(on_enter) = on_enter {
        let (x, y) = get_target_center_coords(target_id);
        on_enter.run(DropEnterEvent {
            types: drag_types.clone(),
            x,
            y,
        });
    }
}

/// Fire `on_drop_exit` on a registered target.
#[cfg(not(feature = "ssr"))]
fn fire_drop_exit(target_id: &str, drag_types: &DragTypes) {
    let on_exit =
        DROP_TARGETS.with(|targets| targets.borrow().get(target_id).and_then(|t| t.on_drop_exit));
    if let Some(on_exit) = on_exit {
        let (x, y) = get_target_center_coords(target_id);
        on_exit.run(DropExitEvent {
            types: drag_types.clone(),
            x,
            y,
        });
    }
}

/// Sorts target IDs by squared Euclidean distance from the source element.
/// Nearest targets come first, giving keyboard users the closest drop target.
#[cfg(not(feature = "ssr"))]
fn sort_targets_by_proximity(source: &web_sys::Element, target_ids: &mut [String]) {
    let source_rect = source.get_bounding_client_rect();
    let sx = source_rect.left() + source_rect.width() / 2.0;
    let sy = source_rect.top() + source_rect.height() / 2.0;

    target_ids.sort_by(|a, b| {
        let dist_a = distance_squared_to_target(a, sx, sy);
        let dist_b = distance_squared_to_target(b, sx, sy);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// Returns the squared distance from (sx, sy) to the center of a target element.
#[cfg(not(feature = "ssr"))]
fn distance_squared_to_target(target_id: &str, sx: f64, sy: f64) -> f64 {
    DROP_TARGETS.with(|targets| {
        let targets = targets.borrow();
        targets.get(target_id).map_or(f64::MAX, |t| {
            let rect = t.element.get_bounding_client_rect();
            let tx = rect.left() + rect.width() / 2.0;
            let ty = rect.top() + rect.height() / 2.0;
            (tx - sx).powi(2) + (ty - sy).powi(2)
        })
    })
}

/// Returns `true` if a virtual (keyboard) drag session is currently active.
#[cfg(not(feature = "ssr"))]
pub fn is_virtual_dragging() -> bool {
    ACTIVE_SESSION.with(|session| session.borrow().is_some())
}

/// Returns `true` if a virtual drag session is active (SSR stub).
#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub fn is_virtual_dragging() -> bool {
    false
}

/// Focus the element belonging to a registered drop target by ID.
#[cfg(not(feature = "ssr"))]
fn focus_element_by_id(target_id: &str) {
    DROP_TARGETS.with(|targets| {
        let targets = targets.borrow();
        if let Some(target) = targets.get(target_id) {
            if let Some(html_el) = target.element.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    });
}

/// Returns `true` if the element (or an ancestor) has `aria-hidden="true"` or `inert`.
#[cfg(not(feature = "ssr"))]
fn is_element_hidden(el: &web_sys::Element) -> bool {
    el.closest(r#"[aria-hidden="true"], [inert]"#)
        .ok()
        .flatten()
        .is_some()
}

/// Finds a valid drop target whose element matches or contains the given element.
/// Returns the index and ID of the first match.
#[cfg(not(feature = "ssr"))]
fn find_target_for_element(
    valid_target_ids: &[String],
    element: &web_sys::Element,
) -> Option<(usize, String)> {
    DROP_TARGETS.with(|targets| {
        let targets = targets.borrow();
        // First pass: exact element match.
        for (i, id) in valid_target_ids.iter().enumerate() {
            if let Some(t) = targets.get(id) {
                if t.element == *element {
                    return Some((i, id.clone()));
                }
            }
        }
        // Second pass: containment (element is inside target).
        for (i, id) in valid_target_ids.iter().enumerate() {
            if let Some(t) = targets.get(id) {
                if crate::utils::dom_ext::node_contains(
                    Some(t.element.as_ref()),
                    Some(element.as_ref()),
                )
                .unwrap_or(false)
                {
                    return Some((i, id.clone()));
                }
            }
        }
        None
    })
}

/// Dispatches a synthetic `focusin` event on the active element.
///
/// After a drop or cancel, focusin events have been suppressed during the session.
/// Re-dispatching corrects focus ring state in components that rely on `focusin`.
#[cfg(not(feature = "ssr"))]
fn dispatch_focusin_on_active_element() {
    let document = web_sys::window().and_then(|w| w.document());
    if let Some(document) = document {
        if let Some(active) = document.active_element() {
            let init = web_sys::FocusEventInit::new();
            init.set_bubbles(true);
            if let Ok(event) = web_sys::FocusEvent::new_with_focus_event_init_dict("focusin", &init)
            {
                let _ = active.dispatch_event(&event);
            }
        }
    }
}
