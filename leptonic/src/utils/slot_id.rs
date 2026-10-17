// Upstream: react-aria/src/utils/useId.ts @ 99e6102368
//! Ids of optional elements, referenced only while rendered (react-aria's `useSlotId`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `use_slot` returns a `Slot`: the slot element's `Part` (its id) and the id to reference it
//   by (react-aria's `useSlotId` returns the id or `undefined`).
//
// ## DIFFERENT BEHAVIOR
// - Whether the element is rendered is the slot `Part`'s lease, taken in the rendering
//   component's body (react-aria: looking its id up with `document.getElementById` after
//   renders). The server and the hydrating client take it in the same order, so a reference
//   renders the same on both sides.
//
// =============================================================================

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{Id, IntoAttrs, Part, utils::id::use_id};

/// An optional element of a component (label, description, error message, ...) that other
/// elements reference by id. See [`use_slot`].
#[derive(Debug)]
pub struct Slot {
    /// The slot element: each mount takes its props, one element at a time.
    pub part: Part<SlotProps>,
    /// The slot's id while an element holds its props, else `None`. Use it in ARIA references
    /// (`aria-describedby`, `aria-labelledby`), so they never point to a missing element.
    pub referenced_id: Signal<Option<Id>>,
}

/// Props for a slot element.
#[derive(Debug)]
pub struct SlotProps {
    pub id: Id,
}

pub type SlotAttrs = Attr<attr::Id, Id>;

impl IntoAttrs for SlotProps {
    type Attrs = SlotAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Attr(attr::Id, self.id)
    }
}

/// An id for an optional element, referenced only while the element is rendered (react-aria's
/// `useSlotId`): while an element holds the slot's [`Part`].
pub fn use_slot(prefix: &'static str) -> Slot {
    let id = use_id(prefix);
    let part = Part::observed({
        let id = id.clone();
        move || SlotProps { id: id.clone() }
    });
    Slot {
        referenced_id: referenced_id(&part, id),
        part,
    }
}

/// `id` while an element holds `part` (an [`observed`](Part::observed) part), for ARIA references
/// to a part that may be missing.
pub(crate) fn referenced_id<P: 'static>(part: &Part<P>, id: Id) -> Signal<Option<Id>> {
    let part = part.clone();
    Signal::derive(move || part.is_held().then(|| id.clone()))
}
