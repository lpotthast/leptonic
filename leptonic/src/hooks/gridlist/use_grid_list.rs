// Upstream: react-aria/src/gridlist/useGridList.ts @ 99e6102368
// Upstream: react-aria/src/gridlist/utils.ts @ 99e6102368
// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
// Upstream: react-aria-components/test/GridList.browser.test.tsx @ 99e6102368
use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

use crate::{
    CapturedElement, Id, IntoAttrs, Orientation,
    hooks::{
        collections::{
            Collection, CollectionOptions, Key, KeyboardDelegate, LinkBehavior, ListLayout,
            ListState, Node, SelectionMode, UseSelectableCollectionAttrs,
            UseSelectableCollectionProps, UseSelectableListInput, use_selectable_list,
        },
        focus::use_has_tabbable_child::{
            UseHasTabbableChildAttrs, UseHasTabbableChildInput, UseHasTabbableChildProps,
            use_has_tabbable_child,
        },
        grid::{
            UseGridSelectionAnnouncementInput, UseHighlightSelectionDescriptionInput,
            use_grid_selection_announcement, use_highlight_selection_description,
        },
        tree::TreeExpansion,
    },
    utils::{
        aria::{AriaMultiselectable, AriaRole},
        id::use_id,
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rows get the grid list's settings through the returned `GridListData`, which the caller
//   hands to `use_grid_list_item`: the dependency is explicit instead of hidden global state
//   (react-aria: a `WeakMap` keyed by the state).
// - `layout` and `orientation` configure the default list keyboard delegate, which
//   react-aria-components' `GridList` builds itself and passes as `keyboardDelegate`.
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowcount`/`aria-colcount`): not yet, the virtualizer doesn't render
//   grid lists yet.
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
#[derive(Clone, Debug)]
pub struct UseGridListInput {
    pub state: ListState,
    /// The grid element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id (rows' ids derive from it). Generated when `None`.
    pub id: Option<Id>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    /// Rows stacked, or wrapping in a grid of cards.
    pub layout: ListLayout,
    /// The primary orientation of the rows, usually the direction the list scrolls
    /// (react-aria-components' default: `Vertical`). In a horizontal list, ArrowUp/ArrowDown move
    /// to the previous/next row in collection order and ArrowLeft/ArrowRight to the neighboring
    /// column (the row in the same line, in a grid layout). Ignored when a `keyboard_delegate` is
    /// given.
    pub orientation: Signal<Orientation>,
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
    pub id: Id,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: Id,
    pub on_action: Option<Callback<Key>>,
    pub link_behavior: Signal<LinkBehavior>,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    pub should_select_on_press_up: bool,
    /// Expansion of tree rows (`None` for flat lists).
    pub tree: Option<TreeExpansion>,
    /// Tree rows: every visible row's position, computed once per collection (`None` for flat
    /// lists).
    pub tree_positions: Option<Memo<Arc<HashMap<Key, TreeRowPosition>>>>,
}

/// Where a tree row is: `aria-level`, `aria-posinset` and `aria-setsize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeRowPosition {
    /// The level, from 1 for top-level rows.
    pub level: usize,
    /// The position among the sibling rows, from 1.
    pub index: usize,
    /// The number of sibling rows.
    pub set_size: usize,
}

/// The position of every row of a tree collection, in one pass (react-aria computes each row's
/// separately, counting its siblings: quadratic in the number of rows).
pub fn tree_row_positions(collection: &Collection) -> HashMap<Key, TreeRowPosition> {
    fn visit<'a>(
        collection: &'a Collection,
        siblings: impl Iterator<Item = &'a Node>,
        positions: &mut HashMap<Key, TreeRowPosition>,
    ) {
        let rows: Vec<&Node> = siblings.filter(|node| node.is_item()).collect();
        let set_size = rows.len();
        for (index, row) in rows.into_iter().enumerate() {
            positions.insert(
                row.key.clone(),
                TreeRowPosition {
                    level: row.level + 1,
                    index: index + 1,
                    set_size,
                },
            );
            if collection.first_child(&row.key).is_some() {
                visit(collection, collection.children(&row.key), positions);
            }
        }
    }
    let mut positions = HashMap::with_capacity(collection.size());
    visit(collection, collection.iter(), &mut positions);
    positions
}

/// The element id of the row `key` in the grid list `list_id` (see [`Key::id_fragment`]).
pub fn grid_list_row_id(list_id: &str, key: &Key) -> Id {
    Id::derived(format_args!("{list_id}-{}", key.id_fragment()))
}

/// The return value of [`use_grid_list`].
#[derive(Debug)]
pub struct UseGridListReturn {
    /// The grid list element.
    pub grid_list: Part<UseGridListProps>,
    pub data: GridListData,
}

/// Props for the grid element.
#[derive(Debug)]
pub struct UseGridListProps {
    pub id: Id,
    pub role: Signal<AriaRole>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_multiselectable: Signal<Option<AriaMultiselectable>>,
    /// How touch users select rows that have actions ("highlight selection").
    pub aria_describedby: Signal<Option<Id>>,
    /// Keyboard navigation, type-ahead and focus handling (`use_selectable_list`).
    pub collection: UseSelectableCollectionProps,
    /// Detects tabbable children of an empty grid (e.g. an "add item" button), which then take
    /// the tab stop.
    pub tabbable_child: UseHasTabbableChildProps,
}

impl Remount for UseGridListProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_multiselectable: self.aria_multiselectable,
            aria_describedby: self.aria_describedby,
            collection: self.collection.remount(),
            tabbable_child: self.tabbable_child.remount(),
        }
    }
}

pub type UseGridListAttrs = (
    FixedAttr,
    SignalAttr<AriaRole>,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<AriaMultiselectable>>,
    SignalAttr<Option<Id>>,
    UseSelectableCollectionAttrs,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseGridListProps {
    type Attrs = UseGridListAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            signal_attr("role", self.role),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-multiselectable", self.aria_multiselectable),
            signal_attr("aria-describedby", self.aria_describedby),
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
    crate::metrics::subject!();
    let UseGridListInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        layout,
        orientation,
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
        orientation,
        layout,
        layout_delegate: None,
        keyboard_delegate,
        options,
    })
    .collection
    .into_props();

    // Touch users learn how to select rows that have actions; selection changes are announced.
    let description = use_highlight_selection_description(UseHighlightSelectionDescriptionInput {
        selection: state.selection,
        has_item_actions: on_action.is_some(),
    });
    use_grid_selection_announcement(UseGridSelectionAnnouncementInput {
        selection: state.selection,
        collection: state.collection,
        get_row_text: None,
    });

    // An empty grid is a tab stop itself, unless it has tabbable content.
    let is_empty = Memo::new(move |_| state.collection.with(|c| c.is_empty()));
    let tabbable_child = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: Signal::derive(move || !is_empty.get()),
    });
    let tabbable_child_props = tabbable_child.target.into_props();
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
            tree_positions: tree.map(|_| {
                // Recomputed only when the collection changes, and then always a change.
                Memo::new_with_compare(
                    move |_| state.collection.with(|c| Arc::new(tree_row_positions(c))),
                    |_, _| true,
                )
            }),
        },
        grid_list: Part::stored(UseGridListProps {
            id,
            role: Signal::stored(AriaRole::Grid),
            aria_label,
            aria_labelledby,
            aria_multiselectable: Signal::derive(move || {
                (state.selection.selection_mode() == SelectionMode::Multiple)
                    .then_some(AriaMultiselectable::True)
            }),
            aria_describedby: description,
            collection,
            tabbable_child: tabbable_child_props,
        }),
    }
}
