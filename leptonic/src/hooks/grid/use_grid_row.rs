// Upstream: react-aria/src/grid/useGridRow.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::GridData;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            Key, SelectionMode, UseSelectableItemAttrs, UseSelectableItemInput,
            UseSelectableItemProps, UseSelectableItemReturn, use_selectable_item,
        },
    },
    utils::{
        CapturedElement,
        aria::{AriaDisabled, AriaRole, AriaSelected},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`).
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
    pub on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
}

/// Return value of [`use_grid_row`].
pub struct UseGridRowReturn {
    pub row_props: PropsWithStyles<UseGridRowProps>,
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

pub type UseGridRowAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    UseSelectableItemAttrs,
);

impl IntoAttrs for UseGridRowProps {
    type Attrs = UseGridRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.item.into_attrs(),
        )
    }
}

/// A row of a grid: selected and activated by press (via `use_selectable_item`).
pub fn use_grid_row(input: UseGridRowInput) -> UseGridRowReturn {
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

    let UseSelectableItemReturn {
        props,
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
        is_disabled: Signal::derive(move || rows.with(|c| c.size() == 0)),
        should_select_on_press_up,
        allows_different_press_origin: false,
        on_action: Signal::stored(on_row_action.map(|on_row_action| {
            let key = key.clone();
            Callback::new(move |()| on_row_action.run(key.clone()))
        })),
        link_behavior: crate::hooks::collections::LinkBehavior::Action,
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu,
    });
    let (item, styles) = props.into_inner();

    UseGridRowReturn {
        row_props: PropsWithStyles::new(
            UseGridRowProps {
                role: AriaRole::Row,
                aria_selected: Signal::derive(move || {
                    (selection.selection_mode() != SelectionMode::None)
                        .then(|| AriaSelected::from(is_selected.get()))
                }),
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
                item,
            },
            styles,
        ),
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    }
}
