// Upstream: react-aria/src/table/useTableColumnHeader.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::{ColumnKind, SortDirection, TableData};
use crate::{
    hooks::{
        CellFocusMode, IntoAttrs, PropsWithStyles, UseGridCellAttrs, UseGridCellInput,
        UseGridCellProps, UseGridCellReturn,
        collections::{Key, SelectionMode},
        focus::use_focusable::{
            FocusableContextAttr, UseFocusableInput, UseFocusableProps, use_focusable,
        },
        interactions::use_press::{UsePressAttrs, UsePressInput, UsePressProps, use_press},
        use_grid_cell,
    },
    utils::{
        ElementCaptureAttr, EventHandler,
        aria::{AriaRole, AriaSort},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Of `useFocusable`, the header only takes a `FocusableContext`'s handlers, element capture,
//   description and attributes (e.g. from a `TooltipTrigger`); the cell keeps its own tabindex
//   (react-aria merges `useFocusable`'s props first, so the grid cell's tabindex wins there too).
//
// ## OMITTED FEATURES
// - The "sortable" description (`aria-describedby`): it needs localized messages.
// - Android's sort direction description (Android gets `aria-sort` as well).
//
// =============================================================================

/// Input of [`use_table_column_header`].
#[derive(Debug, Clone)]
pub struct UseTableColumnHeaderInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The column's key.
    pub key: Key,
    /// Let ArrowLeft/ArrowRight move between the header's children even with
    /// `KeyboardNavigationBehavior::Tab`.
    pub allows_arrow_navigation: bool,
}

/// Return value of [`use_table_column_header`].
pub struct UseTableColumnHeaderReturn {
    pub column_header_props: PropsWithStyles<UseTableColumnHeaderProps>,
    pub is_pressed: Signal<bool>,
}

/// Props for the column header element.
#[derive(Debug)]
pub struct UseTableColumnHeaderProps {
    /// Focus and navigation (`use_grid_cell`), with the `columnheader` role.
    pub cell: UseGridCellProps,
    /// Sorting by pressing the header.
    pub press: UsePressProps,
    pub aria_sort: Signal<Option<AriaSort>>,
    /// What a `FocusableContext` (e.g. a `TooltipTrigger` around the header) adds.
    pub focusable: UseTableColumnHeaderFocusableProps,
}

/// The parts of `use_focusable`'s props a column header takes: a `FocusableContext`'s handlers,
/// element capture, description and attributes (the cell keeps its own tabindex).
#[derive(Debug)]
pub struct UseTableColumnHeaderFocusableProps {
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
    pub aria_describedby: Signal<Option<String>>,
    pub context_attrs: FocusableContextAttr,
}

impl From<UseFocusableProps> for UseTableColumnHeaderFocusableProps {
    fn from(props: UseFocusableProps) -> Self {
        Self {
            on_focus: props.on_focus,
            on_blur: props.on_blur,
            on_keydown: props.on_keydown,
            on_keyup: props.on_keyup,
            element_capture: props.element_capture,
            aria_describedby: props.context_aria_describedby,
            context_attrs: FocusableContextAttr(props.context_attrs),
        }
    }
}

pub type UseTableColumnHeaderFocusableAttrs = (
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    ElementCaptureAttr,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    FocusableContextAttr,
);

impl IntoAttrs for UseTableColumnHeaderFocusableProps {
    type Attrs = UseTableColumnHeaderFocusableAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.element_capture,
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.context_attrs,
        )
    }
}

pub type UseTableColumnHeaderAttrs = (
    UseGridCellAttrs,
    UsePressAttrs,
    Attr<attr::AriaSort, Signal<Option<AriaSort>>>,
    UseTableColumnHeaderFocusableAttrs,
);

impl IntoAttrs for UseTableColumnHeaderProps {
    type Attrs = UseTableColumnHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.cell.into_attrs(),
            self.press.into_attrs(),
            Attr(attr::AriaSort, self.aria_sort),
            self.focusable.into_attrs(),
        )
    }
}

/// A column header of a table: focusable like a cell (its children get focus first), and
/// sorting the table when pressed if the column allows sorting.
pub fn use_table_column_header(input: UseTableColumnHeaderInput) -> UseTableColumnHeaderReturn {
    let UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation,
    } = input;
    let state = table.state;
    let selection = state.grid.list.selection;
    // The column follows the table (e.g. when columns change).
    let column = {
        let key = key.clone();
        Memo::new(move |_| {
            state.table.with(|t| {
                t.column(&key).map_or((false, false), |c| {
                    (c.allows_sorting, c.kind == ColumnKind::SelectionCheckbox)
                })
            })
        })
    };
    let allows_sorting = Signal::derive(move || column.get().0);
    let is_selection_column = Signal::derive(move || column.get().1);

    let UseGridCellReturn {
        grid_cell_props,
        is_pressed: cell_pressed,
    } = use_grid_cell(UseGridCellInput {
        id: Some(table.column_header_id(&key)),
        focus_mode: Some(CellFocusMode::Child),
        allows_arrow_navigation,
        grid: table.grid.clone(),
        key: key.clone(),
        should_select_on_press_up: false,
    });
    let (mut cell, cell_styles) = grid_cell_props.into_inner();
    cell.role = Signal::stored(AriaRole::Columnheader);

    // Without rows, the headers aren't focusable (and lose focus).
    let is_empty = Signal::derive(move || state.table.with(|t| t.size() == 0));
    let cell_tabindex = cell.item.tabindex;
    cell.item.tabindex = Signal::derive(move || {
        if is_empty.get() {
            Some(-1)
        } else {
            cell_tabindex.get()
        }
    });
    let focus_key = key.clone();
    Effect::new(move || {
        if is_empty.get() && untrack(|| selection.is_focused_key(&focus_key)) {
            selection.set_focused_key(None, None);
        }
    });

    let sort_key = key.clone();
    let press = use_press(UsePressInput {
        is_disabled: Signal::derive(move || {
            !allows_sorting.get()
                || (is_selection_column.get()
                    && selection.selection_mode() == SelectionMode::Single)
        }),
        on_press: Some(Callback::new(move |_| state.sort(&sort_key, None))),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    let sorted_key = key;
    let aria_sort = Signal::derive(move || {
        allows_sorting.get().then(|| {
            state
                .sort_descriptor
                .with(|sort| match sort {
                    Some(sort) if sort.column == sorted_key => Some(sort.direction),
                    _ => None,
                })
                .map_or(AriaSort::None, |direction| match direction {
                    SortDirection::Ascending => AriaSort::Ascending,
                    SortDirection::Descending => AriaSort::Descending,
                })
        })
    });
    let is_pressed = press.is_pressed;
    // Picks up a `FocusableContext` (e.g. a tooltip on the header).
    let focusable = use_focusable(UseFocusableInput::default()).props.into();

    UseTableColumnHeaderReturn {
        column_header_props: PropsWithStyles::new(
            UseTableColumnHeaderProps {
                cell,
                press: press_props,
                aria_sort,
                focusable,
            },
            cell_styles.merge(press_styles),
        ),
        is_pressed: Signal::derive(move || cell_pressed.get() || is_pressed.get()),
    }
}
