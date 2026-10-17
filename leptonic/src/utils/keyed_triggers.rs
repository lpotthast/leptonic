// No upstream: per-key change notification for groups and collections (react-aria re-renders
// every item on each change; `documentation/performance.md`, R1 and section 3).
//! Per-key change notification: the items of a group or collection subscribe to their own key,
//! so a change of the shared value reaches only the items whose state changed.

use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;

/// Per-key change notification: an item's state derived from a value shared by all items (a
/// group's selected value, a set of selected or expanded keys) subscribes to the item's key
/// only, and the value's writer notifies the keys whose state changed. A change then costs
/// O(changed keys), not O(items): selecting one radio of 1,000 reruns two radios' readers.
///
/// ```ignore
/// // Once per item, in the item's hook (not per read):
/// let selected = state.keys.subscribe(key.clone());
/// let is_selected = Signal::derive(move || {
///     selected.track();
///     state.value.with_untracked(|value| value.as_ref() == Some(&key))
/// });
/// // In the state's setter, after changing the value: the old and the new key.
/// state.keys.notify([&old, &new]);
/// // A set: the keys that were added or removed.
/// state.keys.notify(previous.symmetric_difference(&current));
/// ```
///
/// Readers track the key's trigger and read the shared value untracked, in a `Signal::derive` or
/// a render closure (not a `Memo`: the trigger already says when to recompute). Writers notify
/// every key whose state may have changed after the value changed (a superset is fine); a value
/// the app may change itself (a bound value) needs one watcher per group that diffs it against
/// the value last notified.
///
/// Notification is synchronous (readers that read right after a change see it) and allocates
/// nothing but the trigger's own notification. A subscription costs the key's trigger (1
/// allocation, shared by the key's subscribers) and one `on_cleanup` (1); it goes when the
/// subscribing item's owner is cleaned up, so unmounted items leave nothing behind. Compared with
/// `reactive_graph::computed::Selector`: no `RwSignal` per key (4 allocations), no async effect
/// (its readers see the old value until it ran), no comparison of every key per change, keys are
/// removed with their items, and it serves sets as well as single values.
#[derive(Debug)]
pub(crate) struct KeyedTriggers<K: Send + Sync + 'static>(StoredValue<HashMap<K, Subscription>>);

impl<K: Send + Sync + 'static> Clone for KeyedTriggers<K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: Send + Sync + 'static> Copy for KeyedTriggers<K> {}

#[derive(Debug)]
struct Subscription {
    trigger: ArcTrigger,
    /// The items subscribed to the key (several items may share one): the entry goes with the
    /// last of them.
    subscribers: usize,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static> KeyedTriggers<K> {
    /// No subscriptions yet (2 allocations: the arena's `StoredValue`).
    pub(crate) fn new() -> Self {
        Self(StoredValue::new(HashMap::new()))
    }

    /// Triggers following the set of keys `keys`: every change of it (by its owner or by the app)
    /// notifies the keys that were added or removed, synchronously (one `ImmediateEffect`, which
    /// keeps a copy of the set last notified).
    pub(crate) fn following(keys: Signal<std::collections::HashSet<K>>) -> Self {
        let triggers = Self::new();
        triggers.follow_set(keys);
        triggers
    }

    /// Notifies these triggers of every change of the set of keys `keys` too (see
    /// [`Self::following`]).
    pub(crate) fn follow_set(self, keys: Signal<std::collections::HashSet<K>>) {
        let notified = StoredValue::new(keys.get_untracked());
        ImmediateEffect::new_scoped(move || {
            let changed = keys.with(|current| {
                notified.try_update_value(|notified| {
                    let changed: Vec<K> = notified.symmetric_difference(current).cloned().collect();
                    if !changed.is_empty() {
                        notified.clone_from(current);
                    }
                    changed
                })
            });
            if let Some(changed) = changed {
                self.notify(&changed);
            }
        });
    }

    /// Triggers following a single key (`key_of` the value of `value`, e.g. a focused or a selected
    /// date): every change of it notifies the previous and the new key, synchronously (one
    /// `ImmediateEffect`).
    pub(crate) fn following_one<T: Send + Sync + 'static>(
        value: Signal<T>,
        key_of: fn(&T) -> Option<K>,
    ) -> Self {
        let triggers = Self::new();
        let notified = StoredValue::new(value.with_untracked(key_of));
        ImmediateEffect::new_scoped(move || {
            let current = value.with(key_of);
            let previous = notified
                .try_update_value(|notified| std::mem::replace(notified, current.clone()))
                .flatten();
            if previous != current {
                triggers.notify(previous.iter().chain(current.iter()));
            }
        });
        triggers
    }

    /// Subscribes an item to changes of `key`: call it once per item, in the item's hook, and
    /// `track()` the returned trigger in the item's readers. The subscription ends when the
    /// current owner is cleaned up (the item unmounts).
    pub(crate) fn subscribe(self, key: K) -> ArcTrigger {
        let trigger = self
            .0
            .try_update_value(|subscriptions| {
                let subscription =
                    subscriptions
                        .entry(key.clone())
                        .or_insert_with(|| Subscription {
                            trigger: ArcTrigger::new(),
                            subscribers: 0,
                        });
                subscription.subscribers += 1;
                subscription.trigger.clone()
            })
            .unwrap_or_default();
        on_cleanup(move || self.release(&key));
        trigger
    }

    /// Subscribes the running reactive computation (if any) to changes of `key` until it runs
    /// again or is disposed: for readers that aren't items (a reader asking about any key, e.g.
    /// `SelectionManager::is_selected`). Items subscribe once instead ([`Self::subscribe`]):
    /// this costs a map update and an `on_cleanup` per run.
    pub(crate) fn track(self, key: &K) {
        if leptos::reactive::graph::Observer::get().is_none() {
            return;
        }
        let Some(trigger) = self.0.try_update_value(|subscriptions| {
            let subscription = subscriptions
                .entry(key.clone())
                .or_insert_with(|| Subscription {
                    trigger: ArcTrigger::new(),
                    subscribers: 0,
                });
            subscription.subscribers += 1;
            subscription.trigger.clone()
        }) else {
            return;
        };
        trigger.track();
        let key = key.clone();
        on_cleanup(move || self.release(&key));
    }

    /// Ends one subscription to `key`: the entry goes with the last one.
    fn release(self, key: &K) {
        self.0.try_update_value(|subscriptions| {
            if let Some(subscription) = subscriptions.get_mut(key) {
                subscription.subscribers = subscription.subscribers.saturating_sub(1);
                if subscription.subscribers == 0 {
                    subscriptions.remove(key);
                }
            }
        });
    }

    /// Notifies the subscribers of `keys` (keys without subscribers are skipped). Never while
    /// the subscriptions are borrowed: a synchronous reader may unmount items, which unsubscribe.
    pub(crate) fn notify<'a>(self, keys: impl IntoIterator<Item = &'a K>) {
        for key in keys {
            let trigger = self
                .0
                .try_with_value(|subscriptions| subscriptions.get(key).map(|s| s.trigger.clone()))
                .flatten();
            if let Some(trigger) = trigger {
                trigger.notify();
            }
        }
    }

    /// Notifies every subscribed key (a change of the whole value, e.g. "select all"): one `Vec`
    /// of the triggers, notified outside the borrow.
    pub(crate) fn notify_all(self) {
        let triggers: Vec<ArcTrigger> = self
            .0
            .try_with_value(|subscriptions| {
                subscriptions
                    .values()
                    .map(|subscription| subscription.trigger.clone())
                    .collect()
            })
            .unwrap_or_default();
        for trigger in triggers {
            trigger.notify();
        }
    }

    /// The number of subscribed keys.
    #[cfg(test)]
    fn subscribed_keys(self) -> usize {
        self.0.with_value(HashMap::len)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    /// An item reading whether its key is in `value`, counting its runs.
    fn item(
        keys: KeyedTriggers<u32>,
        value: RwSignal<HashSet<u32>>,
        key: u32,
    ) -> (Signal<bool>, RwSignal<usize>) {
        let runs = RwSignal::new(0);
        let trigger = keys.subscribe(key);
        let is_selected = Signal::derive(move || {
            trigger.track();
            runs.update_untracked(|runs| *runs += 1);
            value.with_untracked(|value| value.contains(&key))
        });
        (is_selected, runs)
    }

    /// Reads `signal` in a memo, as a view would, so that it reruns when notified.
    fn observe(signal: Signal<bool>) -> Memo<bool> {
        let memo = Memo::new(move |_| signal.get());
        memo.get_untracked();
        memo
    }

    #[test]
    fn notifies_only_the_changed_keys() {
        with_owner(|| {
            let keys = KeyedTriggers::new();
            let value = RwSignal::new(HashSet::from([1]));
            let items: Vec<_> = (0..4)
                .map(|key| {
                    let (is_selected, runs) = item(keys, value, key);
                    (observe(is_selected), runs)
                })
                .collect();
            let runs = || {
                items
                    .iter()
                    .map(|(_, runs)| runs.get_untracked())
                    .collect::<Vec<_>>()
            };
            assert_that!(runs()).is_equal_to(vec![1, 1, 1, 1]);

            let previous = value.get_untracked();
            value.set(HashSet::from([2, 3]));
            keys.notify(previous.symmetric_difference(&value.get_untracked()));
            let selected: Vec<bool> = items.iter().map(|(m, _)| m.get_untracked()).collect();
            assert_that!(selected).is_equal_to(vec![false, false, true, true]);
            assert_that!(runs()).is_equal_to(vec![1, 2, 2, 2]);

            keys.notify_all();
            for (memo, _) in &items {
                memo.get_untracked();
            }
            assert_that!(runs()).is_equal_to(vec![2, 3, 3, 3]);
        });
    }

    #[test]
    fn subscriptions_end_with_their_items() {
        with_owner(|| {
            let keys = KeyedTriggers::new();
            let value = RwSignal::new(HashSet::new());
            let first = Owner::current().expect("an owner").child();
            let second = Owner::current().expect("an owner").child();
            // Two items share key 1.
            first.with(|| item(keys, value, 1));
            second.with(|| {
                item(keys, value, 1);
                item(keys, value, 2);
            });
            assert_that!(keys.subscribed_keys()).is_equal_to(2);
            first.cleanup();
            assert_that!(keys.subscribed_keys()).is_equal_to(2);
            second.cleanup();
            assert_that!(keys.subscribed_keys()).is_equal_to(0);
            // Keys without subscribers are skipped.
            keys.notify([&1, &2]);
            keys.notify_all();
        });
    }

    #[test]
    fn readers_see_a_change_right_away() {
        with_owner(|| {
            let keys = KeyedTriggers::new();
            let value = RwSignal::new(HashSet::new());
            let (is_selected, _) = item(keys, value, 7);
            let memo = observe(is_selected);
            value.update(|value| {
                value.insert(7);
            });
            keys.notify([&7]);
            assert_that!(memo.get_untracked()).is_true();
        });
    }
}
