// Upstream: react-aria/src/switch/useSwitch.ts @ 99e6102368
// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
use super::use_toggle::{UseToggleInput, UseToggleReturn, use_toggle_with};
use crate::utils::aria::AriaRole;

// No deviations from react-aria beyond the project-wide API conventions.

/// Input of [`use_switch`]: the toggle's state and settings.
pub type UseSwitchInput = UseToggleInput;

/// Output of [`use_switch`].
pub type UseSwitchReturn = UseToggleReturn;

/// Provides the behavior and accessibility of a switch: an `<input type="checkbox"
/// role="switch">` inside a `<label>` (render the track and thumb next to the visually hidden
/// input). It submits, resets and validates like a checkbox.
pub fn use_switch(input: UseSwitchInput) -> UseSwitchReturn {
    use_toggle_with(input, Some(AriaRole::Switch), None)
}
