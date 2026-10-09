// Upstream: react-aria/src/toolbar/useToolbar.ts @ 99e6102368
use leptos::{attr, attr::Attr, ev, ev::Capture, prelude::*};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, IntoAttrs, OnEvent,
    hooks::focus::{FocusManager, FocusManagerOptions},
    utils::{
        aria::{AriaOrientation, AriaRole},
        dom_ext::{EventAccessors, node_contains},
        i18n::{WritingDirection, use_direction},
        key::{KeyboardEventKey, KeyboardKey},
        orientation::Orientation,
        shadow_dom::get_active_element,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Restoring the last focused child when focus re-enters the toolbar happens in a microtask:
//   focusing inside a `focus` handler would dispatch a nested `focus` event, which Leptos'
//   handler closures can't take (leptos-and-dom.md, "No Nested Dispatch of the Same Event
//   Type").
//
// =============================================================================

/// Input of [`use_toolbar`].
#[derive(Debug, Clone)]
pub struct UseToolbarInput {
    /// The toolbar element; the hook's props capture it.
    pub element: CapturedElement,
    /// The axis of the arrow keys (react-aria's default: horizontal).
    pub orientation: Signal<Orientation>,
    pub aria_label: MaybeProp<String>,
    /// Ignored when `aria_label` is set.
    pub aria_labelledby: Option<String>,
}

impl Default for UseToolbarInput {
    fn default() -> Self {
        Self {
            element: CapturedElement::new(),
            orientation: Signal::stored(Orientation::Horizontal),
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
        }
    }
}

/// Output of [`use_toolbar`].
#[derive(Debug)]
pub struct UseToolbarReturn {
    /// Props for the toolbar element.
    pub props: UseToolbarProps,
}

/// Props for the toolbar element.
#[derive(Debug)]
pub struct UseToolbarProps {
    /// `toolbar`, or `group` inside another toolbar.
    pub role: Signal<AriaRole>,
    pub aria_orientation: Signal<AriaOrientation>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_focus_capture: EventHandler<FocusEvent>,
    pub on_blur_capture: EventHandler<FocusEvent>,
}

pub type UseToolbarAttrs = (
    Attr<attr::Role, Signal<AriaRole>>,
    Attr<attr::AriaOrientation, Signal<AriaOrientation>>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    ElementCaptureAttr,
    OnEvent<Capture<ev::keydown>>,
    OnEvent<Capture<ev::focus>>,
    OnEvent<Capture<ev::blur>>,
);

impl IntoAttrs for UseToolbarProps {
    type Attrs = UseToolbarAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            self.element_capture,
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
            self.on_focus_capture.into_on(ev::capture(ev::focus)),
            self.on_blur_capture.into_on(ev::capture(ev::blur)),
        )
    }
}

/// Provides the behavior and accessibility of a toolbar: the arrow keys move focus between its
/// focusable children, and the toolbar is one tab stop (Tab leaves it, re-entering restores
/// the child focused last). A toolbar inside another toolbar becomes a `group` of it.
#[allow(clippy::too_many_lines)]
pub fn use_toolbar(input: UseToolbarInput) -> UseToolbarReturn {
    let UseToolbarInput {
        element,
        orientation,
        aria_label,
        aria_labelledby,
    } = input;
    let (is_in_toolbar, set_in_toolbar) = signal(false);
    Effect::new(move || {
        if let Some(el) = element.get() {
            set_in_toolbar.set(
                el.parent_element()
                    .and_then(|parent| parent.closest("[role=\"toolbar\"]").ok().flatten())
                    .is_some(),
            );
        }
    });
    let direction = use_direction();
    // Created once (its state lives in this owner), not per key event.
    let focus_manager = FocusManager::new(move || element.get_untracked().map(|el| (*el).clone()));
    let last_focused: StoredValue<Option<SendWrapper<web_sys::HtmlElement>>> =
        StoredValue::new(None);

    let on_keydown_capture = EventHandler::new(move |e: KeyboardEvent| {
        if is_in_toolbar.get_untracked() {
            return;
        }
        let target = e.expect_target();
        if !node_contains(
            e.expect_current_target().dyn_ref::<web_sys::Node>(),
            target.dyn_ref::<web_sys::Node>(),
        )
        .unwrap_or(false)
        {
            return;
        }
        let orientation = orientation.get_untracked();
        let reverse = direction.get_untracked() == WritingDirection::Rtl
            && orientation == Orientation::Horizontal;
        let (next_key, previous_key) = match orientation {
            Orientation::Horizontal => (KeyboardKey::ArrowRight, KeyboardKey::ArrowLeft),
            Orientation::Vertical => (KeyboardKey::ArrowDown, KeyboardKey::ArrowUp),
        };
        let key = e.typed_key();
        let manager = focus_manager;
        if key == next_key {
            if reverse {
                manager.focus_previous(FocusManagerOptions::default());
            } else {
                manager.focus_next(FocusManagerOptions::default());
            }
        } else if key == previous_key {
            if reverse {
                manager.focus_next(FocusManagerOptions::default());
            } else {
                manager.focus_previous(FocusManagerOptions::default());
            }
        } else if key == KeyboardKey::Tab {
            // Remember where focus was, then let the browser's Tab leave from the toolbar's
            // first or last element.
            let active = leptos_use::use_document()
                .as_ref()
                .and_then(get_active_element)
                .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
            last_focused.set_value(active.map(SendWrapper::new));
            if e.shift_key() {
                manager.focus_first(FocusManagerOptions::default());
            } else {
                manager.focus_last(FocusManagerOptions::default());
            }
            return;
        } else {
            return;
        }
        e.stop_propagation();
        e.prevent_default();
    });

    let on_blur_capture = EventHandler::new(move |e: FocusEvent| {
        if is_in_toolbar.get_untracked() {
            return;
        }
        let leaves = !node_contains(
            e.expect_current_target().dyn_ref::<web_sys::Node>(),
            e.related_target()
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>()),
        )
        .unwrap_or(false);
        if leaves && last_focused.with_value(Option::is_none) {
            let target = e.expect_target().dyn_into::<web_sys::HtmlElement>().ok();
            last_focused.set_value(target.map(SendWrapper::new));
        }
    });

    let on_focus_capture = EventHandler::new(move |e: FocusEvent| {
        if is_in_toolbar.get_untracked() {
            return;
        }
        let from_outside = !node_contains(
            e.expect_current_target().dyn_ref::<web_sys::Node>(),
            e.related_target()
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>()),
        )
        .unwrap_or(false);
        let into_toolbar = element.get_untracked().is_some_and(|toolbar| {
            node_contains(
                Some(toolbar.unchecked_ref::<web_sys::Node>()),
                e.expect_target().dyn_ref::<web_sys::Node>(),
            )
            .unwrap_or(false)
        });
        if from_outside
            && into_toolbar
            && let Some(last) = last_focused.get_value()
        {
            last_focused.set_value(None);
            // Not within this `focus` dispatch (see the deviations).
            queue_microtask(move || {
                let _ = last.focus();
            });
        }
    });

    UseToolbarReturn {
        props: UseToolbarProps {
            role: Signal::derive(move || {
                if is_in_toolbar.get() {
                    AriaRole::Group
                } else {
                    AriaRole::Toolbar
                }
            }),
            aria_orientation: Signal::derive(move || orientation.get().into()),
            aria_label,
            // Only without `aria_label` (react-aria), also when the label changes.
            aria_labelledby: Signal::derive(move || {
                aria_labelledby
                    .clone()
                    .filter(|_| aria_label.read().is_none())
            }),
            element_capture: element.attr(),
            on_keydown_capture,
            on_focus_capture,
            on_blur_capture,
        },
    }
}
