// Upstream: react-aria/src/grid/useGrid.ts @ 99e6102368
// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;

use super::{
    GridKeyboardDelegate, GridState, UseGridSelectionAnnouncementInput,
    UseHighlightSelectionDescriptionInput, use_grid_selection_announcement,
    use_highlight_selection_description,
};
use crate::{
    CapturedElement, EventHandler, Id, IntoAttrs,
    hooks::{
        collections::{
            CollectionOptions, DomLayoutDelegate, Key, KeyboardDelegate, LinkBehavior,
            SelectionMode, UseSelectableCollectionAttrs, UseSelectableCollectionInput,
            UseSelectableCollectionProps, keyboard_delegate_memo, use_selectable_collection,
        },
        focus::use_has_tabbable_child::{
            UseHasTabbableChildAttrs, UseHasTabbableChildInput, UseHasTabbableChildProps,
            use_has_tabbable_child,
        },
        gridlist::KeyboardNavigationBehavior,
    },
    utils::{
        aria::{AriaMultiselectable, AriaRole},
        dom_ext::EventAccessors,
        filter::{CollatorOptions, use_collator},
        i18n::{WritingDirection, use_direction},
        id::use_id,
        intl_strings::{GridStrings, use_localized_strings},
        part::{Part, Remount},
        shadow_dom::{get_event_target, node_contains},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
    },
};

/// `handler`, inactive while keyboard navigation is disabled.
fn unless_navigation_disabled<E: Clone + 'static>(
    navigation_disabled: Signal<bool>,
    handler: EventHandler<E>,
) -> EventHandler<E> {
    EventHandler::new(move |e: E| {
        if !navigation_disabled.get_untracked() {
            handler.call(e);
        }
    })
}

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rows and cells get the grid's settings through the returned `GridData`, which the caller
//   hands to them: the dependency is explicit instead of hidden global state (react-aria: a
//   `WeakMap` keyed by the state).
//
// ## OMITTED FEATURES
// - `getRowText` (the text the selection announcement reads for a row is its text value).
// - Virtualization (`aria-rowcount`/`aria-colcount`): not yet, the virtualizer doesn't render
//   grids yet.
//
// =============================================================================

/// Input of [`use_grid`].
#[derive(Clone)]
pub struct UseGridInput {
    pub state: GridState,
    /// The grid element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<Id>,
    pub aria_label: MaybeProp<String>,
    /// Ids of elements labelling the grid.
    pub aria_labelledby: Signal<Option<Id>>,
    /// Replaces the grid keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Keyboard and focus behavior.
    pub options: CollectionOptions,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
    /// Called with the key of an activated row.
    pub on_row_action: Option<Callback<Key>>,
    /// Called with the key of an activated cell.
    pub on_cell_action: Option<Callback<Key>>,
}

/// What rows and cells need to know about their grid. Pass it to `use_grid_row` and
/// `use_grid_cell`.
#[derive(Clone)]
pub struct GridData {
    pub state: GridState,
    pub delegate: Signal<Arc<dyn KeyboardDelegate>>,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: Id,
    /// The rows' action (`UseSelectableItemInput::on_action`), one signal for all rows.
    pub on_row_action: Signal<Option<Callback<Key>>>,
    /// The cells' action, one signal for all cells.
    pub on_cell_action: Signal<Option<Callback<Key>>>,
    /// The rows' and cells' link behavior (`LinkBehavior::Action`), one signal for all.
    pub(crate) link_behavior: Signal<LinkBehavior>,
    /// The cells' role (`gridcell`), one signal for all.
    pub(crate) cell_role: Signal<AriaRole>,
    pub should_select_on_press_up: bool,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// The reading direction, for all rows and cells (instead of a context lookup each).
    pub direction: Signal<WritingDirection>,
    /// The grid's localized messages, for all rows and cells (e.g.
    /// `UseGridSelectionCheckboxInput::strings`).
    pub strings: Memo<GridStrings>,
}

impl std::fmt::Debug for GridData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridData")
            .field("collection_id", &self.collection_id)
            .field(
                "keyboard_navigation_behavior",
                &self.keyboard_navigation_behavior,
            )
            .finish_non_exhaustive()
    }
}

/// The return value of [`use_grid`].
#[derive(Debug)]
pub struct UseGridReturn {
    /// The grid element.
    pub grid: Part<UseGridProps>,
    pub data: GridData,
}

/// Props for the grid element.
#[derive(Debug)]
pub struct UseGridProps {
    pub id: Id,
    pub role: Signal<AriaRole>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_multiselectable: Signal<Option<AriaMultiselectable>>,
    /// E.g. a table's sort description.
    pub aria_describedby: Signal<Option<Id>>,
    /// Keyboard navigation, type-ahead and focus handling (`use_selectable_collection`).
    pub collection: UseSelectableCollectionProps,
    pub tabbable_child: UseHasTabbableChildProps,
}

impl Remount for UseGridProps {
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

pub type UseGridAttrs = (
    FixedAttr,
    SignalAttr<AriaRole>,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<AriaMultiselectable>>,
    SignalAttr<Option<Id>>,
    UseSelectableCollectionAttrs,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseGridProps {
    type Attrs = UseGridAttrs;

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

/// Input of [`use_grid_keyboard_delegate`].
#[derive(Debug, Clone, Copy)]
pub struct UseGridKeyboardDelegateInput {
    pub state: GridState,
    /// The grid element, in which the delegate measures the rendered rows and cells.
    pub element: CapturedElement,
}

/// The keyboard delegate of a grid: a [`GridKeyboardDelegate`] measuring the rendered rows and
/// cells in `element`, with the current locale's reading direction and collation.
pub fn use_grid_keyboard_delegate(
    input: UseGridKeyboardDelegateInput,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    crate::metrics::subject!();
    let UseGridKeyboardDelegateInput { state, element } = input;
    // One collator per locale, not per read of the delegate.
    let collator = use_collator(CollatorOptions::default());
    let direction = use_direction();
    let layout_delegate = Arc::new(DomLayoutDelegate::new(element, state.list.item_elements));
    keyboard_delegate_memo(move || {
        let collator = collator.get();
        Arc::new(
            GridKeyboardDelegate::new(
                state.list.collection,
                state.list.selection,
                layout_delegate.clone(),
            )
            .with_direction(direction.get())
            .with_collator(collator)
            .with_focus_mode(state.focus_mode),
        ) as Arc<dyn KeyboardDelegate>
    })
}

/// A grid: rows of cells, navigated in two dimensions with the arrow keys; rows (or cells) can
/// be selected and activated. Render rows with `use_grid_row` and cells with `use_grid_cell`.
pub fn use_grid(input: UseGridInput) -> UseGridReturn {
    crate::metrics::subject!();
    let UseGridInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        keyboard_delegate,
        options,
        keyboard_navigation_behavior,
        should_select_on_press_up,
        on_row_action,
        on_cell_action,
    } = input;

    if aria_label.get_untracked().is_none() && aria_labelledby.with_untracked(Option::is_none) {
        crate::utils::dev_warn!(
            "use_grid: an aria_label or aria_labelledby is required for accessibility"
        );
    }
    let id = id.unwrap_or_else(|| use_id("grid"));
    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_grid_keyboard_delegate(UseGridKeyboardDelegateInput { state, element })
    });

    let mut collection = use_selectable_collection(UseSelectableCollectionInput {
        selection: state.list.selection,
        item_elements: state.list.item_elements,
        delegate,
        element,
        options,
    })
    .collection
    .into_props();

    // While keyboard navigation is disabled (e.g. while a column is resized with the arrow keys),
    // the grid only tracks whether it is focused.
    let selection = state.list.selection;
    let navigation_disabled = state.is_keyboard_navigation_disabled();
    collection.on_keydown_capture =
        unless_navigation_disabled(navigation_disabled, collection.on_keydown_capture);
    collection.on_keydown = unless_navigation_disabled(navigation_disabled, collection.on_keydown);
    collection.on_keyup = unless_navigation_disabled(navigation_disabled, collection.on_keyup);
    collection.on_mousedown =
        unless_navigation_disabled(navigation_disabled, collection.on_mousedown);
    let on_focusin = collection.on_focusin;
    collection.on_focusin = EventHandler::new(move |e: FocusEvent| {
        if !navigation_disabled.get_untracked() {
            on_focusin.call(e);
            return;
        }
        let target = get_event_target(&e).and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let contains = e
            .expect_current_target()
            .dyn_into::<web_sys::Node>()
            .is_ok_and(|grid| target.is_some_and(|target| node_contains(&grid, &target)));
        if selection.is_focused() {
            // A focus event bubbled through a portal.
            if !contains {
                selection.set_focused(false);
            }
        } else if contains {
            selection.set_focused(true);
        }
    });

    // Touch users learn how to select rows that have actions; selection changes are announced.
    let description = use_highlight_selection_description(UseHighlightSelectionDescriptionInput {
        selection,
        has_item_actions: on_row_action.is_some() || on_cell_action.is_some(),
    });
    use_grid_selection_announcement(UseGridSelectionAnnouncementInput {
        selection,
        collection: state.list.collection,
        get_row_text: None,
    });

    // An empty grid is a tab stop itself, unless it has tabbable content. A memo of its own:
    // `GridState::is_empty` is notified earlier (by the node diff), which reorders this
    // `tabindex` against `use_has_tabbable_child`'s effect (emptying a table wrote other
    // intermediate values).
    let rows = state.list.collection;
    let is_empty = Memo::new(move |_| rows.with(|c| c.size() == 0));
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

    UseGridReturn {
        data: GridData {
            state,
            delegate,
            collection_id: collection.collection_id.clone(),
            // Constants, stored once for all rows and cells instead of once per item.
            on_row_action: Signal::stored(on_row_action),
            on_cell_action: Signal::stored(on_cell_action),
            link_behavior: Signal::stored(LinkBehavior::Action),
            cell_role: Signal::stored(AriaRole::Gridcell),
            should_select_on_press_up,
            keyboard_navigation_behavior,
            direction: use_direction(),
            strings: use_localized_strings::<GridStrings>(),
        },
        grid: Part::stored(UseGridProps {
            id,
            role: Signal::stored(AriaRole::Grid),
            aria_label,
            aria_labelledby,
            aria_multiselectable: Signal::derive(move || {
                (selection.selection_mode() == SelectionMode::Multiple)
                    .then_some(AriaMultiselectable::True)
            }),
            aria_describedby: description,
            collection,
            tabbable_child: tabbable_child_props,
        }),
    }
}
