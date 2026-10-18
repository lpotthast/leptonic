// Upstream: react-aria/src/grid/useGridRow.ts @ 99e6102368
// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
use leptos::prelude::*;

use super::GridData;
use crate::{
    CapturedElement, IntoAttrs, PropsWithStyles,
    hooks::collections::{
        Key, SelectionMode, UseSelectableItemAttrs, UseSelectableItemInput, UseSelectableItemProps,
        UseSelectableItemReturn, use_selectable_item,
    },
    utils::{
        aria::{AriaDisabled, AriaRole, AriaSelected, aria_true},
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`): not yet, the virtualizer doesn't render grids yet.
//
// =============================================================================

/// Input of [`use_grid_row`].
#[derive(Debug, Clone)]
pub struct UseGridRowInput {
    /// The grid (from `use_grid`).
    pub grid: GridData,
    /// The row's key in the grid's collection.
    pub key: Key,
    /// Called when a context menu is requested on the row (right click, Shift+F10, the context
    /// menu key; a long press on iOS unless it selects).
    pub on_context_menu: Option<Callback<crate::hooks::interactions::ContextMenuEvent>>,
}

/// The return value of [`use_grid_row`].
#[derive(Debug)]
pub struct UseGridRowReturn {
    /// The row element: `let (attrs, styles) = row.attrs();`.
    pub row: Part<PropsWithStyles<UseGridRowProps>>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub allows_selection: Signal<bool>,
    pub has_action: Signal<bool>,
}

/// Props for the row element.
#[derive(Debug)]
pub struct UseGridRowProps {
    pub role: AriaRole,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub item: UseSelectableItemProps,
}

impl Remount for UseGridRowProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_selected: self.aria_selected,
            aria_disabled: self.aria_disabled,
            item: self.item.remount(),
        }
    }
}

pub type UseGridRowAttrs = (
    FixedAttr,
    SignalAttr<Option<AriaSelected>>,
    SignalAttr<Option<AriaDisabled>>,
    UseSelectableItemAttrs,
);

impl IntoAttrs for UseGridRowProps {
    type Attrs = UseGridRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-selected", self.aria_selected),
            signal_attr("aria-disabled", self.aria_disabled),
            self.item.into_attrs(),
        )
    }
}

/// A row of a grid: selected and activated by press (via `use_selectable_item`).
pub fn use_grid_row(input: UseGridRowInput) -> UseGridRowReturn {
    crate::metrics::subject!();
    let UseGridRowInput {
        grid,
        key,
        on_context_menu,
    } = input;
    let GridData {
        state,
        collection_id,
        on_row_action,
        should_select_on_press_up,
        ..
    } = grid;
    let selection = state.list.selection;
    let rows = state.list.collection;
    if !untrack(|| rows.with(|c| c.contains_key(&key))) {
        crate::utils::dev_warn!("use_grid_row: the key {key:?} is not in the grid's collection");
    }

    let UseSelectableItemReturn {
        item: props,
        is_pressed,
        is_selected,
        is_focused,
        is_disabled,
        allows_selection,
        has_action,
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.list.item_elements,
        key: key.clone(),
        element: CapturedElement::new(),
        id: None,
        collection_id,
        is_disabled: state.is_empty(),
        should_select_on_press_up,
        allows_different_press_origin: false,
        on_action: Signal::stored(on_row_action),
        link_behavior: Signal::stored(crate::hooks::collections::LinkBehavior::Action),
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu,
    });
    let (item, styles) = props.into_props().into_inner();

    UseGridRowReturn {
        row: Part::stored(PropsWithStyles::new(
            UseGridRowProps {
                role: AriaRole::Row,
                aria_selected: Signal::derive(move || {
                    (selection.selection_mode() != SelectionMode::None)
                        .then(|| AriaSelected::from(is_selected.get()))
                }),
                aria_disabled: aria_true(is_disabled),
                item,
            },
            styles,
        )),
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    }
}
