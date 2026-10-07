// Upstream: react-aria/src/button/useToggleButton.ts @ 99e6102368
use leptos::prelude::*;

use super::use_button::UseButtonInput;
use crate::{
    hooks::{PressEvent, form::ToggleState},
    utils::aria::AriaPressed,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `UseButtonInput` for `use_button` (the button's own hook) instead of calling
//   `useButton` and returning DOM props, as composite hooks do here (project-wide convention).
//
// =============================================================================

/// Input of [`use_toggle_button`].
#[derive(Debug, Clone)]
pub struct UseToggleButtonInput {
    pub state: ToggleState,
    /// The button's further settings (`on_press` runs after toggling).
    pub button: UseButtonInput,
}

/// A button that toggles `state` when pressed (`aria-pressed`). Returns the input of
/// [`use_button`](super::use_button):
///
/// ```ignore
/// let state = use_toggle_state(UseToggleStateInput::default());
/// let button = use_button(use_toggle_button(UseToggleButtonInput {
///     state,
///     button: UseButtonInput::default(),
/// }));
/// ```
pub fn use_toggle_button(input: UseToggleButtonInput) -> UseButtonInput {
    let UseToggleButtonInput { state, mut button } = input;
    let on_press = button.on_press;
    button.on_press = Some(Callback::new(move |e: PressEvent| {
        state.toggle();
        if let Some(on_press) = on_press {
            on_press.run(e);
        }
    }));
    let is_selected = state.is_selected;
    button.aria_pressed = Signal::derive(move || Some(AriaPressed::from(is_selected.get())));
    button
}
