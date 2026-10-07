// Upstream: react-aria/src/dnd/useDroppableItem.ts @ 99e6102368
use std::rc::Rc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{
    drag_manager::{self, DroppableItemOptions, use_drag_session},
    types::{DragTypes, DropOperation, DropTarget},
    use_droppable_collection::DroppableCollectionData,
    use_droppable_collection_state::DropOperationEvent,
    use_virtual_drop::use_virtual_drop,
    utils::{dragging_keys, is_internal_drop_operation},
};
use crate::{
    hooks::IntoAttrs,
    utils::{CapturedElement, aria::AriaHidden},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The item's element is a `CapturedElement` (react-aria: a ref); `target` is a signal (a
//   changed target re-registers the item with the drag manager).
// - The props hide items that can't take the dragged data with `aria-hidden` themselves
//   (react-aria computes the same `aria-hidden` per render).
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

/// Return value of [`use_droppable_item`].
#[derive(Debug)]
pub struct UseDroppableItemReturn {
    pub drop_props: UseDroppableItemProps,
    pub is_drop_target: Signal<bool>,
}

/// Props for the droppable item element.
#[derive(Debug)]
pub struct UseDroppableItemProps {
    /// How to drop, during keyboard drags.
    pub aria_describedby: Signal<Option<String>>,
    /// Items that can't take the dragged data are hidden during keyboard drags.
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

pub type UseDroppableItemAttrs = (
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
);

impl IntoAttrs for UseDroppableItemProps {
    type Attrs = UseDroppableItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// An item of a droppable collection (or a drop indicator between items): a drop target of
/// keyboard and screen reader drags.
pub fn use_droppable_item(input: UseDroppableItemInput) -> UseDroppableItemReturn {
    let UseDroppableItemInput {
        collection,
        target,
        element,
        activate_button,
    } = input;
    let state = collection.state;
    let collection_element = collection.element;
    let describedby = use_virtual_drop();

    // Also called by the drag manager, which may outlive this item's signals briefly.
    let operation = move |types: &DragTypes, allowed: &[DropOperation]| {
        let Some(target) = target.try_get() else {
            return DropOperation::Cancel;
        };
        let collection_element = collection_element.get_untracked().map(|e| (*e).clone());
        state.get_drop_operation(&DropOperationEvent {
            target,
            types: types.clone(),
            allowed_operations: allowed.to_vec(),
            is_internal: is_internal_drop_operation(collection_element.as_ref()),
            dragging_keys: dragging_keys(),
        })
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
    let is_valid_drop_target = Signal::derive(move || {
        session.with(|s| {
            s.as_ref().is_some_and(|s| {
                operation(&DragTypes::of_items(&s.items), &s.allowed_drop_operations)
                    != DropOperation::Cancel
            })
        })
    });
    let is_drop_target = Signal::derive(move || target.with(|t| state.is_drop_target(Some(t))));

    // During keyboard drags, the drop target has focus.
    Effect::new(move || {
        if session.with(Option::is_some)
            && is_drop_target.get()
            && let Some(el) = element.get()
            && let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&*el)
        {
            let _ = el.focus();
        }
    });

    UseDroppableItemReturn {
        drop_props: UseDroppableItemProps {
            aria_describedby: describedby,
            aria_hidden: Signal::derive(move || {
                (session.with(Option::is_some) && !is_valid_drop_target.get())
                    .then_some(AriaHidden::True)
            }),
        },
        is_drop_target,
    }
}
