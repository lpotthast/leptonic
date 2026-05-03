use leptos::prelude::*;

/// Returns a reactive signal that yields `Some(id)` when the slot element is
/// rendered, or `None` when it is not.
///
/// Use the returned signal when building ARIA reference attributes
/// (`aria-describedby`, `aria-labelledby`, etc.) to prevent dangling references
/// to non-existent elements.
pub fn use_slot_id(id: String, is_rendered: Signal<bool>) -> Signal<Option<String>> {
    Signal::derive(move || is_rendered.get().then(|| id.clone()))
}

/// Joins multiple optional slot IDs into a space-separated string for ARIA
/// reference attributes. Returns `None` if all inputs are `None`.
pub fn join_slot_ids(ids: &[Signal<Option<String>>]) -> Signal<Option<String>> {
    let ids = ids.to_vec();
    Signal::derive(move || {
        let parts: Vec<String> = ids.iter().filter_map(Signal::get).collect();
        (!parts.is_empty()).then(|| parts.join(" "))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use leptos::prelude::Owner;

    #[test]
    fn use_slot_id_returns_some_when_rendered() {
        Owner::new().with(|| {
            let result = use_slot_id("desc-123".to_owned(), Signal::stored(true));
            assert_that(result.get_untracked())
                .is_some()
                .is_equal_to("desc-123".to_owned());
        });
    }

    #[test]
    fn use_slot_id_returns_none_when_not_rendered() {
        Owner::new().with(|| {
            let result = use_slot_id("desc-123".to_owned(), Signal::stored(false));
            assert_that(result.get_untracked()).is_none();
        });
    }

    #[test]
    fn join_slot_ids_all_none_returns_none() {
        Owner::new().with(|| {
            let a = Signal::stored(None);
            let b = Signal::stored(None);
            let result = join_slot_ids(&[a, b]);
            assert_that(result.get_untracked()).is_none();
        });
    }

    #[test]
    fn join_slot_ids_mixed_returns_joined_some_values() {
        Owner::new().with(|| {
            let a = Signal::stored(Some("id-a".to_owned()));
            let b = Signal::stored(None);
            let c = Signal::stored(Some("id-c".to_owned()));
            let result = join_slot_ids(&[a, b, c]);
            assert_that(result.get_untracked())
                .is_some()
                .is_equal_to("id-a id-c".to_owned());
        });
    }

    #[test]
    fn join_slot_ids_single_some_returns_that_value() {
        Owner::new().with(|| {
            let a = Signal::stored(None);
            let b = Signal::stored(Some("only-one".to_owned()));
            let result = join_slot_ids(&[a, b]);
            assert_that(result.get_untracked())
                .is_some()
                .is_equal_to("only-one".to_owned());
        });
    }
}
