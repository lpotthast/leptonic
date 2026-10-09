// Upstream: react-stately/src/menu/useMenuTriggerState.ts @ 99e6102368
// Upstream: react-aria/test/menu/useMenuTrigger.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::{
        collections::{FocusStrategy, Key},
        overlay::{
            OverlayState, OverlayTriggerState, UseOverlayTriggerStateInput,
            use_overlay_trigger_state,
        },
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_open` + `on_open_change`, or `value` bound to app state,
//   instead of a controlled `isOpen`.
// - A `Copy` struct of signals and methods (C3); the overlay state is its `overlay` field
//   (react-aria spreads it into the menu state). States with a menu (select, combo box) work with
//   `use_menu_trigger` through the `MenuTriggerStateApi` trait (react-aria: structural typing).
// - Closing the overlay state directly (as a popover does) also closes the submenus.
//
// =============================================================================

/// Input of [`use_menu_trigger_state`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseMenuTriggerStateInput {
    /// Whether the menu starts open. Ignored when `value` is bound.
    pub default_open: bool,
    /// The open state as app state, replacing `default_open`.
    pub value: Option<ValueBinding<bool>>,
    /// Called when the menu opens or closes.
    pub on_open_change: Option<Callback<bool>>,
}

/// The state of a menu trigger: whether its menu is open, where focus goes when it opens, and
/// which submenus are open.
#[derive(Debug, Clone, Copy)]
pub struct MenuTriggerState {
    pub overlay: OverlayTriggerState,
    /// Where focus goes when the menu opens (set by `open`/`toggle`).
    pub focus_strategy: Signal<Option<FocusStrategy>>,
    /// The keys of the items whose submenus are open, by level.
    pub expanded_keys_stack: Signal<Vec<Key>>,
    set_focus_strategy: WriteSignal<Option<FocusStrategy>>,
    set_expanded_keys_stack: WriteSignal<Vec<Key>>,
}

impl MenuTriggerState {
    pub fn is_open(&self) -> bool {
        self.overlay.is_open.get()
    }

    /// Opens the menu, focusing according to `focus_strategy` (`None`: the menu itself).
    pub fn open(&self, focus_strategy: Option<FocusStrategy>) {
        self.set_focus_strategy.set(focus_strategy);
        self.overlay.open();
    }

    /// Toggles the menu, focusing according to `focus_strategy` when it opens.
    pub fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        self.set_focus_strategy.set(focus_strategy);
        self.overlay.toggle();
    }

    /// Closes the menu and all its submenus.
    pub fn close(&self) {
        self.set_expanded_keys_stack.set(Vec::new());
        self.overlay.close();
    }

    pub fn set_open(&self, is_open: bool) {
        if is_open {
            self.overlay.open();
        } else {
            self.close();
        }
    }

    /// Opens the submenu of the item `trigger_key` at `level` (closing deeper ones).
    pub fn open_submenu(&self, trigger_key: Key, level: usize) {
        self.set_expanded_keys_stack.maybe_update(|stack| {
            if level > stack.len() || (stack.len() == level + 1 && stack[level] == trigger_key) {
                return false;
            }
            stack.truncate(level);
            stack.push(trigger_key);
            true
        });
    }

    /// Closes the submenu of the item `trigger_key` at `level` (and deeper ones).
    pub fn close_submenu(&self, trigger_key: &Key, level: usize) {
        self.set_expanded_keys_stack.maybe_update(|stack| {
            if stack.get(level) != Some(trigger_key) {
                return false;
            }
            stack.truncate(level);
            true
        });
    }
}

/// The state a menu trigger opens and closes: a [`MenuTriggerState`], or a state with a menu
/// (a select's, a combo box's) adding its own rules for opening.
pub trait MenuTriggerStateApi: OverlayState {
    /// Where focus goes when the menu opens (tracked).
    fn focus_strategy(&self) -> Option<FocusStrategy>;
    fn open(&self, focus_strategy: Option<FocusStrategy>);
    fn toggle(&self, focus_strategy: Option<FocusStrategy>);
    /// Sets where a context menu opened (viewport coordinates).
    fn set_point(&self, point: Option<crate::Point>);
}

impl OverlayState for MenuTriggerState {
    fn is_open(&self) -> bool {
        MenuTriggerState::is_open(self)
    }

    fn close(&self) {
        MenuTriggerState::close(self);
    }

    fn point(&self) -> Signal<Option<crate::Point>> {
        self.overlay.point
    }
}

impl MenuTriggerStateApi for MenuTriggerState {
    fn focus_strategy(&self) -> Option<FocusStrategy> {
        self.focus_strategy.get()
    }

    fn open(&self, focus_strategy: Option<FocusStrategy>) {
        MenuTriggerState::open(self, focus_strategy);
    }

    fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        MenuTriggerState::toggle(self, focus_strategy);
    }

    fn set_point(&self, point: Option<crate::Point>) {
        self.overlay.set_point(point);
    }
}

/// Manages the state of a menu trigger (see `use_menu_trigger`).
///
/// ```ignore
/// let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
/// state.open(Some(FocusStrategy::First));
/// state.close();
/// ```
pub fn use_menu_trigger_state(input: UseMenuTriggerStateInput) -> MenuTriggerState {
    let UseMenuTriggerStateInput {
        default_open,
        value,
        on_open_change,
    } = input;
    let overlay = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        default_open,
        value,
        on_open_change,
    });
    let (focus_strategy, set_focus_strategy) = signal(None);
    let (expanded_keys_stack, set_expanded_keys_stack) = signal(Vec::<Key>::new());
    // However the menu closes (its `close`, the overlay state, e.g. dismissed by its popover, or bound app state),
    // its submenus close with it.
    let is_open = overlay.is_open;
    Effect::new(move |_| {
        if !is_open.get() && !expanded_keys_stack.with_untracked(Vec::is_empty) {
            set_expanded_keys_stack.set(Vec::new());
        }
    });
    MenuTriggerState {
        overlay,
        focus_strategy: focus_strategy.into(),
        expanded_keys_stack: expanded_keys_stack.into(),
        set_focus_strategy,
        set_expanded_keys_stack,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::{flush_effects, with_owner};

    #[test]
    fn closing_closes_submenus() {
        with_owner(|| {
            let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
            state.open(Some(FocusStrategy::First));
            assert_that!(state.focus_strategy.get_untracked())
                .is_equal_to(Some(FocusStrategy::First));
            state.open_submenu(Key::from("a"), 0);
            state.open_submenu(Key::from("b"), 1);
            // A level beyond the stack is ignored.
            state.open_submenu(Key::from("x"), 5);
            assert_that!(state.expanded_keys_stack.get_untracked())
                .is_equal_to(vec![Key::from("a"), Key::from("b")]);
            // Only the key open at that level closes.
            state.close_submenu(&Key::from("x"), 0);
            assert_that!(state.expanded_keys_stack.get_untracked()).has_length(2);
            state.close_submenu(&Key::from("b"), 1);
            assert_that!(state.expanded_keys_stack.get_untracked())
                .is_equal_to(vec![Key::from("a")]);
            state.close();
            assert_that!(state.overlay.is_open.get_untracked()).is_false();
            assert_that!(state.expanded_keys_stack.get_untracked()).is_empty();
        });
    }

    #[test]
    fn reopening_the_open_submenu_notifies_nobody() {
        with_owner(|| {
            let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
            state.open(None);
            state.open_submenu(Key::from("a"), 0);
            let runs = RwSignal::new(0);
            Effect::new(move |_| {
                state.expanded_keys_stack.track();
                runs.update(|runs| *runs += 1);
            });
            flush_effects();
            state.open_submenu(Key::from("a"), 0);
            state.close_submenu(&Key::from("x"), 0);
            flush_effects();
            assert_that!(runs.get_untracked()).is_equal_to(1);
            state.close_submenu(&Key::from("a"), 0);
            flush_effects();
            assert_that!(runs.get_untracked()).is_equal_to(2);
        });
    }
}
