//! Hook for tracking CSS enter animations on an element.
//!
//! Based on: <https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/animation.ts>

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Uses `CapturedElement` instead of React's `RefObject<HTMLElement>`.
// - Uses `spawn_local` + `JsFuture` instead of React's `useLayoutEffect` cleanup.
// - No `flushSync` equivalent needed — Leptos signals update synchronously.
//
// ## DIFFERENT BEHAVIOR
//
// - Returns `UseEnterAnimationReturn` struct with `is_entering: Signal<bool>`
//   instead of React-aria's direct boolean return. The signal combines internal
//   entering state with `is_ready`, matching react-aria's `isAnimationReady`.
//

/// Input for [`use_enter_animation`].
pub struct UseEnterAnimationInput {
    /// The element to track enter animations on.
    pub element: CapturedElement,

    /// Delays animation tracking until ready (e.g., popover placement calculated).
    /// Default: always ready.
    pub is_ready: Signal<bool>,
}

/// Return value of [`use_enter_animation`].
pub struct UseEnterAnimationReturn {
    /// `true` while CSS enter animation is in flight; `false` when done or no animations.
    pub is_entering: Signal<bool>,
}

/// Tracks CSS enter animations on an element.
///
/// Returns `is_entering: true` while enter animations are in flight, and `false`
/// once they complete (or if no animations are present). Pre-existing CSS
/// transitions are cancelled before tracking begins.
///
/// Use `is_ready` to delay animation tracking until preconditions are met (e.g.,
/// popover placement has been calculated). The animation is not tracked until
/// both the element is captured and `is_ready` is `true`.
///
/// # Example
///
/// ```ignore
/// let element = CapturedElement::new();
///
/// let UseEnterAnimationReturn { is_entering } = use_enter_animation(UseEnterAnimationInput {
///     element,
///     is_ready: Signal::derive(|| true),
/// });
///
/// // Use `is_entering` as a `data-entering` attribute for CSS targeting:
/// // .overlay[data-entering] { animation: fade-in 200ms; }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_enter_animation(input: UseEnterAnimationInput) -> UseEnterAnimationReturn {
    let UseEnterAnimationInput { element, is_ready } = input;

    #[cfg(feature = "ssr")]
    {
        // During SSR, there are no animations — always report as not entering.
        let _ = (element, is_ready);
        UseEnterAnimationReturn {
            is_entering: Signal::derive(|| false),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use super::use_animation::watch_animations;
        use wasm_bindgen::JsCast;

        let (is_entering_raw, set_is_entering) = signal(true);

        let cleanup: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            // Clean up previous watcher.
            cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });

            let is_animation_ready = is_entering_raw.get() && is_ready.get();

            if is_animation_ready {
                if let Some(el) = element.get() {
                    // Cancel pre-existing CSS transitions before tracking animations.
                    let animations = el.get_animations();
                    for i in 0..animations.length() {
                        if let Some(transition) =
                            animations.get(i).dyn_ref::<web_sys::CssTransition>()
                        {
                            transition.cancel();
                        }
                    }

                    // Watch for all remaining animations to complete.
                    let cancel = watch_animations(&el, move || {
                        set_is_entering.set(false);
                    });
                    cleanup.set_value(Some(Box::new(cancel)));
                }
            }
        });

        on_cleanup(move || {
            cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });
        });

        // Combine entering state with readiness to match react-aria's behavior:
        // the hook reports "entering" only while both conditions hold.
        let is_entering = Signal::derive(move || is_entering_raw.get() && is_ready.get());

        UseEnterAnimationReturn { is_entering }
    }
}
