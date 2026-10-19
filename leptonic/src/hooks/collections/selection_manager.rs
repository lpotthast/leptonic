// Upstream: react-stately/src/selection/SelectionManager.ts @ 99e6102368
// Upstream: react-stately/src/selection/useMultipleSelectionState.ts @ 99e6102368
use std::{
    collections::HashSet,
    sync::{Arc, OnceLock},
};

use leptos::{prelude::*, reactive::owner::WeakOwner};

use super::{CollectionMemo, Key, Node, NodeKind, SelectedKeys, Selection};
use crate::{
    ValueBinding,
    hooks::collections::{DisabledBehavior, FocusStrategy, SelectionBehavior, SelectionMode},
    utils::{keyed_triggers::KeyedTriggers, pointer_type::PointerType},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state (C4): no controlled `selectedKeys` prop; `default_selection`, the manager's
//   mutation methods and `on_selection_change`, or `selection` bound to app state.
// - `SelectionManager` is a `Copy` handle (one arena slot holding the signals), not a class
//   instance recreated on every render. Queries track their signals (use them in views and derived signals); mutations don't.
// - `select` takes the pointer type instead of the triggering event; that's all it needs.
// - `Selection` is an enum (`All` / `Keys`) instead of `'all' | Set<Key>`.
//
// ## OMITTED FEATURES
// - `allowsCellSelection` and range selection through a layout delegate's `getKeyRange`: not yet
//   (no grid layout delegate; ranges follow collection order).
// - `isSelectionEqual`, `getItemProps`: React-specific helpers.
//
// =============================================================================

/// Selection configuration of a collection state, see [`use_list_state`](super::use_list_state).
#[derive(Debug, Clone)]
pub struct SelectionOptions {
    /// Whether nothing, one or many items can be selected. Defaults to `None`.
    pub selection_mode: Signal<SelectionMode>,
    /// How pointer presses change the selection (toggle the item, or replace the selection).
    /// A change applies right away (also ending the touch selection mode a long press entered).
    pub selection_behavior: Signal<SelectionBehavior>,
    /// The initial selection. Ignored when `selection` is bound.
    pub default_selection: Selection,
    /// The selection as app state, replacing `default_selection`: the collection shows it, and
    /// selecting writes it.
    pub selection: Option<ValueBinding<Selection>>,
    /// Called with the new selection whenever it changes.
    pub on_selection_change: Option<Callback<Selection>>,
    /// Prevent deselecting the last selected item.
    pub disallow_empty_selection: Signal<bool>,
    /// Items that can't be selected (and, with `DisabledBehavior::All`, not focused or used).
    pub disabled_keys: Signal<HashSet<Key>>,
    pub disabled_behavior: DisabledBehavior,
    /// Call `on_selection_change` even when an interaction leaves the selection unchanged.
    pub allow_duplicate_selection_events: bool,
}

impl Default for SelectionOptions {
    fn default() -> Self {
        Self {
            selection_mode: Signal::stored(SelectionMode::None),
            selection_behavior: Signal::stored(SelectionBehavior::Toggle),
            default_selection: Selection::default(),
            selection: None,
            on_selection_change: None,
            disallow_empty_selection: Signal::stored(false),
            disabled_keys: Signal::stored(HashSet::new()),
            disabled_behavior: DisabledBehavior::All,
            allow_duplicate_selection_events: false,
        }
    }
}

/// The selection and focus state of a collection, and the operations on it.
///
/// Created by the collection state hooks ([`use_list_state`](super::use_list_state), ...).
/// Queries (`is_selected`, `focused_key`, ...) track their signals, so they can be used in views
/// and derived signals. Mutations (`select`, `toggle_selection`, ...) read untracked.
///
/// A small handle: every item's closures capture it, so the manager's signals live in one arena
/// slot (2 allocations per manager) instead of being copied into each closure.
#[derive(Debug, Clone, Copy)]
pub struct SelectionManager(StoredValue<Manager>);

/// The handles a [`SelectionManager`] stands for (each call copies them out of the arena).
#[derive(Debug, Clone, Copy)]
struct Manager {
    collection: CollectionMemo,
    /// Readers of single nodes (items' disabled state, links, labels), by key: notified when the
    /// node's content or its disabled key changes, not on every change of the collection
    /// (`performance.md` R1).
    nodes: KeyedTriggers<Key>,
    /// Notified when the collection becomes empty or stops being empty (by the node diff), not
    /// on its other changes: per-item readers of whether it is empty (a grid's rows and cells)
    /// aren't checked again on every change of the collection (`performance.md` R1).
    emptiness: Trigger,
    /// The unfiltered collection, when `collection` is a filtered view (combo boxes).
    full_collection: Option<CollectionMemo>,
    state: SelectionStateSignals,
    /// Grids in cell focus mode: focusing a row focuses one of its cells instead.
    cell_focus: bool,
}

#[derive(Debug, Clone, Copy)]
struct SelectionStateSignals {
    selection_mode: Signal<SelectionMode>,
    /// The `selection_behavior` option.
    configured_selection_behavior: Signal<SelectionBehavior>,
    selection_behavior: RwSignal<SelectionBehavior>,
    disallow_empty_selection: Signal<bool>,
    disabled_keys: Signal<HashSet<Key>>,
    disabled_behavior: DisabledBehavior,
    allow_duplicate_selection_events: bool,
    selection: Signal<Selection>,
    selection_binding: ValueBinding<Selection>,
    on_selection_change: Option<Callback<Selection>>,
    /// Readers of single keys' selection ([`SelectionManager::is_selected`]).
    selected_subscribers: KeyedTriggers<Key>,
    /// The selection those readers were last notified of.
    notified_selection: StoredValue<Selection>,
    is_empty: Memo<bool>,
    focused_key: RwSignal<Option<Key>>,
    /// Readers of single keys' focus ([`SelectionManager::is_focused_key`]).
    focused_subscribers: KeyedTriggers<Key>,
    /// Whether `focused_key` is set: written with it, only when this changes (a memo would be
    /// checked again with all its readers, every item of a tag group, on each focus move).
    has_focused_key: RwSignal<bool>,
    child_focus_strategy: RwSignal<Option<FocusStrategy>>,
    is_focused: RwSignal<bool>,
}

/// An item's subscription to the selection of its key ([`SelectionManager::is_selected_by`]): made
/// on the first read that tracks the key (none while nothing reads it or nothing can be
/// selected), under the item's owner, so it lasts until the item unmounts. Readers then track
/// the key's trigger without a subscription per read (a map update and an `on_cleanup` on every
/// run, `performance.md` section 3).
pub(crate) struct SelectionSubscription {
    /// The owner the item was created under.
    owner: Option<WeakOwner>,
    /// The item key subscribed to and its trigger (`None`: the owner is gone).
    subscribed: OnceLock<Option<(Key, ArcTrigger)>>,
}

impl SelectionSubscription {
    /// A subscription for an item created under the current owner (made on its first read).
    pub(crate) fn new() -> Self {
        Self {
            owner: Owner::current().map(|owner| owner.downgrade()),
            subscribed: OnceLock::new(),
        }
    }

    /// Tracks the selection of `key` (an item key): through the subscription when it is the key
    /// subscribed to, else per read.
    fn track(&self, triggers: KeyedTriggers<Key>, key: &Key) {
        let subscribed = self.subscribed.get_or_init(|| {
            let owner = self.owner.as_ref().and_then(WeakOwner::upgrade)?;
            Some((key.clone(), owner.with(|| triggers.subscribe(key.clone()))))
        });
        match subscribed {
            Some((subscribed, trigger)) if subscribed == key => trigger.track(),
            _ => triggers.track(key),
        }
    }
}

impl SelectionManager {
    /// Create the selection state for `collection`. The state is owned by the manager.
    pub fn new(collection: CollectionMemo, options: SelectionOptions) -> Self {
        Self::store(&Manager::new(collection, options))
    }

    /// Stores the handles in the arena.
    fn store(manager: &Manager) -> Self {
        Self(StoredValue::new(*manager))
    }

    /// The manager's handles.
    fn get(self) -> Manager {
        self.0.get_value()
    }

    /// Subscribes the running reactive computation to changes of the node `key` (its content,
    /// its disabled key): a reader of one node reads the collection untracked after this.
    pub(crate) fn track_node(&self, key: &Key) {
        self.get().nodes.track(key);
    }

    /// Whether the collection has no items, tracking only changes of that (not every change of
    /// the collection): for readers in every item.
    pub(crate) fn is_collection_empty(&self) -> bool {
        let manager = self.get();
        manager.emptiness.track();
        manager.collection.with_untracked(|c| c.is_empty())
    }

    /// The same selection state, applied to another view of the collection (e.g. a filtered
    /// one). "Select all" still refers to the full collection.
    #[must_use]
    pub fn with_collection(&self, collection: CollectionMemo) -> Self {
        Self::store(&self.get().with_collection(collection))
    }

    /// A selection of its own over the same collection, sharing this manager's focus state (the
    /// focused key, whether the collection is focused) and disabled keys: for a group of items with
    /// a selection mode of its own, e.g. a menu section (react-aria-components'
    /// `GroupSelectionManager`). `options.disabled_keys` and `options.disabled_behavior` are
    /// ignored.
    #[must_use]
    pub fn with_own_selection(&self, options: SelectionOptions) -> Self {
        Self::store(&self.get().with_own_selection(options))
    }

    /// This manager, for a grid in cell focus mode (see [`SelectionManager::set_focused_key`]).
    #[must_use]
    pub fn with_cell_focus(&self) -> Self {
        Self::store(&Manager {
            cell_focus: true,
            ..self.get()
        })
    }

    /// The collection this manager operates on.
    pub fn collection(&self) -> CollectionMemo {
        self.get().collection
    }

    // ---- Configuration ----

    pub fn selection_mode(&self) -> SelectionMode {
        self.get().selection_mode()
    }

    /// The selection mode as the signal it comes from (no wrapper per reader): e.g. for an
    /// attribute of every item, which stays constant when the mode is.
    pub fn selection_mode_signal(&self) -> Signal<SelectionMode> {
        self.get().state.selection_mode
    }

    pub fn selection_behavior(&self) -> SelectionBehavior {
        self.get().state.selection_behavior.get()
    }

    /// The selection behavior the collection is configured with (the `selection_behavior` of
    /// its [`SelectionOptions`]). [`Self::selection_behavior`] can differ from it: a long press
    /// switches `Replace` to `Toggle` (touch selection mode).
    pub fn configured_selection_behavior(&self) -> SelectionBehavior {
        self.get().state.configured_selection_behavior.get()
    }

    pub fn set_selection_behavior(&self, behavior: SelectionBehavior) {
        self.get().state.selection_behavior.set(behavior);
    }

    pub fn disallow_empty_selection(&self) -> bool {
        self.get().disallow_empty_selection()
    }

    pub fn disabled_behavior(&self) -> DisabledBehavior {
        self.get().state.disabled_behavior
    }

    // ---- Focus ----

    /// Whether focus is within the collection.
    pub fn is_focused(&self) -> bool {
        self.get().state.is_focused.get()
    }

    pub fn set_focused(&self, focused: bool) {
        let is_focused = self.get().state.is_focused;
        // Like `set_focused_key`: an equal value notifies no one.
        if is_focused.get_untracked() != focused {
            is_focused.set(focused);
        }
    }

    /// The key of the focused item, if any.
    pub fn focused_key(&self) -> Option<Key> {
        self.get().state.focused_key.get()
    }

    /// For items with focusable children (grid rows): whether their first or last child gets
    /// focus.
    pub fn child_focus_strategy(&self) -> Option<FocusStrategy> {
        self.get().state.child_focus_strategy.get()
    }

    /// Focus `key` (`None`: no item). Keys that aren't part of the collection are ignored.
    pub fn set_focused_key(&self, key: Option<Key>, child_focus_strategy: Option<FocusStrategy>) {
        self.get().set_focused_key(key, child_focus_strategy);
    }

    /// Whether `key` is the focused item. Tracks only this key's focus: a reader re-runs when
    /// `key` gains or loses focus, not when focus moves between other items.
    pub fn is_focused_key(&self, key: &Key) -> bool {
        let manager = self.get();
        manager.state.focused_subscribers.track(key);
        manager.is_focused_key_untracked(key)
    }

    /// Subscribes an item to the focus of its `key` (once per item, until it unmounts): its
    /// readers pass the trigger to [`Self::is_focused_key_by`] instead of subscribing per read.
    pub(crate) fn subscribe_focus(&self, key: Key) -> ArcTrigger {
        self.get().state.focused_subscribers.subscribe(key)
    }

    /// [`Self::is_focused_key`] for an item subscribed to the focus of `key`
    /// ([`Self::subscribe_focus`]): tracks the subscription's `trigger`.
    pub(crate) fn is_focused_key_by(&self, key: &Key, trigger: &ArcTrigger) -> bool {
        trigger.track();
        self.get().is_focused_key_untracked(key)
    }

    /// Whether any item is focused. Tracks only that, not which item.
    pub fn has_focused_key(&self) -> bool {
        self.get().state.has_focused_key.get()
    }

    // ---- Selection queries ----

    /// The selection as stored: `All` stays `All`.
    pub fn raw_selection(&self) -> Selection {
        self.get().state.selection.get()
    }

    /// The selected keys, with `All` resolved to every selectable item.
    pub fn selected_keys(&self) -> HashSet<Key> {
        self.get().selected_keys()
    }

    /// Whether `key` (or, for a node that isn't an item, the item it belongs to) is selected.
    /// Tracks only this key's selection: a reader re-runs when the key is selected or
    /// deselected, not on every selection change.
    pub fn is_selected(&self, key: &Key) -> bool {
        self.get().is_selected(key, None)
    }

    /// [`Self::is_selected`] for the item `key` whose readers share `subscription` (one per item).
    pub(crate) fn is_selected_by(&self, key: &Key, subscription: &SelectionSubscription) -> bool {
        self.get().is_selected(key, Some(subscription))
    }

    /// Whether nothing is selected. Tracks only that, not which keys are selected.
    pub fn is_empty(&self) -> bool {
        self.get().is_empty()
    }

    /// Whether every selectable item is selected.
    pub fn is_select_all(&self) -> bool {
        self.get().is_select_all()
    }

    /// The selected key that comes first in the collection.
    pub fn first_selected_key(&self) -> Option<Key> {
        self.get().selected_by_order(std::cmp::Ordering::Less)
    }

    /// The selected key that comes last in the collection.
    pub fn last_selected_key(&self) -> Option<Key> {
        self.get().selected_by_order(std::cmp::Ordering::Greater)
    }

    /// Whether `key` can be selected: it exists, is an item, and isn't disabled.
    pub fn can_select_item(&self, key: &Key) -> bool {
        self.get().can_select_item(key)
    }

    /// Whether `key` is disabled (in `disabled_keys`, or the item itself), whatever the
    /// disabled behavior.
    pub fn is_item_disabled(&self, key: &Key) -> bool {
        self.get().is_item_disabled(key)
    }

    /// Whether `key` is disabled for interaction: with `DisabledBehavior::All`, disabled items
    /// can't be focused or used. With `DisabledBehavior::Selection`, they only can't be
    /// selected, so this is `false`. An item's own `disabled_behavior` overrides the
    /// collection's.
    pub fn is_disabled(&self, key: &Key) -> bool {
        self.get().is_disabled(key)
    }

    /// Whether `key` is an item that navigates somewhere.
    pub fn is_link(&self, key: &Key) -> bool {
        self.get().is_link(key)
    }

    // ---- Selection mutations ----

    /// Select `key` the way a press does: depending on the selection mode and behavior, toggle
    /// it or replace the selection with it. Touch and screen reader presses always toggle in
    /// multiple selection.
    pub fn select(&self, key: &Key, pointer_type: Option<&PointerType>) {
        self.get().select(key, pointer_type);
    }

    /// Add `key` to the selection, or remove it.
    pub fn toggle_selection(&self, key: &Key) {
        self.get().toggle_selection(key);
    }

    /// Select only `key` (or nothing, if it can't be selected).
    pub fn replace_selection(&self, key: &Key) {
        self.get().replace_selection(key);
    }

    /// Extend the selection from the anchor to `to` (Shift+click, Shift+Arrow): the previously
    /// extended range is replaced by the range between the anchor and `to`.
    pub fn extend_selection(&self, to: &Key) {
        self.get().extend_selection(to);
    }

    /// Replace the selection with `keys` (in single selection mode: the first of them).
    pub fn set_selected_keys(&self, keys: impl IntoIterator<Item = Key>) {
        self.get().set_selected_keys(keys);
    }

    /// Select every item (multiple selection only).
    pub fn select_all(&self) {
        self.get().select_all();
    }

    /// Deselect everything (unless empty selections are disallowed).
    pub fn clear_selection(&self) {
        self.get().clear_selection();
    }

    /// Select all, or clear the selection if everything is selected.
    pub fn toggle_select_all(&self) {
        let manager = self.get();
        if untrack(|| manager.is_select_all()) {
            manager.clear_selection();
        } else {
            manager.select_all();
        }
    }
}

impl Manager {
    fn new(collection: CollectionMemo, options: SelectionOptions) -> Self {
        let SelectionOptions {
            selection_mode,
            selection_behavior,
            default_selection,
            selection,
            on_selection_change,
            disallow_empty_selection,
            disabled_keys,
            disabled_behavior,
            allow_duplicate_selection_events,
        } = options;
        let selection_binding =
            selection.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_selection)));
        let focused_key = RwSignal::new(None::<Key>);
        let state = SelectionStateSignals {
            selection_mode,
            configured_selection_behavior: selection_behavior,
            selection_behavior: RwSignal::new(selection_behavior.get_untracked()),
            disallow_empty_selection,
            disabled_keys,
            disabled_behavior,
            allow_duplicate_selection_events,
            selection: selection_binding.value,
            selection_binding,
            on_selection_change,
            selected_subscribers: KeyedTriggers::new(),
            notified_selection: StoredValue::new(selection_binding.value.get_untracked()),
            is_empty: Memo::new(move |_| selection_binding.value.with(Selection::is_empty)),
            focused_key,
            focused_subscribers: KeyedTriggers::new(),
            has_focused_key: RwSignal::new(false),
            child_focus_strategy: RwSignal::new(None),
            is_focused: RwSignal::new(false),
        };

        // A bound selection the app changes (not through the manager): tell the readers of the
        // keys that changed. Changes through the manager are told right away.
        Effect::new(move |_| {
            state.selection.track();
            notify_selection_change(&state);
        });

        // A changed `selection_behavior` applies (react-aria: `useMultipleSelectionState`).
        Effect::new(move |previous: Option<SelectionBehavior>| {
            let behavior = selection_behavior.get();
            if previous.is_some_and(|previous| previous != behavior) {
                state.selection_behavior.set(behavior);
            }
            behavior
        });
        // With `Replace` behavior, a long press switches to `Toggle` (touch selection mode); once
        // the selection is empty again, go back to `Replace`.
        Effect::new(move |_| {
            if selection_behavior.get() == SelectionBehavior::Replace
                && state.selection.with(Selection::is_empty)
                && state.selection_behavior.get_untracked() == SelectionBehavior::Toggle
            {
                state.selection_behavior.set(SelectionBehavior::Replace);
            }
        });

        let (nodes, emptiness) = node_triggers(collection, disabled_keys);
        Self {
            collection,
            nodes,
            emptiness,
            full_collection: None,
            state,
            cell_focus: false,
        }
    }

    fn with_collection(&self, collection: CollectionMemo) -> Self {
        let (nodes, emptiness) = node_triggers(collection, self.state.disabled_keys);
        Self {
            collection,
            nodes,
            emptiness,
            full_collection: Some(self.full_collection.unwrap_or(self.collection)),
            state: self.state,
            cell_focus: self.cell_focus,
        }
    }

    fn with_own_selection(&self, options: SelectionOptions) -> Self {
        let mut group = Self::new(
            self.collection,
            SelectionOptions {
                disabled_keys: self.state.disabled_keys,
                disabled_behavior: self.state.disabled_behavior,
                ..options
            },
        );
        group.state.focused_key = self.state.focused_key;
        group.state.focused_subscribers = self.state.focused_subscribers;
        group.state.has_focused_key = self.state.has_focused_key;
        group.state.child_focus_strategy = self.state.child_focus_strategy;
        group.state.is_focused = self.state.is_focused;
        group.full_collection = self.full_collection;
        group.cell_focus = self.cell_focus;
        group
    }

    fn selection_mode(&self) -> SelectionMode {
        self.state.selection_mode.get()
    }

    fn disallow_empty_selection(&self) -> bool {
        self.state.disallow_empty_selection.get()
    }

    fn set_focused_key(&self, key: Option<Key>, child_focus_strategy: Option<FocusStrategy>) {
        // In cell focus mode, a row's first (or, with `FocusStrategy::Last`, last) cell gets
        // focus instead of the row.
        let key = match key {
            Some(row) if self.cell_focus => {
                Some(self.collection.with_untracked(|c| match c.get(&row) {
                    Some(node) if node.kind == NodeKind::Item => {
                        let cell = if child_focus_strategy == Some(FocusStrategy::Last) {
                            c.last_child(&row)
                        } else {
                            c.first_child(&row)
                        };
                        cell.map_or(row, |cell| cell.key.clone())
                    }
                    _ => row,
                }))
            }
            key => key,
        };
        if key
            .as_ref()
            .is_some_and(|key| !self.collection.with_untracked(|c| c.contains_key(key)))
        {
            return;
        }
        // Setting the same values again notifies no one (React bails out of equal state
        // updates): items refocus only when their focus actually changes.
        if self.state.child_focus_strategy.get_untracked() != child_focus_strategy {
            self.state.child_focus_strategy.set(child_focus_strategy);
        }
        let previous = self
            .state
            .focused_key
            .with_untracked(|focused| (*focused != key).then(|| focused.clone()));
        if let Some(previous) = previous {
            self.state.focused_key.set(key.clone());
            if previous.is_some() != key.is_some() {
                self.state.has_focused_key.set(key.is_some());
            }
            self.state
                .focused_subscribers
                .notify(previous.iter().chain(key.iter()));
        }
    }

    fn is_focused_key_untracked(&self, key: &Key) -> bool {
        self.state
            .focused_key
            .with_untracked(|focused| focused.as_ref() == Some(key))
    }

    fn selected_keys(&self) -> HashSet<Key> {
        match self.state.selection.get() {
            Selection::All => self.select_all_keys().into_iter().collect(),
            Selection::Keys(keys) => keys.into_iter().collect(),
        }
    }

    /// `subscription`: the item's own (else the reader subscribes per read).
    fn is_selected(&self, key: &Key, subscription: Option<&SelectionSubscription>) -> bool {
        if self.selection_mode() == SelectionMode::None {
            return false;
        }
        let Some(key) = self.item_key(key) else {
            return false;
        };
        match subscription {
            Some(subscription) => subscription.track(self.state.selected_subscribers, &key),
            None => self.state.selected_subscribers.track(&key),
        }
        let all = match &*self.state.selection.read_untracked() {
            Selection::All => true,
            Selection::Keys(keys) => return keys.contains(&key),
        };
        all && self.can_select_item(&key)
    }

    fn is_empty(&self) -> bool {
        self.state.is_empty.get()
    }

    fn is_select_all(&self) -> bool {
        match &*self.state.selection.read() {
            Selection::All => true,
            Selection::Keys(keys) if keys.is_empty() => false,
            Selection::Keys(keys) => self.select_all_keys().iter().all(|k| keys.contains(k)),
        }
    }

    fn selected_by_order(&self, wanted: std::cmp::Ordering) -> Option<Key> {
        let selection = self.state.selection.read();
        let Selection::Keys(keys) = &*selection else {
            return None;
        };
        self.collection.with(|c| {
            keys.iter()
                .filter(|k| c.contains_key(k))
                .fold(None::<&Key>, |best, key| match best {
                    Some(best) if c.compare_order(key, best) != Some(wanted) => Some(best),
                    _ => Some(key),
                })
                .cloned()
        })
    }

    fn can_select_item(&self, key: &Key) -> bool {
        self.can_select_item_in(key, self.collection)
    }

    fn can_select_item_in(&self, key: &Key, collection: CollectionMemo) -> bool {
        if self.selection_mode() == SelectionMode::None {
            return false;
        }
        let check = || {
            !self.state.disabled_keys.with(|keys| keys.contains(key))
                && collection.with(|c| {
                    c.get(key)
                        .is_some_and(|node| node.kind == NodeKind::Item && !node.is_disabled)
                })
        };
        if collection == self.collection {
            // The manager's own collection: only this node's changes reach the reader.
            self.nodes.track(key);
            untrack(check)
        } else {
            check()
        }
    }

    fn is_item_disabled(&self, key: &Key) -> bool {
        self.nodes.track(key);
        untrack(|| {
            self.state.disabled_keys.with(|keys| keys.contains(key))
                || self
                    .collection
                    .with(|c| c.get(key).is_some_and(|node| node.is_disabled))
        })
    }

    fn is_disabled(&self, key: &Key) -> bool {
        if self.state.disabled_behavior != DisabledBehavior::All {
            return false;
        }
        // Only this node's changes reach the reader.
        self.nodes.track(key);
        let (item_disabled, item_behavior) = self.collection.with_untracked(|c| {
            c.get(key).map_or((false, None), |node| {
                (node.is_disabled, node.disabled_behavior)
            })
        });
        (self
            .state
            .disabled_keys
            .with_untracked(|keys| keys.contains(key))
            || item_disabled)
            && item_behavior != Some(DisabledBehavior::Selection)
    }

    fn is_link(&self, key: &Key) -> bool {
        self.nodes.track(key);
        self.collection
            .with_untracked(|c| c.get(key).is_some_and(|node| node.link.is_some()))
    }

    fn select(&self, key: &Key, pointer_type: Option<&PointerType>) {
        match self.state.selection_mode.get_untracked() {
            SelectionMode::None => {}
            SelectionMode::Single => {
                if untrack(|| self.is_selected(key, None))
                    && !self.state.disallow_empty_selection.get_untracked()
                {
                    self.toggle_selection(key);
                } else {
                    self.replace_selection(key);
                }
            }
            SelectionMode::Multiple => {
                let toggles = self.state.selection_behavior.get_untracked()
                    == SelectionBehavior::Toggle
                    || matches!(
                        pointer_type,
                        Some(PointerType::Touch | PointerType::Virtual)
                    );
                if toggles {
                    self.toggle_selection(key);
                } else {
                    self.replace_selection(key);
                }
            }
        }
    }

    fn toggle_selection(&self, key: &Key) {
        untrack(|| {
            let mode = self.selection_mode();
            if mode == SelectionMode::None {
                return;
            }
            if mode == SelectionMode::Single && !self.is_selected(key, None) {
                self.replace_selection(key);
                return;
            }
            let Some(key) = self.item_key(key) else {
                return;
            };
            let mut keys: SelectedKeys = match self.state.selection.get() {
                Selection::All => self.select_all_keys().into_iter().collect(),
                Selection::Keys(keys) => keys,
            };
            if keys.contains(&key) {
                keys.remove(&key);
            } else if self.can_select_item(&key) {
                keys.insert(key.clone());
                keys = keys.with_range(Some(key.clone()), Some(key));
            }
            if self.disallow_empty_selection() && keys.is_empty() {
                return;
            }
            self.set_selection(Selection::Keys(keys));
        });
    }

    fn replace_selection(&self, key: &Key) {
        untrack(|| {
            if self.selection_mode() == SelectionMode::None {
                return;
            }
            let Some(key) = self.item_key(key) else {
                return;
            };
            let keys = if self.can_select_item(&key) {
                std::iter::once(key.clone())
                    .collect::<SelectedKeys>()
                    .with_range(Some(key.clone()), Some(key))
            } else {
                SelectedKeys::default()
            };
            self.set_selection(Selection::Keys(keys));
        });
    }

    fn extend_selection(&self, to: &Key) {
        untrack(|| match self.selection_mode() {
            SelectionMode::None => {}
            SelectionMode::Single => self.replace_selection(to),
            SelectionMode::Multiple => {
                let Some(to) = self.item_key(to) else {
                    return;
                };
                let keys = match self.state.selection.get() {
                    Selection::All => std::iter::once(to.clone())
                        .collect::<SelectedKeys>()
                        .with_range(Some(to.clone()), Some(to)),
                    Selection::Keys(selected) => {
                        let anchor = selected.anchor().cloned().unwrap_or_else(|| to.clone());
                        let current = selected.current().cloned().unwrap_or_else(|| to.clone());
                        let mut keys = selected.with_range(Some(anchor.clone()), Some(to.clone()));
                        for key in self.key_range(&anchor, &current) {
                            keys.remove(&key);
                        }
                        for key in self.key_range(&to, &anchor) {
                            if self.can_select_item(&key) {
                                keys.insert(key);
                            }
                        }
                        keys
                    }
                };
                self.set_selection(Selection::Keys(keys));
            }
        });
    }

    fn set_selected_keys(&self, keys: impl IntoIterator<Item = Key>) {
        untrack(|| {
            let mode = self.selection_mode();
            if mode == SelectionMode::None {
                return;
            }
            let mut selected = SelectedKeys::default();
            for key in keys {
                if let Some(key) = self.item_key(&key) {
                    selected.insert(key);
                    if mode == SelectionMode::Single {
                        break;
                    }
                }
            }
            self.set_selection(Selection::Keys(selected));
        });
    }

    fn select_all(&self) {
        untrack(|| {
            if !self.is_select_all() && self.selection_mode() == SelectionMode::Multiple {
                self.set_selection(Selection::All);
            }
        });
    }

    fn clear_selection(&self) {
        untrack(|| {
            if !self.disallow_empty_selection() && !self.is_empty() {
                self.set_selection(Selection::default());
            }
        });
    }

    fn set_selection(&self, selection: Selection) {
        let unchanged = self
            .state
            .selection
            .with_untracked(|current| *current == selection);
        if unchanged && !self.state.allow_duplicate_selection_events {
            return;
        }
        self.state.selection_binding.set(selection.clone());
        notify_selection_change(&self.state);
        if let Some(on_selection_change) = self.state.on_selection_change {
            on_selection_change.run(selection);
        }
    }

    /// The item a key belongs to: the key itself for items, the nearest item ancestor for other
    /// nodes. Unknown keys are passed through (they may refer to items not loaded yet).
    fn item_key(&self, key: &Key) -> Option<Key> {
        self.collection.with_untracked(|c| {
            let Some(mut node) = c.get(key) else {
                return Some(key.clone());
            };
            while node.kind != NodeKind::Item {
                node = c.get(node.parent_key.as_ref()?)?;
            }
            Some(node.key.clone())
        })
    }

    /// The items from `from` to `to` (in either order), in collection order.
    fn key_range(&self, from: &Key, to: &Key) -> Vec<Key> {
        self.collection
            .with_untracked(|c| c.item_keys_between(from, to))
    }

    /// Every selectable item of the full collection, including tree children of sections.
    fn select_all_keys(&self) -> Vec<Key> {
        let collection = self.full_collection.unwrap_or(self.collection);
        let keys: Vec<Key> = collection.with(|c| c.items().map(|node| node.key.clone()).collect());
        keys.into_iter()
            .filter(|key| self.can_select_item_in(key, collection))
            .collect()
    }
}

/// The triggers of the nodes of `collection`: a new collection notifies the keys whose node was
/// added, removed or changed (`Node::same_content`; positions don't count), nested nodes
/// included, and a change of `disabled_keys` the keys added or removed. A header added, removed
/// or changed also notifies its section (the section's heading is its header child). One diff per
/// change instead of every item reading the whole collection. The second trigger is notified when
/// the collection becomes empty or stops being empty.
fn node_triggers(
    collection: CollectionMemo,
    disabled_keys: Signal<HashSet<Key>>,
) -> (KeyedTriggers<Key>, Trigger) {
    let nodes = KeyedTriggers::new();
    let emptiness = Trigger::new();
    let previous = StoredValue::new(collection.get_untracked());
    // Isomorphic: on the server too, its readers follow the collection (as the memos it
    // replaces did); `ImmediateEffect::new` does nothing without reactive_graph's `effects`
    // feature, which servers don't enable.
    let diff = ImmediateEffect::new_isomorphic(move || {
        let current = collection.get();
        let Some(previous) = previous.try_update_value(|p| std::mem::replace(p, current.clone()))
        else {
            return;
        };
        if Arc::ptr_eq(&previous, &current) {
            return;
        }
        if previous.is_empty() != current.is_empty() {
            emptiness.notify();
        }
        let mut changed: Vec<Key> = Vec::new();
        let mut note = |node: &Node| {
            changed.push(node.key.clone());
            if node.kind == NodeKind::Header
                && let Some(section) = &node.parent_key
            {
                changed.push(section.clone());
            }
        };
        for node in current.all_nodes() {
            match previous.get(&node.key) {
                Some(before) if before.same_content(node) => {}
                Some(before) => {
                    // A header moved to another section: the old one loses its heading.
                    if before.kind == NodeKind::Header && before.parent_key != node.parent_key {
                        note(before);
                    }
                    note(node);
                }
                None => note(node),
            }
        }
        for node in previous.all_nodes() {
            if !current.contains_key(&node.key) {
                note(node);
            }
        }
        nodes.notify(&changed);
    });
    on_cleanup(move || diff.dispose());
    if !crate::utils::probe::is_constant(disabled_keys) {
        nodes.follow_set(disabled_keys);
    }
    (nodes, emptiness)
}

/// Tells the readers of single keys' selection about the keys whose selection changed since
/// they were last told (`All` and back: every key).
fn notify_selection_change(state: &SelectionStateSignals) {
    let current = state.selection.get_untracked();
    let Some(previous) = state
        .notified_selection
        .try_update_value(|notified| std::mem::replace(notified, current.clone()))
    else {
        return;
    };
    match (&previous, &current) {
        (Selection::Keys(previous), Selection::Keys(current)) => {
            state.selected_subscribers.notify(
                previous
                    .iter()
                    .filter(|key| !current.contains(key))
                    .chain(current.iter().filter(|key| !previous.contains(key))),
            );
        }
        (Selection::All, Selection::All) => {}
        _ => state.selected_subscribers.notify_all(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::hooks::collections::Collection;

    #[test]
    fn readers_of_whether_a_key_is_focused_ignore_focus_moves() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions::default());
            let runs = RwSignal::new(0);
            let has_focused_key = Memo::new(move |_| {
                runs.update_untracked(|runs| *runs += 1);
                m.has_focused_key()
            });
            assert_that!(has_focused_key.get_untracked()).is_false();
            m.set_focused_key(Some(k("apple")), None);
            assert_that!(has_focused_key.get_untracked()).is_true();
            // Moving the focus: still a focused key, the reader isn't run again.
            m.set_focused_key(Some(k("banana")), None);
            assert_that!(has_focused_key.get_untracked()).is_true();
            assert_that!(runs.get_untracked()).is_equal_to(2);
            m.set_focused_key(None, None);
            assert_that!(has_focused_key.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(3);
        });
    }

    #[test]
    fn emptiness_readers_follow_only_its_changes() {
        crate::testing::with_owner(|| {
            let keys = RwSignal::new(vec!["a"]);
            let collection: CollectionMemo = Memo::new(move |_| {
                Arc::new(Collection::build(|b| {
                    for key in keys.get() {
                        b.item(key, key);
                    }
                }))
            });
            let selection = SelectionManager::new(collection, SelectionOptions::default());
            let runs = RwSignal::new(0);
            let is_empty = Memo::new(move |_| {
                runs.update_untracked(|runs| *runs += 1);
                selection.is_collection_empty()
            });
            assert_that!(is_empty.get_untracked()).is_false();
            // Another item: still not empty, the reader isn't run again.
            keys.set(vec!["a", "b"]);
            assert_that!(is_empty.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(1);
            keys.set(Vec::new());
            assert_that!(is_empty.get_untracked()).is_true();
            keys.set(vec!["c"]);
            assert_that!(is_empty.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(3);
        });
    }

    #[test]
    fn node_readers_follow_their_node_only() {
        crate::testing::with_owner(|| {
            let label = RwSignal::new("Cat");
            let collection: CollectionMemo = Memo::new(move |_| {
                let label = label.get();
                Arc::new(Collection::build(|b| {
                    b.section("pets", |s| {
                        let _ = s.item("cat", label).aria_label(label);
                    });
                    b.item("dog", "Dog");
                }))
            });
            let selection = SelectionManager::new(collection, SelectionOptions::default());
            let cat = crate::hooks::collections::use_node_aria_label(
                crate::hooks::collections::UseNodeAriaLabelInput {
                    selection,
                    key: Key::from("cat"),
                },
            );
            let dog_runs = RwSignal::new(0);
            let dog = Memo::new(move |_| {
                selection.track_node(&Key::from("dog"));
                dog_runs.update_untracked(|runs| *runs += 1);
            });
            dog.get_untracked();
            assert_that!(cat.get_untracked()).is_equal_to(Some("Cat".to_owned()));
            label.set("Kitten");
            assert_that!(cat.get_untracked()).is_equal_to(Some("Kitten".to_owned()));
            dog.get_untracked();
            assert_that!(dog_runs.get_untracked()).is_equal_to(1);
        });
    }

    fn fruits() -> CollectionMemo {
        Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                b.item("apple", "Apple");
                b.section("s", |s| {
                    s.header("h", "More");
                    s.item("banana", "Banana");
                    s.item("cherry", "Cherry").disabled(true);
                    s.item("durian", "Durian");
                });
                b.item("elderberry", "Elderberry");
            }))
        })
    }

    fn manager(options: SelectionOptions) -> SelectionManager {
        SelectionManager::new(fruits(), options)
    }

    fn multiple() -> SelectionOptions {
        SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..Default::default()
        }
    }

    fn selected(m: &SelectionManager) -> Vec<String> {
        let mut keys: Vec<String> = m.selected_keys().iter().map(ToString::to_string).collect();
        keys.sort();
        keys
    }

    fn k(s: &str) -> Key {
        Key::from(s)
    }

    #[test]
    fn node_readers_follow_removed_nested_nodes_and_section_headers() {
        crate::testing::with_owner(|| {
            let header = RwSignal::new(Some("Pets"));
            let with_cat = RwSignal::new(true);
            let collection: CollectionMemo = Memo::new(move |_| {
                let (header, with_cat) = (header.get(), with_cat.get());
                Arc::new(Collection::build(|b| {
                    b.section("pets", |s| {
                        if let Some(header) = header {
                            s.header("pets-header", header);
                        }
                        if with_cat {
                            let _ = s.item("cat", "Cat");
                        }
                        let _ = s.item("dog", "Dog");
                    });
                }))
            });
            let selection = SelectionManager::new(collection, SelectionOptions::default());
            let reader = |key: &'static str| {
                let runs = RwSignal::new(0);
                let reader = Memo::new(move |_| {
                    selection.track_node(&Key::from(key));
                    runs.update_untracked(|runs| *runs += 1);
                });
                reader.get_untracked();
                (reader, runs)
            };
            let (section, section_runs) = reader("pets");
            let (cat, cat_runs) = reader("cat");
            let read = || {
                section.get_untracked();
                cat.get_untracked();
                (section_runs.get_untracked(), cat_runs.get_untracked())
            };

            header.set(Some("Animals"));
            assert_that!(read()).is_equal_to((2, 1));
            header.set(None);
            assert_that!(read()).is_equal_to((3, 1));
            header.set(Some("Pets"));
            assert_that!(read()).is_equal_to((4, 1));
            // A removed item inside the section reaches its readers, not the section's.
            with_cat.set(false);
            assert_that!(read()).is_equal_to((4, 2));
        });
    }

    #[test]
    fn a_group_has_its_own_selection_and_shares_focus_and_disabled_keys() {
        crate::testing::with_owner(|| {
            let menu = manager(multiple());
            let group = menu.with_own_selection(SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Single),
                ..Default::default()
            });
            group.select(&k("banana"), None);
            assert_that!(selected(&group)).is_equal_to(vec!["banana".to_owned()]);
            assert_that!(menu.is_empty()).is_true();
            assert_that!(group.selection_mode()).is_equal_to(SelectionMode::Single);
            assert_that!(menu.selection_mode()).is_equal_to(SelectionMode::Multiple);
            // Focus is shared.
            group.set_focused_key(Some(k("durian")), None);
            assert_that!(menu.focused_key()).is_equal_to(Some(k("durian")));
            menu.set_focused(true);
            assert_that!(group.is_focused()).is_true();
            // The disabled item stays disabled in the group.
            assert_that!(group.is_disabled(&k("cherry"))).is_true();
        });
    }

    #[test]
    fn nothing_is_selectable_in_selection_mode_none() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions::default());
            m.select(&k("apple"), None);
            assert_that!(m.is_selected(&k("apple"))).is_false();
            assert_that!(m.can_select_item(&k("apple"))).is_false();
        });
    }

    #[test]
    fn single_selection_replaces_and_toggles_off() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Single),
                ..Default::default()
            });
            m.select(&k("apple"), None);
            m.select(&k("banana"), None);
            assert_that!(selected(&m)).is_equal_to(vec!["banana".to_owned()]);
            m.select(&k("banana"), None);
            assert_that!(m.is_empty()).is_true();
        });
    }

    // Upstream: ListBox.test.js "does not select all with Mod+A when selection mode is single".
    #[test]
    fn select_all_selects_nothing_in_single_selection() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Single),
                ..Default::default()
            });
            m.select(&k("apple"), None);
            m.select_all();
            assert_that!(selected(&m)).is_equal_to(vec!["apple".to_owned()]);
            assert_that!(m.raw_selection()).is_not_equal_to(Selection::All);
        });
    }

    #[test]
    fn single_selection_keeps_the_last_item_when_empty_selection_is_disallowed() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Single),
                disallow_empty_selection: Signal::stored(true),
                ..Default::default()
            });
            m.select(&k("apple"), None);
            m.select(&k("apple"), None);
            assert_that!(selected(&m)).is_equal_to(vec!["apple".to_owned()]);
        });
    }

    #[test]
    fn the_touch_selection_mode_keeps_the_configured_selection_behavior() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                selection_behavior: Signal::stored(SelectionBehavior::Replace),
                ..multiple()
            });
            // A long press enters the touch selection mode.
            m.set_selection_behavior(SelectionBehavior::Toggle);
            assert_that!(m.selection_behavior()).is_equal_to(SelectionBehavior::Toggle);
            assert_that!(m.configured_selection_behavior()).is_equal_to(SelectionBehavior::Replace);
        });
    }

    #[test]
    fn multiple_selection_toggles_or_replaces_by_behavior() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            m.select(&k("apple"), None);
            m.select(&k("banana"), None);
            assert_that!(selected(&m)).is_equal_to(vec!["apple".to_owned(), "banana".to_owned()]);

            let m = manager(SelectionOptions {
                selection_behavior: Signal::stored(SelectionBehavior::Replace),
                ..multiple()
            });
            m.select(&k("apple"), None);
            m.select(&k("banana"), Some(&PointerType::Mouse));
            assert_that!(selected(&m)).is_equal_to(vec!["banana".to_owned()]);
            // Touch toggles even with `Replace` behavior.
            m.select(&k("apple"), Some(&PointerType::Touch));
            assert_that!(selected(&m)).is_equal_to(vec!["apple".to_owned(), "banana".to_owned()]);
        });
    }

    #[test]
    fn disabled_items_cannot_be_selected() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                disabled_keys: Signal::stored(HashSet::from([k("durian")])),
                ..multiple()
            });
            m.toggle_selection(&k("cherry"));
            m.toggle_selection(&k("durian"));
            assert_that!(m.is_empty()).is_true();
            assert_that!(m.is_disabled(&k("cherry"))).is_true();
            assert_that!(m.is_disabled(&k("durian"))).is_true();
            assert_that!(m.is_disabled(&k("apple"))).is_false();
        });
    }

    #[test]
    fn disabled_behavior_selection_keeps_items_usable() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                disabled_behavior: DisabledBehavior::Selection,
                ..multiple()
            });
            assert_that!(m.is_disabled(&k("cherry"))).is_false();
            assert_that!(m.can_select_item(&k("cherry"))).is_false();
        });
    }

    #[test]
    fn extend_selection_selects_ranges_across_sections_and_skips_disabled() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            m.replace_selection(&k("apple"));
            m.extend_selection(&k("durian"));
            assert_that!(selected(&m)).is_equal_to(vec![
                "apple".to_owned(),
                "banana".to_owned(),
                "durian".to_owned(),
            ]);
            // Extending again replaces the previous range (anchor stays at "apple").
            m.extend_selection(&k("banana"));
            assert_that!(selected(&m)).is_equal_to(vec!["apple".to_owned(), "banana".to_owned()]);
            // Backwards from the anchor.
            m.replace_selection(&k("elderberry"));
            m.extend_selection(&k("durian"));
            assert_that!(selected(&m))
                .is_equal_to(vec!["durian".to_owned(), "elderberry".to_owned()]);
        });
    }

    #[test]
    fn select_all_and_toggling_out_of_it() {
        crate::testing::with_owner(|| {
            let m = manager(SelectionOptions {
                disabled_keys: Signal::stored(HashSet::from([k("durian")])),
                ..multiple()
            });
            m.select_all();
            assert_that!(m.raw_selection()).is_equal_to(Selection::All);
            assert_that!(m.is_select_all()).is_true();
            // `All` only covers what can be selected.
            assert_that!(m.is_selected(&k("cherry"))).is_false();
            assert_that!(selected(&m)).is_equal_to(vec![
                "apple".to_owned(),
                "banana".to_owned(),
                "elderberry".to_owned(),
            ]);
            // Deselecting one item turns `All` into an explicit selection.
            m.toggle_selection(&k("apple"));
            assert_that!(selected(&m))
                .is_equal_to(vec!["banana".to_owned(), "elderberry".to_owned()]);
            m.toggle_select_all();
            assert_that!(m.is_select_all()).is_true();
            m.toggle_select_all();
            assert_that!(m.is_empty()).is_true();
        });
    }

    #[test]
    fn first_and_last_selected_follow_collection_order() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            m.set_selected_keys([k("elderberry"), k("banana"), k("apple")]);
            assert_that!(m.first_selected_key()).is_equal_to(Some(k("apple")));
            assert_that!(m.last_selected_key()).is_equal_to(Some(k("elderberry")));
        });
    }

    #[test]
    fn selection_change_events_skip_unchanged_selections_by_default() {
        crate::testing::with_owner(|| {
            let events = RwSignal::new(0);
            let m = manager(SelectionOptions {
                on_selection_change: Some(Callback::new(move |_| events.update(|e| *e += 1))),
                ..multiple()
            });
            m.replace_selection(&k("apple"));
            m.replace_selection(&k("apple"));
            assert_that!(events.get_untracked()).is_equal_to(1);

            let m = manager(SelectionOptions {
                on_selection_change: Some(Callback::new(move |_| events.update(|e| *e += 1))),
                allow_duplicate_selection_events: true,
                ..multiple()
            });
            m.replace_selection(&k("apple"));
            m.replace_selection(&k("apple"));
            assert_that!(events.get_untracked()).is_equal_to(3);
        });
    }

    // Upstream: useMultipleSelectionState ("If the selectionBehavior prop changes, update the
    // state as well").
    #[test]
    fn a_changed_selection_behavior_applies() {
        crate::testing::with_owner(|| {
            let behavior = RwSignal::new(SelectionBehavior::Toggle);
            let m = manager(SelectionOptions {
                selection_behavior: behavior.into(),
                ..multiple()
            });
            crate::testing::flush_effects();
            assert_that!(m.selection_behavior()).is_equal_to(SelectionBehavior::Toggle);
            behavior.set(SelectionBehavior::Replace);
            crate::testing::flush_effects();
            assert_that!(m.selection_behavior()).is_equal_to(SelectionBehavior::Replace);
            m.select(&k("apple"), None);
            m.select(&k("banana"), None);
            assert_that!(selected(&m)).is_equal_to(vec!["banana".to_owned()]);
            // Touch selection mode (a long press) ends once the selection is empty again.
            m.set_selection_behavior(SelectionBehavior::Toggle);
            m.clear_selection();
            crate::testing::flush_effects();
            assert_that!(m.selection_behavior()).is_equal_to(SelectionBehavior::Replace);
        });
    }

    // Upstream: SelectionManager.isDisabled (an item's `disabledBehavior: 'selection'`).
    #[test]
    fn an_item_can_stay_focusable_while_disabled() {
        crate::testing::with_owner(|| {
            let collection = Memo::new(|_| {
                Arc::new(Collection::build(|b| {
                    b.item("apple", "Apple").disabled(true);
                    b.item("banana", "Banana")
                        .disabled(true)
                        .disabled_behavior(DisabledBehavior::Selection);
                }))
            });
            let m = SelectionManager::new(collection, multiple());
            assert_that!(m.is_disabled(&k("apple"))).is_true();
            assert_that!(m.is_disabled(&k("banana"))).is_false();
            // Neither can be selected.
            assert_that!(m.can_select_item(&k("apple"))).is_false();
            assert_that!(m.can_select_item(&k("banana"))).is_false();
        });
    }

    /// Runs an Effect per key reading the key's focus and selection; returns how often each ran
    /// since the last `flush_and_take`.
    fn count_item_runs(
        m: &SelectionManager,
        keys: &'static [&'static str],
    ) -> impl Fn() -> Vec<(&'static str, usize)> {
        let m = *m;
        let runs = StoredValue::new(vec![0_usize; keys.len()]);
        for (i, key) in keys.iter().enumerate() {
            Effect::new(move |_| {
                m.is_focused_key(&k(key));
                m.is_selected(&k(key));
                runs.update_value(|runs| runs[i] += 1);
            });
        }
        move || {
            crate::testing::flush_effects();
            let counts = runs.get_value();
            runs.set_value(vec![0; keys.len()]);
            keys.iter()
                .copied()
                .zip(counts)
                .filter(|(_, n)| *n > 0)
                .collect()
        }
    }

    #[test]
    fn readers_of_a_key_rerun_only_when_that_key_changes() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            let runs = count_item_runs(&m, &["apple", "banana", "durian", "elderberry"]);
            assert_that!(runs()).has_length(4);

            m.set_focused_key(Some(k("apple")), None);
            assert_that!(runs()).is_equal_to(vec![("apple", 1)]);
            m.set_focused_key(Some(k("banana")), None);
            assert_that!(runs()).is_equal_to(vec![("apple", 1), ("banana", 1)]);
            m.set_focused_key(Some(k("banana")), None);
            assert_that!(runs()).is_empty();

            m.toggle_selection(&k("durian"));
            assert_that!(runs()).is_equal_to(vec![("durian", 1)]);
            m.replace_selection(&k("elderberry"));
            assert_that!(runs()).is_equal_to(vec![("durian", 1), ("elderberry", 1)]);
            // `All` (and back) concerns every key.
            m.select_all();
            assert_that!(runs()).has_length(4);
            m.clear_selection();
            assert_that!(runs()).has_length(4);
        });
    }

    #[test]
    fn readers_of_a_key_see_changes_of_a_bound_selection() {
        crate::testing::with_owner(|| {
            let bound = RwSignal::new(Selection::default());
            let m = manager(SelectionOptions {
                selection: Some(bound.into()),
                ..multiple()
            });
            let runs = count_item_runs(&m, &["apple", "banana"]);
            runs();
            // The app changes its state: the readers of the changed key re-run.
            bound.set(Selection::keys([k("banana")]));
            assert_that!(runs()).is_equal_to(vec![("banana", 1)]);
            assert_that!(m.is_selected(&k("banana"))).is_true();
        });
    }

    #[test]
    fn a_key_read_right_after_a_change_is_current() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            let apple_selected = Memo::new(move |_| m.is_selected(&k("apple")));
            let apple_focused = Memo::new(move |_| m.is_focused_key(&k("apple")));
            assert_that!(apple_selected.get_untracked()).is_false();
            assert_that!(apple_focused.get_untracked()).is_false();
            // No Effects run in between: the memos are notified synchronously.
            m.toggle_selection(&k("apple"));
            m.set_focused_key(Some(k("apple")), None);
            assert_that!(apple_selected.get_untracked()).is_true();
            assert_that!(apple_focused.get_untracked()).is_true();
        });
    }

    #[test]
    fn focused_key_must_exist() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            m.set_focused_key(Some(k("banana")), None);
            m.set_focused_key(Some(k("missing")), None);
            assert_that!(m.focused_key()).is_equal_to(Some(k("banana")));
            assert_that!(m.is_focused_key(&k("banana"))).is_true();
            m.set_focused_key(None, None);
            assert_that!(m.focused_key()).is_none();
        });
    }

    #[test]
    fn a_manager_is_a_handle_of_one_arena_slot() {
        // Every closure of every item captures it (the handles inline: 336 bytes).
        assert_that!(size_of::<SelectionManager>()).is_less_or_equal_to(16);
    }

    #[test]
    fn readers_sharing_an_item_subscription_rerun_only_for_its_key() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            let runs = RwSignal::new(0);
            let subscription = SelectionSubscription::new();
            let banana = Memo::new(move |_| {
                runs.update_untracked(|runs| *runs += 1);
                m.is_selected_by(&k("banana"), &subscription)
            });
            assert_that!(banana.get_untracked()).is_false();
            m.toggle_selection(&k("apple"));
            assert_that!(banana.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(1);
            m.toggle_selection(&k("banana"));
            assert_that!(banana.get_untracked()).is_true();
            m.toggle_selection(&k("banana"));
            assert_that!(banana.get_untracked()).is_false();
            assert_that!(runs.get_untracked()).is_equal_to(3);
        });
    }

    #[test]
    fn selecting_a_header_selects_nothing() {
        crate::testing::with_owner(|| {
            let m = manager(multiple());
            m.toggle_selection(&k("h"));
            assert_that!(m.is_empty()).is_true();
        });
    }
}
