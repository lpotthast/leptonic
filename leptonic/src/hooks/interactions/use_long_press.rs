use leptos::attr;
use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

use crate::hooks::interactions::use_press::{use_press, PressEvent, UsePressInput, UsePressProps};
use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::focus::focus_element;
use crate::utils::pointer_type::PointerType;
use crate::utils::Modifiers;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useLongPress.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// - `LongPressEvent` does not include a `continuePropagation` method.
//   React-aria's `LongPressEvent` also omits it (it is only present on `PressEvent`).
//
// - `accessibility_description` is set directly as a static `aria-describedby` attribute
//   rather than using a visually hidden `<span>` with an auto-generated ID.
//   Rationale: React-aria creates a hidden DOM element with a unique ID and sets
//   `aria-describedby` to that ID. In Leptos, directly setting the `aria-describedby`
//   attribute to the description text is simpler and achieves equivalent accessibility.
//   If consumers need ID-based `aria-describedby` (e.g. for shared descriptions), they
//   can manage the ID externally.

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
#[derive(Debug)]
pub struct LongPressEvent {
    /// The type of long press event.
    pub event_type: LongPressEventType,

    /// The pointer type that triggered the long press event.
    pub pointer_type: PointerType,

    /// The target element of the long press event.
    pub target: Option<SendWrapper<web_sys::EventTarget>>,

    /// States which modifier keys were held during the long press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub y: Option<f64>,
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

    /// A description for assistive technology users indicating that a long press
    /// action is available, e.g. "Long press to open menu".
    pub accessibility_description: Option<&'static str>,
}

/// The return value of the `use_long_press` hook.
#[derive(Debug, Clone)]
pub struct UseLongPressReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseLongPressProps,
}

/// Props from `use_long_press` that can be extracted and merged programmatically.
///
/// Contains the underlying press props, an element capture attribute for DOM access,
/// and an optional `aria-describedby` attribute for accessibility.
#[derive(Debug, Clone)]
pub struct UseLongPressProps {
    /// The underlying press props.
    pub press_props: UsePressProps,
    /// Element capture attribute for DOM access (focus management, `pointercancel` dispatch).
    pub element_capture: ElementCaptureAttr,
    /// Accessibility description for long press action.
    pub aria_describedby: Option<&'static str>,
}

impl UseLongPressProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseLongPressAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseLongPressAttrs {
        (
            self.press_props.on_keydown.into_on(leptos::ev::keydown),
            self.press_props.on_click.into_on(leptos::ev::click),
            self.press_props
                .on_pointerdown
                .into_on(leptos::ev::pointerdown),
            self.press_props.on_dragstart.into_on(leptos::ev::dragstart),
            self.press_props.on_dblclick.into_on(leptos::ev::dblclick),
            self.element_capture,
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseLongPressAttrs = (
    On<leptos::ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<leptos::ev::click, SharedEventCallback<MouseEvent>>,
    On<leptos::ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<leptos::ev::dragstart, SharedEventCallback<web_sys::DragEvent>>,
    On<leptos::ev::dblclick, SharedEventCallback<MouseEvent>>,
    ElementCaptureAttr,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
);

/// State for the ongoing long press interaction.
struct LongPressState {
    /// The timer handle for the threshold timeout.
    timeout_handle: Option<i32>,
}

/// Handles long press interactions across mouse and touch devices.
/// Supports a customizable time threshold and normalizes behavior across browsers and devices.
///
/// Long press is recognized when the user presses and holds the target for a specified
/// duration (default 500ms). This is commonly used to reveal context menus or secondary actions.
///
/// When the threshold is met, this hook:
/// 1. Dispatches a synthetic `pointercancel` event to cancel sibling `use_press` handlers.
/// 2. Focuses the target element without scrolling.
/// 3. Fires `on_long_press`.
///
/// On touch devices, the native context menu is automatically prevented during the interaction.
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
///     accessibility_description: Some("Long press to open menu"),
/// });
///
/// view! {
///     <button {..long_press.props.into_attrs()}>
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
    let element = CapturedElement::new();

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
                on_long_press_start.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressStart,
                    pointer_type: e.pointer_type.clone(),
                    target: e.target.clone(),
                    modifiers: e.modifiers,
                    x: e.x,
                    y: e.y,
                });
            }

            // Capture values for the timeout closure
            let pointer_type = e.pointer_type.clone();
            let modifiers = e.modifiers;
            let target = e.target.clone();
            let x = e.x;
            let y = e.y;

            let callback = Closure::once(Box::new(move || {
                // Dispatch pointercancel to cancel sibling use_press handlers.
                // This is synchronous — the use_press on_press_end callback (and thus
                // our on_long_press_end) will execute before the code after dispatch_event.
                if let Some(el) = element.get_untracked() {
                    let cancel_event = PointerEvent::new("pointercancel")
                        .expect("should create pointercancel event");
                    let _ = el.dispatch_event(&cancel_event);

                    // Focus the element without scrolling, matching react-aria's
                    // focusWithoutScrolling behavior.
                    focus_element(&el, true);
                }

                // Fire the long press event (after pointercancel has been processed).
                if let Some(on_long_press) = on_long_press {
                    on_long_press.run(LongPressEvent {
                        event_type: LongPressEventType::LongPress,
                        pointer_type: pointer_type.clone(),
                        target: target.clone(),
                        modifiers,
                        x,
                        y,
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

            // For touch, prevent the context menu on the event target (not the document).
            if e.pointer_type == PointerType::Touch {
                if let Some(target) = e.target.as_ref() {
                    let target_et: &web_sys::EventTarget = target.as_ref();

                    // Add a one-time contextmenu prevention listener on the target element.
                    let prevent_context_menu = Closure::once(Box::new(move |e: MouseEvent| {
                        e.prevent_default();
                    })
                        as Box<dyn FnOnce(MouseEvent)>);

                    let options = web_sys::AddEventListenerOptions::new();
                    options.set_once(true);
                    let _ = target_et
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "contextmenu",
                            prevent_context_menu.as_ref().unchecked_ref(),
                            &options,
                        );
                    prevent_context_menu.forget();
                }
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
                on_long_press_end.run(LongPressEvent {
                    event_type: LongPressEventType::LongPressEnd,
                    pointer_type: e.pointer_type.clone(),
                    target: e.target.clone(),
                    modifiers: e.modifiers,
                    x: e.x,
                    y: e.y,
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
        on_double_press: None,
    });

    // Cleanup on unmount
    on_cleanup(move || {
        clear_timeout();
        cleanup_context_menu();
    });

    // Only set aria-describedby when on_long_press is provided and not disabled.
    let aria_describedby = if input.on_long_press.is_some() {
        input.accessibility_description
    } else {
        None
    };

    UseLongPressReturn {
        props: UseLongPressProps {
            press_props: press.props,
            element_capture: element.attr(),
            aria_describedby,
        },
    }
}
