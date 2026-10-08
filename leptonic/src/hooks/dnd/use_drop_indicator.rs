// Upstream: react-aria/src/dnd/useDropIndicator.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{
    drag_manager::use_drag_session,
    types::{DropPosition, DropTarget},
    use_droppable_collection::DroppableCollectionData,
    use_droppable_item::{UseDroppableItemInput, UseDroppableItemReturn, use_droppable_item},
};
use crate::utils::intl_strings::{DndStrings, InsertBetweenArgs, use_localized_strings};
use crate::{
    hooks::{
        IntoAttrs,
        collections::{Collection, Key, NodeKind},
    },
    utils::{CapturedElement, ElementCaptureAttr, aria::AriaHidden, id::use_id},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `target` is a signal (see `use_droppable_item`); `is_hidden` says when to render nothing
//   (react-aria's components check `aria-hidden` and `isDropTarget` themselves).
//
// =============================================================================

/// Input of [`use_drop_indicator`].
#[derive(Debug, Clone)]
pub struct UseDropIndicatorInput {
    /// The droppable collection (from `use_droppable_collection`).
    pub collection: DroppableCollectionData,
    /// The position the indicator marks (e.g. `DropTarget::item(key, DropPosition::Before).into()`).
    pub target: Signal<DropTarget>,
    /// A button activating the target during keyboard drags.
    pub activate_button: Option<CapturedElement>,
}

/// Return value of [`use_drop_indicator`].
#[derive(Debug)]
pub struct UseDropIndicatorReturn {
    pub drop_indicator_props: UseDropIndicatorProps,
    /// The drag is over this position.
    pub is_drop_target: Signal<bool>,
    /// Render nothing (but keep the element) while hidden.
    pub is_hidden: Signal<bool>,
}

/// Props for the drop indicator element.
#[derive(Debug)]
pub struct UseDropIndicatorProps {
    pub id: String,
    pub aria_roledescription: Signal<String>,
    pub aria_label: Signal<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_hidden: Signal<Option<AriaHidden>>,
    pub tabindex: i32,
    pub element_capture: ElementCaptureAttr,
}

pub type UseDropIndicatorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaRoledescription, Signal<String>>,
    Attr<attr::AriaLabel, Signal<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
    Attr<attr::Tabindex, i32>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseDropIndicatorProps {
    type Attrs = UseDropIndicatorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaRoledescription, self.aria_roledescription),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaHidden, self.aria_hidden),
            Attr(attr::Tabindex, self.tabindex),
            self.element_capture,
        )
    }
}

fn text(collection: &Collection, key: &Key) -> String {
    collection
        .get(key)
        .map(|n| n.text_value.to_string())
        .unwrap_or_default()
}

/// The label of a drop position: "Insert between A and B", "Drop on A", ...
fn label(strings: &DndStrings, collection: &Collection, target: &DropTarget) -> String {
    let DropTarget::Item(target) = target else {
        return strings.drop_on_root();
    };
    if target.drop_position == DropPosition::On {
        return strings.drop_on_item(&text(collection, &target.key));
    }
    let item_key = |key: Option<&Key>| {
        key.and_then(|k| collection.get(k))
            .filter(|n| n.kind == NodeKind::Item)
            .map(|n| n.key.clone())
    };
    let node = collection.get(&target.key);
    let before = if target.drop_position == DropPosition::Before {
        item_key(node.and_then(|n| n.prev_key.as_ref()))
    } else {
        Some(target.key.clone())
    };
    let after = if target.drop_position == DropPosition::After {
        item_key(node.and_then(|n| n.next_key.as_ref()))
    } else {
        Some(target.key.clone())
    };
    match (before, after) {
        (Some(before), Some(after)) => strings.insert_between(InsertBetweenArgs {
            before_item_text: &text(collection, &before),
            after_item_text: &text(collection, &after),
        }),
        (Some(before), None) => strings.insert_after(&text(collection, &before)),
        (None, Some(after)) => strings.insert_before(&text(collection, &after)),
        (None, None) => String::new(),
    }
}

/// A drop indicator: a drop position between items (or on the collection), reachable during
/// keyboard drags, shown while the drag is over it.
pub fn use_drop_indicator(input: UseDropIndicatorInput) -> UseDropIndicatorReturn {
    let UseDropIndicatorInput {
        collection,
        target,
        activate_button,
    } = input;
    let id = use_id("drop-indicator");
    let strings = use_localized_strings::<DndStrings>();
    let element = CapturedElement::new();
    let session = use_drag_session();
    let collection_id = collection.id.clone();
    let labelled_id = id.clone();
    let aria_labelledby = Signal::derive(move || {
        target
            .with(|t| *t == DropTarget::Root)
            .then(|| format!("{labelled_id} {collection_id}"))
    });
    let items = collection.state.list.collection;
    let UseDroppableItemReturn {
        drop_props,
        is_drop_target,
    } = use_droppable_item(UseDroppableItemInput {
        collection,
        target,
        element,
        activate_button,
    });
    let item_hidden = drop_props.aria_hidden;
    let aria_hidden = Signal::derive(move || {
        if session.with(Option::is_none) {
            Some(AriaHidden::True)
        } else {
            item_hidden.get()
        }
    });

    UseDropIndicatorReturn {
        drop_indicator_props: UseDropIndicatorProps {
            id,
            aria_roledescription: Signal::derive(move || strings.read().drop_indicator()),
            aria_label: Signal::derive(move || {
                let strings = strings.read();
                items.with(|c| target.with(|t| label(&strings, c, t)))
            }),
            aria_labelledby,
            aria_describedby: drop_props.aria_describedby,
            aria_hidden,
            tabindex: -1,
            element_capture: element.attr(),
        },
        is_drop_target,
        is_hidden: Signal::derive(move || !is_drop_target.get() && aria_hidden.get().is_some()),
    }
}
