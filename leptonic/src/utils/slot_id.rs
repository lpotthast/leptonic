// Upstream: react-aria/src/utils/useId.ts @ 99e6102368 (useSlotId)
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{
    hooks::IntoAttrs,
    utils::{CapturedElement, ElementCaptureAttr, id::use_id},
};

/// An optional element of a component (label, description, error message, ...) that other
/// elements reference by id. See [`use_slot`].
#[derive(Debug)]
pub struct Slot {
    /// Spread onto the slot element.
    pub props: SlotProps,
    /// The slot's id while its element is rendered, else `None`. Use it in ARIA references
    /// (`aria-describedby`, `aria-labelledby`), so they never point to a missing element.
    pub referenced_id: Signal<Option<String>>,
}

/// Props for a slot element.
#[derive(Debug, Clone)]
pub struct SlotProps {
    pub id: String,
    pub element_capture: ElementCaptureAttr,
}

pub type SlotAttrs = (Attr<attr::Id, String>, ElementCaptureAttr);

impl IntoAttrs for SlotProps {
    type Attrs = SlotAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), self.element_capture)
    }
}

/// An id for an optional element, referenced only while the element is rendered (detected by
/// capturing it). Like react-aria's `useSlotId`, the reference appears after mount, not in
/// server-rendered HTML.
pub fn use_slot(prefix: &str) -> Slot {
    let id = use_id(prefix);
    let element = CapturedElement::new();
    let referenced = id.clone();
    Slot {
        props: SlotProps {
            id,
            element_capture: element.attr(),
        },
        referenced_id: Signal::derive(move || element.get().is_some().then(|| referenced.clone())),
    }
}

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
    use assertr::prelude::*;
    use leptos::prelude::Owner;

    use super::*;

    #[test]
    fn use_slot_id_returns_some_when_rendered() {
        Owner::new().with(|| {
            let result = use_slot_id("desc-123".to_owned(), Signal::stored(true));
            assert_that!(result.get_untracked())
                .get_some()
                .is_equal_to("desc-123".to_owned());
        });
    }

    #[test]
    fn use_slot_id_returns_none_when_not_rendered() {
        Owner::new().with(|| {
            let result = use_slot_id("desc-123".to_owned(), Signal::stored(false));
            assert_that!(result.get_untracked()).is_none();
        });
    }

    #[test]
    fn join_slot_ids_all_none_returns_none() {
        Owner::new().with(|| {
            let a = Signal::stored(None);
            let b = Signal::stored(None);
            let result = join_slot_ids(&[a, b]);
            assert_that!(result.get_untracked()).is_none();
        });
    }

    #[test]
    fn join_slot_ids_mixed_returns_joined_some_values() {
        Owner::new().with(|| {
            let a = Signal::stored(Some("id-a".to_owned()));
            let b = Signal::stored(None);
            let c = Signal::stored(Some("id-c".to_owned()));
            let result = join_slot_ids(&[a, b, c]);
            assert_that!(result.get_untracked())
                .get_some()
                .is_equal_to("id-a id-c".to_owned());
        });
    }

    #[test]
    fn join_slot_ids_single_some_returns_that_value() {
        Owner::new().with(|| {
            let a = Signal::stored(None);
            let b = Signal::stored(Some("only-one".to_owned()));
            let result = join_slot_ids(&[a, b]);
            assert_that!(result.get_untracked())
                .get_some()
                .is_equal_to("only-one".to_owned());
        });
    }
}
