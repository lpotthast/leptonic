//! Hook for tracking CSS exit animations on an element.
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
// - Exposes `exit_state: Signal<ExitState>` in addition to `is_exiting: Signal<bool>`
//   for consumers that need fine-grained state machine access.
//

/// Exit animation state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitState {
    /// The element is logically open (visible).
    Open,
    /// Exit animation is in flight — element should remain in the DOM.
    Exiting,
    /// Exit animation completed — element can be removed from the DOM.
    Closed,
}

/// Input for [`use_exit_animation`].
pub struct UseExitAnimationInput {
    /// The element to track exit animations on.
    pub element: CapturedElement,

    /// Logically open state. When this goes `false`, the exit animation begins.
    pub is_open: Signal<bool>,
}

/// Return value of [`use_exit_animation`].
pub struct UseExitAnimationReturn {
    /// `true` while exit animation is in flight. Keep the element in the DOM while `true`.
    pub is_exiting: Signal<bool>,

    /// Full state machine for fine-grained control.
    pub exit_state: Signal<ExitState>,
}

/// Tracks CSS exit animations on an element.
///
/// When `is_open` transitions from `true` to `false`, the hook enters the
/// `Exiting` state and watches for all active CSS animations on the element to
/// complete. Once they do, it transitions to `Closed`. If `is_open` goes back
/// to `true` during the exit animation, the animation is interrupted and the
/// state returns to `Open`.
///
/// Consumers should keep the element in the DOM while `is_exiting` is `true`
/// (or while `exit_state` is not `Closed`), and apply exit animation CSS via
/// a `data-exiting` attribute.
///
/// # Example
///
/// ```ignore
/// let element = CapturedElement::new();
///
/// let UseExitAnimationReturn { is_exiting, exit_state } =
///     use_exit_animation(UseExitAnimationInput {
///         element,
///         is_open: state.is_open,
///     });
///
/// // Keep element mounted while exiting:
/// // <Show when=move || state.is_open.get() || is_exiting.get()>
/// //     <div {..attrs} data-exiting=is_exiting />
/// // </Show>
/// //
/// // CSS: .overlay[data-exiting] { animation: fade-out 150ms; }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_exit_animation(input: UseExitAnimationInput) -> UseExitAnimationReturn {
    let UseExitAnimationInput { element, is_open } = input;

    #[cfg(feature = "ssr")]
    {
        // During SSR, no animations — state directly follows is_open.
        let _ = element;
        UseExitAnimationReturn {
            is_exiting: Signal::derive(|| false),
            exit_state: Signal::derive(move || {
                if is_open.get() {
                    ExitState::Open
                } else {
                    ExitState::Closed
                }
            }),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use super::use_animation::watch_animations;

        let initial_state = if is_open.get_untracked() {
            ExitState::Open
        } else {
            ExitState::Closed
        };
        let (exit_state, set_exit_state) = signal(initial_state);

        // State machine transitions driven by is_open changes.
        Effect::new(move |_| {
            let open = is_open.get();
            let state = exit_state.get_untracked();
            match state {
                ExitState::Open => {
                    if !open {
                        set_exit_state.set(ExitState::Exiting);
                    }
                }
                ExitState::Exiting | ExitState::Closed => {
                    if open {
                        set_exit_state.set(ExitState::Open);
                    }
                }
            }
        });

        // Animation watching driven by exit_state changes.
        let cleanup: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            // Clean up previous watcher.
            cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });

            if exit_state.get() == ExitState::Exiting {
                if let Some(el) = element.get() {
                    let cancel = watch_animations(&el, move || {
                        // Only transition to Closed if still Exiting (not interrupted).
                        set_exit_state.update(|state| {
                            if *state == ExitState::Exiting {
                                *state = ExitState::Closed;
                            }
                        });
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

        let is_exiting = Signal::derive(move || exit_state.get() == ExitState::Exiting);

        UseExitAnimationReturn {
            is_exiting,
            exit_state: exit_state.into(),
        }
    }
}
