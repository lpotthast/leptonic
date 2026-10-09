// Upstream: react-aria/src/selection/useSelectableList.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::{
    CollectionOptions, DomLayoutDelegate, KeyboardDelegate, LayoutDelegate, ListKeyboardDelegate,
    ListLayout, ListState, UseSelectableCollectionInput, UseSelectableCollectionReturn,
    keyboard_delegate_memo, use_selectable_collection,
};
use crate::{
    CapturedElement,
    utils::{
        filter::{CollatorOptions, use_collator},
        i18n::use_direction,
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input of [`use_selectable_list`].
#[derive(Clone)]
pub struct UseSelectableListInput {
    pub state: ListState,
    /// The list element; the hook's props capture it.
    pub element: CapturedElement,
    pub orientation: Signal<Orientation>,
    /// Items stacked (one per row/column) or wrapping in a grid.
    pub layout: ListLayout,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Where the items are, for the list keyboard delegate. Default: measured in the DOM (a
    /// virtualizer's layout knows items that aren't rendered).
    pub layout_delegate: Option<Arc<dyn LayoutDelegate>>,
    pub options: CollectionOptions,
}

impl std::fmt::Debug for UseSelectableListInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseSelectableListInput")
            .field("orientation", &self.orientation)
            .field("layout", &self.layout)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

/// [`use_selectable_collection`] for lists: keyboard navigation through the items in collection
/// order (or in a grid layout), with type-ahead using the current locale's collation.
pub fn use_selectable_list(input: UseSelectableListInput) -> UseSelectableCollectionReturn {
    let UseSelectableListInput {
        state,
        element,
        orientation,
        layout,
        keyboard_delegate,
        layout_delegate,
        options,
    } = input;

    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state,
            element,
            orientation,
            layout,
            layout_delegate,
        })
    });

    use_selectable_collection(UseSelectableCollectionInput {
        selection: state.selection,
        item_elements: state.item_elements,
        delegate,
        element,
        options,
    })
}

/// Input of [`use_list_keyboard_delegate`].
#[derive(Clone)]
pub struct UseListKeyboardDelegateInput {
    pub state: ListState,
    /// The list element, in which the default layout delegate measures the rendered items.
    pub element: CapturedElement,
    pub orientation: Signal<Orientation>,
    pub layout: ListLayout,
    /// Where the items are. `None`: measured in the DOM, inside `element` (a virtualizer's layout
    /// knows items that aren't rendered).
    pub layout_delegate: Option<Arc<dyn LayoutDelegate>>,
}

impl std::fmt::Debug for UseListKeyboardDelegateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseListKeyboardDelegateInput")
            .field("orientation", &self.orientation)
            .field("layout", &self.layout)
            .field("layout_delegate", &self.layout_delegate.is_some())
            .finish_non_exhaustive()
    }
}

/// The keyboard delegate of a list: a [`ListKeyboardDelegate`] with the current locale's
/// reading direction and collation (for type-ahead).
pub fn use_list_keyboard_delegate(
    input: UseListKeyboardDelegateInput,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    let UseListKeyboardDelegateInput {
        state,
        element,
        orientation,
        layout,
        layout_delegate,
    } = input;

    let layout_delegate = layout_delegate
        .unwrap_or_else(|| Arc::new(DomLayoutDelegate::new(element, state.item_elements)));
    let collator = use_collator(CollatorOptions::default());
    let direction = use_direction();
    keyboard_delegate_memo(move || {
        Arc::new(
            ListKeyboardDelegate::new(state.collection, state.selection, layout_delegate.clone())
                .with_layout(layout)
                .with_orientation(orientation.get())
                .with_direction(direction.get())
                .with_collator(collator.get()),
        ) as Arc<dyn KeyboardDelegate>
    })
}
