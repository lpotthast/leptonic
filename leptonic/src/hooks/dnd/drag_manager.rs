// Upstream: react-aria/src/dnd/DragManager.ts @ 99e6102368
//! Keyboard and screen reader drag and drop: a drag session lets the user move between drop
//! targets (Tab, or a collection's own keys), drop (Enter) or cancel (Escape), while the rest of
//! the page is inert (`inert`, not only `aria-hidden`, as react-aria's `shouldUseInert`).

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use leptos::prelude::*;
use leptos_use::{use_document, use_window};
use wasm_bindgen::{JsCast, closure::Closure};

use super::{
    messages,
    types::{
        DragEndEvent, DragItem, DragTypes, DropActivateEvent, DropEnterEvent, DropEvent,
        DropExitEvent, DropItem, DropOperation, DropTarget, TextDropItem,
    },
    utils::{DragModality, get_drag_modality},
};
use crate::utils::{
    CapturedElement, EventAccessors,
    aria_hide_outside::{AriaHideOutsideOptions, HideMode, aria_hide_outside},
    event_listeners::{Listener, listen},
    key::{KeyboardEventKey, KeyboardKey},
    live_announcer::{Assertiveness, announce},
    node_contains,
    virtual_click::{is_virtual_click, is_virtual_pointer_event},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Registrations return an id to unregister with (react-aria: a cleanup function), so that
//   hooks can unregister in `on_cleanup`.
// - The session's document and window listeners are `Fn` closures: moving focus inside a focus
//   listener dispatches nested focus events into the same listener.
// - Hooks observe the session through [`use_drag_session`], a signal of the dragged items and
//   allowed operations.
//
// =============================================================================

/// Decides the drop operation for dragged data of `types`, allowing `allowed` (in order).
pub(crate) type GetDropOperation = Rc<dyn Fn(&DragTypes, &[DropOperation]) -> DropOperation>;

/// The source of a keyboard drag.
#[derive(Clone)]
pub(crate) struct DragTarget {
    pub element: web_sys::Element,
    pub items: Vec<DragItem>,
    pub allowed_drop_operations: Vec<DropOperation>,
    pub on_drag_end: Option<Rc<dyn Fn(DragEndEvent)>>,
}

/// A drop target taking part in keyboard drags.
#[derive(Default)]
pub(crate) struct DropTargetOptions {
    pub element: Option<web_sys::Element>,
    pub prevent_focus_on_drop: bool,
    pub get_drop_operation: Option<GetDropOperation>,
    pub on_drop_enter: Option<Rc<dyn Fn(DropEnterEvent, &DragTarget)>>,
    pub on_drop_exit: Option<Rc<dyn Fn(DropExitEvent)>>,
    pub on_drop_target_enter: Option<Rc<dyn Fn(Option<DropTarget>)>>,
    pub on_drop_activate: Option<Rc<dyn Fn(DropActivateEvent, Option<DropTarget>)>>,
    pub on_drop: Option<Rc<dyn Fn(DropEvent, Option<DropTarget>)>>,
    pub on_key_down: Option<Rc<dyn Fn(&web_sys::KeyboardEvent, &DragTarget)>>,
    pub activate_button: Option<CapturedElement>,
}

/// An item of a collection drop target (it is focused when it is the current drop position).
pub(crate) struct DroppableItemOptions {
    pub element: web_sys::Element,
    pub target: DropTarget,
    pub get_drop_operation: Option<GetDropOperation>,
    pub activate_button: Option<CapturedElement>,
}

/// What hooks rendering during a drag need to know about it.
#[derive(Debug, Clone, PartialEq)]
pub struct DragSessionInfo {
    pub items: Vec<DragItem>,
    pub allowed_drop_operations: Vec<DropOperation>,
}

type Target = Rc<DropTargetOptions>;
type Item = Rc<DroppableItemOptions>;

thread_local! {
    static DROP_TARGETS: RefCell<Vec<(u64, Target)>> = const { RefCell::new(Vec::new()) };
    static DROP_ITEMS: RefCell<Vec<(u64, Item)>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    static SESSION: RefCell<Option<Rc<DragSession>>> = const { RefCell::new(None) };
    static SESSION_INFO: ArcRwSignal<Option<DragSessionInfo>> = ArcRwSignal::new(None);
}

fn next_id() -> u64 {
    NEXT_ID.with(|id| {
        let next = id.get();
        id.set(next.wrapping_add(1));
        next
    })
}

fn session() -> Option<Rc<DragSession>> {
    SESSION.with(|s| s.borrow().clone())
}

/// Register a drop target, replacing another registration of the same element. Unregister it
/// with [`unregister_drop_target`].
pub(crate) fn register_drop_target(target: DropTargetOptions) -> u64 {
    let id = next_id();
    DROP_TARGETS.with(|t| {
        let mut targets = t.borrow_mut();
        targets.retain(|(_, existing)| existing.element != target.element);
        targets.push((id, Rc::new(target)));
    });
    update_session_drop_targets();
    id
}

pub(crate) fn unregister_drop_target(id: u64) {
    DROP_TARGETS.with(|t| t.borrow_mut().retain(|(i, _)| *i != id));
    update_session_drop_targets();
}

/// A drop target came or went during a drag: re-validate the session's targets. Hooks
/// (un)register in Effects, and this runs the targets' `get_drop_operation` and enter/exit
/// callbacks, which must not subscribe the registering Effect to the signals they read (it would
/// re-register, and lose the current drop target, whenever they change).
fn update_session_drop_targets() {
    if let Some(session) = session() {
        untrack(|| session.update_valid_drop_targets());
    }
}

/// Register a drop item, replacing another registration of the same element. Unregister it
/// with [`unregister_drop_item`].
pub(crate) fn register_drop_item(item: DroppableItemOptions) -> u64 {
    let id = next_id();
    DROP_ITEMS.with(|i| {
        let mut items = i.borrow_mut();
        items.retain(|(_, existing)| existing.element != item.element);
        items.push((id, Rc::new(item)));
    });
    id
}

pub(crate) fn unregister_drop_item(id: u64) {
    DROP_ITEMS.with(|i| i.borrow_mut().retain(|(i, _)| *i != id));
}

fn drop_targets() -> Vec<Target> {
    DROP_TARGETS.with(|t| t.borrow().iter().map(|(_, t)| t.clone()).collect())
}

fn drop_items() -> Vec<Item> {
    DROP_ITEMS.with(|i| i.borrow().iter().map(|(_, i)| i.clone()).collect())
}

fn drop_item_for(element: &web_sys::Element) -> Option<Item> {
    DROP_ITEMS.with(|i| {
        i.borrow()
            .iter()
            .find(|(_, item)| item.element == *element)
            .map(|(_, item)| item.clone())
    })
}

/// Start a keyboard (or screen reader) drag of `target`.
pub(crate) fn begin_dragging(target: DragTarget) {
    if session().is_some() {
        leptos::logging::error!("Cannot begin dragging while already dragging");
        return;
    }
    let session = Rc::new(DragSession::new(target));
    SESSION.with(|s| *s.borrow_mut() = Some(session.clone()));
    SESSION_INFO.with(|info| {
        info.set(Some(DragSessionInfo {
            items: session.drag_target.items.clone(),
            allowed_drop_operations: session.drag_target.allowed_drop_operations.clone(),
        }));
    });
    request_animation_frame(move || {
        if is_virtual_dragging() {
            session.setup();
            if get_drag_modality() == DragModality::Keyboard {
                session.next();
            }
        }
    });
}

/// The current keyboard drag session, if any (react-aria's `useDragSession`).
pub fn use_drag_session() -> Signal<Option<DragSessionInfo>> {
    let info = SESSION_INFO.with(Clone::clone);
    Signal::derive(move || info.get())
}

/// Whether a keyboard (or screen reader) drag is in progress.
pub fn is_virtual_dragging() -> bool {
    session().is_some()
}

fn end_dragging() {
    SESSION.with(|s| *s.borrow_mut() = None);
    SESSION_INFO.with(|info| info.set(None));
}

/// Events the session blocks (it handles focus and keys itself).
const CANCELED_EVENTS: [&str; 19] = [
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

/// Events whose default must stay, so that clicks are still synthesized.
const CLICK_EVENTS: [&str; 3] = ["pointerup", "mouseup", "touchend"];

fn is_hidden(element: &web_sys::Element) -> bool {
    element
        .closest("[aria-hidden=\"true\"], [inert]")
        .ok()
        .flatten()
        .is_some()
}

fn contains(container: Option<&web_sys::Element>, node: Option<&web_sys::Node>) -> bool {
    node_contains(container.map(JsCast::unchecked_ref::<web_sys::Node>), node).unwrap_or(false)
}

fn center(element: &web_sys::Element) -> (f64, f64) {
    let rect = element.get_bounding_client_rect();
    (
        rect.left() + rect.width() / 2.0,
        rect.top() + rect.height() / 2.0,
    )
}

fn focus(element: &web_sys::Element) {
    if let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() {
        let _ = element.focus();
    }
}

fn active_element() -> Option<web_sys::Element> {
    use_document()
        .as_ref()
        .and_then(web_sys::Document::active_element)
}

#[derive(Default)]
struct SessionState {
    valid_drop_targets: Vec<Target>,
    current_drop_target: Option<Target>,
    current_drop_item: Option<Item>,
    drop_operation: Option<DropOperation>,
    restore_aria_hidden: Option<Box<dyn FnOnce()>>,
    is_virtual_click: bool,
    initial_focused: bool,
}

struct DragSession {
    drag_target: DragTarget,
    state: RefCell<SessionState>,
    listeners: RefCell<Vec<Listener>>,
    mutation_observer: RefCell<Option<(web_sys::MutationObserver, Closure<dyn Fn()>)>>,
}

impl DragSession {
    fn new(drag_target: DragTarget) -> Self {
        Self {
            drag_target,
            state: RefCell::new(SessionState::default()),
            listeners: RefCell::new(Vec::new()),
            mutation_observer: RefCell::new(None),
        }
    }

    fn setup(self: &Rc<Self>) {
        let (Some(document), Some(window)) = (
            use_document().as_ref().cloned(),
            use_window().as_ref().cloned(),
        ) else {
            return;
        };
        let document: web_sys::EventTarget = document.into();
        let window: web_sys::EventTarget = window.into();
        let mut listeners = Vec::new();
        let this = self.clone();
        listeners.push(listen(&document, "keydown", true, move |e| {
            this.on_key_down(e.unchecked_ref());
        }));
        let this = self.clone();
        listeners.push(listen(&document, "keyup", true, move |e| {
            this.on_key_up(e.unchecked_ref());
        }));
        let this = self.clone();
        listeners.push(listen(&window, "focus", true, move |e| {
            this.on_focus(e.unchecked_ref());
        }));
        let this = self.clone();
        listeners.push(listen(&window, "blur", true, move |e| {
            this.on_blur(e.unchecked_ref());
        }));
        let this = self.clone();
        listeners.push(listen(&document, "click", true, move |e| {
            this.on_click(e.unchecked_ref());
        }));
        let this = self.clone();
        listeners.push(listen(&document, "pointerdown", true, move |e| {
            this.on_pointer_down(e.unchecked_ref());
        }));
        for event in CANCELED_EVENTS {
            let this = self.clone();
            listeners.push(listen(&document, event, true, move |e| {
                this.cancel_event(&e);
            }));
        }
        *self.listeners.borrow_mut() = listeners;

        let this = Rc::downgrade(self);
        let callback = Closure::<dyn Fn()>::new(move || {
            if let Some(this) = this.upgrade() {
                this.update_valid_drop_targets();
            }
        });
        if let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()) {
            *self.mutation_observer.borrow_mut() = Some((observer, callback));
        }
        self.update_valid_drop_targets();

        announce(
            messages::drag_started(get_drag_modality()),
            Assertiveness::Assertive,
        );
    }

    fn teardown(&self) {
        self.listeners.borrow_mut().clear();
        if let Some((observer, _)) = self.mutation_observer.borrow_mut().take() {
            observer.disconnect();
        }
        let restore = self.state.borrow_mut().restore_aria_hidden.take();
        if let Some(restore) = restore {
            restore();
        }
    }

    fn current_drop_target(&self) -> Option<Target> {
        self.state.borrow().current_drop_target.clone()
    }

    fn current_drop_item(&self) -> Option<Item> {
        self.state.borrow().current_drop_item.clone()
    }

    fn types(&self) -> DragTypes {
        DragTypes::of_items(&self.drag_target.items)
    }

    fn on_key_down(self: &Rc<Self>, e: &web_sys::KeyboardEvent) {
        self.cancel_event(e);
        if e.typed_key() == KeyboardKey::Escape {
            self.cancel();
            return;
        }
        if e.typed_key() == KeyboardKey::Tab && !(e.meta_key() || e.alt_key() || e.ctrl_key()) {
            if e.shift_key() {
                self.previous();
            } else {
                self.next();
            }
        }
        if let Some(on_key_down) = self
            .current_drop_target()
            .and_then(|t| t.on_key_down.clone())
        {
            on_key_down(e, &self.drag_target);
        }
    }

    fn on_key_up(self: &Rc<Self>, e: &web_sys::KeyboardEvent) {
        self.cancel_event(e);
        if e.typed_key() == KeyboardKey::Enter {
            let target = e.expect_target().dyn_into::<web_sys::Node>().ok();
            let on_activate_button =
                contains(self.current_activate_button().as_ref(), target.as_ref());
            if e.alt_key() || on_activate_button {
                Self::activate(self.current_drop_target(), self.current_drop_item());
            } else {
                self.drop(None);
            }
        }
    }

    fn current_activate_button(&self) -> Option<web_sys::Element> {
        let state = self.state.borrow();
        state
            .current_drop_item
            .as_ref()
            .and_then(|item| item.activate_button)
            .or_else(|| {
                state
                    .current_drop_target
                    .as_ref()
                    .and_then(|t| t.activate_button)
            })
            .and_then(|button| button.get_untracked())
            .map(|button| (*button).clone())
    }

    fn on_focus(self: &Rc<Self>, e: &web_sys::FocusEvent) {
        let event_target = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        if event_target.is_some() && event_target == self.current_activate_button() {
            self.cancel_event(e);
            return;
        }
        if event_target.as_ref() != Some(&self.drag_target.element) {
            self.cancel_event(e);
        }
        let Some(event_target) = event_target
            .filter(JsCast::is_instance_of::<web_sys::HtmlElement>)
            .filter(|t| *t != self.drag_target.element)
        else {
            return;
        };
        let valid = self.state.borrow().valid_drop_targets.clone();
        let drop_target = valid
            .iter()
            .find(|t| t.element.as_ref() == Some(&event_target))
            .or_else(|| {
                valid
                    .iter()
                    .find(|t| contains(t.element.as_ref(), Some(event_target.unchecked_ref())))
            })
            .cloned();
        let Some(drop_target) = drop_target else {
            match self.current_drop_target().and_then(|t| t.element.clone()) {
                Some(element) => focus(&element),
                None => focus(&self.drag_target.element),
            }
            return;
        };
        let item = drop_item_for(&event_target);
        self.set_current_drop_target(Some(drop_target), item);
    }

    fn on_blur(self: &Rc<Self>, e: &web_sys::FocusEvent) {
        let related = e.related_target();
        if let Some(button) = self.current_activate_button()
            && related
                .as_ref()
                .is_some_and(|r| r == button.unchecked_ref::<web_sys::EventTarget>())
        {
            self.cancel_event(e);
            return;
        }
        let event_target = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        if event_target.as_ref() != Some(&self.drag_target.element) {
            self.cancel_event(e);
        }
        if !related.is_some_and(|r| r.is_instance_of::<web_sys::HtmlElement>()) {
            match self.current_drop_target().and_then(|t| t.element.clone()) {
                Some(element) => focus(&element),
                None => focus(&self.drag_target.element),
            }
        }
    }

    fn on_click(self: &Rc<Self>, e: &web_sys::MouseEvent) {
        self.cancel_event(e);
        let is_virtual = self.state.borrow().is_virtual_click;
        if !(is_virtual_click(e) || is_virtual) {
            return;
        }
        let Some(event_target) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let node: &web_sys::Node = event_target.unchecked_ref();
        let item = drop_items().into_iter().find(|item| {
            item.element == event_target
                || contains(
                    item.activate_button
                        .and_then(|b| b.get_untracked())
                        .map(|b| (*b).clone())
                        .as_ref(),
                    Some(node),
                )
        });
        let drop_target = self
            .state
            .borrow()
            .valid_drop_targets
            .iter()
            .find(|t| contains(t.element.as_ref(), Some(node)))
            .cloned();
        let activate_button = item
            .as_ref()
            .and_then(|i| i.activate_button)
            .or_else(|| drop_target.as_ref().and_then(|t| t.activate_button))
            .and_then(|b| b.get_untracked())
            .map(|b| (*b).clone());
        if let Some(drop_target) = &drop_target
            && contains(activate_button.as_ref(), Some(node))
        {
            Self::activate(Some(drop_target.clone()), item);
            return;
        }
        if event_target == self.drag_target.element {
            self.cancel();
            return;
        }
        if let Some(drop_target) = drop_target {
            self.set_current_drop_target(Some(drop_target), item.clone());
            self.drop(item);
        }
    }

    fn on_pointer_down(&self, e: &web_sys::PointerEvent) {
        self.cancel_event(e);
        self.state.borrow_mut().is_virtual_click = is_virtual_pointer_event(e);
    }

    fn cancel_event(&self, e: &web_sys::Event) {
        let kind = e.type_();
        if kind == "focusin" || kind == "focusout" {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            if target.is_some()
                && (target.as_ref() == Some(&self.drag_target.element)
                    || target == self.current_activate_button())
            {
                return;
            }
        }
        if !CLICK_EVENTS.contains(&kind.as_str()) {
            e.prevent_default();
        }
        e.stop_propagation();
        e.stop_immediate_propagation();
    }

    fn update_valid_drop_targets(self: &Rc<Self>) {
        let Some(observer) = self
            .mutation_observer
            .borrow()
            .as_ref()
            .map(|(observer, _)| observer.clone())
        else {
            return;
        };
        observer.disconnect();
        let restore = self.state.borrow_mut().restore_aria_hidden.take();
        if let Some(restore) = restore {
            restore();
        }

        let mut valid = find_valid_drop_targets(&self.drag_target);
        if !valid.is_empty() {
            let nearest = self.find_nearest_drop_target(&valid);
            valid.rotate_left(nearest);
        }
        self.state
            .borrow_mut()
            .valid_drop_targets
            .clone_from(&valid);

        let current = self.current_drop_target();
        if let Some(current) = current
            && !valid.iter().any(|t| Rc::ptr_eq(t, &current))
        {
            self.set_current_drop_target(valid.first().cloned(), None);
        }

        let types = self.types();
        let allowed = &self.drag_target.allowed_drop_operations;
        let valid_items: Vec<Item> = drop_items()
            .into_iter()
            .filter(|item| {
                item.get_drop_operation
                    .as_ref()
                    .is_none_or(|get| get(&types, allowed) != DropOperation::Cancel)
            })
            .collect();
        let visible_targets = valid.iter().filter(|t| {
            !valid_items
                .iter()
                .any(|item| contains(t.element.as_ref(), Some(item.element.unchecked_ref())))
        });

        let mut keep = vec![self.drag_target.element.clone()];
        let button =
            |b: Option<CapturedElement>| b.and_then(|b| b.get_untracked()).map(|b| (*b).clone());
        for item in &valid_items {
            keep.push(item.element.clone());
            keep.extend(button(item.activate_button));
        }
        for target in visible_targets {
            keep.extend(target.element.clone());
            keep.extend(button(target.activate_button));
        }
        // `inert`: what can't take the drop is neither announced nor focusable or clickable.
        let restore = aria_hide_outside(
            &keep,
            AriaHideOutsideOptions {
                mode: HideMode::Inert,
                ..AriaHideOutsideOptions::default()
            },
        );
        self.state.borrow_mut().restore_aria_hidden = Some(restore);

        if let Some(body) = use_document().as_ref().and_then(web_sys::Document::body) {
            let options = web_sys::MutationObserverInit::new();
            options.set_subtree(true);
            options.set_attributes(true);
            let filter = js_sys::Array::of2(&"aria-hidden".into(), &"inert".into());
            options.set_attribute_filter(&filter);
            let _ = observer.observe_with_options(&body, &options);
        }
    }

    fn next(self: &Rc<Self>) {
        let (valid, current) = {
            let state = self.state.borrow();
            (
                state.valid_drop_targets.clone(),
                state.current_drop_target.clone(),
            )
        };
        let index = current
            .as_ref()
            .and_then(|c| valid.iter().position(|t| Rc::ptr_eq(t, c)));
        match index {
            None => self.set_current_drop_target(valid.first().cloned(), None),
            Some(index) if index + 1 == valid.len() => {
                // Past the last target: the drag source, unless it is hidden.
                if is_hidden(&self.drag_target.element) {
                    self.set_current_drop_target(valid.first().cloned(), None);
                } else {
                    self.set_current_drop_target(None, None);
                    focus(&self.drag_target.element);
                }
            }
            Some(index) => self.set_current_drop_target(valid.get(index + 1).cloned(), None),
        }
    }

    fn previous(self: &Rc<Self>) {
        let (valid, current) = {
            let state = self.state.borrow();
            (
                state.valid_drop_targets.clone(),
                state.current_drop_target.clone(),
            )
        };
        let index = current
            .as_ref()
            .and_then(|c| valid.iter().position(|t| Rc::ptr_eq(t, c)));
        match index {
            None => self.set_current_drop_target(valid.last().cloned(), None),
            Some(0) => {
                if is_hidden(&self.drag_target.element) {
                    self.set_current_drop_target(valid.last().cloned(), None);
                } else {
                    self.set_current_drop_target(None, None);
                    focus(&self.drag_target.element);
                }
            }
            Some(index) => self.set_current_drop_target(valid.get(index - 1).cloned(), None),
        }
    }

    /// The drop target containing the drag source, or else the nearest one.
    fn find_nearest_drop_target(&self, valid: &[Target]) -> usize {
        let drag_rect = self.drag_target.element.get_bounding_client_rect();
        let mut min_distance = f64::INFINITY;
        let mut nearest = 0;
        let mut ancestor = None;
        for (i, target) in valid.iter().enumerate() {
            let Some(element) = &target.element else {
                continue;
            };
            if ancestor.is_none()
                && contains(
                    Some(element),
                    Some(self.drag_target.element.unchecked_ref()),
                )
            {
                ancestor = Some(i);
            }
            let rect = element.get_bounding_client_rect();
            let dx = rect.left() - drag_rect.left();
            let dy = rect.top() - drag_rect.top();
            let distance = dx * dx + dy * dy;
            if distance < min_distance {
                min_distance = distance;
                nearest = i;
            }
        }
        ancestor.unwrap_or(nearest)
    }

    fn set_current_drop_target(self: &Rc<Self>, drop_target: Option<Target>, item: Option<Item>) {
        let current = self.current_drop_target();
        let changed = match (&drop_target, &current) {
            (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
            (None, None) => false,
            _ => true,
        };
        if changed {
            if let Some(current) = &current
                && let (Some(on_exit), Some(element)) = (&current.on_drop_exit, &current.element)
            {
                let (x, y) = center(element);
                on_exit(DropExitEvent { x, y });
            }
            self.state
                .borrow_mut()
                .current_drop_target
                .clone_from(&drop_target);
            if let Some(target) = &drop_target {
                if let (Some(on_enter), Some(element)) = (&target.on_drop_enter, &target.element) {
                    let (x, y) = center(element);
                    on_enter(DropEnterEvent { x, y }, &self.drag_target);
                }
                if item.is_none()
                    && let Some(element) = &target.element
                {
                    focus(element);
                }
            }
        }

        let current_item = self.current_drop_item();
        if let Some(item) = item
            && current_item.as_ref().is_none_or(|c| !Rc::ptr_eq(c, &item))
        {
            if let Some(on_target_enter) = self
                .current_drop_target()
                .and_then(|t| t.on_drop_target_enter.clone())
            {
                on_target_enter(Some(item.target.clone()));
            }
            focus(&item.element);
            let announce_label = {
                let mut state = self.state.borrow_mut();
                state.current_drop_item = Some(item.clone());
                let first = !state.initial_focused;
                state.initial_focused = true;
                first
            };
            if announce_label && let Some(label) = item.element.get_attribute("aria-label") {
                announce(label, Assertiveness::Polite);
            }
        }
    }

    fn end(self: &Rc<Self>) {
        self.teardown();
        end_dragging();

        let (current, drop_operation) = {
            let state = self.state.borrow();
            (state.current_drop_target.clone(), state.drop_operation)
        };
        if let Some(on_drag_end) = &self.drag_target.on_drag_end {
            let element = match &current {
                Some(target) if drop_operation != Some(DropOperation::Cancel) => {
                    target.element.clone()
                }
                _ => None,
            }
            .unwrap_or_else(|| self.drag_target.element.clone());
            let (x, y) = center(&element);
            on_drag_end(DragEndEvent {
                x,
                y,
                drop_operation: drop_operation.unwrap_or(DropOperation::Cancel),
            });
        }
        if current.is_some_and(|t| !t.prevent_focus_on_drop) {
            dispatch_focusin_on_active_element();
        }
        self.set_current_drop_target(None, None);
    }

    fn cancel(self: &Rc<Self>) {
        self.set_current_drop_target(None, None);
        self.end();
        if !is_hidden(&self.drag_target.element) {
            focus(&self.drag_target.element);
        }
        dispatch_focusin_on_active_element();
        announce(messages::DROP_CANCELED, Assertiveness::Assertive);
    }

    fn drop(self: &Rc<Self>, item: Option<Item>) {
        let Some(target) = self.current_drop_target() else {
            self.cancel();
            return;
        };
        let types = self.types();
        let allowed = &self.drag_target.allowed_drop_operations;
        let operation = if let Some(get) = item.as_ref().and_then(|i| i.get_drop_operation.as_ref())
        {
            get(&types, allowed)
        } else if let Some(get) = &target.get_drop_operation {
            get(&types, allowed)
        } else {
            allowed.first().copied().unwrap_or(DropOperation::Cancel)
        };
        self.state.borrow_mut().drop_operation = Some(operation);

        if let (Some(on_drop), Some(element)) = (&target.on_drop, &target.element) {
            let items = self
                .drag_target
                .items
                .iter()
                .map(|item| {
                    DropItem::Text(TextDropItem::new(
                        item.iter()
                            .map(|(k, v)| (k.to_owned(), v.to_owned()))
                            .collect(),
                    ))
                })
                .collect();
            let (x, y) = center(element);
            on_drop(
                DropEvent {
                    x,
                    y,
                    items,
                    drop_operation: operation,
                },
                item.map(|i| i.target.clone()),
            );
        }
        self.end();
        announce(messages::DROP_COMPLETE, Assertiveness::Assertive);
    }

    fn activate(drop_target: Option<Target>, item: Option<Item>) {
        if let Some(target) = drop_target
            && let (Some(on_activate), Some(element)) = (&target.on_drop_activate, &target.element)
        {
            let (x, y) = center(element);
            on_activate(DropActivateEvent { x, y }, item.map(|i| i.target.clone()));
        }
    }
}

fn dispatch_focusin_on_active_element() {
    if let Some(active) = active_element() {
        let init = web_sys::FocusEventInit::new();
        init.set_bubbles(true);
        if let Ok(event) = web_sys::FocusEvent::new_with_focus_event_init_dict("focusin", &init) {
            let _ = active.dispatch_event(&event);
        }
    }
}

fn find_valid_drop_targets(drag_target: &DragTarget) -> Vec<Target> {
    let types = DragTypes::of_items(&drag_target.items);
    drop_targets()
        .into_iter()
        .filter(|target| {
            let Some(element) = &target.element else {
                return false;
            };
            if is_hidden(element) {
                return false;
            }
            target.get_drop_operation.as_ref().is_none_or(|get| {
                get(&types, &drag_target.allowed_drop_operations) != DropOperation::Cancel
            })
        })
        .collect()
}
