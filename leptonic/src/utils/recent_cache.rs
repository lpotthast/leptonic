// No upstream: a bounded cache of values that are costly to build (react-aria builds them per use
// or caches them in unbounded maps).
//! [`RecentCache`]: the values built last, by key, for things like ICU4X formatters per locale that
//! are costly to build and asked for on every change (kept in a `thread_local!`).

/// The `capacity` values built last, by key. Lookups are linear: for a few entries (the locales
/// and formats in use), not for many. When full, the entry built first makes room.
#[derive(Debug)]
pub(crate) struct RecentCache<K, V> {
    entries: Vec<(K, V)>,
    capacity: usize,
}

impl<K, V: Clone> RecentCache<K, V> {
    /// An empty cache keeping at most `capacity` values (at least one).
    pub(crate) const fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity: if capacity == 0 { 1 } else { capacity },
        }
    }

    /// The value of the first entry whose key `is_key` accepts, or the value `insert` builds
    /// (kept with its key). Taking a predicate lets callers look up without building a key.
    pub(crate) fn get_or_insert_with(
        &mut self,
        is_key: impl Fn(&K) -> bool,
        insert: impl FnOnce() -> (K, V),
    ) -> V {
        if let Some((_, value)) = self.entries.iter().find(|(key, _)| is_key(key)) {
            return value.clone();
        }
        let (key, value) = insert();
        if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }
        self.entries.push((key, value.clone()));
        value
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn builds_a_value_once_per_key() {
        let mut cache = RecentCache::new(2);
        let mut built = 0;
        for key in [1, 2, 1, 2] {
            cache.get_or_insert_with(
                |k| *k == key,
                || {
                    built += 1;
                    (key, key * 10)
                },
            );
        }
        assert_that!(built).is_equal_to(2);
        assert_that!(cache.get_or_insert_with(|k| *k == 2, || (2, 0))).is_equal_to(20);
    }

    #[test]
    fn the_entry_built_first_makes_room() {
        let mut cache = RecentCache::new(2);
        cache.get_or_insert_with(|k| *k == 1, || (1, "one"));
        cache.get_or_insert_with(|k| *k == 2, || (2, "two"));
        cache.get_or_insert_with(|k| *k == 3, || (3, "three"));
        assert_that!(cache.get_or_insert_with(|k| *k == 1, || (1, "rebuilt")))
            .is_equal_to("rebuilt");
        assert_that!(cache.get_or_insert_with(|k| *k == 3, || (3, "rebuilt"))).is_equal_to("three");
    }
}
