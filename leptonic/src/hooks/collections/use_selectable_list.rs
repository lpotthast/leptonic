// Upstream: react-aria/src/selection/useSelectableList.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::{
    CollectionOptions, DomLayoutDelegate, KeyboardDelegate, LayoutDelegate, ListKeyboardDelegate,
    ListLayout, ListState, UseSelectableCollectionInput, UseSelectableCollectionReturn,
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
    /// Where the items are, for the list keyboard delegate. Default: measured in the DOM (a
    /// virtualizer's layout knows items that aren't rendered).
    pub layout_delegate: Option<Arc<dyn LayoutDelegate>>,
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
        layout_delegate,
        options,
    } = input;

    let delegate = keyboard_delegate.unwrap_or_else(|| {
        let layout_delegate = layout_delegate
            .unwrap_or_else(|| Arc::new(DomLayoutDelegate::new(element, state.item_elements)));
        use_list_keyboard_delegate_with(state, orientation, layout, layout_delegate)
    });

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
    let layout_delegate = Arc::new(DomLayoutDelegate::new(element, state.item_elements));
    use_list_keyboard_delegate_with(state, orientation, layout, layout_delegate)
}

/// The keyboard delegate of a list with the given [`LayoutDelegate`] (e.g. a virtualizer's
/// layout).
pub fn use_list_keyboard_delegate_with(
    state: ListState,
    orientation: Orientation,
    layout: ListLayout,
    layout_delegate: Arc<dyn LayoutDelegate>,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    let locale = use_locale();
    let direction = use_direction();
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
