// Upstream: react-aria/src/utils/useId.ts @ 99e6102368
// (Ported: `useSlotId`.)
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{CapturedElement, ElementCaptureAttr, IntoAttrs, utils::id::use_id};

/// An optional element of a component (label, description, error message, ...) that other
/// elements reference by id. See [`use_slot`].
#[derive(Debug)]
pub struct Slot {
    /// Spread onto the slot element.
    pub props: SlotProps,
    /// The slot's id after its element is rendered, else `None`. Use it in ARIA references
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
    let referenced_id = RwSignal::new(None);
    // Captures run during hydration. A slot before its control must not change the control's
    // initial attribute value: the server rendered no reference, and hydration skips writing
    // that initial value. Publish the reference after hydration, then follow mount/unmount.
    Effect::new(move || {
        referenced_id.set(element.get().is_some().then(|| referenced.clone()));
    });
    Slot {
        props: SlotProps {
            id,
            element_capture: element.attr(),
        },
        referenced_id: referenced_id.into(),
    }
}
