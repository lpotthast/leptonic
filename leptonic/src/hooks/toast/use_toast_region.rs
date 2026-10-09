// Upstream: react-aria/src/toast/useToastRegion.ts @ 99e6102368
// Upstream: react-aria/test/toast/useToast.test.js @ 99e6102368
use leptos::{
    attr::custom::{CustomAttr, custom_attribute},
    ev::{self},
    prelude::*,
};
use send_wrapper::SendWrapper;
use web_sys::{FocusEvent, PointerEvent};

use super::use_toast_state::{ToastKey, ToastQueue};
use crate::{
    CapturedElement, EventHandler, IntoAttrs, OnEvent,
    hooks::{
        focus::{FocusWithinEvent, Modality, UseFocusWithinInput, get_modality, use_focus_within},
        interactions::{UseHoverInput, use_hover},
        landmark::{LandmarkRole, UseLandmarkInput, UseLandmarkProps, use_landmark},
    },
    utils::intl_strings::{ToastStrings, use_localized_strings},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The region is marked `data-leptonic-top-layer` (react-aria: `data-react-aria-top-layer`).
// - The queue and the region's element go into the input (C8; react-aria: `state` and `ref`).
//
// ## DIFFERENT BEHAVIOR
// - The focused toast is tracked by its key (react-aria: its index), so a toast added while
//   another is focused doesn't make the region lose it.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Removing the focused toast blurs it (React suppresses events while it commits, so
//   react-aria never sees that blur): a blur without a related target is checked a microtask
//   later, and a blur of a removed element keeps the state the focus is moved with. Where the
//   focus then goes, the state follows (react-aria: its global focus listener).
//
// =============================================================================

/// Input of [`use_toast_region`].
#[derive(Clone)]
pub struct UseToastRegionInput<T: Clone + Send + Sync + 'static> {
    /// The toasts to show.
    pub queue: ToastQueue<T>,
    /// The region's element (captured by the caller).
    pub element: CapturedElement,
    /// Default: "1 notification." / "2 notifications.".
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
}

/// Return value of [`use_toast_region`].
#[derive(Debug)]
pub struct UseToastRegionReturn {
    pub region_props: UseToastRegionProps,
}

/// Props of a toast region.
#[derive(Debug, Clone)]
pub struct UseToastRegionProps {
    pub landmark: UseLandmarkProps,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseToastRegionProps {
    type Attrs = (
        <UseLandmarkProps as IntoAttrs>::Attrs,
        CustomAttr<&'static str, &'static str>,
        OnEvent<ev::pointerenter>,
        OnEvent<ev::pointerleave>,
        OnEvent<ev::focusin>,
        OnEvent<ev::focusout>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            self.landmark.into_attrs(),
            // Not hidden when an overlay opens, focusable outside a focus scope, not dismissing
            // overlays when pressed.
            custom_attribute("data-leptonic-top-layer", "true"),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Focuses an element as the user would (keyboard) or without scrolling (pointer).
fn restore_focus(element: &web_sys::Element) {
    crate::utils::focus::focus_element(element, get_modality() == Some(Modality::Pointer));
}

/// Behavior and accessibility of the region showing toasts (react-aria's `useToastRegion`): a
/// landmark (F6 reaches it) whose timeouts pause while it is hovered or focused; a closing
/// focused toast hands the focus to the next one, and the focus returns to where it came from
/// when the last one closes.
#[allow(clippy::too_many_lines)]
pub fn use_toast_region<T: Clone + Send + Sync + 'static>(
    input: UseToastRegionInput<T>,
) -> UseToastRegionReturn {
    let UseToastRegionInput {
        queue,
        element,
        aria_label,
        aria_labelledby,
    } = input;
    let strings = use_localized_strings::<ToastStrings>();
    let label = Signal::derive(move || {
        aria_label.get().or_else(|| {
            let count = queue.visible_toasts.with(Vec::len);
            Some(strings.read().notifications(count))
        })
    });
    let landmark = use_landmark(UseLandmarkInput {
        element,
        aria_label: label.into(),
        aria_labelledby,
        role: LandmarkRole::Region,
        focus: None,
    });

    let is_hovered = StoredValue::new(false);
    let is_focused = StoredValue::new(false);
    let update_timers = move || {
        if is_hovered.get_value() || is_focused.get_value() {
            queue.pause_all();
        } else {
            queue.resume_all();
        }
    };
    let hover = use_hover(UseHoverInput {
        on_hover_start: Some(Callback::new(move |_| {
            is_hovered.set_value(true);
            update_timers();
        })),
        on_hover_end: Some(Callback::new(move |_| {
            is_hovered.set_value(false);
            update_timers();
        })),
        ..UseHoverInput::default()
    });

    // Where the focus came from, to return it to. Thread-safe storage: the hook may run during
    // server-side rendering.
    let last_focused = StoredValue::new(None::<SendWrapper<web_sys::Element>>);
    let leave = move || {
        // After the region's disposal ("Blur After Disposal").
        if is_focused.try_get_value().is_none() {
            return;
        }
        is_focused.set_value(false);
        last_focused.set_value(None);
        update_timers();
    };
    let focus_within =
        use_focus_within(UseFocusWithinInput {
            on_focus_within: Some(Callback::new(move |e: FocusWithinEvent| {
                let came_from = e.event.related_target().and_then(|target| {
                    wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target).ok()
                });
                // Coming from nowhere after a focused toast was removed: still from where it came
                // before.
                if !(came_from.is_none() && is_focused.get_value()) {
                    last_focused.set_value(came_from.map(SendWrapper::new));
                }
                is_focused.set_value(true);
                update_timers();
            })),
            on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
                if e.event.related_target().is_some() {
                    leave();
                    return;
                }
                let Some(target) = e.event.target().and_then(|target| {
                    wasm_bindgen::JsCast::dyn_into::<web_sys::Node>(target).ok()
                }) else {
                    leave();
                    return;
                };
                // A removed toast's blur (see above) keeps the state.
                queue_microtask(move || {
                    if target.is_connected() {
                        leave();
                    }
                });
            })),
            ..UseFocusWithinInput::default()
        });

    // The keys of the toasts as rendered last (newest first), and the key of the focused toast
    // (`None`: none has the focus).
    let previous = StoredValue::new(Vec::<ToastKey>::new());
    let focused_toast = StoredValue::new(None::<ToastKey>);
    let query_toasts = move || -> Vec<web_sys::Element> {
        let Some(region) = element.get_untracked() else {
            return Vec::new();
        };
        let Ok(toasts) = region.query_selector_all("[role=alertdialog]") else {
            return Vec::new();
        };
        (0..toasts.length())
            .filter_map(|index| toasts.item(index))
            .filter_map(|node| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(node).ok())
            .collect()
    };
    let on_focusin = EventHandler::new(move |e: FocusEvent| {
        let toast = crate::utils::shadow_dom::get_event_target(&e)
            .and_then(|target| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target).ok())
            .and_then(|target| target.closest("[role=alertdialog]").ok().flatten());
        // The rendered toasts are in the order of the visible toasts as processed last.
        let key = toast
            .and_then(|toast| query_toasts().iter().position(|element| *element == toast))
            .and_then(|index| previous.with_value(|keys| keys.get(index).copied()));
        focused_toast.set_value(key);
    });
    let on_focusout = EventHandler::new(move |e: FocusEvent| {
        let removed = e.related_target().is_none().then(|| {
            e.target()
                .and_then(|target| wasm_bindgen::JsCast::dyn_into::<web_sys::Node>(target).ok())
        });
        match removed.flatten() {
            // Maybe a removed toast's blur (see above): it keeps the index.
            Some(target) => queue_microtask(move || {
                if target.is_connected() {
                    let _ = focused_toast.try_set_value(None);
                }
            }),
            None => {
                let _ = focused_toast.try_set_value(None);
            }
        }
    });

    // A closing focused toast hands the focus to the next newer one (else the next older one);
    // with a pointer, the focus leaves the region (the timeouts would seem stuck).
    Effect::new(move |_| {
        let visible: Vec<ToastKey> = queue
            .visible_toasts
            .with(|toasts| toasts.iter().map(|toast| toast.key).collect());
        let previous_visible = previous.get_value();
        previous.set_value(visible.clone());
        if visible.is_empty() || previous_visible == visible {
            return;
        }
        let removed: Vec<bool> = previous_visible
            .iter()
            .map(|key| !visible.contains(key))
            .collect();
        let Some(removed_index) = focused_toast
            .get_value()
            .and_then(|key| {
                previous_visible
                    .iter()
                    .position(|previous| *previous == key)
            })
            .filter(|index| removed.get(*index).copied().unwrap_or(false))
        else {
            return;
        };
        focused_toast.set_value(None);
        if get_modality() == Some(Modality::Pointer)
            && let Some(last) = last_focused.get_value().filter(|last| last.is_connected())
        {
            restore_focus(&last);
            leave();
            return;
        }
        let mut previous_toast = None;
        let mut next_toast = None;
        let mut index = 0;
        while index <= removed_index {
            if !removed[index] {
                previous_toast = Some(index.saturating_sub(1));
            }
            index += 1;
        }
        while index < removed.len() {
            if !removed[index] {
                next_toast = Some(index - 1);
                break;
            }
            index += 1;
        }
        // One toast at a time: the new one is at index 0.
        if previous_toast.is_none() && next_toast.is_none() {
            previous_toast = Some(0);
        }
        // The toasts once rendered without the removed one.
        request_animation_frame(move || {
            // The region may be gone by then ("Blur After Disposal").
            if focused_toast.try_get_value().is_none() {
                return;
            }
            let toasts = query_toasts();
            if let Some(toast) = previous_toast
                .and_then(|index| toasts.get(index))
                .or_else(|| next_toast.and_then(|index| toasts.get(index)))
            {
                crate::utils::focus::focus_element(toast, true);
            }
        });
    });

    // Without toasts (or when the region goes away), the focus returns to where it came from.
    Effect::new(move |_| {
        if queue.visible_toasts.with(Vec::is_empty)
            && let Some(last) = last_focused.get_value().filter(|last| last.is_connected())
        {
            restore_focus(&last);
            leave();
        }
    });
    on_cleanup(move || {
        if let Some(Some(last)) = last_focused.try_get_value()
            && last.is_connected()
        {
            restore_focus(&last);
        }
    });

    UseToastRegionReturn {
        region_props: UseToastRegionProps {
            landmark: UseLandmarkProps {
                // Focusable by script (landmark navigation, restoring the focus).
                tabindex: Signal::stored(Some(-1)),
                ..landmark.props
            },
            on_pointerenter: hover.props.on_pointerenter,
            on_pointerleave: hover.props.on_pointerleave,
            on_focusin: focus_within.props.on_focusin.chain(on_focusin),
            on_focusout: focus_within.props.on_focusout.chain(on_focusout),
        },
    }
}
