use educe::Educe;
use leptos::prelude::*;
use leptos_use::use_event_listener;
use send_wrapper::SendWrapper;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::hooks::interactions::use_press::{
    use_press, PressEvent, UsePressAttrs, UsePressInput, UsePressProps,
};
use crate::utils::pointer_type::PointerType;
use crate::utils::Modifiers;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useLongPress.ts

/// The default long press threshold in milliseconds.
const DEFAULT_THRESHOLD: u64 = 500;

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
#[derive(Educe)]
#[educe(Debug)]
pub struct LongPressEvent {
    /// The type of long press event.
    pub event_type: LongPressEventType,

    /// The pointer type that triggered the long press event.
    pub pointer_type: PointerType,

    /// The target element of the long press event.
    pub target: Option<SendWrapper<web_sys::EventTarget>>,

    /// States which modifier keys were held during the long press event.
    pub modifiers: Modifiers,

    /// By default, long press events stop propagation to parent elements.
    /// Call this to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Arc<dyn Fn() + Send + Sync + 'static>,
}

/// Input parameters for the `use_long_press` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseLongPressInput {
    /// Whether long press events should be disabled.
    pub disabled: Signal<bool>,

    /// Handler that is called when a long press interaction starts.
    pub on_long_press_start: Option<Callback<LongPressEvent>>,

    /// Handler that is called when a long press interaction ends, either
    /// over the target or when the pointer leaves the target.
    pub on_long_press_end: Option<Callback<LongPressEvent>>,

    /// Handler that is called when the threshold time is met while
    /// the press is over the target.
    pub on_long_press: Option<Callback<LongPressEvent>>,

    /// The amount of time in milliseconds to wait before triggering a long press.
    /// Default is 500ms.
    pub threshold: Option<u64>,
}

/// The return value of the `use_long_press` hook.
#[derive(Debug, Clone)]
pub struct UseLongPressReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseLongPressProps,
}

/// Props from `use_long_press` that can be extracted and merged programmatically.
/// This is a wrapper around [`UsePressProps`].
pub type UseLongPressProps = UsePressProps;

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseLongPressAttrs = UsePressAttrs;

/// State for the ongoing long press interaction.
struct LongPressState {
    /// The timer handle for the threshold timeout.
    timeout_handle: Option<i32>,
}

fn use_continue_propagation() -> (Arc<AtomicBool>, Arc<dyn Fn() + Send + Sync + 'static>) {
    let continue_propagation_state = Arc::new(AtomicBool::new(false));
    let state = continue_propagation_state.clone();
    let continue_propagation = Arc::new(move || {
        state.store(true, Ordering::Release);
    });
    (continue_propagation_state, continue_propagation)
}

/// Handles long press interactions across mouse and touch devices.
/// Supports a customizable time threshold and normalizes behavior across browsers and devices.
///
/// Long press is recognized when the user presses and holds the target for a specified
/// duration (default 500ms). This is commonly used to reveal context menus or secondary actions.
///
/// # Example
///
/// ```ignore
/// let long_press = use_long_press(UseLongPressInput {
///     disabled: Signal::derive(|| false),
///     on_long_press: Some(Callback::new(|e| {
///         // Handle long press
///     })),
///     on_long_press_start: None,
///     on_long_press_end: None,
///     threshold: Some(500),
/// });
///
/// view! {
///     <button {..long_press.attrs}>
///         "Long press me"
///     </button>
/// }
/// ```
///
/// # Panics
///
/// Panics if the `window` object is not available or if setting a timeout fails.
#[allow(clippy::too_many_lines)]
pub fn use_long_press(input: UseLongPressInput) -> UseLongPressReturn {
    let threshold = input.threshold.unwrap_or(DEFAULT_THRESHOLD);

    let state: StoredValue<Option<LongPressState>, LocalStorage> = StoredValue::new_local(None);
    let context_menu_cleanup: StoredValue<Option<Box<dyn Fn()>>, LocalStorage> =
        StoredValue::new_local(None);

    let clear_timeout = move || {
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                if let Some(handle) = s.timeout_handle.take() {
                    let window = web_sys::window().expect("window should exist");
                    window.clear_timeout_with_handle(handle);
                }
            }
        });
    };

    let cleanup_context_menu = move || {
        context_menu_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });
    };

    let on_press_start = {
        let on_long_press_start = input.on_long_press_start;
        let on_long_press = input.on_long_press;

        Callback::new(move |e: PressEvent| {
            // Allow the press event to continue propagation
            (e.continue_propagation)();

            // Only handle mouse and touch (not keyboard, pen, etc. for long press)
            if e.pointer_type != PointerType::Mouse && e.pointer_type != PointerType::Touch {
                return;
            }

            // Store the press state
            state.set_value(Some(LongPressState {
                timeout_handle: None,
            }));

            // Fire on_long_press_start
            if let Some(on_long_press_start) = on_long_press_start {
                let (continue_propagation_state, continue_propagation) = use_continue_propagation();
                on_long_press_start.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressStart,
                    pointer_type: e.pointer_type.clone(),
                    target: None,
                    modifiers: e.modifiers,
                    continue_propagation,
                });
                let _ = continue_propagation_state; // unused for start event
            }

            // Set up the threshold timeout
            let pointer_type = e.pointer_type.clone();
            let modifiers = e.modifiers;

            let callback = Closure::once(Box::new(move || {
                // Dispatch pointercancel to prevent other usePress handlers
                // from also handling this event
                // Note: In practice, this is harder to do in Rust/WASM without a target reference

                // Fire the long press event
                if let Some(on_long_press) = on_long_press {
                    let (_continue_propagation_state, continue_propagation) =
                        use_continue_propagation();
                    on_long_press.run(LongPressEvent {
                        event_type: LongPressEventType::LongPress,
                        pointer_type: pointer_type.clone(),
                        target: None,
                        modifiers,
                        continue_propagation,
                    });
                }

                // Clear the timeout handle from state
                state.update_value(|s| {
                    if let Some(s) = s.as_mut() {
                        s.timeout_handle = None;
                    }
                });
            }) as Box<dyn FnOnce()>);

            let window = web_sys::window().expect("window should exist");
            #[allow(clippy::cast_possible_truncation)]
            let threshold_ms = threshold as i32;
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    threshold_ms,
                )
                .expect("should set timeout");

            // Prevent the closure from being dropped
            callback.forget();

            state.update_value(|s| {
                if let Some(s) = s.as_mut() {
                    s.timeout_handle = Some(handle);
                }
            });

            // For touch, prevent the context menu
            if e.pointer_type == PointerType::Touch {
                // Set up context menu prevention on the event target's owner document
                // This correctly handles elements in iframes or shadow DOM
                if let Some(target) = e.target.as_ref() {
                    if let Some(target_node) = target.dyn_ref::<web_sys::Node>() {
                        if let Some(document) = target_node.owner_document() {
                            let cleanup = use_event_listener(
                                document,
                                leptos::ev::contextmenu,
                                move |e: web_sys::MouseEvent| {
                                    e.prevent_default();
                                },
                            );

                            context_menu_cleanup.set_value(Some(Box::new(cleanup)));
                        }
                    }
                }

                // Set up cleanup after pointerup
                let window = web_sys::window().expect("window should exist");
                let cleanup_after_pointerup = Closure::once(Box::new(move || {
                    // Remove context menu handler after a short delay
                    let window = web_sys::window().expect("window should exist");
                    let cleanup_callback = Closure::once(Box::new(move || {
                        cleanup_context_menu();
                    })
                        as Box<dyn FnOnce()>);

                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cleanup_callback.as_ref().unchecked_ref(),
                        30,
                    );
                    cleanup_callback.forget();
                })
                    as Box<dyn FnOnce()>);

                let options = web_sys::AddEventListenerOptions::new();
                options.set_once(true);
                let _ = window.add_event_listener_with_callback_and_add_event_listener_options(
                    "pointerup",
                    cleanup_after_pointerup.as_ref().unchecked_ref(),
                    &options,
                );
                cleanup_after_pointerup.forget();
            }
        })
    };

    let on_press_end = {
        let on_long_press_end = input.on_long_press_end;

        Callback::new(move |e: PressEvent| {
            // Clear the timeout
            clear_timeout();

            // Only handle mouse and touch
            if e.pointer_type != PointerType::Mouse && e.pointer_type != PointerType::Touch {
                return;
            }

            // Fire on_long_press_end
            if let Some(on_long_press_end) = on_long_press_end {
                let (_continue_propagation_state, continue_propagation) =
                    use_continue_propagation();
                on_long_press_end.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressEnd,
                    pointer_type: e.pointer_type.clone(),
                    target: None,
                    modifiers: e.modifiers,
                    continue_propagation,
                });
            }

            // Clear state
            state.set_value(None);
        })
    };

    // Use the press hook internally
    let press = use_press(UsePressInput {
        disabled: input.disabled,
        force_prevent_default: false,
        allow_propagation: true,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        on_press: Callback::new(|_| {}), // We don't use the regular press event
        on_press_up: None,
        on_press_start: Some(on_press_start),
        on_press_end: Some(on_press_end),
        on_press_change: None,
    });

    // Cleanup on unmount
    on_cleanup(move || {
        clear_timeout();
        cleanup_context_menu();
    });

    UseLongPressReturn { props: press.props }
}
