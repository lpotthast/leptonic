// Upstream: react-aria/src/utils/animation.ts @ 99e6102368
//! Hook for tracking CSS enter animations on an element.

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
use send_wrapper::SendWrapper;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns `is_entering: Signal<bool>` (react-aria: a boolean per render).
// - `on_enter` is a `Callback` of the element: it can start Web Animations, which are awaited
//   like CSS ones (react-aria: a function that may return a promise to await as well).
//
// =============================================================================

/// Hides an element preparing for entry, with styles that don't affect layout.
#[cfg(not(feature = "ssr"))]
const HIDING: [(&str, &str); 4] = [
    ("opacity", "0"),
    ("clip", "rect(0 0 0 0)"),
    ("clip-path", "inset(50%)"),
    ("mask-image", "linear-gradient(#0000, #0000)"),
];

/// Input for [`use_enter_animation`].
#[derive(Debug, Clone, Copy)]
pub struct UseEnterAnimationInput {
    /// The element to track enter animations on.
    pub element: CapturedElement,
    /// Delays the entry until ready (e.g. a popover until its placement is known); the element is
    /// hidden until then. Default: always ready.
    pub is_ready: Signal<bool>,
    /// Called with the element when the entry starts (e.g. to start a Web Animation).
    pub on_enter: Option<Callback<SendWrapper<web_sys::Element>>>,
}

/// Return value of [`use_enter_animation`].
#[derive(Debug, Clone, Copy)]
pub struct UseEnterAnimationReturn {
    /// `true` while the element is ready and its enter animations run (for `data-entering`).
    pub is_entering: Signal<bool>,
}

/// Tracks the enter animations of an element: `is_entering` is `true` from the moment it is
/// ready until its animations (CSS animations started by `[data-entering]` styles, transitions
/// from them, Web Animations of `on_enter`) finished.
///
/// While not ready, the element is hidden with styles that don't affect layout, so it doesn't
/// flash (e.g. a popover before its placement is calculated). Transitions that started before it
/// was ready are cancelled.
///
/// ```ignore
/// let element = CapturedElement::new();
/// let UseEnterAnimationReturn { is_entering } =
///     use_enter_animation(UseEnterAnimationInput {
///         element,
///         is_ready: Signal::stored(true),
///         on_enter: None,
///     });
/// // .overlay[data-entering] { animation: fade-in 200ms; }
/// ```
pub fn use_enter_animation(input: UseEnterAnimationInput) -> UseEnterAnimationReturn {
    let UseEnterAnimationInput {
        element,
        is_ready,
        on_enter,
    } = input;

    #[cfg(feature = "ssr")]
    {
        // Nothing animates on the server: the element renders entered.
        let _ = (element, is_ready, on_enter);
        UseEnterAnimationReturn {
            is_entering: Signal::stored(false),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use super::use_animation::watch_animations;

        let (is_entering, set_is_entering) = signal(true);
        let is_animation_ready = Signal::derive(move || is_entering.get() && is_ready.get());

        // Hide the element while it prepares for entry.
        let restore: StoredValue<Option<SendWrapper<Box<dyn FnOnce()>>>> = StoredValue::new(None);
        let unhide = move || {
            if let Some(restore) = restore.try_update_value(Option::take).flatten() {
                (restore.take())();
            }
        };
        Effect::new(move || {
            unhide();
            let Some(el) = element.get() else {
                return;
            };
            if is_ready.get() {
                return;
            }
            let Some(el) = el.dyn_ref::<web_sys::HtmlElement>().cloned() else {
                return;
            };
            let style = el.style();
            let previous: Vec<(&str, String)> = HIDING
                .iter()
                .map(|(property, value)| {
                    let previous = style.get_property_value(property).unwrap_or_default();
                    let _ = style.set_property(property, value);
                    (*property, previous)
                })
                .collect();
            let undo: Box<dyn FnOnce()> = Box::new(move || {
                let style = el.style();
                for (property, previous) in previous {
                    let _ = if previous.is_empty() {
                        style.remove_property(property).map(|_| ())
                    } else {
                        style.set_property(property, &previous)
                    };
                }
            });
            restore.set_value(Some(SendWrapper::new(undo)));
        });

        let cancel: StoredValue<Option<SendWrapper<Box<dyn FnOnce()>>>> = StoredValue::new(None);
        let cancel_watch = move || {
            if let Some(cancel) = cancel.try_update_value(Option::take).flatten() {
                (cancel.take())();
            }
        };
        Effect::new(move || {
            cancel_watch();
            if !is_animation_ready.get() {
                return;
            }
            let Some(el) = element.get() else {
                return;
            };
            // Transitions that started before the entry (e.g. while not ready) restart with it:
            // the entering styles apply now.
            let animations = el.get_animations();
            for i in 0..animations.length() {
                if let Some(transition) = animations.get(i).dyn_ref::<web_sys::CssTransition>() {
                    transition.cancel();
                }
            }
            let stop: Box<dyn FnOnce()> = Box::new(watch_animations(&el, on_enter, move || {
                set_is_entering.set(false);
            }));
            cancel.set_value(Some(SendWrapper::new(stop)));
        });

        on_cleanup(move || {
            cancel_watch();
            unhide();
        });

        UseEnterAnimationReturn {
            is_entering: is_animation_ready,
        }
    }
}
