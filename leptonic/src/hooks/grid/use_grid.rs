// Upstream: react-aria/src/grid/useGrid.ts @ 99e6102368
// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;

use super::{
    GridKeyboardDelegate, GridState, UseGridSelectionAnnouncementInput,
    UseHighlightSelectionDescriptionInput, use_grid_selection_announcement,
    use_highlight_selection_description,
};
use crate::{
    CapturedElement, EventHandler, IntoAttrs,
    hooks::{
        collections::{
            CollectionOptions, DomLayoutDelegate, Key, KeyboardDelegate, SelectionMode,
            UseSelectableCollectionAttrs, UseSelectableCollectionInput,
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
        i18n::use_direction,
        id::use_id,
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
// - Rows and cells get the grid's settings through the returned `GridData` (react-aria: a
//   `WeakMap` keyed by the state).
//
// ## OMITTED FEATURES
// - `getRowText` (the text the selection announcement reads for a row is its text value).
// - Virtualization (`aria-rowcount`/`aria-colcount`).
//
// =============================================================================

/// Input of [`use_grid`].
#[derive(Clone)]
pub struct UseGridInput {
    pub state: GridState,
    /// The grid element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    /// Ids of elements labelling the grid.
    pub aria_labelledby: Signal<Option<String>>,
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
    pub collection_id: String,
    pub on_row_action: Option<Callback<Key>>,
    pub on_cell_action: Option<Callback<Key>>,
    pub should_select_on_press_up: bool,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
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

/// Return value of [`use_grid`].
#[derive(Debug)]
pub struct UseGridReturn {
    pub props: UseGridProps,
    pub data: GridData,
}

/// Props for the grid element.
#[derive(Debug)]
pub struct UseGridProps {
    pub id: String,
    pub role: Signal<AriaRole>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_multiselectable: Signal<Option<AriaMultiselectable>>,
    /// E.g. a table's sort description.
    pub aria_describedby: Signal<Option<String>>,
    /// Keyboard navigation, type-ahead and focus handling (`use_selectable_collection`).
    pub collection: UseSelectableCollectionProps,
    pub tabbable_child: UseHasTabbableChildProps,
}

pub type UseGridAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, Signal<AriaRole>>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaMultiselectable, Signal<Option<AriaMultiselectable>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    UseSelectableCollectionAttrs,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseGridProps {
    type Attrs = UseGridAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaDescribedby, self.aria_describedby),
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
    .props;

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
        let target = e.expect_target().dyn_into::<web_sys::Node>().ok();
        let contains = e
            .expect_current_target()
            .dyn_into::<web_sys::Node>()
            .is_ok_and(|grid| grid.contains(target.as_ref()));
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

    // An empty grid is a tab stop itself, unless it has tabbable content.
    let rows = state.list.collection;
    let is_empty = Signal::derive(move || rows.with(|c| c.size() == 0));
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

    UseGridReturn {
        data: GridData {
            state,
            delegate,
            collection_id: collection.collection_id.clone(),
            on_row_action,
            on_cell_action,
            should_select_on_press_up,
            keyboard_navigation_behavior,
        },
        props: UseGridProps {
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
            tabbable_child: tabbable_child.props,
        },
    }
}
