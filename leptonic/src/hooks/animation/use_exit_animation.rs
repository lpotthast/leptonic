// Upstream: react-aria/src/utils/animation.ts @ 99e6102368
//! Hook for tracking CSS exit animations on an element.

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The element is a `CapturedElement` (react-aria: a ref).
// - Returns `is_exiting: Signal<bool>` and the state machine as `exit_state` (react-aria: a
//   boolean per render).
// - `on_exit` is a `Callback` of the element: it can start Web Animations, which are awaited like
//   CSS ones (react-aria: a function that may return a promise to await as well).
//
// ## DIFFERENT BEHAVIOR
// - A watch is cancelled when the effect re-runs or the owner is disposed (react-aria lets the
//   promise resolve and ignores the result through its state machine).
//
// =============================================================================

/// Exit animation state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitState {
    /// The element is logically open (visible).
    Open,
    /// Exit animation is in flight: the element should remain in the DOM.
    Exiting,
    /// Exit animation completed: the element can be removed from the DOM.
    Closed,
}

/// Input for [`use_exit_animation`].
#[derive(Debug, Clone, Copy)]
pub struct UseExitAnimationInput {
    /// The element to track exit animations on.
    pub element: CapturedElement,

    /// Logically open state. When this goes `false`, the exit animation begins.
    pub is_open: Signal<bool>,

    /// Called with the element when the exit starts (e.g. to start a Web Animation, which is
    /// awaited like CSS ones).
    pub on_exit: Option<Callback<send_wrapper::SendWrapper<web_sys::Element>>>,
}

/// Return value of [`use_exit_animation`].
#[derive(Debug)]
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
///         on_exit: None,
///     });
///
/// // Keep element mounted while exiting:
/// // <Show when=move || state.is_open.get() || is_exiting.get()>
/// //     <div {..attrs} data-exiting=flag(is_exiting) />
/// // </Show>
/// //
/// // CSS: .overlay[data-exiting] { animation: fade-out 150ms; }
/// ```
pub fn use_exit_animation(input: UseExitAnimationInput) -> UseExitAnimationReturn {
    crate::metrics::subject!();
    let UseExitAnimationInput {
        element,
        is_open,
        on_exit,
    } = input;

    #[cfg(feature = "ssr")]
    {
        // During SSR, no animations: the state directly follows `is_open`.
        let _ = (element, on_exit);
        UseExitAnimationReturn {
            is_exiting: Signal::stored(false),
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
        let cancel: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);
        let cancel_watch = move || {
            if let Some(cancel) = cancel.try_update_value(Option::take).flatten() {
                cancel();
            }
        };

        Effect::new(move |_| {
            cancel_watch();

            if exit_state.get() != ExitState::Exiting {
                return;
            }
            // Never rendered (e.g. a modal without `ModalContent`): nothing can animate.
            let Some(el) = element.get() else {
                set_exit_state.set(ExitState::Closed);
                return;
            };
            let stop = watch_animations(&el, on_exit, move || {
                // Only transition to Closed if still Exiting (not interrupted).
                set_exit_state.update(|state| {
                    if *state == ExitState::Exiting {
                        *state = ExitState::Closed;
                    }
                });
            });
            cancel.set_value(Some(Box::new(stop)));
        });

        on_cleanup(cancel_watch);

        // Exiting from the moment `is_open` turns false (react-aria derives it while rendering):
        // the state machine's effect catches up after, but a consumer keeping the element rendered
        // while exiting must not drop it in between.
        let is_exiting = Signal::derive(move || {
            !is_open.get() && matches!(exit_state.get(), ExitState::Open | ExitState::Exiting)
        });

        UseExitAnimationReturn {
            is_exiting,
            exit_state: exit_state.into(),
        }
    }
}
