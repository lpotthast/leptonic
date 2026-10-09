// Upstream: react-aria/src/landmark/useLandmark.ts @ 99e6102368
// Upstream: react-aria/test/landmark/useLandmark.test.tsx @ 99e6102368
// Upstream: react-aria/test/landmark/useLandmark.ssr.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{CapturedElement, IntoAttrs, utils::aria::AriaRole};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One landmark manager per wasm module (react-aria: a versioned singleton attached to the
//   document, shared by copies of react-aria on a page).
// - `LandmarkController` disposes itself when dropped (react-aria: `dispose()`).
// - The landmark's element goes into the input (C8; react-aria: `ref`).
//
// =============================================================================

/// The roles of landmarks (react-aria's `AriaLandmarkRole`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LandmarkRole {
    Main,
    Region,
    Search,
    Navigation,
    Form,
    Banner,
    Contentinfo,
    Complementary,
}

impl LandmarkRole {
    pub fn aria_role(self) -> AriaRole {
        match self {
            Self::Main => AriaRole::Main,
            Self::Region => AriaRole::Region,
            Self::Search => AriaRole::Search,
            Self::Navigation => AriaRole::Navigation,
            Self::Form => AriaRole::Form,
            Self::Banner => AriaRole::Banner,
            Self::Contentinfo => AriaRole::Contentinfo,
            Self::Complementary => AriaRole::Complementary,
        }
    }
}

/// The direction of landmark navigation (F6: forward, Shift+F6: backward).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LandmarkDirection {
    Forward,
    Backward,
}

#[cfg(not(feature = "ssr"))]
mod manager {
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::{JsCast, closure::Closure};

    use super::{LandmarkDirection, LandmarkRole};
    use crate::utils::shadow_dom::get_event_target;

    pub(super) struct Landmark {
        pub id: u64,
        pub element: web_sys::Element,
        pub role: LandmarkRole,
        pub label: Option<String>,
        pub last_focused: Option<web_sys::Element>,
        pub focus: Rc<dyn Fn(LandmarkDirection)>,
        pub blur: Rc<dyn Fn()>,
    }

    type Listener = Closure<dyn FnMut(web_sys::Event)>;

    #[derive(Default)]
    struct Manager {
        landmarks: Vec<Landmark>,
        listeners: Option<[Listener; 3]>,
        controllers: usize,
        next_id: u64,
    }

    thread_local! {
        static MANAGER: RefCell<Manager> = RefCell::new(Manager::default());
    }

    fn document() -> Option<web_sys::Document> {
        leptos_use::use_document().as_ref().cloned()
    }

    fn capture_options() -> web_sys::AddEventListenerOptions {
        let options = web_sys::AddEventListenerOptions::new();
        options.set_capture(true);
        options
    }

    /// Listens on the document while landmarks or controllers exist.
    fn setup_if_needed() {
        let needs_setup = MANAGER.with_borrow(|manager| manager.listeners.is_none());
        let Some(document) = document().filter(|_| needs_setup) else {
            return;
        };
        let keydown: Listener = Closure::new(|e: web_sys::Event| {
            if let Some(e) = e.dyn_ref::<web_sys::KeyboardEvent>() {
                on_keydown(e);
            }
        });
        let focusin: Listener = Closure::new(|e: web_sys::Event| {
            if let Some(e) = e.dyn_ref::<web_sys::FocusEvent>() {
                on_focusin(e);
            }
        });
        let focusout: Listener = Closure::new(|e: web_sys::Event| {
            if let Some(e) = e.dyn_ref::<web_sys::FocusEvent>() {
                on_focusout(e);
            }
        });
        for (event, listener) in [
            ("keydown", &keydown),
            ("focusin", &focusin),
            ("focusout", &focusout),
        ] {
            let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
                event,
                listener.as_ref().unchecked_ref(),
                &capture_options(),
            );
        }
        MANAGER.with_borrow_mut(|manager| manager.listeners = Some([keydown, focusin, focusout]));
    }

    fn teardown_if_needed() {
        let listeners = MANAGER.with_borrow_mut(|manager| {
            if manager.landmarks.is_empty() && manager.controllers == 0 {
                manager.listeners.take()
            } else {
                None
            }
        });
        let (Some(listeners), Some(document)) = (listeners, document()) else {
            return;
        };
        for (event, listener) in ["keydown", "focusin", "focusout"]
            .into_iter()
            .zip(&listeners)
        {
            let _ = document.remove_event_listener_with_callback_and_bool(
                event,
                listener.as_ref().unchecked_ref(),
                true,
            );
        }
    }

    /// Warns about landmarks of one role that can't be told apart (react-aria's `checkLabels`),
    /// and about more than one main landmark. Checked a microtask later, for the landmarks still
    /// in the document: during a route change, the new page's landmarks register before the old
    /// page's unregister (React runs the old page's cleanups first).
    fn check_labels(role: LandmarkRole) {
        leptos::prelude::queue_microtask(move || {
            MANAGER.with_borrow(|manager| check_connected_labels(&manager.landmarks, role));
        });
    }

    fn check_connected_labels(landmarks: &[Landmark], role: LandmarkRole) {
        let same_role: Vec<&Landmark> = landmarks
            .iter()
            .filter(|landmark| landmark.role == role && landmark.element.is_connected())
            .collect();
        if same_role.len() <= 1 {
            return;
        }
        if role == LandmarkRole::Main {
            crate::utils::dev_warn!(
                "The page can contain no more than one landmark with the role main."
            );
        }
        if same_role.iter().any(|landmark| landmark.label.is_none()) {
            crate::utils::dev_warn!(
                "The page has more than one landmark with the role {role:?}: label each (aria-label or aria-labelledby)."
            );
        } else {
            let labels: Vec<&Option<String>> =
                same_role.iter().map(|landmark| &landmark.label).collect();
            for (index, label) in labels.iter().enumerate() {
                if labels[..index].contains(label) {
                    crate::utils::dev_warn!(
                        "The page has more than one landmark with the role {role:?} and the label {label:?}: label them uniquely."
                    );
                }
            }
        }
    }

    /// Registers a landmark (in document order), or updates it. Returns its id.
    pub(super) fn register(mut landmark: Landmark) -> u64 {
        setup_if_needed();
        MANAGER.with_borrow_mut(|manager| {
            if let Some(existing) = manager
                .landmarks
                .iter_mut()
                .find(|existing| existing.element == landmark.element)
            {
                existing.role = landmark.role;
                existing.label = landmark.label.take();
                existing.focus = landmark.focus;
                existing.blur = landmark.blur;
                let (id, role) = (existing.id, existing.role);
                check_labels(role);
                return id;
            }
            manager.next_id += 1;
            landmark.id = manager.next_id;
            let id = landmark.id;
            let role = landmark.role;
            // In document order: after the landmarks that precede it or contain it.
            let index = manager.landmarks.partition_point(|existing| {
                let position = landmark
                    .element
                    .compare_document_position(&existing.element);
                position
                    & (web_sys::Node::DOCUMENT_POSITION_PRECEDING
                        | web_sys::Node::DOCUMENT_POSITION_CONTAINS)
                    != 0
            });
            manager.landmarks.insert(index, landmark);
            check_labels(role);
            id
        })
    }

    pub(super) fn update_label(id: u64, label: Option<String>) {
        MANAGER.with_borrow_mut(|manager| {
            if let Some(landmark) = manager
                .landmarks
                .iter_mut()
                .find(|landmark| landmark.id == id)
            {
                landmark.label = label;
                let role = landmark.role;
                check_labels(role);
            }
        });
    }

    pub(super) fn unregister(id: u64) {
        MANAGER.with_borrow_mut(|manager| manager.landmarks.retain(|landmark| landmark.id != id));
        teardown_if_needed();
    }

    /// The landmark containing `element` (its id and element).
    fn closest(element: &web_sys::Element) -> Option<(u64, web_sys::Element)> {
        MANAGER.with_borrow(|manager| {
            let mut current = Some(element.clone());
            let body = document().and_then(|document| document.body());
            while let Some(candidate) = current {
                if let Some(landmark) = manager
                    .landmarks
                    .iter()
                    .find(|landmark| landmark.element == candidate)
                {
                    return Some((landmark.id, landmark.element.clone()));
                }
                if body.as_ref().is_some_and(|body| **body == candidate) {
                    return None;
                }
                current = candidate.parent_element();
            }
            None
        })
    }

    /// Asks the page whether navigation may wrap at the end of the landmarks (an app can cancel
    /// the `react-aria-landmark-navigation` event, e.g. to go on in another frame).
    fn may_wrap(from: &web_sys::Element, direction: LandmarkDirection) -> bool {
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &detail,
            &"direction".into(),
            &(if direction == LandmarkDirection::Backward {
                "backward"
            } else {
                "forward"
            })
            .into(),
        );
        let init = web_sys::CustomEventInit::new();
        init.set_bubbles(true);
        init.set_cancelable(true);
        init.set_detail(&detail);
        web_sys::CustomEvent::new_with_event_init_dict("react-aria-landmark-navigation", &init)
            .ok()
            .is_none_or(|event| from.dispatch_event(&event).unwrap_or(true))
    }

    /// The next landmark from `from` (the landmark containing it, else the first or last),
    /// skipping hidden ones; `None` at the end if the page cancels wrapping.
    fn next_landmark(from: &web_sys::Element, direction: LandmarkDirection) -> Option<u64> {
        let backward = direction == LandmarkDirection::Backward;
        let current = closest(from).map(|(id, _)| id);
        let ids: Vec<(u64, web_sys::Element)> = MANAGER.with_borrow(|manager| {
            manager
                .landmarks
                .iter()
                .map(|landmark| (landmark.id, landmark.element.clone()))
                .collect()
        });
        let count = ids.len().cast_signed();
        if count == 0 {
            return None;
        }
        let mut index =
            match current.and_then(|current| ids.iter().position(|(id, _)| *id == current)) {
                Some(position) => position.cast_signed() + if backward { -1 } else { 1 },
                None if backward => count - 1,
                None => 0,
            };
        let wrap = |index: &mut isize| -> bool {
            if *index < 0 {
                if !may_wrap(from, LandmarkDirection::Backward) {
                    return false;
                }
                *index = count - 1;
            } else if *index >= count {
                if !may_wrap(from, LandmarkDirection::Forward) {
                    return false;
                }
                *index = 0;
            }
            true
        };
        if !wrap(&mut index) {
            return None;
        }
        let start = index;
        while ids[index.cast_unsigned()]
            .1
            .closest("[aria-hidden=true], [inert]")
            .ok()
            .flatten()
            .is_some()
        {
            index += if backward { -1 } else { 1 };
            if !wrap(&mut index) {
                return None;
            }
            if index == start {
                break;
            }
        }
        Some(ids[index.cast_unsigned()].0)
    }

    /// Moves to the next landmark: to what had the focus there last, else to the landmark.
    pub(super) fn navigate(from: &web_sys::Element, direction: LandmarkDirection) -> bool {
        let Some(id) = next_landmark(from, direction) else {
            return false;
        };
        let target = MANAGER.with_borrow(|manager| {
            manager
                .landmarks
                .iter()
                .find(|landmark| landmark.id == id)
                .map(|landmark| {
                    (
                        landmark.last_focused.clone(),
                        landmark.element.clone(),
                        Rc::clone(&landmark.focus),
                    )
                })
        });
        let Some((last_focused, element, focus)) = target else {
            return false;
        };
        if let Some(last_focused) = last_focused.filter(|element| element.is_connected())
            && let Some(last_focused) = last_focused.dyn_ref::<web_sys::HtmlElement>()
        {
            let _ = last_focused.focus();
            return true;
        }
        if element.is_connected() {
            focus(direction);
            return true;
        }
        false
    }

    pub(super) fn focus_main() -> bool {
        let main = MANAGER.with_borrow(|manager| {
            manager
                .landmarks
                .iter()
                .find(|landmark| landmark.role == LandmarkRole::Main)
                .map(|landmark| (landmark.element.clone(), Rc::clone(&landmark.focus)))
        });
        match main {
            Some((element, focus)) if element.is_connected() => {
                focus(LandmarkDirection::Forward);
                true
            }
            _ => false,
        }
    }

    fn on_keydown(e: &web_sys::KeyboardEvent) {
        if e.key() != "F6" {
            return;
        }
        let handled = if e.alt_key() {
            focus_main()
        } else {
            get_event_target(e)
                .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|from| {
                    navigate(
                        &from,
                        if e.shift_key() {
                            LandmarkDirection::Backward
                        } else {
                            LandmarkDirection::Forward
                        },
                    )
                })
        };
        if handled {
            e.prevent_default();
            e.stop_propagation();
        }
    }

    /// Remembers what has the focus in a landmark; blurs a landmark the focus left.
    fn on_focusin(e: &web_sys::FocusEvent) {
        let target =
            get_event_target(e).and_then(|target| target.dyn_into::<web_sys::Element>().ok());
        if let Some(target) = target
            && let Some((id, element)) = closest(&target)
            && element != target
        {
            MANAGER.with_borrow_mut(|manager| {
                if let Some(landmark) = manager
                    .landmarks
                    .iter_mut()
                    .find(|landmark| landmark.id == id)
                {
                    landmark.last_focused = Some(target);
                }
            });
        }
        let previous = e
            .related_target()
            .and_then(|previous| previous.dyn_into::<web_sys::Element>().ok());
        blur_if_landmark(previous.as_ref());
    }

    fn on_focusout(e: &web_sys::FocusEvent) {
        if e.related_target().is_none() {
            let previous =
                get_event_target(e).and_then(|target| target.dyn_into::<web_sys::Element>().ok());
            blur_if_landmark(previous.as_ref());
        }
    }

    /// A landmark that had the focus itself (not a descendant) blurs.
    fn blur_if_landmark(previous: Option<&web_sys::Element>) {
        let Some(previous) = previous else {
            return;
        };
        let blur = closest(previous)
            .filter(|(_, element)| element == previous)
            .and_then(|(id, _)| {
                MANAGER.with_borrow(|manager| {
                    manager
                        .landmarks
                        .iter()
                        .find(|landmark| landmark.id == id)
                        .map(|landmark| Rc::clone(&landmark.blur))
                })
            });
        if let Some(blur) = blur {
            blur();
        }
    }

    pub(super) fn add_controller() {
        MANAGER.with_borrow_mut(|manager| manager.controllers += 1);
        setup_if_needed();
    }

    pub(super) fn remove_controller() {
        MANAGER
            .with_borrow_mut(|manager| manager.controllers = manager.controllers.saturating_sub(1));
        teardown_if_needed();
    }

    pub(super) fn active_element() -> Option<web_sys::Element> {
        document()
            .as_ref()
            .and_then(crate::utils::shadow_dom::get_active_element)
    }
}

/// Moves between landmarks from code (react-aria's `UNSTABLE_createLandmarkController`), e.g.
/// for a "skip to content" button. Dropping it stops it.
pub struct LandmarkController {
    _private: (),
}

impl LandmarkController {
    pub fn new() -> Self {
        #[cfg(not(feature = "ssr"))]
        manager::add_controller();
        Self { _private: () }
    }

    /// To the next (or previous) landmark from `from` (default: the focused element).
    pub fn navigate(&self, direction: LandmarkDirection, from: Option<web_sys::Element>) -> bool {
        #[cfg(not(feature = "ssr"))]
        {
            from.or_else(manager::active_element)
                .is_some_and(|from| manager::navigate(&from, direction))
        }
        #[cfg(feature = "ssr")]
        {
            let _ = (direction, from);
            false
        }
    }

    pub fn focus_next(&self, from: Option<web_sys::Element>) -> bool {
        self.navigate(LandmarkDirection::Forward, from)
    }

    pub fn focus_previous(&self, from: Option<web_sys::Element>) -> bool {
        self.navigate(LandmarkDirection::Backward, from)
    }

    /// To the main landmark.
    pub fn focus_main(&self) -> bool {
        #[cfg(not(feature = "ssr"))]
        {
            manager::focus_main()
        }
        #[cfg(feature = "ssr")]
        {
            false
        }
    }
}

impl Default for LandmarkController {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LandmarkController {
    fn drop(&mut self) {
        #[cfg(not(feature = "ssr"))]
        manager::remove_controller();
    }
}

/// Input of [`use_landmark`].
#[derive(Clone)]
pub struct UseLandmarkInput {
    /// The landmark's element (captured by the caller).
    pub element: CapturedElement,
    pub role: LandmarkRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// How the landmark takes the focus when navigated to. Default: it focuses itself.
    pub focus: Option<Callback<LandmarkDirection>>,
}

/// Return value of [`use_landmark`].
#[derive(Debug)]
pub struct UseLandmarkReturn {
    pub props: UseLandmarkProps,
}

/// Props of a landmark.
#[derive(Debug, Clone)]
pub struct UseLandmarkProps {
    pub role: AriaRole,
    /// While navigated to: focusable, to take the focus.
    pub tabindex: Signal<Option<i32>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
}

impl IntoAttrs for UseLandmarkProps {
    type Attrs = (
        Attr<attr::Role, AriaRole>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, Option<String>>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// A landmark (react-aria's `useLandmark`): a region of the page that F6 and Shift+F6 move
/// between (Alt+F6 to the main one), returning to what had the focus there last.
pub fn use_landmark(input: UseLandmarkInput) -> UseLandmarkReturn {
    let UseLandmarkInput {
        element,
        role,
        aria_label,
        aria_labelledby,
        focus,
    } = input;
    let is_focused = RwSignal::new(false);
    #[cfg(feature = "ssr")]
    let _ = (element, focus);

    #[cfg(not(feature = "ssr"))]
    {
        use std::rc::Rc;

        // The registered landmark: its id and element (react-aria registers the ref, so it follows
        // `ref.current`; here a re-created element registers again).
        let registration = StoredValue::new_local(None::<(u64, web_sys::Element)>);
        let labelledby = aria_labelledby.clone();
        let label = move || aria_label.get().or_else(|| labelledby.clone());
        Effect::new(move |_| {
            let label = label();
            let Some(landmark_element) = element.get() else {
                if let Some((id, _)) = registration.get_value() {
                    manager::unregister(id);
                    registration.set_value(None);
                }
                return;
            };
            if let Some((id, registered)) = registration.get_value() {
                if registered == *landmark_element {
                    manager::update_label(id, label);
                    return;
                }
                manager::unregister(id);
            }
            let focus_landmark: Rc<dyn Fn(LandmarkDirection)> = match focus {
                Some(focus) => Rc::new(move |direction| {
                    let _ = focus.try_run(direction);
                }),
                None => Rc::new(move |_| {
                    is_focused.try_set(true);
                }),
            };
            let id = manager::register(manager::Landmark {
                id: 0,
                element: (*landmark_element).clone(),
                role,
                label,
                last_focused: None,
                focus: focus_landmark,
                blur: Rc::new(move || {
                    is_focused.try_set(false);
                }),
            });
            registration.set_value(Some((id, (*landmark_element).clone())));
        });
        on_cleanup(move || {
            if let Some(Some((id, _))) = registration.try_get_value() {
                manager::unregister(id);
            }
        });
        // Navigated to: it takes the focus.
        Effect::new(move |_| {
            if is_focused.get()
                && let Some(landmark_element) = element.get_untracked()
            {
                crate::utils::focus::focus_element(&landmark_element, false);
            }
        });
    }

    UseLandmarkReturn {
        props: UseLandmarkProps {
            role: role.aria_role(),
            tabindex: Signal::derive(move || is_focused.get().then_some(-1)),
            aria_label,
            aria_labelledby,
        },
    }
}
