// Upstream: react-aria/src/gridlist/useGridList.ts @ 99e6102368
// Upstream: react-aria/src/gridlist/utils.ts @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{
    hooks::{
        IntoAttrs, Orientation,
        collections::{
            CollectionOptions, Key, KeyboardDelegate, LinkBehavior, ListLayout, ListState,
            SelectionMode, UseSelectableCollectionAttrs, UseSelectableCollectionProps,
            UseSelectableListInput, use_selectable_list,
        },
        focus::use_has_tabbable_child::{
            UseHasTabbableChildAttrs, UseHasTabbableChildInput, UseHasTabbableChildProps,
            use_has_tabbable_child,
        },
        tree::TreeExpansion,
    },
    utils::{
        CapturedElement,
        aria::{AriaMultiselectable, AriaRole},
        id::use_id,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rows get the grid list's settings through the returned `GridListData` (react-aria: a
//   `WeakMap` keyed by the state), which the caller hands to `use_grid_list_item`.
//
// ## OMITTED FEATURES
// - Selection announcements (`useGridSelectionAnnouncement`) and the "highlight selection"
//   description (`useHighlightSelectionDescription`): they need localized messages.
// - Virtualization (`aria-rowcount`/`aria-colcount`).
//
// =============================================================================

/// How the keyboard moves between the rows' interactive children.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum KeyboardNavigationBehavior {
    /// ArrowLeft/ArrowRight move between a row and its focusable children.
    #[default]
    Arrow,
    /// Tab moves between a row's focusable children (arrow keys stay with them, e.g. for text
    /// inputs).
    Tab,
}

/// Input of [`use_grid_list`].
#[derive(Clone)]
pub struct UseGridListInput {
    pub state: ListState,
    /// The grid element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id (rows' ids derive from it). Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    /// Rows stacked, or wrapping in a grid of cards.
    pub layout: ListLayout,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Keyboard and focus behavior.
    pub options: CollectionOptions,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
    /// Called with the key of an activated row (pressed without selection, double-clicked or
    /// Enter in `Replace` selection behavior).
    pub on_action: Option<Callback<Key>>,
    /// Makes the rows tree items (see `use_tree`).
    pub tree: Option<TreeExpansion>,
}

/// What rows need to know about their grid list. Pass it to `use_grid_list_item`.
#[derive(Debug, Clone)]
pub struct GridListData {
    pub state: ListState,
    /// The grid element id; row ids derive from it.
    pub id: String,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: String,
    pub on_action: Option<Callback<Key>>,
    pub link_behavior: LinkBehavior,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    pub should_select_on_press_up: bool,
    /// Expansion of tree rows (`None` for flat lists).
    pub tree: Option<TreeExpansion>,
}

/// The element id of the row `key` in the grid list `list_id` (whitespace removed from the
/// key).
pub fn grid_list_row_id(list_id: &str, key: &Key) -> String {
    let key: String = key
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    format!("{list_id}-{key}")
}

/// Return value of [`use_grid_list`].
#[derive(Debug)]
pub struct UseGridListReturn {
    pub props: UseGridListProps,
    pub data: GridListData,
}

/// Props for the grid element.
#[derive(Debug)]
pub struct UseGridListProps {
    pub id: String,
    pub role: Signal<AriaRole>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_multiselectable: Signal<Option<AriaMultiselectable>>,
    /// Keyboard navigation, type-ahead and focus handling (`use_selectable_list`).
    pub collection: UseSelectableCollectionProps,
    /// Detects tabbable children of an empty grid (e.g. an "add item" button), which then take
    /// the tab stop.
    pub tabbable_child: UseHasTabbableChildProps,
}

pub type UseGridListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, Signal<AriaRole>>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaMultiselectable, Signal<Option<AriaMultiselectable>>>,
    UseSelectableCollectionAttrs,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseGridListProps {
    type Attrs = UseGridListAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            self.collection.into_attrs(),
            self.tabbable_child.into_attrs(),
        )
    }
}

/// A grid list: a list of interactive rows (`role="grid"` with one cell per row), selectable and
/// navigable like a listbox, whose rows may contain buttons, checkboxes or links.
///
/// The rows come from the state's collection; render one `use_grid_list_item` per item, in
/// collection order.
pub fn use_grid_list(input: UseGridListInput) -> UseGridListReturn {
    let UseGridListInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        layout,
        keyboard_delegate,
        options,
        keyboard_navigation_behavior,
        should_select_on_press_up,
        on_action,
        tree,
    } = input;

    if aria_label.get_untracked().is_none() && aria_labelledby.with_untracked(Option::is_none) {
        crate::utils::dev_warn!(
            "use_grid_list: an aria_label or aria_labelledby is required for accessibility"
        );
    }
    let id = id.unwrap_or_else(|| use_id("grid-list"));

    let mut collection = use_selectable_list(UseSelectableListInput {
        state,
        element,
        orientation: Orientation::Vertical,
        layout,
        layout_delegate: None,
        keyboard_delegate,
        options,
    })
    .props;

    // An empty grid is a tab stop itself, unless it has tabbable content.
    let is_empty = Signal::derive(move || state.collection.with(|c| c.is_empty()));
    let tabbable_child = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: Signal::derive(move || !is_empty.get()),
    });
    let has_tabbable_child = tabbable_child.has_tabbable_child;
    let list_tabindex = collection.tabindex;
    collection.tabindex = Signal::derive(move || {
        if is_empty.get() {
            Some(if has_tabbable_child.get() { -1 } else { 0 })
        } else {
            list_tabindex.get()
        }
    });

    UseGridListReturn {
        data: GridListData {
            state,
            id: id.clone(),
            collection_id: collection.collection_id.clone(),
            on_action,
            link_behavior: options.link_behavior,
            keyboard_navigation_behavior,
            should_select_on_press_up,
            tree,
        },
        props: UseGridListProps {
            id,
            role: Signal::stored(AriaRole::Grid),
            aria_label,
            aria_labelledby,
            aria_multiselectable: Signal::derive(move || {
                (state.selection.selection_mode() == SelectionMode::Multiple)
                    .then_some(AriaMultiselectable::True)
            }),
            collection,
            tabbable_child: tabbable_child.props,
        },
    }
}
