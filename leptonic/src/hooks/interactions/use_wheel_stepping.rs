// Upstream: react-aria/src/numberfield/useNumberField.ts @ 99e6102368
// Upstream: react-aria/src/color/useColorField.ts @ 99e6102368
//! The scroll wheel stepping a field's value while the field has focus (`use_wheel_stepping`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A hook of its own, shared by `use_number_field` and `use_color_field` (react-aria: the same
//   `useFocusWithin` + `useScrollWheel` code in `useNumberField` and `useColorField`), so that
//   fields of your own step as leptonic's do.
//
// =============================================================================

use leptos::prelude::*;

use crate::{
    Part,
    hooks::{
        focus::{UseFocusWithinInput, UseFocusWithinProps, UseFocusWithinReturn, use_focus_within},
        interactions::{ScrollEvent, UseScrollWheelInput, UseScrollWheelProps, use_scroll_wheel},
    },
};

/// Input of [`use_wheel_stepping`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseWheelSteppingInput {
    /// Whether the field is disabled: its focus isn't followed, the wheel doesn't step.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only: the wheel doesn't step.
    pub is_read_only: Signal<bool>,

    /// Whether the scroll wheel leaves the value alone (the page scrolls instead).
    pub is_wheel_disabled: bool,

    /// Called when the wheel scrolls down while the field has focus.
    pub on_increment: Option<Callback<()>>,

    /// Called when the wheel scrolls up while the field has focus.
    pub on_decrement: Option<Callback<()>>,
}

/// The return value of [`use_wheel_stepping`].
#[derive(Debug)]
pub struct UseWheelSteppingReturn {
    /// The element whose focus lets the wheel step: the field's group around the input, or the
    /// input itself.
    pub group: Part<UseFocusWithinProps>,

    /// The input the wheel steps over (scrolling there doesn't scroll the page while it steps).
    pub input: Part<UseScrollWheelProps>,
}

/// The scroll wheel stepping a field's value while focus is within the field: scrolling down
/// increments, scrolling up decrements, a mostly horizontal scroll (a trackpad's) does nothing.
///
/// `use_number_field` and `use_color_field` use it; use it for a field of your own with a value
/// that steps (a duration, a rating). Spread `group` onto the element around the input (or onto
/// the input, if there is none) and `input` onto the input. While the wheel steps, it doesn't
/// scroll the page; while the field doesn't have focus, the page scrolls as usual.
///
/// ```ignore
/// use leptonic::hooks::interactions::{UseWheelSteppingInput, UseWheelSteppingReturn, use_wheel_stepping};
///
/// let count = RwSignal::new(0);
/// let UseWheelSteppingReturn { group, input } = use_wheel_stepping(UseWheelSteppingInput {
///     on_increment: Some(Callback::new(move |()| count.update(|c| *c += 1))),
///     on_decrement: Some(Callback::new(move |()| count.update(|c| *c -= 1))),
///     ..UseWheelSteppingInput::default()
/// });
/// view! {
///     <div {..group.attrs()}>
///         <input {..input.attrs()} prop:value=move || count.get().to_string() />
///     </div>
/// }
/// ```
pub fn use_wheel_stepping(input: UseWheelSteppingInput) -> UseWheelSteppingReturn {
    crate::metrics::subject!();
    let UseWheelSteppingInput {
        is_disabled,
        is_read_only,
        is_wheel_disabled,
        on_increment,
        on_decrement,
    } = input;
    let UseFocusWithinReturn {
        target: group,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        is_disabled,
        on_focus_within: None,
        on_blur_within: None,
        on_focus_within_change: None,
    });
    let wheel = use_scroll_wheel(UseScrollWheelInput {
        is_disabled: Signal::derive(move || {
            is_wheel_disabled || is_disabled.get() || is_read_only.get() || !is_focus_within.get()
        }),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            // Mostly horizontal (a trackpad): probably not meant to step.
            if e.delta_y.abs() <= e.delta_x.abs() {
                return;
            }
            let step = if e.delta_y > 0.0 {
                on_increment
            } else {
                on_decrement
            };
            if let Some(step) = step {
                step.run(());
            }
        })),
    });
    UseWheelSteppingReturn {
        group,
        input: wheel.target,
    }
}
