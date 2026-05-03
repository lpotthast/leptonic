use leptos::prelude::*;

use crate::hooks::selection::use_selection_state::FocusStrategy;

// ## OMITTED FEATURES
// - `useControlledState` / controlled `isOpen` prop: React Aria supports both
//   controlled (`isOpen` from parent) and uncontrolled (`defaultOpen`) patterns
//   because React components cannot share mutable state. In Leptos, `Signal<T>`
//   is `Copy` and inherently shared, so controlled state is unnecessary. The
//   hook always owns its `WriteSignal` internally and exposes a read-only
//   `Signal<bool>`. This ensures `on_open_change` always fires and the hook
//   can enforce invariants. See documentation/hooks-implementation.md for the
//   full rationale.
// - Submenu state (`RootMenuTriggerState` with `expandedKeysStack`,
//   `openSubmenu`, `closeSubmenu`): Deferred until leptonic adds submenu
//   support.

/// Input for `use_menu_trigger_state` hook.
#[derive(Debug, Clone, Default)]
pub struct UseMenuTriggerStateInput {
    /// Initial open state (defaults to false).
    pub default_open: bool,

    /// Called whenever the open state changes.
    pub on_open_change: Option<Callback<bool>>,
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
    /// Toggle the menu with optional focus strategy.
    pub toggle: Callback<Option<FocusStrategy>>,
    /// Set the open state directly.
    pub set_open: Callback<bool>,
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
    let UseMenuTriggerStateInput {
        default_open,
        on_open_change,
    } = input;

    let (is_open, set_is_open) = signal(default_open);
    let (focus_strategy, set_focus_strategy) = signal::<Option<FocusStrategy>>(None);

    let update_open = move |value: bool| {
        set_is_open.set(value);
        if let Some(cb) = on_open_change {
            cb.run(value);
        }
    };

    let open = Callback::new(move |strategy: Option<FocusStrategy>| {
        set_focus_strategy.set(strategy);
        update_open(true);
    });

    let close = Callback::new(move |_: ()| {
        update_open(false);
    });

    let toggle = Callback::new(move |strategy: Option<FocusStrategy>| {
        set_focus_strategy.set(strategy);
        update_open(!is_open.get_untracked());
    });

    UseMenuTriggerStateReturn {
        is_open: is_open.into(),
        focus_strategy: focus_strategy.into(),
        open,
        close,
        toggle,
        set_open: Callback::new(update_open),
    }
}
