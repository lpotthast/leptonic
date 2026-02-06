use leptos::prelude::*;

use crate::hooks::selection::use_selectable_collection::FocusStrategy;

/// Input for `use_menu_trigger_state` hook.
#[derive(Debug, Clone, Default)]
pub struct UseMenuTriggerStateInput {
    /// Initial open state (defaults to false).
    pub default_open: bool,
}

/// Return value of `use_menu_trigger_state` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseMenuTriggerStateReturn {
    /// Whether the menu is currently open.
    pub is_open: Signal<bool>,
    /// The focus strategy to use when the menu opens.
    /// Updated when `open()` or `toggle()` is called with a strategy.
    pub focus_strategy: Signal<Option<FocusStrategy>>,
    /// Open the menu with optional focus strategy.
    pub open: Callback<Option<FocusStrategy>>,
    /// Close the menu.
    pub close: Callback<()>,
    /// Toggle the menu with optional focus strategy for opening.
    pub toggle: Callback<Option<FocusStrategy>>,
}

/// Manages the state for a menu trigger.
///
/// This hook provides centralized state management for menu open/close behavior
/// and focus strategy. It should be used in conjunction with `use_menu_trigger`
/// and `use_menu` to create accessible dropdown menus.
///
/// # Example
///
/// ```ignore
/// let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
///
/// // Check if open
/// if state.is_open.get() { ... }
///
/// // Open with first item focused
/// state.open.run(Some(FocusStrategy::First));
///
/// // Close
/// state.close.run(());
///
/// // Toggle (opens with strategy, closes without)
/// state.toggle.run(Some(FocusStrategy::First));
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_menu_trigger_state(input: UseMenuTriggerStateInput) -> UseMenuTriggerStateReturn {
    let (is_open, set_is_open) = signal(input.default_open);
    let (focus_strategy, set_focus_strategy) = signal::<Option<FocusStrategy>>(None);

    let open = Callback::new(move |strategy: Option<FocusStrategy>| {
        set_focus_strategy.set(strategy);
        set_is_open.set(true);
    });

    let close = Callback::new(move |_: ()| {
        set_focus_strategy.set(None);
        set_is_open.set(false);
    });

    let toggle = Callback::new(move |strategy: Option<FocusStrategy>| {
        if is_open.get_untracked() {
            set_focus_strategy.set(None);
            set_is_open.set(false);
        } else {
            set_focus_strategy.set(strategy);
            set_is_open.set(true);
        }
    });

    UseMenuTriggerStateReturn {
        is_open: is_open.into(),
        focus_strategy: focus_strategy.into(),
        open,
        close,
        toggle,
    }
}
