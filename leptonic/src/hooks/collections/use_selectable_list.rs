// Upstream: react-aria/src/selection/useSelectableList.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::{
    CollectionOptions, DomLayoutDelegate, KeyboardDelegate, ListKeyboardDelegate, ListLayout,
    ListState, UseSelectableCollectionInput, UseSelectableCollectionReturn,
    use_selectable_collection,
};
use crate::utils::{
    CapturedElement,
    filter::{Collator, CollatorOptions},
    i18n::{use_direction, use_locale},
    orientation::Orientation,
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
    pub orientation: Orientation,
    /// Items stacked (one per row/column) or wrapping in a grid.
    pub layout: ListLayout,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    pub options: CollectionOptions,
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
        options,
    } = input;

    let delegate = keyboard_delegate
        .unwrap_or_else(|| use_list_keyboard_delegate(state, element, orientation, layout));

    use_selectable_collection(UseSelectableCollectionInput {
        selection: state.selection,
        item_elements: state.item_elements,
        delegate,
        element,
        options,
    })
}

/// The keyboard delegate of a list: a [`ListKeyboardDelegate`] measuring the rendered items in
/// `element`, with the current locale's reading direction and collation (for type-ahead).
pub fn use_list_keyboard_delegate(
    state: ListState,
    element: CapturedElement,
    orientation: Orientation,
    layout: ListLayout,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    let locale = use_locale();
    let direction = use_direction();
    let layout_delegate = Arc::new(DomLayoutDelegate::new(element, state.item_elements));
    Signal::derive(move || {
        let collator = locale.with(|locale| Collator::new(locale, &CollatorOptions::default()));
        Arc::new(
            ListKeyboardDelegate::new(state.collection, state.selection, layout_delegate.clone())
                .with_layout(layout)
                .with_orientation(orientation)
                .with_direction(direction.get())
                .with_collator(Arc::new(collator)),
        ) as Arc<dyn KeyboardDelegate>
    })
}
