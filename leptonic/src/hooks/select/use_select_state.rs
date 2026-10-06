// Upstream: react-stately/src/select/useSelectState.ts @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;

use crate::hooks::{
    collections::{
        CollectionMemo, FocusStrategy, Key, ListState, Node, Selection, SelectionMode,
        SelectionOptions, UseListStateInput, use_list_state,
    },
    form::use_form_validation_state::{
        UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
        use_form_validation_state,
    },
    menu::use_menu_trigger_state::{
        MenuTriggerState, MenuTriggerStateApi, UseMenuTriggerStateInput, use_menu_trigger_state,
    },
    overlay::use_overlay_trigger_state::OverlayState,
};
use crate::utils::ValueBinding;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_value` and `set_value`, or `value` bound to app state, instead
//   of a controlled `value`.
// - The value is a `Vec<Key>` in both modes (at most one key in `Single` mode), in collection
//   order. react-aria: `Key | null` or `Key[]` depending on the mode.
// - The deprecated `selectedKey`/`defaultSelectedKey`/`onSelectionChange` aliases are not
//   offered.
//
// =============================================================================

/// How many options a select can have selected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectMode {
    #[default]
    Single,
    Multiple,
}

/// Input of [`use_select_state`].
#[derive(Clone)]
pub struct UseSelectStateInput {
    /// The options.
    pub collection: CollectionMemo,
    pub selection_mode: SelectMode,
    /// The initially selected keys (at most one in `Single` mode). Ignored when `value` is bound.
    pub default_value: Vec<Key>,
    /// The selected keys as app state, replacing `default_value`: the select shows them, and
    /// selecting writes them (in collection order).
    pub value: Option<ValueBinding<Vec<Key>>>,
    /// Called when the selected keys change.
    pub on_change: Option<Callback<Vec<Key>>>,
    pub disabled_keys: Signal<HashSet<Key>>,
    /// Close the popover when an option is selected. `None`: in `Single` mode.
    pub should_close_on_select: Option<bool>,
    /// Allow opening the popover without options (e.g. to show an empty state).
    pub allows_empty_collection: bool,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    /// Marks the value invalid, regardless of `validate`.
    pub is_invalid: Signal<bool>,
    /// Validates the selected keys.
    pub validate: Option<ValidateFn<Vec<Key>>>,
    pub validation_behavior: ValidationBehavior,
    /// The form field name (matches server-side validation errors).
    pub name: Option<String>,
}

impl UseSelectStateInput {
    /// A single-selection select over `collection`, with nothing selected.
    pub fn new(collection: CollectionMemo) -> Self {
        Self {
            collection,
            selection_mode: SelectMode::Single,
            default_value: Vec::new(),
            value: None,
            on_change: None,
            disabled_keys: Signal::stored(HashSet::new()),
            should_close_on_select: None,
            allows_empty_collection: false,
            default_open: false,
            on_open_change: None,
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The state of a select: its options and selection, whether its popover is open, focus and
/// validation.
#[derive(Clone, Copy)]
pub struct SelectState {
    pub list: ListState,
    pub selection_mode: SelectMode,
    /// The popover's open state. Open it through [`SelectState::open`]/[`SelectState::toggle`],
    /// which only open it if there are options (or `allows_empty_collection` is set).
    pub menu_trigger: MenuTriggerState,
    allows_empty_collection: bool,
    pub validation: UseFormValidationStateReturn,
    is_focused: RwSignal<bool>,
    default_value: StoredValue<Vec<Key>>,
}

impl OverlayState for SelectState {
    fn is_open(&self) -> bool {
        SelectState::is_open(self)
    }

    fn close(&self) {
        SelectState::close(self);
    }
}

impl MenuTriggerStateApi for SelectState {
    fn focus_strategy(&self) -> Option<FocusStrategy> {
        SelectState::focus_strategy(self)
    }

    fn open(&self, focus_strategy: Option<FocusStrategy>) {
        SelectState::open(self, focus_strategy);
    }

    fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        SelectState::toggle(self, focus_strategy);
    }
}

impl SelectState {
    /// The selected keys, in collection order.
    pub fn value(&self) -> Vec<Key> {
        ordered(self.list.collection, self.list.selection.selected_keys())
    }

    /// The selected key (the first one in `Multiple` mode).
    pub fn selected_key(&self) -> Option<Key> {
        self.value().into_iter().next()
    }

    /// The nodes of the selected options.
    pub fn selected_items(&self) -> Vec<Node> {
        let value = self.value();
        self.list
            .collection
            .with(|c| value.iter().filter_map(|key| c.get(key).cloned()).collect())
    }

    /// Select `keys` (at most one in `Single` mode).
    pub fn set_value(&self, keys: Vec<Key>) {
        let keys = match self.selection_mode {
            SelectMode::Single => keys.into_iter().take(1).collect(),
            SelectMode::Multiple => keys,
        };
        self.list.selection.set_selected_keys(keys);
    }

    /// The value the select started with (restored on form reset).
    pub fn default_value(&self) -> Vec<Key> {
        self.default_value.get_value()
    }

    pub fn is_open(&self) -> bool {
        self.menu_trigger.is_open()
    }

    /// How focus enters the popover's listbox when it opens.
    pub fn focus_strategy(&self) -> Option<FocusStrategy> {
        self.menu_trigger.focus_strategy.get()
    }

    /// Whether the popover may open: with options, or with `allows_empty_collection`.
    fn can_open(&self) -> bool {
        self.allows_empty_collection || self.list.collection.with_untracked(|c| !c.is_empty())
    }

    pub fn open(&self, focus_strategy: Option<FocusStrategy>) {
        if self.can_open() {
            self.menu_trigger.open(focus_strategy);
        }
    }

    pub fn close(&self) {
        self.menu_trigger.close();
    }

    pub fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        if self.is_open_untracked() || self.can_open() {
            self.menu_trigger.toggle(focus_strategy);
        }
    }

    fn is_open_untracked(&self) -> bool {
        self.menu_trigger.overlay.is_open.get_untracked()
    }

    /// Whether the select (its trigger or popover) has focus.
    pub fn is_focused(&self) -> bool {
        self.is_focused.get()
    }

    pub fn set_focused(&self, focused: bool) {
        self.is_focused.set(focused);
    }
}

/// Creates the state of a select (see [`SelectState`]).
pub fn use_select_state(input: UseSelectStateInput) -> SelectState {
    let UseSelectStateInput {
        collection,
        selection_mode,
        default_value,
        value,
        on_change,
        disabled_keys,
        should_close_on_select,
        allows_empty_collection,
        default_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;
    let should_close_on_select =
        should_close_on_select.unwrap_or(selection_mode == SelectMode::Single);

    let menu_trigger = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open,
        on_open_change,
        ..UseMenuTriggerStateInput::default()
    });

    // Set once validation exists (it needs the list's value).
    let commit_validation: StoredValue<Option<Callback<()>>> = StoredValue::new(None);
    let last_value = StoredValue::new(value.map_or_else(
        || default_value.clone(),
        |value| value.value.get_untracked(),
    ));
    // The bound value as the list's selection ("select all" doesn't apply to selects).
    let selection_binding = value.map(|value| {
        ValueBinding::new(
            Signal::derive(move || Selection::keys(value.value.get())),
            Callback::new(move |selection: Selection| {
                if let Selection::Keys(keys) = selection {
                    value.set(ordered(collection, keys));
                }
            }),
        )
    });

    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(match selection_mode {
                SelectMode::Single => SelectionMode::Single,
                SelectMode::Multiple => SelectionMode::Multiple,
            }),
            default_selection: Selection::keys(default_value.clone()),
            selection: selection_binding,
            on_selection_change: Some(Callback::new(move |selection: Selection| {
                // "Select all" doesn't apply to selects.
                let Selection::Keys(keys) = selection else {
                    return;
                };
                let value = ordered(collection, keys);
                if value != last_value.get_value() {
                    last_value.set_value(value.clone());
                    if let Some(on_change) = on_change {
                        on_change.run(value);
                    }
                }
                if should_close_on_select && menu_trigger.overlay.is_open.get_untracked() {
                    menu_trigger.close();
                }
                if let Some(commit) = commit_validation.get_value() {
                    commit.run(());
                }
            })),
            disallow_empty_selection: Signal::stored(selection_mode == SelectMode::Single),
            disabled_keys,
            // Selecting the selected option again still closes the popover.
            allow_duplicate_selection_events: true,
            ..SelectionOptions::default()
        },
    });

    let selection = list.selection;
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: Signal::derive(move || ordered(collection, selection.selected_keys())),
        validate,
        validation_behavior,
        name,
    });
    commit_validation.set_value(Some(validation.commit_validation));

    SelectState {
        list,
        selection_mode,
        menu_trigger,
        allows_empty_collection,
        validation,
        is_focused: RwSignal::new(false),
        default_value: StoredValue::new(default_value),
    }
}

/// `keys` in collection order.
fn ordered(collection: CollectionMemo, keys: impl IntoIterator<Item = Key>) -> Vec<Key> {
    collection.with(|c| c.sorted_keys(keys))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::hooks::collections::Collection;

    fn fruits(keys: &'static [&'static str]) -> CollectionMemo {
        Memo::new(move |_| {
            Arc::new(Collection::build(|b| {
                for key in keys {
                    b.item(*key, key.to_uppercase());
                }
            }))
        })
    }

    #[test]
    fn on_change_fires_only_when_the_value_changes() {
        Owner::new().with(|| {
            let changes = RwSignal::new(Vec::new());
            let state = use_select_state(UseSelectStateInput {
                default_value: vec![Key::from("b")],
                on_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                ..UseSelectStateInput::new(fruits(&["a", "b", "c"]))
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            state.list.selection.select(&Key::from("b"), None);
            state.list.selection.select(&Key::from("c"), None);
            assert_that!(changes.get_untracked()).is_equal_to(vec![vec![Key::from("c")]]);
        });
    }

    #[test]
    fn a_bound_value_is_shown_and_written() {
        Owner::new().with(|| {
            let value = RwSignal::new(vec![Key::from("b")]);
            let state = use_select_state(UseSelectStateInput {
                value: Some(value.into()),
                ..UseSelectStateInput::new(fruits(&["a", "b", "c"]))
            });
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("b")));
            // Selecting writes the app state.
            state.list.selection.select(&Key::from("c"), None);
            assert_that!(value.get_untracked()).is_equal_to(vec![Key::from("c")]);
            // Changes of the app state are shown, including clearing it.
            value.set(vec![Key::from("a")]);
            assert_that!(state.selected_key()).is_equal_to(Some(Key::from("a")));
            value.set(Vec::new());
            assert_that!(state.selected_key()).is_none();
        });
    }

    #[test]
    fn selecting_closes_in_single_mode() {
        Owner::new().with(|| {
            let state = use_select_state(UseSelectStateInput::new(fruits(&["a", "b"])));
            state.open(None);
            assert_that!(state.menu_trigger.overlay.is_open.get_untracked()).is_true();
            state.list.selection.select(&Key::from("a"), None);
            assert_that!(state.menu_trigger.overlay.is_open.get_untracked()).is_false();
        });
    }

    #[test]
    fn multiple_mode_keeps_collection_order_and_stays_open() {
        Owner::new().with(|| {
            let state = use_select_state(UseSelectStateInput {
                selection_mode: SelectMode::Multiple,
                ..UseSelectStateInput::new(fruits(&["a", "b", "c"]))
            });
            state.open(None);
            state.set_value(vec![Key::from("c"), Key::from("a")]);
            assert_that!(untrack(|| state.value()))
                .is_equal_to(vec![Key::from("a"), Key::from("c")]);
            assert_that!(state.menu_trigger.overlay.is_open.get_untracked()).is_true();
        });
    }

    #[test]
    fn single_mode_keeps_one_key() {
        Owner::new().with(|| {
            let state = use_select_state(UseSelectStateInput::new(fruits(&["a", "b"])));
            state.set_value(vec![Key::from("b"), Key::from("a")]);
            assert_that!(untrack(|| state.value())).is_equal_to(vec![Key::from("b")]);
        });
    }

    #[test]
    fn does_not_open_without_options() {
        Owner::new().with(|| {
            let state = use_select_state(UseSelectStateInput::new(fruits(&[])));
            state.open(None);
            assert_that!(state.menu_trigger.overlay.is_open.get_untracked()).is_false();
            let state = use_select_state(UseSelectStateInput {
                allows_empty_collection: true,
                ..UseSelectStateInput::new(fruits(&[]))
            });
            state.open(None);
            assert_that!(state.menu_trigger.overlay.is_open.get_untracked()).is_true();
        });
    }
}
