// Upstream: react-stately/src/combobox/useComboBoxState.ts @ 99e6102368
// Upstream: react-stately/test/combobox/useComboBoxState.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/combobox/ComboBox.test.js @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::{
        collections::{
            Collection, CollectionMemo, FocusStrategy, Key, ListState, Node, Selection,
            SelectionMode, SelectionOptions, UseListStateInput, UseListStateViewInput,
            use_list_state, use_list_state_view,
        },
        form::use_form_validation_state::{
            FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
            use_form_validation_state,
        },
        menu::use_menu_trigger_state::{UseMenuTriggerStateInput, use_menu_trigger_state},
        select::SelectMode,
    },
};

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
// - `on_open_change` receives a `ComboBoxOpenChange` (react-aria: `(isOpen, menuTrigger)`).
//
// ## DIFFERENT BEHAVIOR
// - With both `value` and `input_value` bound to app state, the combo box still keeps the text in
//   sync with the selection (the selected option's text on selection and commit, an emptied text
//   clears a single selection); react-aria leaves both to the app then.
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

/// The popover opened or closed (`on_open_change`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComboBoxOpenChange {
    pub is_open: bool,
    /// What opened it; `None` when it closed.
    pub trigger: Option<MenuTriggerAction>,
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

/// Decides which options match the input: called with the input text whenever it changes, it
/// returns the matcher of option texts, so the input is prepared once, not once per option.
///
/// ```ignore
/// let starts_with: ComboBoxFilter = Arc::new(|input: &str| {
///     let input = input.to_lowercase();
///     Box::new(move |text: &str| text.to_lowercase().starts_with(&input))
/// });
/// ```
pub type ComboBoxFilter = Arc<dyn Fn(&str) -> OptionMatcher + Send + Sync>;

/// Whether an option's text matches the input (see [`ComboBoxFilter`]).
pub type OptionMatcher = Box<dyn Fn(&str) -> bool>;

/// A filter matching options whose text contains the input, ignoring case and accents in the
/// current locale (react-aria: `useFilter({sensitivity: 'base'}).contains`).
pub fn use_contains_filter() -> ComboBoxFilter {
    // One filter per locale; reading it tracked lets a filtered collection follow the locale.
    let filter = crate::utils::filter::use_filter(crate::utils::filter::CollatorOptions::default());
    Arc::new(move |input: &str| {
        let filter = filter.get();
        let query = crate::utils::filter::FilterQuery::new(input);
        Box::new(move |text: &str| filter.contains(text, &query))
    })
}

/// The value validated by a combo box (react-aria validates the input text and the selection
/// together): the selected keys here, the typed value (`Option<V>` or `Vec<V>`) in the
/// `ComboBox` atom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboBoxValue<T = Vec<Key>> {
    pub input_value: String,
    pub value: T,
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
    /// When the popover opens.
    pub menu_trigger: Signal<ComboBoxMenuTrigger>,
    /// Open the popover even without options (e.g. to show an empty state).
    pub allows_empty_collection: Signal<bool>,
    /// Keep text that matches no option (the value is cleared) instead of resetting it.
    pub allows_custom_value: Signal<bool>,
    /// Commit (or revert) the input when focus leaves the combo box.
    pub should_close_on_blur: bool,
    pub is_read_only: Signal<bool>,
    pub on_open_change: Option<Callback<ComboBoxOpenChange>>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<ComboBoxValue>>,
    pub validation_behavior: ValidationBehavior,
    pub name: Option<String>,
}

impl std::fmt::Debug for UseComboBoxStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseComboBoxStateInput")
            .field("selection_mode", &self.selection_mode)
            .field("default_value", &self.default_value)
            .field("default_input_value", &self.default_input_value)
            .field("validation_behavior", &self.validation_behavior)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// The state of a combo box: its options (filtered by the input text), selection, input text,
/// popover and validation.
#[derive(Clone, Copy)]
pub struct ComboBoxState {
    /// The displayed options (filtered while open) with their selection and focus.
    pub list: ListState,
    pub selection_mode: SelectMode,
    pub validation: FormValidationState,
    inner: Inner,
}

impl std::fmt::Debug for ComboBoxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComboBoxState")
            .field("list", &self.list)
            .field("selection_mode", &self.selection_mode)
            .finish_non_exhaustive()
    }
}

// The settings mirror `UseComboBoxStateInput`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
struct Inner {
    original: CollectionMemo,
    filtered: CollectionMemo,
    /// The selected keys, in collection order.
    value: Memo<Vec<Key>>,
    input_value: Signal<String>,
    input_binding: ValueBinding<String>,
    is_focused: RwSignal<bool>,
    show_all_items: RwSignal<bool>,
    last_collection: RwSignal<Arc<Collection>>,
    menu: crate::hooks::menu::use_menu_trigger_state::MenuTriggerState,
    menu_open_trigger: StoredValue<Option<MenuTriggerAction>>,
    closed_due_to_empty: StoredValue<bool>,
    last_value: StoredValue<String>,
    /// The value the reconciliation saw last (react-aria: `lastValueRef`).
    last_value_ref: StoredValue<Vec<Key>>,
    value_on_focus: StoredValue<(String, Vec<Key>)>,
    default_value: StoredValue<Vec<Key>>,
    default_input_value: StoredValue<String>,
    on_input_change: Option<Callback<String>>,
    on_open_change: Option<Callback<ComboBoxOpenChange>>,
    menu_trigger: Signal<ComboBoxMenuTrigger>,
    allows_empty_collection: Signal<bool>,
    allows_custom_value: Signal<bool>,
    should_close_on_blur: bool,
    is_read_only: Signal<bool>,
    name: StoredValue<Option<String>>,
    validation_behavior: ValidationBehavior,
    filtering: bool,
}

impl crate::hooks::overlay::OverlayState for ComboBoxState {
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

    fn set_point(&self, point: Option<crate::Point>) {
        self.inner.menu.overlay.set_point(point);
    }
}

impl ComboBoxState {
    /// The selected keys, in collection order.
    pub fn value(&self) -> Vec<Key> {
        self.inner.value.get()
    }

    /// The selected key (`Single` mode).
    pub fn selected_key(&self) -> Option<Key> {
        match self.selection_mode {
            SelectMode::Single => self.inner.value.with(|value| value.first().cloned()),
            SelectMode::Multiple => None,
        }
    }

    /// The nodes of the selected options.
    pub fn selected_items(&self) -> Vec<Node> {
        let value = self.value();
        self.inner
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
        self.inner.default_value.get_value()
    }

    pub fn input_value(&self) -> String {
        self.inner.input_value.get()
    }

    /// Whether the combo box is read-only (the popover doesn't open, the text can't change).
    pub fn is_read_only_signal(&self) -> Signal<bool> {
        self.inner.is_read_only
    }

    /// The form field name.
    pub fn name(&self) -> Option<String> {
        self.inner.name.get_value()
    }

    /// How validation errors are shown (native form validation or ARIA only).
    pub fn validation_behavior(&self) -> ValidationBehavior {
        self.inner.validation_behavior
    }

    /// Whether typed text that matches no option is kept (tracked).
    pub fn allows_custom_value(&self) -> bool {
        self.inner.allows_custom_value.get()
    }

    /// The input text as a signal.
    pub fn input_value_signal(&self) -> Signal<String> {
        self.inner.input_value
    }

    pub fn default_input_value(&self) -> String {
        self.inner.default_input_value.get_value()
    }

    pub fn set_input_value(&self, value: String) {
        let inner = self.inner;
        if inner.input_value.with_untracked(|v| *v != value) {
            inner.input_binding.set(value.clone());
            if let Some(on_input_change) = inner.on_input_change {
                on_input_change.run(value);
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.inner.menu.overlay.is_open.get()
    }

    /// How focus enters the listbox when the popover opens.
    pub fn focus_strategy(&self) -> Option<FocusStrategy> {
        self.inner.menu.focus_strategy.get()
    }

    pub fn is_focused(&self) -> bool {
        self.inner.is_focused.get()
    }

    /// Opens the popover, if there are options to show (or `allows_empty_collection`).
    pub fn open(&self, focus_strategy: Option<FocusStrategy>, trigger: MenuTriggerAction) {
        let inner = self.inner;
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
        let inner = self.inner;
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
        let inner = self.inner;
        inner.closed_due_to_empty.set_value(false);
        if inner.allows_custom_value.get_untracked() && untrack(|| self.selected_key()).is_none() {
            self.commit_custom_value();
        } else {
            self.commit_selection();
        }
    }

    /// Focus entered (`true`) or left (`false`) the combo box.
    pub fn set_focused(&self, focused: bool) {
        let inner = self.inner;
        if focused {
            inner
                .value_on_focus
                .set_value((inner.input_value.get_untracked(), untrack(|| self.value())));
            if inner.menu_trigger.get_untracked() == ComboBoxMenuTrigger::Focus
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
                self.validation.commit_validation();
            }
        }
        inner.is_focused.set(focused);
    }

    fn displays_all_items(&self, trigger: MenuTriggerAction) -> bool {
        trigger == MenuTriggerAction::Manual
            || (trigger == MenuTriggerAction::Focus
                && self.inner.menu_trigger.get_untracked() == ComboBoxMenuTrigger::Focus)
    }

    /// Whether there is something to show: options, or `allows_empty_collection`. A combo box
    /// filtered by the app (no `filter`) always opens, even while its collection is empty, so
    /// that the app can load the options on opening (react-aria: controlled `items`).
    fn can_open(&self, display_all: bool) -> bool {
        let inner = self.inner;
        inner.allows_empty_collection.get_untracked()
            || inner.filtered.with_untracked(|c| c.size() > 0)
            || (display_all && inner.original.with_untracked(|c| c.size() > 0))
            || !inner.filtering
    }

    fn update_last_collection(&self) {
        let inner = self.inner;
        let shown = if inner.show_all_items.get_untracked() {
            inner.original.get_untracked()
        } else {
            inner.filtered.get_untracked()
        };
        inner.last_collection.set(shown);
    }

    fn close_menu(&self) {
        let inner = self.inner;
        if inner.menu.overlay.is_open.get_untracked() {
            self.update_last_collection();
            inner.menu.close();
        }
    }

    fn selected_text(&self) -> String {
        let inner = self.inner;
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
        self.inner.last_value.set_value(text.clone());
        self.set_input_value(text);
    }

    fn commit_custom_value(&self) {
        let inner = self.inner;
        if self.selection_mode == SelectMode::Multiple {
            inner
                .last_value
                .set_value(inner.input_value.get_untracked());
            self.close_menu();
            return;
        }
        // The reconciliation must not reset the input to the (empty) selection's text: the
        // custom text stays.
        inner.last_value_ref.set_value(Vec::new());
        if !untrack(|| self.value()).is_empty() {
            self.set_value(Vec::new());
        }
        self.close_menu();
    }

    fn commit_selection(&self) {
        // Closing first keeps the options shown for the typed text (the input's reset would
        // filter them again).
        self.close_menu();
        self.reset_input_value();
    }

    fn commit_value(&self) {
        let inner = self.inner;
        inner.closed_due_to_empty.set_value(false);
        if inner.allows_custom_value.get_untracked()
            && inner.input_value.get_untracked() != self.selected_text()
        {
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
            let matches = filter(&input_value.get());
            original.with(|c| Arc::new(c.filter(|item_text: &str, _: &Node| matches(item_text))))
        }
        None => original.get(),
    });

    // The value `on_change` last reported, to report only changes. A bound value changed by the app
    // counts as reported (picking the previous value again is a change).
    let last_reported = StoredValue::new(value.map_or_else(
        || default_value.clone(),
        |value| value.value.get_untracked(),
    ));
    if let Some(value) = value {
        Effect::watch(
            move || value.value.get(),
            move |value, _, _| last_reported.set_value(value.clone()),
            false,
        );
    }
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
                            state.close_menu();
                            state.reset_input_value();
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
                let inner = state.inner;
                if let Some(on_open_change) = inner.on_open_change {
                    on_open_change.run(ComboBoxOpenChange {
                        is_open: open,
                        trigger: open.then(|| inner.menu_open_trigger.get_value()).flatten(),
                    });
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
    let list = use_list_state_view(UseListStateViewInput {
        state: base,
        collection: displayed,
    });
    // The displayed collection drives rendering and the view's focus reconciliation, but the
    // public selection manager acts on all options, as react-stately's combo box does. A caller
    // can select an option even while the previous selection's text filters it out.
    let list = ListState {
        selection: base.selection,
        ..list
    };

    let selection = base.selection;
    let value = Memo::new(move |_| ordered(original, selection.selected_keys()));
    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid,
        value: Signal::derive(move || ComboBoxValue {
            input_value: input_value.get(),
            value: value.get(),
        }),
        validate,
        validation_behavior,
        names: name.clone().into_iter().collect(),
    });

    let is_focused = RwSignal::new(false);
    let inner = Inner {
        original,
        filtered,
        value,
        input_value,
        input_binding,
        is_focused,
        show_all_items,
        last_collection,
        menu,
        menu_open_trigger: StoredValue::new(None),
        closed_due_to_empty: StoredValue::new(false),
        last_value: StoredValue::new(initial_text.clone()),
        last_value_ref: StoredValue::new(default_value.clone()),
        value_on_focus: StoredValue::new((String::new(), Vec::new())),
        default_value: StoredValue::new(default_value),
        default_input_value: StoredValue::new(initial_text),
        on_input_change,
        on_open_change,
        menu_trigger,
        allows_empty_collection,
        allows_custom_value,
        should_close_on_blur,
        is_read_only,
        name: StoredValue::new(name),
        validation_behavior,
        filtering,
    };
    let state = ComboBoxState {
        list,
        selection_mode,
        validation,
        inner,
    };
    state_slot.set_value(Some(state));

    // Reconciles the popover, input text and selection after changes, as react-aria does on
    // every render.
    let last_value_ref = inner.last_value_ref;
    let last_selected_text = StoredValue::new(untrack(|| state.selected_text()));
    Effect::new(move |_| {
        // Upstream sources before the memos derived from them (the input text, value and
        // collection before `filtered`). reactive_graph 0.2 checks an effect's sources in read order, with
        // the effect as observer: a memo recomputed while checking a downstream memo doesn't mark
        // the effect dirty, and is clean (unchanged) when checked itself. A value and collection
        // derived from one app memo, read after `filtered` (unchanged by the update), never
        // re-ran this effect (agnite dev-ui, `combobox_tests`).
        let input = input_value.get();
        let value = ordered(original, selection.selected_keys());
        let selected_text = original.with(|c| {
            value
                .first()
                .filter(|_| selection_mode == SelectMode::Single)
                .and_then(|key| c.get(key).map(|n| n.text_value.to_string()))
                .unwrap_or_default()
        });
        let focused = is_focused.get();
        let is_open = menu.overlay.is_open.get();
        let filtered_size = filtered.with(|c| c.size());
        let show_all = show_all_items.get();

        untrack(|| {
            let menu_trigger = menu_trigger.get();
            let allows_empty_collection = allows_empty_collection.get();
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
    use crate::testing::{flush_effects, with_owner};

    fn fruits() -> CollectionMemo {
        Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                b.item("apple", "Apple");
                b.item("banana", "Banana");
            }))
        })
    }

    fn fruit_input() -> UseComboBoxStateInput {
        UseComboBoxStateInput {
            collection: Memo::new(|_| {
                Arc::new(Collection::build(|b| {
                    b.item("apple", "Apple");
                    b.item("banana", "Banana");
                    b.item("cherry", "Cherry");
                }))
            }),
            filter: Some(use_contains_filter()),
            selection_mode: SelectMode::Single,
            default_value: Vec::new(),
            value: None,
            on_change: None,
            default_input_value: None,
            input_value: None,
            on_input_change: None,
            disabled_keys: Signal::stored(HashSet::new()),
            menu_trigger: Signal::stored(ComboBoxMenuTrigger::Input),
            allows_empty_collection: Signal::stored(false),
            allows_custom_value: Signal::stored(false),
            should_close_on_blur: true,
            is_read_only: Signal::stored(false),
            on_open_change: None,
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }

    /// The keys of the displayed options.
    fn displayed(state: &ComboBoxState) -> Vec<String> {
        state
            .list
            .collection
            .with_untracked(|c| c.items().map(|n| n.key.to_string()).collect())
    }

    /// Records every `on_change` call.
    fn changes() -> (RwSignal<Vec<Vec<Key>>>, Callback<Vec<Key>>) {
        let changes = RwSignal::new(Vec::new());
        let callback = Callback::new(move |value: Vec<Key>| changes.update(|c| c.push(value)));
        (changes, callback)
    }

    /// The user focuses the combo box and types `text`.
    fn type_text(state: &ComboBoxState, text: &str) {
        state.set_focused(true);
        flush_effects();
        state.set_input_value(text.to_owned());
        flush_effects();
    }

    // Upstream: useComboBoxState's `commitCustomValue` (ComboBox.test.js "allowsCustomValue").
    #[test]
    fn committing_custom_text_keeps_it_and_clears_the_selection() {
        with_owner(|| {
            let (changes, on_change) = changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                default_value: vec![Key::from("apple")],
                on_change: Some(on_change),
                allows_custom_value: Signal::stored(true),
                ..fruit_input()
            });
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Apple".to_owned());

            type_text(&state, "Kiwi");
            state.commit();
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Kiwi".to_owned());
            assert_that!(state.value()).is_empty();
            assert_that!(state.is_open()).is_false();
            assert_that!(changes.get_untracked()).is_equal_to(vec![Vec::new()]);

            // Committing the same custom text again reports nothing.
            state.commit();
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Kiwi".to_owned());
            assert_that!(changes.get_untracked().len()).is_equal_to(1);
        });
    }

    #[test]
    fn blur_commits_custom_text() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                allows_custom_value: Signal::stored(true),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "Kiwi");
            assert_that!(state.is_open()).is_false();
            state.set_focused(false);
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Kiwi".to_owned());
            assert_that!(state.value()).is_empty();
        });
    }

    #[test]
    fn revert_without_selection_keeps_custom_text() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                allows_custom_value: Signal::stored(true),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "Ba");
            assert_that!(state.is_open()).is_true();
            state.revert();
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Ba".to_owned());
            assert_that!(state.is_open()).is_false();
        });
    }

    #[test]
    fn revert_restores_the_selected_text() {
        with_owner(|| {
            let (changes, on_change) = changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                default_value: vec![Key::from("banana")],
                on_change: Some(on_change),
                allows_custom_value: Signal::stored(true),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "Ch");
            assert_that!(state.is_open()).is_true();
            state.revert();
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Banana".to_owned());
            assert_that!(state.value()).is_equal_to(vec![Key::from("banana")]);
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    #[test]
    fn commit_without_custom_values_resets_the_text() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                default_value: vec![Key::from("banana")],
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "Kiwi");
            state.commit();
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("Banana".to_owned());
            assert_that!(state.value()).is_equal_to(vec![Key::from("banana")]);
        });
    }

    #[test]
    fn commit_selects_the_focused_option() {
        with_owner(|| {
            let (changes, on_change) = changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                on_change: Some(on_change),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "an");
            assert_that!(state.is_open()).is_true();
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
            state
                .list
                .selection
                .set_focused_key(Some(Key::from("banana")), None);
            state.commit();
            flush_effects();
            assert_that!(state.value()).is_equal_to(vec![Key::from("banana")]);
            assert_that!(state.input_value()).is_equal_to("Banana".to_owned());
            assert_that!(state.is_open()).is_false();
            assert_that!(changes.get_untracked()).is_equal_to(vec![vec![Key::from("banana")]]);
        });
    }

    #[test]
    fn menu_trigger_input_opens_on_typing() {
        with_owner(|| {
            let state = use_combobox_state(fruit_input());
            flush_effects();
            state.set_focused(true);
            flush_effects();
            assert_that!(state.is_open()).is_false();
            state.set_input_value("e".to_owned());
            flush_effects();
            assert_that!(state.is_open()).is_true();
            // Nothing matches: the popover closes, and reopens once something matches again.
            state.set_input_value("ex".to_owned());
            flush_effects();
            assert_that!(state.is_open()).is_false();
            state.set_input_value("e".to_owned());
            flush_effects();
            assert_that!(state.is_open()).is_true();
        });
    }

    #[test]
    fn menu_trigger_focus_opens_on_focus_with_all_items() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                default_input_value: Some("Ban".to_owned()),
                menu_trigger: Signal::stored(ComboBoxMenuTrigger::Focus),
                ..fruit_input()
            });
            flush_effects();
            state.set_focused(true);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            assert_that!(displayed(&state).len()).is_equal_to(3);
            // Typing filters again.
            state.set_input_value("Che".to_owned());
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["cherry".to_owned()]);
        });
    }

    #[test]
    fn menu_trigger_manual_opens_only_manually() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                menu_trigger: Signal::stored(ComboBoxMenuTrigger::Manual),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "a");
            assert_that!(state.is_open()).is_false();
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(state.is_open()).is_true();
        });
    }

    #[test]
    fn opening_manually_shows_all_items_until_typing() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                default_input_value: Some("Ban".to_owned()),
                ..fruit_input()
            });
            flush_effects();
            state.set_focused(true);
            state.toggle(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            assert_that!(displayed(&state).len()).is_equal_to(3);
            state.set_input_value("Bana".to_owned());
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
            // Closing keeps the shown options (no flicker while the popover animates out).
            state.toggle(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(state.is_open()).is_false();
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
        });
    }

    #[test]
    fn on_open_change_reports_the_trigger() {
        with_owner(|| {
            let events = RwSignal::new(Vec::new());
            let state = use_combobox_state(UseComboBoxStateInput {
                on_open_change: Some(Callback::new(move |event: ComboBoxOpenChange| {
                    events.update(|e| e.push(event));
                })),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "a");
            state.close();
            flush_effects();
            assert_that!(events.get_untracked()).is_equal_to(vec![
                ComboBoxOpenChange {
                    is_open: true,
                    trigger: Some(MenuTriggerAction::Input),
                },
                ComboBoxOpenChange {
                    is_open: false,
                    trigger: None,
                },
            ]);
        });
    }

    #[test]
    fn multiple_selection_keeps_its_selection_on_custom_text() {
        with_owner(|| {
            let state = use_combobox_state(UseComboBoxStateInput {
                selection_mode: SelectMode::Multiple,
                default_value: vec![Key::from("apple")],
                allows_custom_value: Signal::stored(true),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "Kiwi");
            state.commit();
            flush_effects();
            assert_that!(state.value()).is_equal_to(vec![Key::from("apple")]);
            assert_that!(state.input_value()).is_equal_to("Kiwi".to_owned());
        });
    }

    #[test]
    fn bound_value_and_input_text_are_shown_and_written() {
        with_owner(|| {
            let value = RwSignal::new(vec![Key::from("banana")]);
            let text = RwSignal::new("Ban".to_owned());
            let state = use_combobox_state(UseComboBoxStateInput {
                value: Some(value.into()),
                input_value: Some(text.into()),
                collection: fruits(),
                filter: None,
                selection_mode: SelectMode::Single,
                default_value: Vec::new(),
                on_change: None,
                default_input_value: None,
                on_input_change: None,
                disabled_keys: Signal::stored(HashSet::new()),
                menu_trigger: Signal::stored(ComboBoxMenuTrigger::Input),
                allows_empty_collection: Signal::stored(false),
                allows_custom_value: Signal::stored(false),
                should_close_on_blur: true,
                is_read_only: Signal::stored(false),
                on_open_change: None,
                is_invalid: Signal::stored(false),
                validate: None,
                validation_behavior: ValidationBehavior::default(),
                name: None,
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
    /// A collection of `items` (key and text alike), changing with it.
    fn collection_of(items: RwSignal<Vec<&'static str>>) -> CollectionMemo {
        Memo::new(move |_| {
            Arc::new(Collection::build(|b| {
                for item in items.get() {
                    b.item(item, item);
                }
            }))
        })
    }

    /// Records every `on_open_change` call.
    fn open_changes() -> (
        RwSignal<Vec<ComboBoxOpenChange>>,
        Callback<ComboBoxOpenChange>,
    ) {
        let events = RwSignal::new(Vec::new());
        let callback =
            Callback::new(move |event: ComboBoxOpenChange| events.update(|e| e.push(event)));
        (events, callback)
    }

    fn opened(trigger: MenuTriggerAction) -> ComboBoxOpenChange {
        ComboBoxOpenChange {
            is_open: true,
            trigger: Some(trigger),
        }
    }

    const CLOSED: ComboBoxOpenChange = ComboBoxOpenChange {
        is_open: false,
        trigger: None,
    };

    // Upstream: useComboBoxState.test.js "onOpenChange should return the reason that open was
    // called" and "... that toggle was called when opening".
    #[test]
    fn open_and_toggle_report_their_reason() {
        with_owner(|| {
            let (events, on_open_change) = open_changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                on_open_change: Some(on_open_change),
                ..fruit_input()
            });
            flush_effects();
            assert_that!(state.is_open()).is_false();
            for trigger in [
                MenuTriggerAction::Focus,
                MenuTriggerAction::Input,
                MenuTriggerAction::Manual,
            ] {
                state.open(None, trigger);
                flush_effects();
                assert_that!(state.is_open()).is_true();
                state.close();
                flush_effects();
                state.toggle(None, trigger);
                flush_effects();
                assert_that!(state.is_open()).is_true();
                state.toggle(None, trigger);
                flush_effects();
                assert_that!(state.is_open()).is_false();
            }
            assert_that!(events.get_untracked()).is_equal_to(vec![
                opened(MenuTriggerAction::Focus),
                CLOSED,
                opened(MenuTriggerAction::Focus),
                CLOSED,
                opened(MenuTriggerAction::Input),
                CLOSED,
                opened(MenuTriggerAction::Input),
                CLOSED,
                opened(MenuTriggerAction::Manual),
                CLOSED,
                opened(MenuTriggerAction::Manual),
                CLOSED,
            ]);
        });
    }

    // Upstream: useComboBoxState.test.js "can have a default value", "fires an event when the
    // value is changed and updates if uncontrolled" and "starts blank if no (default) value".
    #[test]
    fn input_text_starts_with_its_default_and_reports_changes() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::<String>::new());
            let on_input_change =
                Callback::new(move |text: String| changes.update(|c| c.push(text)));
            let state = use_combobox_state(UseComboBoxStateInput {
                default_input_value: Some("hello".to_owned()),
                on_input_change: Some(on_input_change),
                ..fruit_input()
            });
            flush_effects();
            assert_that!(state.input_value()).is_equal_to("hello".to_owned());
            assert_that!(changes.get_untracked()).is_empty();
            state.set_input_value("hellow".to_owned());
            assert_that!(state.input_value()).is_equal_to("hellow".to_owned());
            assert_that!(changes.get_untracked()).is_equal_to(vec!["hellow".to_owned()]);

            let blank = use_combobox_state(fruit_input());
            flush_effects();
            assert_that!(blank.input_value()).is_equal_to(String::new());
        });
    }

    // Upstream: useComboBoxState.test.js "does not change selection on close".
    #[test]
    fn closing_doesnt_select_the_focused_option() {
        with_owner(|| {
            let state = use_combobox_state(fruit_input());
            flush_effects();
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            state
                .list
                .selection
                .set_focused_key(Some(Key::from("apple")), None);
            state.close();
            flush_effects();
            assert_that!(state.value()).is_empty();
        });
    }

    // Upstream: useComboBoxState.test.js "support defaultSelectedKey" and "supports default no
    // selection".
    #[test]
    fn selecting_replaces_the_selection() {
        with_owner(|| {
            for default_value in [vec![Key::from("apple")], Vec::new()] {
                let (changes, on_change) = changes();
                let state = use_combobox_state(UseComboBoxStateInput {
                    default_value: default_value.clone(),
                    on_change: Some(on_change),
                    ..fruit_input()
                });
                flush_effects();
                assert_that!(state.value()).is_equal_to(default_value);
                state.list.selection.replace_selection(&Key::from("banana"));
                flush_effects();
                assert_that!(state.value()).is_equal_to(vec![Key::from("banana")]);
                assert_that!(changes.get_untracked()).is_equal_to(vec![vec![Key::from("banana")]]);
            }
        });
    }

    // Upstream: useComboBoxState.test.js "won't update the returned collection if the combobox
    // is closed (uncontrolled items)": while closed (and closing), the options stay as they were
    // shown; opening shows the current ones.
    #[test]
    fn a_closed_combo_box_keeps_the_options_it_showed() {
        with_owner(|| {
            let state = use_combobox_state(fruit_input());
            flush_effects();
            assert_that!(displayed(&state)).has_length(3);
            state.open(None, MenuTriggerAction::Manual);
            state.set_input_value("Ban".to_owned());
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
            // Blurring reverts the text; the shown options stay.
            state.set_focused(false);
            flush_effects();
            assert_that!(state.input_value()).is_equal_to(String::new());
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
            state.commit();
            state.close();
            state.revert();
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["banana".to_owned()]);
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(displayed(&state)).has_length(3);
        });
    }

    // Upstream: useComboBoxState.test.js "won't update the returned collection if the combobox
    // is closed (controlled items)": the app's (server-filtered) options change while closed.
    #[test]
    fn a_closed_server_filtered_combo_box_keeps_the_options_it_showed() {
        with_owner(|| {
            let items = RwSignal::new(vec!["one", "onomatopoeia"]);
            let state = use_combobox_state(UseComboBoxStateInput {
                collection: collection_of(items),
                filter: None,
                ..fruit_input()
            });
            flush_effects();
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            items.set(vec!["onomatopoeia"]);
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["onomatopoeia".to_owned()]);
            state.set_focused(false);
            flush_effects();
            items.set(vec!["one", "onomatopoeia"]);
            flush_effects();
            assert_that!(displayed(&state)).is_equal_to(vec!["onomatopoeia".to_owned()]);
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(displayed(&state)).has_length(2);
        });
    }

    // A server-filtered combo box (react-aria: controlled `items`) opens without options, so
    // that the app can load them when it opens (react-aria's `open`: `|| props.items`).
    #[test]
    fn a_server_filtered_combo_box_opens_without_options() {
        with_owner(|| {
            let items = RwSignal::new(Vec::new());
            let (events, on_open_change) = open_changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                collection: collection_of(items),
                filter: None,
                on_open_change: Some(on_open_change),
                ..fruit_input()
            });
            flush_effects();
            state.set_focused(true);
            state.open(None, MenuTriggerAction::Manual);
            flush_effects();
            assert_that!(events.get_untracked().first())
                .is_equal_to(Some(&opened(MenuTriggerAction::Manual)));
            // Nothing to show yet: it closes, and opens once the options arrived.
            items.set(vec!["Luke Skywalker"]);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            assert_that!(displayed(&state)).is_equal_to(vec!["Luke Skywalker".to_owned()]);
        });
    }

    // Upstream: useComboBoxState.test.js "should re-open the menu when controlled items go from
    // empty to non-empty".
    #[test]
    fn server_filtered_options_reopen_after_an_empty_result() {
        with_owner(|| {
            let items = RwSignal::new(vec!["Luke Skywalker"]);
            let (events, on_open_change) = open_changes();
            let state = use_combobox_state(UseComboBoxStateInput {
                collection: collection_of(items),
                filter: None,
                on_open_change: Some(on_open_change),
                ..fruit_input()
            });
            flush_effects();
            state.set_focused(true);
            state.open(None, MenuTriggerAction::Input);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            items.set(Vec::new());
            flush_effects();
            assert_that!(state.is_open()).is_false();
            items.set(vec!["Luke Skywalker"]);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            assert_that!(displayed(&state)).has_length(1);
            assert_that!(events.get_untracked().last())
                .is_equal_to(Some(&opened(MenuTriggerAction::Input)));
        });
    }

    // Upstream: useComboBoxState.test.js "should not re-open after user dismisses with Escape
    // (revert)".
    #[test]
    fn server_filtered_options_dont_reopen_after_escape() {
        with_owner(|| {
            let items = RwSignal::new(vec!["Luke Skywalker"]);
            let state = use_combobox_state(UseComboBoxStateInput {
                collection: collection_of(items),
                filter: None,
                ..fruit_input()
            });
            flush_effects();
            state.set_focused(true);
            state.open(None, MenuTriggerAction::Input);
            flush_effects();
            items.set(Vec::new());
            flush_effects();
            assert_that!(state.is_open()).is_false();
            state.revert();
            flush_effects();
            items.set(vec!["Luke Skywalker"]);
            flush_effects();
            assert_that!(state.is_open()).is_false();
        });
    }

    // Upstream: useComboBoxState.test.js "should still close the menu when uncontrolled items
    // are empty".
    #[test]
    fn filtered_options_close_when_nothing_matches() {
        with_owner(|| {
            let state = use_combobox_state(fruit_input());
            flush_effects();
            state.set_focused(true);
            state.open(None, MenuTriggerAction::Input);
            flush_effects();
            assert_that!(state.is_open()).is_true();
            state.set_input_value("zzz".to_owned());
            flush_effects();
            assert_that!(state.is_open()).is_false();
        });
    }

    #[test]
    fn settings_follow_their_signals() {
        with_owner(|| {
            let menu_trigger = RwSignal::new(ComboBoxMenuTrigger::Manual);
            let state = use_combobox_state(UseComboBoxStateInput {
                menu_trigger: menu_trigger.into(),
                ..fruit_input()
            });
            flush_effects();
            type_text(&state, "a");
            assert_that!(state.is_open()).is_false();
            menu_trigger.set(ComboBoxMenuTrigger::Input);
            state.set_input_value("an".to_owned());
            flush_effects();
            assert_that!(state.is_open()).is_true();
        });
    }
}
