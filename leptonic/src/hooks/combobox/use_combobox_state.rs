// Upstream: react-stately/src/combobox/useComboBoxState.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use crate::hooks::{
    collections::{
        Collection, CollectionMemo, FocusStrategy, Key, ListState, Node, Selection, SelectionMode,
        SelectionOptions, UseListStateInput, use_list_state, use_list_state_view,
    },
    form::use_form_validation_state::{
        UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
        use_form_validation_state,
    },
    menu::use_menu_trigger_state::{UseMenuTriggerStateInput, use_menu_trigger_state},
    select::SelectMode,
};
use crate::utils::ValueBinding;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): `default_value`/`default_input_value` with `set_value`/
//   `set_input_value`, or `value`/`input_value` bound to app state, instead of controlled
//   `value`/`inputValue`; the deprecated `selectedKey` aliases are not offered.
// - The value is a `Vec<Key>` in both modes (at most one key in `Single` mode), in collection
//   order.
// - `filter: None` shows the collection as is (react-aria: controlled `items`, filtered by the
//   caller).
//
// =============================================================================

/// What opened the popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuTriggerAction {
    /// The input got focus (with `ComboBoxMenuTrigger::Focus`).
    Focus,
    /// The user typed.
    Input,
    /// The trigger button or ArrowDown/ArrowUp.
    Manual,
}

/// When the popover opens.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComboBoxMenuTrigger {
    /// When the user types.
    #[default]
    Input,
    /// When the input gets focus.
    Focus,
    /// Only by the button or ArrowDown/ArrowUp.
    Manual,
}

/// Decides whether an option's text matches the input (`option text`, `input`).
pub type ComboBoxFilter = Arc<dyn Fn(&str, &str) -> bool + Send + Sync>;

/// A filter matching options whose text contains the input, ignoring case and accents in the
/// current locale (react-aria: `useFilter({sensitivity: 'base'}).contains`).
pub fn use_contains_filter() -> ComboBoxFilter {
    let locale = crate::utils::i18n::use_locale();
    Arc::new(move |text: &str, input: &str| {
        crate::utils::filter::Filter::new(
            &locale.get_untracked(),
            &crate::utils::filter::CollatorOptions::default(),
        )
        .contains(text, input)
    })
}

/// The value validated by a combo box (react-aria validates the input text and the selection
/// together).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboBoxValue {
    pub input_value: String,
    pub value: Vec<Key>,
}

/// Input of [`use_combobox_state`].
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct UseComboBoxStateInput {
    /// All options.
    pub collection: CollectionMemo,
    /// Shows the options matching the input text. `None`: the collection is shown as is (e.g.
    /// because it is filtered on the server).
    pub filter: Option<ComboBoxFilter>,
    pub selection_mode: SelectMode,
    /// The initially selected keys (at most one in `Single` mode). Ignored when `value` is bound.
    pub default_value: Vec<Key>,
    /// The selected keys as app state, replacing `default_value`: the combo box shows them, and
    /// selecting writes them (in collection order).
    pub value: Option<ValueBinding<Vec<Key>>>,
    /// Called when the selected keys change.
    pub on_change: Option<Callback<Vec<Key>>>,
    /// The initial input text. `None`: the selected option's text. Ignored when `input_value` is
    /// bound.
    pub default_input_value: Option<String>,
    /// The input text as app state, replacing `default_input_value`.
    pub input_value: Option<ValueBinding<String>>,
    pub on_input_change: Option<Callback<String>>,
    pub disabled_keys: Signal<HashSet<Key>>,
    pub menu_trigger: ComboBoxMenuTrigger,
    /// Open the popover even without options (e.g. to show an empty state).
    pub allows_empty_collection: bool,
    /// Keep text that matches no option (the value is cleared) instead of resetting it.
    pub allows_custom_value: bool,
    /// Commit (or revert) the input when focus leaves the combo box.
    pub should_close_on_blur: bool,
    pub is_read_only: Signal<bool>,
    pub on_open_change: Option<Callback<(bool, Option<MenuTriggerAction>)>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<ComboBoxValue>>,
    pub validation_behavior: ValidationBehavior,
    pub name: Option<String>,
}

impl UseComboBoxStateInput {
    /// A single-selection combo box over `collection`, showing all options (no filter).
    pub fn new(collection: CollectionMemo) -> Self {
        Self {
            collection,
            filter: None,
            selection_mode: SelectMode::Single,
            default_value: Vec::new(),
            value: None,
            on_change: None,
            default_input_value: None,
            input_value: None,
            on_input_change: None,
            disabled_keys: Signal::stored(HashSet::new()),
            menu_trigger: ComboBoxMenuTrigger::Input,
            allows_empty_collection: false,
            allows_custom_value: false,
            should_close_on_blur: true,
            is_read_only: Signal::stored(false),
            on_open_change: None,
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The state of a combo box: its options (filtered by the input text), selection, input text,
/// popover and validation.
#[derive(Clone, Copy)]
pub struct ComboBoxState {
    /// The displayed options (filtered while open) with their selection and focus.
    pub list: ListState,
    pub selection_mode: SelectMode,
    pub validation: UseFormValidationStateReturn,
    inner: StoredValue<Inner>,
}

// The settings mirror `UseComboBoxStateInput`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
struct Inner {
    original: CollectionMemo,
    filtered: CollectionMemo,
    input_value: Signal<String>,
    input_binding: ValueBinding<String>,
    is_focused: RwSignal<bool>,
    show_all_items: RwSignal<bool>,
    last_collection: RwSignal<Arc<Collection>>,
    menu: crate::hooks::menu::use_menu_trigger_state::MenuTriggerState,
    menu_open_trigger: StoredValue<Option<MenuTriggerAction>>,
    closed_due_to_empty: StoredValue<bool>,
    last_value: StoredValue<String>,
    last_reported: StoredValue<Vec<Key>>,
    value_on_focus: StoredValue<(String, Vec<Key>)>,
    default_value: StoredValue<Vec<Key>>,
    default_input_value: StoredValue<String>,
    on_input_change: Option<Callback<String>>,
    on_open_change: Option<Callback<(bool, Option<MenuTriggerAction>)>>,
    menu_trigger: ComboBoxMenuTrigger,
    allows_empty_collection: bool,
    allows_custom_value: bool,
    should_close_on_blur: bool,
    is_read_only: Signal<bool>,
    filtering: bool,
}

impl crate::hooks::OverlayState for ComboBoxState {
    fn is_open(&self) -> bool {
        ComboBoxState::is_open(self)
    }

    /// Closes the popover with the combo box's own logic (committing or reverting the input).
    fn close(&self) {
        ComboBoxState::close(self);
    }
}

impl crate::hooks::menu::use_menu_trigger_state::MenuTriggerStateApi for ComboBoxState {
    fn focus_strategy(&self) -> Option<FocusStrategy> {
        ComboBoxState::focus_strategy(self)
    }

    /// The button opens the popover like a menu (react-aria: the combo box state passed to
    /// `useMenuTrigger`), with a manual trigger.
    fn open(&self, focus_strategy: Option<FocusStrategy>) {
        ComboBoxState::open(self, focus_strategy, MenuTriggerAction::Manual);
    }

    fn toggle(&self, focus_strategy: Option<FocusStrategy>) {
        ComboBoxState::toggle(self, focus_strategy, MenuTriggerAction::Manual);
    }
}

impl ComboBoxState {
    fn inner(&self) -> Inner {
        self.inner.get_value()
    }

    /// The selected keys, in collection order.
    pub fn value(&self) -> Vec<Key> {
        let inner = self.inner();
        ordered(inner.original, self.list.selection.selected_keys())
    }

    /// The selected key (`Single` mode).
    pub fn selected_key(&self) -> Option<Key> {
        match self.selection_mode {
            SelectMode::Single => self.value().into_iter().next(),
            SelectMode::Multiple => None,
        }
    }

    /// The nodes of the selected options.
    pub fn selected_items(&self) -> Vec<Node> {
        let value = self.value();
        self.inner()
            .original
            .with(|c| value.iter().filter_map(|key| c.get(key).cloned()).collect())
    }

    /// Select `keys` (at most one in `Single` mode).
    pub fn set_value(&self, keys: Vec<Key>) {
        let keys: Vec<Key> = match self.selection_mode {
            SelectMode::Single => keys.into_iter().take(1).collect(),
            SelectMode::Multiple => keys,
        };
        self.list.selection.set_selected_keys(keys);
    }

    /// The value the combo box started with (restored on form reset).
    pub fn default_value(&self) -> Vec<Key> {
        self.inner().default_value.get_value()
    }

    pub fn input_value(&self) -> String {
        self.inner().input_value.get()
    }

    /// Whether typed text that matches no option is kept.
    pub fn allows_custom_value(&self) -> bool {
        self.inner().allows_custom_value
    }

    /// The input text as a signal.
    pub fn input_value_signal(&self) -> Signal<String> {
        self.inner().input_value
    }

    pub fn default_input_value(&self) -> String {
        self.inner().default_input_value.get_value()
    }

    pub fn set_input_value(&self, value: String) {
        let inner = self.inner();
        if inner.input_value.with_untracked(|v| *v != value) {
            inner.input_binding.set(value.clone());
            if let Some(on_input_change) = inner.on_input_change {
                on_input_change.run(value);
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.inner().menu.overlay.is_open.get()
    }

    /// How focus enters the listbox when the popover opens.
    pub fn focus_strategy(&self) -> Option<FocusStrategy> {
        self.inner().menu.focus_strategy.get()
    }

    pub fn is_focused(&self) -> bool {
        self.inner().is_focused.get()
    }

    /// Opens the popover, if there are options to show (or `allows_empty_collection`).
    pub fn open(&self, focus_strategy: Option<FocusStrategy>, trigger: MenuTriggerAction) {
        let inner = self.inner();
        let display_all = self.displays_all_items(trigger);
        if self.can_open(display_all) {
            if display_all && !inner.menu.overlay.is_open.get_untracked() && inner.filtering {
                inner.show_all_items.set(true);
            }
            inner.menu_open_trigger.set_value(Some(trigger));
            inner.menu.open(focus_strategy);
        }
    }

    /// Opens or closes the popover.
    pub fn toggle(&self, focus_strategy: Option<FocusStrategy>, trigger: MenuTriggerAction) {
        let inner = self.inner();
        let display_all = self.displays_all_items(trigger);
        let is_open = inner.menu.overlay.is_open.get_untracked();
        if !self.can_open(display_all) && !is_open {
            return;
        }
        if display_all && !is_open && inner.filtering {
            inner.show_all_items.set(true);
        }
        if !is_open {
            inner.menu_open_trigger.set_value(Some(trigger));
        }
        if is_open {
            self.update_last_collection();
        }
        inner.menu.toggle(focus_strategy);
    }

    /// Commits the input (see [`ComboBoxState::commit`] without a focused option) and closes the
    /// popover.
    pub fn close(&self) {
        self.commit_value();
    }

    /// Enter: selects the focused option, or commits the typed text.
    pub fn commit(&self) {
        let focused = untrack(|| self.list.selection.focused_key());
        if untrack(|| self.is_open())
            && let Some(focused) = focused
        {
            if untrack(|| self.list.selection.is_selected(&focused))
                && self.selection_mode == SelectMode::Single
            {
                self.commit_selection();
            } else {
                self.list.selection.select(&focused, None);
            }
        } else {
            self.commit_value();
        }
    }

    /// Escape: restores the input to the selected option's text (or keeps custom text).
    pub fn revert(&self) {
        let inner = self.inner();
        inner.closed_due_to_empty.set_value(false);
        if inner.allows_custom_value && untrack(|| self.selected_key()).is_none() {
            self.commit_custom_value();
        } else {
            self.commit_selection();
        }
    }

    /// Focus entered (`true`) or left (`false`) the combo box.
    pub fn set_focused(&self, focused: bool) {
        let inner = self.inner();
        if focused {
            inner
                .value_on_focus
                .set_value((inner.input_value.get_untracked(), untrack(|| self.value())));
            if inner.menu_trigger == ComboBoxMenuTrigger::Focus
                && !inner.is_read_only.get_untracked()
            {
                self.open(None, MenuTriggerAction::Focus);
            }
        } else {
            if inner.should_close_on_blur {
                self.commit_value();
            }
            let (input_on_focus, value_on_focus) = inner.value_on_focus.get_value();
            if inner.input_value.get_untracked() != input_on_focus
                || untrack(|| self.value()) != value_on_focus
            {
                self.validation.commit_validation.run(());
            }
        }
        inner.is_focused.set(focused);
    }

    fn displays_all_items(&self, trigger: MenuTriggerAction) -> bool {
        trigger == MenuTriggerAction::Manual
            || (trigger == MenuTriggerAction::Focus
                && self.inner().menu_trigger == ComboBoxMenuTrigger::Focus)
    }

    fn can_open(&self, display_all: bool) -> bool {
        let inner = self.inner();
        inner.allows_empty_collection
            || inner.filtered.with_untracked(|c| c.size() > 0)
            || (display_all && inner.original.with_untracked(|c| c.size() > 0))
    }

    fn update_last_collection(&self) {
        let inner = self.inner();
        let shown = if inner.show_all_items.get_untracked() {
            inner.original.get_untracked()
        } else {
            inner.filtered.get_untracked()
        };
        inner.last_collection.set(shown);
    }

    fn close_menu(&self) {
        let inner = self.inner();
        if inner.menu.overlay.is_open.get_untracked() {
            self.update_last_collection();
            inner.menu.close();
        }
    }

    fn selected_text(&self) -> String {
        let inner = self.inner();
        untrack(|| self.selected_key())
            .and_then(|key| {
                inner
                    .original
                    .with_untracked(|c| c.get(&key).map(|n| n.text_value.to_string()))
            })
            .unwrap_or_default()
    }

    fn reset_input_value(&self) {
        let text = self.selected_text();
        self.inner().last_value.set_value(text.clone());
        self.set_input_value(text);
    }

    fn commit_custom_value(&self) {
        let inner = self.inner();
        if self.selection_mode == SelectMode::Multiple {
            inner
                .last_value
                .set_value(inner.input_value.get_untracked());
            self.close_menu();
            return;
        }
        inner.last_reported.set_value(Vec::new());
        self.set_value(Vec::new());
        self.close_menu();
    }

    fn commit_selection(&self) {
        self.reset_input_value();
        self.close_menu();
    }

    fn commit_value(&self) {
        let inner = self.inner();
        inner.closed_due_to_empty.set_value(false);
        if inner.allows_custom_value && inner.input_value.get_untracked() != self.selected_text() {
            self.commit_custom_value();
        } else {
            self.commit_selection();
        }
    }
}

/// Creates the state of a combo box (see [`ComboBoxState`]).
#[allow(clippy::too_many_lines)]
pub fn use_combobox_state(input: UseComboBoxStateInput) -> ComboBoxState {
    let UseComboBoxStateInput {
        collection: original,
        filter,
        selection_mode,
        default_value,
        value,
        on_change,
        default_input_value,
        input_value: input_binding,
        on_input_change,
        disabled_keys,
        menu_trigger,
        allows_empty_collection,
        allows_custom_value,
        should_close_on_blur,
        is_read_only,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;
    let filtering = filter.is_some();

    // The combo box state is needed by callbacks of the list state created below.
    let state_slot: StoredValue<Option<ComboBoxState>> = StoredValue::new(None);
    let with_state = move |f: &dyn Fn(ComboBoxState)| {
        if let Some(state) = state_slot.get_value() {
            f(state);
        }
    };

    let default_value = value.map_or(default_value, |value| value.value.get_untracked());
    let default_input_value =
        input_binding.map_or(default_input_value, |text| Some(text.value.get_untracked()));
    let initial_text = default_input_value.clone().unwrap_or_else(|| {
        default_value
            .first()
            .filter(|_| selection_mode == SelectMode::Single)
            .and_then(|key| {
                original.with_untracked(|c| c.get(key).map(|n| n.text_value.to_string()))
            })
            .unwrap_or_default()
    });
    let input_binding =
        input_binding.unwrap_or_else(|| ValueBinding::from(RwSignal::new(initial_text.clone())));
    let input_value = input_binding.value;

    let filtered: CollectionMemo = Memo::new(move |_| match &filter {
        Some(filter) => {
            let text = input_value.get();
            original
                .with(|c| Arc::new(c.filter(|item_text: &str, _: &Node| filter(item_text, &text))))
        }
        None => original.get(),
    });

    let last_reported = StoredValue::new(default_value.clone());
    let base = use_list_state(UseListStateInput {
        collection: original,
        selection: SelectionOptions {
            selection_mode: Signal::stored(match selection_mode {
                SelectMode::Single => SelectionMode::Single,
                SelectMode::Multiple => SelectionMode::Multiple,
            }),
            default_selection: Selection::keys(default_value.clone()),
            // The bound value as the list's selection ("select all" doesn't apply).
            selection: value.map(|value| {
                ValueBinding::new(
                    Signal::derive(move || Selection::keys(value.value.get())),
                    Callback::new(move |selection: Selection| {
                        if let Selection::Keys(keys) = selection {
                            value.set(ordered(original, keys));
                        }
                    }),
                )
            }),
            on_selection_change: Some(Callback::new(move |selection: Selection| {
                let Selection::Keys(keys) = selection else {
                    return;
                };
                let value = ordered(original, keys);
                if value == last_reported.get_value() {
                    // The selected option was picked again.
                    if selection_mode == SelectMode::Single {
                        with_state(&|state| {
                            state.reset_input_value();
                            state.close_menu();
                        });
                    }
                    return;
                }
                last_reported.set_value(value.clone());
                if let Some(on_change) = on_change {
                    on_change.run(value);
                }
            })),
            disallow_empty_selection: Signal::stored(selection_mode == SelectMode::Single),
            disabled_keys,
            allow_duplicate_selection_events: true,
            ..SelectionOptions::default()
        },
    });

    let menu = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open: false,
        on_open_change: Some(Callback::new(move |open: bool| {
            with_state(&|state| {
                let inner = state.inner();
                if let Some(on_open_change) = inner.on_open_change {
                    on_open_change.run((
                        open,
                        open.then(|| inner.menu_open_trigger.get_value()).flatten(),
                    ));
                }
                state.list.selection.set_focused(open);
                if !open {
                    state.list.selection.set_focused_key(None, None);
                }
            });
        })),
        ..UseMenuTriggerStateInput::default()
    });

    let show_all_items = RwSignal::new(false);
    let last_collection = RwSignal::new(filtered.get_untracked());
    let displayed: CollectionMemo = Memo::new(move |_| {
        if menu.overlay.is_open.get() {
            if show_all_items.get() {
                original.get()
            } else {
                filtered.get()
            }
        } else {
            last_collection.get()
        }
    });
    let list = use_list_state_view(base, displayed);

    let selection = base.selection;
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: Signal::derive(move || ComboBoxValue {
            input_value: input_value.get(),
            value: ordered(original, selection.selected_keys()),
        }),
        validate,
        validation_behavior,
        name,
    });

    let is_focused = RwSignal::new(false);
    let inner = Inner {
        original,
        filtered,
        input_value,
        input_binding,
        is_focused,
        show_all_items,
        last_collection,
        menu,
        menu_open_trigger: StoredValue::new(None),
        closed_due_to_empty: StoredValue::new(false),
        last_value: StoredValue::new(initial_text.clone()),
        last_reported,
        value_on_focus: StoredValue::new((String::new(), Vec::new())),
        default_value: StoredValue::new(default_value.clone()),
        default_input_value: StoredValue::new(initial_text),
        on_input_change,
        on_open_change,
        menu_trigger,
        allows_empty_collection,
        allows_custom_value,
        should_close_on_blur,
        is_read_only,
        filtering,
    };
    let state = ComboBoxState {
        list,
        selection_mode,
        validation,
        inner: StoredValue::new(inner),
    };
    state_slot.set_value(Some(state));

    // Reconciles the popover, input text and selection after changes, as react-aria does on
    // every render.
    let last_value_ref = StoredValue::new(default_value);
    let last_selected_text = StoredValue::new(untrack(|| state.selected_text()));
    Effect::new(move |_| {
        let focused = is_focused.get();
        let is_open = menu.overlay.is_open.get();
        let input = input_value.get();
        let filtered_size = filtered.with(|c| c.size());
        let show_all = show_all_items.get();
        let value = ordered(original, selection.selected_keys());
        let selected_text = original.with(|c| {
            value
                .first()
                .filter(|_| selection_mode == SelectMode::Single)
                .and_then(|key| c.get(key).map(|n| n.text_value.to_string()))
                .unwrap_or_default()
        });

        untrack(|| {
            let last_value = inner.last_value.get_value();
            let input_changed = input != last_value;
            let value_changed = value != last_value_ref.get_value();

            // Typing opens the popover.
            if focused
                && (filtered_size > 0 || allows_empty_collection)
                && !is_open
                && input_changed
                && menu_trigger != ComboBoxMenuTrigger::Manual
            {
                state.open(None, MenuTriggerAction::Input);
            }
            // Nothing matches: close, and reopen once something matches again.
            if !show_all && !allows_empty_collection && is_open && filtered_size == 0 {
                inner.closed_due_to_empty.set_value(true);
                state.close_menu();
            }
            if focused
                && inner.closed_due_to_empty.get_value()
                && filtered_size > 0
                && !is_open
                && menu_trigger != ComboBoxMenuTrigger::Manual
            {
                inner.closed_due_to_empty.set_value(false);
                state.open(None, MenuTriggerAction::Input);
            }
            // Selecting an option closes the popover (single selection).
            if !value.is_empty() && value_changed && selection_mode == SelectMode::Single {
                state.close_menu();
            }
            if input_changed {
                selection.set_focused_key(None, None);
                show_all_items.set(false);
                if selection_mode == SelectMode::Single && input.is_empty() {
                    state.set_value(Vec::new());
                }
            }
            // A new selection shows its text in the input.
            if value_changed {
                state.reset_input_value();
            } else if input_changed {
                inner.last_value.set_value(input.clone());
            }
            // The selected option's text changed (e.g. the collection was reloaded).
            if !focused
                && selection_mode == SelectMode::Single
                && !value.is_empty()
                && !value_changed
                && last_selected_text.get_value() != selected_text
            {
                inner.last_value.set_value(selected_text.clone());
                state.set_input_value(selected_text.clone());
            }
            last_value_ref.set_value(value);
            last_selected_text.set_value(selected_text);
        });
    });

    state
}

/// `keys` in collection order.
fn ordered(collection: CollectionMemo, keys: impl IntoIterator<Item = Key>) -> Vec<Key> {
    collection.with(|c| c.sorted_keys(keys))
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn fruits() -> CollectionMemo {
        Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                b.item("apple", "Apple");
                b.item("banana", "Banana");
            }))
        })
    }

    #[test]
    fn bound_value_and_input_text_are_shown_and_written() {
        Owner::new().with(|| {
            let value = RwSignal::new(vec![Key::from("banana")]);
            let text = RwSignal::new("Ban".to_owned());
            let state = use_combobox_state(UseComboBoxStateInput {
                value: Some(value.into()),
                input_value: Some(text.into()),
                ..UseComboBoxStateInput::new(fruits())
            });
            assert_that!(state.value()).is_equal_to(vec![Key::from("banana")]);
            assert_that!(state.input_value()).is_equal_to("Ban".to_owned());
            // Typing writes the app state, and changes of it are shown.
            state.set_input_value("App".to_owned());
            assert_that!(text.get_untracked()).is_equal_to("App".to_owned());
            text.set("Apple".to_owned());
            assert_that!(state.input_value()).is_equal_to("Apple".to_owned());
            // Selecting writes the app state, and changes of it are shown.
            state.list.selection.select(&Key::from("apple"), None);
            assert_that!(value.get_untracked()).is_equal_to(vec![Key::from("apple")]);
            value.set(Vec::new());
            assert_that!(state.value()).is_empty();
        });
    }
}
