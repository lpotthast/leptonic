// Upstream: react-stately/src/menu/useSubmenuTriggerState.ts @ 99e6102368
// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/SubMenuTrigger.test.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::{
        collections::{FocusStrategy, Key},
        menu::MenuTriggerState,
        overlay::{OverlayTriggerState, UseOverlayTriggerStateInput, use_overlay_trigger_state},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `overlay` is an `OverlayTriggerState` view of the submenu (for its popover), instead of
//   placeholder `setOpen`/`point` members that make the state pass as one.
// - The submenu's `level` is an input (the menu's `MenuData::submenu_level`), not the number of
//   submenus open when the state is created: a trigger mounted while another submenu is open
//   (e.g. an item added to the root menu) still opens its submenu on its menu's level.
//
// =============================================================================

/// Input of [`use_submenu_trigger_state`].
#[derive(Debug, Clone)]
pub struct UseSubmenuTriggerStateInput {
    /// The key of the item that opens the submenu.
    pub trigger_key: Key,
    /// The state of the menu tree's root trigger, which tracks the open submenus.
    pub root: MenuTriggerState,
    /// The submenu's level in the menu tree: 0 for a submenu of the root menu (the menu's
    /// `MenuData::submenu_level`).
    pub level: usize,
}

/// The state of a submenu trigger: whether its submenu is open, its level in the menu tree, and
/// where focus goes when it opens.
#[derive(Debug, Clone, Copy)]
pub struct SubmenuTriggerState {
    /// Whether the submenu is open.
    pub is_open: Signal<bool>,
    /// Where focus goes when the submenu opens (`None`: it stays on the trigger).
    pub focus_strategy: Signal<Option<FocusStrategy>>,
    /// The submenu's level: 0 for a submenu of the root menu.
    pub level: usize,
    /// The submenu as an overlay (opening and closing it), for its popover.
    pub overlay: OverlayTriggerState,
    trigger_key: StoredValue<Key>,
    root: MenuTriggerState,
    set_focus_strategy: WriteSignal<Option<FocusStrategy>>,
}

impl SubmenuTriggerState {
    /// Opens the submenu, focusing according to `focus_strategy`.
    pub fn open(&self, focus_strategy: Option<FocusStrategy>) {
        self.set_focus_strategy.set(focus_strategy);
        self.root
            .open_submenu(self.trigger_key.get_value(), self.level);
    }

    /// Closes the submenu (and the ones it opened).
    pub fn close(&self) {
        self.set_focus_strategy.set(None);
        self.trigger_key
            .with_value(|key| self.root.close_submenu(key, self.level));
    }

    /// Closes all menus of the menu tree.
    pub fn close_all(&self) {
        self.root.close();
    }

    /// Toggles the submenu, focusing according to `focus_strategy` when it opens.
    pub fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        if self.is_open.get_untracked() {
            self.close();
        } else {
            self.open(focus_strategy);
        }
    }
}

/// Creates the state of a submenu trigger: the item `trigger_key` of a menu on `level` in the tree
/// of the root trigger `root`.
pub fn use_submenu_trigger_state(input: UseSubmenuTriggerStateInput) -> SubmenuTriggerState {
    let UseSubmenuTriggerStateInput {
        trigger_key,
        root,
        level,
    } = input;
    let trigger_key = StoredValue::new(trigger_key);
    let is_open = Signal::derive(move || {
        root.expanded_keys_stack
            .with(|stack| trigger_key.with_value(|key| stack.get(level) == Some(key)))
    });
    let (focus_strategy, set_focus_strategy) = signal(None);
    let overlay = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        value: Some(ValueBinding::new(
            is_open,
            Callback::new(move |open: bool| {
                if open {
                    set_focus_strategy.set(None);
                    root.open_submenu(trigger_key.get_value(), level);
                } else {
                    set_focus_strategy.set(None);
                    trigger_key.with_value(|key| root.close_submenu(key, level));
                }
            }),
        )),
        ..UseOverlayTriggerStateInput::default()
    });
    SubmenuTriggerState {
        is_open,
        focus_strategy: focus_strategy.into(),
        level,
        overlay,
        trigger_key,
        root,
        set_focus_strategy,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::menu::{UseMenuTriggerStateInput, use_menu_trigger_state},
        testing::{flush_effects, with_owner},
    };

    fn submenu(root: MenuTriggerState, key: &str, level: usize) -> SubmenuTriggerState {
        use_submenu_trigger_state(UseSubmenuTriggerStateInput {
            trigger_key: Key::from(key),
            root,
            level,
        })
    }

    #[test]
    fn submenus_open_by_level_and_close_deeper_ones() {
        with_owner(|| {
            let root = use_menu_trigger_state(UseMenuTriggerStateInput::default());
            root.open(None);
            let share = submenu(root, "share", 0);
            let edit = submenu(root, "edit", 0);
            assert_that!(share.level).is_equal_to(0);
            share.open(Some(FocusStrategy::First));
            assert_that!(share.is_open.get_untracked()).is_true();
            assert_that!(share.focus_strategy.get_untracked())
                .is_equal_to(Some(FocusStrategy::First));
            // A submenu of the open submenu is on the next level.
            let email = submenu(root, "email", 1);
            email.open(None);
            assert_that!(root.expanded_keys_stack.get_untracked())
                .is_equal_to(vec![Key::from("share"), Key::from("email")]);
            // Opening a sibling submenu closes the open one and the ones below it.
            edit.open(None);
            assert_that!(share.is_open.get_untracked()).is_false();
            assert_that!(email.is_open.get_untracked()).is_false();
            assert_that!(edit.is_open.get_untracked()).is_true();
            // Closing all closes the root menu too.
            edit.close_all();
            assert_that!(edit.is_open.get_untracked()).is_false();
            assert_that!(root.overlay.is_open.get_untracked()).is_false();
        });
    }

    #[test]
    fn the_overlay_view_opens_and_closes_the_submenu() {
        with_owner(|| {
            let root = use_menu_trigger_state(UseMenuTriggerStateInput::default());
            let share = submenu(root, "share", 0);
            share.overlay.open();
            assert_that!(share.is_open.get_untracked()).is_true();
            share.overlay.close();
            assert_that!(share.is_open.get_untracked()).is_false();
        });
    }

    // Upstream: useSubmenuTriggerState's level (react-stately has no test of it): a trigger
    // mounted while another submenu is open opens its submenu on its own menu's level.
    #[test]
    fn a_trigger_created_while_a_submenu_is_open_keeps_its_level() {
        with_owner(|| {
            let root = use_menu_trigger_state(UseMenuTriggerStateInput::default());
            root.open(None);
            let share = submenu(root, "share", 0);
            share.open(None);
            // An item added to the root menu while "share" is open.
            let edit = submenu(root, "edit", 0);
            edit.open(None);
            flush_effects();
            assert_that!(root.expanded_keys_stack.get_untracked())
                .is_equal_to(vec![Key::from("edit")]);
            assert_that!(share.is_open.get_untracked()).is_false();
            assert_that!(edit.is_open.get_untracked()).is_true();
        });
    }
}
