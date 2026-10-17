// Upstream: react-aria/src/switch/useSwitch.ts @ 99e6102368
// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
use super::{
    use_toggle::{ToggleOptions, ToggleRole, UseToggleInput, UseToggleReturn, use_toggle},
    use_toggle_state::ToggleState,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Calls `use_toggle` with `role: ToggleRole::Switch` (react-aria adds `role: 'switch'` to
//   `useToggle`'s returned `inputProps`).
//
// =============================================================================

/// Input of [`use_switch`]: the toggle's state and settings.
#[derive(Debug, Clone)]
pub struct UseSwitchInput {
    pub state: ToggleState,
    pub options: ToggleOptions,
}

/// Output of [`use_switch`].
pub type UseSwitchReturn = UseToggleReturn;

/// Provides the behavior and accessibility of a switch: an `<input type="checkbox"
/// role="switch">` inside a `<label>` (render the track and thumb next to the visually hidden
/// input). It submits, resets and validates like a checkbox.
pub fn use_switch(input: UseSwitchInput) -> UseSwitchReturn {
    crate::metrics::subject!();
    let UseSwitchInput { state, options } = input;
    use_toggle(UseToggleInput {
        state,
        role: ToggleRole::Switch,
        validation: None,
        options,
    })
}
