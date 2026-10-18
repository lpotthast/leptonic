// Upstream: react-aria/src/dnd/useDroppableItem.ts @ 99e6102368
// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
use std::rc::Rc;

use leptos::prelude::*;

use super::{
    drag_manager::{self, DroppableItemOptions, use_drag_session},
    types::{DragTypes, DropOperation, DropTarget},
    use_droppable_collection::DroppableCollectionData,
};
use crate::{
    CapturedElement, Id, IntoAttrs,
    utils::{
        aria::AriaHidden,
        part::{Part, Remount},
        signal_attr::{SignalAttr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `target` is a signal: a changed target re-registers the item with the drag manager.
// - The props hide items that can't take the dragged data with `aria-hidden` themselves
//   (react-aria computes the same `aria-hidden` per render).
// - The drop description is the collection's (`DroppableCollectionData::drop_description`), one
//   for all items (react-aria: each item's `useVirtualDrop`, whose `useDescription` shares one
//   element per text).
//
// =============================================================================

/// Input of [`use_droppable_item`].
#[derive(Debug, Clone)]
pub struct UseDroppableItemInput {
    /// The droppable collection (from `use_droppable_collection`).
    pub collection: DroppableCollectionData,
    /// The drop target this item (or drop indicator) stands for (e.g.
    /// `DropTarget::item(key, DropPosition::On).into()`).
    pub target: Signal<DropTarget>,
    /// The item's element (captured by its props, or by the element's own hook).
    pub element: CapturedElement,
    /// A button activating the target (e.g. opening a folder) during keyboard drags.
    pub activate_button: Option<CapturedElement>,
}

/// The return value of [`use_droppable_item`].
#[derive(Debug)]
pub struct UseDroppableItemReturn {
    /// The droppable item element.
    pub item: Part<UseDroppableItemProps>,
    pub is_drop_target: Signal<bool>,
}

/// Props for the droppable item element.
#[derive(Debug)]
pub struct UseDroppableItemProps {
    /// How to drop, during keyboard drags.
    pub aria_describedby: Signal<Option<Id>>,
    /// Items that can't take the dragged data are hidden during keyboard drags.
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

impl Remount for UseDroppableItemProps {
    fn remount(&self) -> Self {
        Self {
            aria_describedby: self.aria_describedby,
            aria_hidden: self.aria_hidden,
        }
    }
}

pub type UseDroppableItemAttrs = (SignalAttr<Option<Id>>, SignalAttr<Option<AriaHidden>>);

impl IntoAttrs for UseDroppableItemProps {
    type Attrs = UseDroppableItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            signal_attr("aria-describedby", self.aria_describedby),
            signal_attr("aria-hidden", self.aria_hidden),
        )
    }
}

/// An item of a droppable collection (or a drop indicator between items): a drop target of
/// keyboard and screen reader drags.
pub fn use_droppable_item(input: UseDroppableItemInput) -> UseDroppableItemReturn {
    crate::metrics::subject!();
    let UseDroppableItemInput {
        collection,
        target,
        element,
        activate_button,
    } = input;
    let state = collection.state;
    let collection_element = collection.element;
    // The collection's one drop description (the same text for every item).
    let describedby = collection.drop_description;

    // Also called by the drag manager, which may outlive this item's signals briefly.
    let operation = move |types: &DragTypes, allowed: &[DropOperation]| {
        let Some(target) = target.try_get_untracked() else {
            return DropOperation::Cancel;
        };
        let collection_element = collection_element.get_untracked().map(|e| (*e).clone());
        state.drop_operation_at(collection_element.as_ref(), &target, types, allowed)
    };

    let registration: StoredValue<Option<u64>> = StoredValue::new(None);
    let unregister = move || {
        if let Some(id) = registration.try_get_value().flatten() {
            drag_manager::unregister_drop_item(id);
            registration.set_value(None);
        }
    };
    Effect::new(move || {
        unregister();
        let Some(el) = element.get() else {
            return;
        };
        let id = drag_manager::register_drop_item(DroppableItemOptions {
            element: (*el).clone(),
            target: target.get(),
            get_drop_operation: Some(Rc::new(operation)),
            activate_button,
        });
        registration.set_value(Some(id));
    });
    on_cleanup(unregister);

    let session = use_drag_session();
    // Follows the drag session and the item's target (the operation itself is a query).
    let is_valid_drop_target = Memo::new(move |_| {
        target.track();
        session.with(|s| {
            s.as_ref().is_some_and(|s| {
                operation(&s.types, &s.allowed_drop_operations) != DropOperation::Cancel
            })
        })
    });
    let is_drop_target = Memo::new(move |_| target.with(|t| state.is_drop_target(Some(t))));

    // During keyboard drags, the drop target has focus. Only the drop target reads the session:
    // a drag starting or ending doesn't rerun every item's effect.
    Effect::new(move || {
        if is_drop_target.get()
            && session.with(Option::is_some)
            && let Some(el) = element.get()
            && let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&*el)
        {
            let _ = el.focus();
        }
    });

    UseDroppableItemReturn {
        item: Part::stored(UseDroppableItemProps {
            aria_describedby: describedby,
            aria_hidden: Signal::derive(move || {
                (session.with(Option::is_some) && !is_valid_drop_target.get())
                    .then_some(AriaHidden::True)
            }),
        }),
        is_drop_target: is_drop_target.into(),
    }
}
