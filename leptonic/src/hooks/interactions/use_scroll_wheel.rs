use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::WheelEvent;

use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useScrollWheel.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Scroll event data.
#[derive(Debug, Clone, Copy)]
pub struct ScrollEvent {
    /// The horizontal scroll delta.
    pub delta_x: f64,
    /// The vertical scroll delta.
    pub delta_y: f64,
}

/// Input parameters for the `use_scroll_wheel` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseScrollWheelInput {
    /// Whether the scroll listener should be disabled.
    pub disabled: Signal<bool>,

    /// Handler called when the user scrolls with the mouse wheel.
    pub on_scroll: Option<Callback<ScrollEvent>>,
}

/// Return value from the `use_scroll_wheel` hook.
#[derive(Debug, Clone)]
pub struct UseScrollWheelReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseScrollWheelProps,
}

/// Props from `use_scroll_wheel` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseScrollWheelProps {
    pub on_wheel: EventHandler<WheelEvent>,
}

impl UseScrollWheelProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseScrollWheelAttrs {
        (self.on_wheel.to_on(ev::wheel),)
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseScrollWheelAttrs {
        (self.on_wheel.into_on(ev::wheel),)
    }
}

/// Attributes returned by `use_scroll_wheel` that must be spread onto an element.
pub type UseScrollWheelAttrs = (On<ev::wheel, SharedEventCallback<WheelEvent>>,);

/// Handles scroll wheel events on an element.
///
/// This hook returns attributes that:
/// 1. Prevent the default scroll behavior (stops page scrolling)
/// 2. Stop event propagation
/// 3. Ignore zoom events (when ctrl is pressed)
/// 4. Call the `on_scroll` handler with the scroll delta
///
/// This is useful for custom scroll implementations, sliders, number inputs,
/// or any component that needs to respond to mouse wheel input without
/// causing the page to scroll.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(50.0f64);
///
/// let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
///     disabled: Signal::derive(|| false),
///     on_scroll: Some(Callback::new(move |e: ScrollEvent| {
///         // e.delta_x, e.delta_y give scroll amounts
///         // Positive delta_y = scroll down, negative = scroll up
///
///         // Only respond to primarily vertical scrolling
///         if e.delta_y.abs() > e.delta_x.abs() {
///             set_value.update(|v| {
///                 *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0);
///             });
///         }
///     })),
/// });
///
/// view! {
///     <div {..scroll_wheel.attrs} tabindex="0">
///         "Value: " { move || value.get().round() as i32 }
///     </div>
/// }
/// ```
pub fn use_scroll_wheel(input: UseScrollWheelInput) -> UseScrollWheelReturn {
    let UseScrollWheelInput {
        disabled,
        on_scroll,
    } = input;

    let handle_wheel = move |e: WheelEvent| {
        if disabled.get_untracked() {
            return;
        }

        // If the ctrlKey is pressed, this is a zoom event, do nothing.
        if e.ctrl_key() {
            return;
        }

        // Stop scrolling the page
        e.prevent_default();
        e.stop_propagation();

        if let Some(on_scroll) = on_scroll {
            on_scroll.run(ScrollEvent {
                delta_x: e.delta_x(),
                delta_y: e.delta_y(),
            });
        }
    };

    UseScrollWheelReturn {
        props: UseScrollWheelProps {
            on_wheel: EventHandler::new(handle_wheel),
        },
    }
}
