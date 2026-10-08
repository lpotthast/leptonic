// Upstream: react-aria/src/table/useTableRow.ts @ 99e6102368
use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use web_sys::KeyboardEvent;

use super::TableData;
use crate::{
    hooks::{
        IntoAttrs, PressEvent, PropsWithStyles, UseButtonInput, UseGridRowAttrs, UseGridRowInput,
        UseGridRowProps, UseGridRowReturn,
        collections::{Key, Node},
        use_grid_row,
    },
    utils::{
        EventHandler,
        aria::AriaExpanded,
        focusability::{PreventFocusAttr, prevent_focus_attr},
        i18n::use_direction,
        id::use_id,
        intl_strings::{TableStrings, use_localized_strings},
        key::{KeyboardEventKey, KeyboardKey},
        locale::WritingDirection,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A tree table's expand button is configured, not rendered: `expand_button` is the
//   `UseButtonInput` for `use_button`; its `data-leptonic-prevent-focus` attribute comes
//   separately (`expand_button_attrs`), as `UseButtonInput` takes no extra attributes.
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`), synthetic link props, `expandedKeys: 'all'`.
//
// =============================================================================

/// Input of [`use_table_row`].
#[derive(Debug, Clone)]
pub struct UseTableRowInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The row's key.
    pub key: Key,
    /// Called when a context menu is requested on the row (right click, Shift+F10, the context
    /// menu key; a long press on iOS unless it selects).
    pub on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
}

/// Return value of [`use_table_row`].
pub struct UseTableRowReturn {
    pub row_props: PropsWithStyles<UseTableRowProps>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub allows_selection: Signal<bool>,
    pub has_action: Signal<bool>,
    /// In a tree table: the expand button's configuration, for `use_button` (render it in the
    /// tree column's cell of rows with child rows). Labelled "Expand" or "Collapse", plus the
    /// row.
    pub expand_button: Option<UseButtonInput>,
    /// Spread onto the expand button besides `use_button`'s attributes
    /// (`data-leptonic-prevent-focus`: focus walks skip it).
    pub expand_button_attrs: PreventFocusAttr,
    /// Whether the row's child rows are shown (tree tables).
    pub is_expanded: Signal<bool>,
    /// Whether the row has child rows (tree tables; follows the collection).
    pub has_child_rows: Signal<bool>,
    /// The row's level, from 1 for top-level rows (tree tables; `None` otherwise).
    pub level: Signal<Option<usize>>,
}

/// Props for the row element.
#[derive(Debug)]
pub struct UseTableRowProps {
    pub row: UseGridRowProps,
    /// The row header cells label the row.
    pub aria_labelledby: Signal<String>,
    /// Tree tables: whether the row's child rows are shown (rows with child rows only).
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// Tree tables: the row's level (from 1), its position among its sibling rows and their
    /// number.
    pub aria_level: Signal<Option<usize>>,
    pub aria_posinset: Signal<Option<usize>>,
    pub aria_setsize: Signal<Option<usize>>,
    /// Tree tables: ArrowRight expands the focused row, ArrowLeft collapses it or moves to the
    /// parent row (mirrored in right-to-left text).
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
}

pub type UseTableRowAttrs = (
    UseGridRowAttrs,
    Attr<attr::AriaLabelledby, Signal<String>>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    CustomAttr<&'static str, Signal<Option<usize>>>,
    Attr<attr::AriaPosinset, Signal<Option<usize>>>,
    Attr<attr::AriaSetsize, Signal<Option<usize>>>,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
);

impl IntoAttrs for UseTableRowProps {
    type Attrs = UseTableRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.row.into_attrs(),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaExpanded, self.aria_expanded),
            custom_attribute("aria-level", self.aria_level),
            Attr(attr::AriaPosinset, self.aria_posinset),
            Attr(attr::AriaSetsize, self.aria_setsize),
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
        )
    }
}

/// The row's position in a tree table: its level (from 1), its position among its sibling rows
/// (from 1) and their number. Siblings are rows (items), not cells.
fn tree_position(
    collection: &crate::hooks::Collection,
    key: &Key,
) -> Option<(usize, usize, usize)> {
    let node = collection.get(key)?;
    let siblings: Vec<&Node> = match &node.parent_key {
        Some(parent) => collection
            .children(parent)
            .filter(|n| n.is_item())
            .collect(),
        None => collection.iter().filter(|n| n.is_item()).collect(),
    };
    let index = siblings.iter().position(|n| n.key == *key)?;
    Some((node.level + 1, index + 1, siblings.len()))
}

/// A body row of a table, labelled by its row header cells. In a tree table also a tree item:
/// `aria-level`, `aria-posinset`, `aria-setsize`, `aria-expanded`, the expansion keys and the
/// expand button.
#[allow(clippy::too_many_lines)]
pub fn use_table_row(input: UseTableRowInput) -> UseTableRowReturn {
    let UseTableRowInput {
        table,
        key,
        on_context_menu,
    } = input;
    let aria_labelledby = {
        let table = table.clone();
        let key = key.clone();
        Signal::derive(move || table.row_labelledby(&key))
    };
    let state = table.state;
    let tree = state.tree;
    let selection = state.grid.list.selection;
    let UseGridRowReturn {
        row_props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    } = use_grid_row(UseGridRowInput {
        grid: table.grid.clone(),
        key: key.clone(),
        on_context_menu,
    });
    let (row, styles) = row_props.into_inner();

    // -- Tree tables --
    // From the whole table: collapsed rows keep their child rows there.
    let full = state.table;
    let row_key = StoredValue::new(key);
    let position = Memo::new(move |_| {
        tree?;
        row_key.with_value(|key| full.with(|t| tree_position(t.collection(), key)))
    });
    let has_child_rows = Memo::new(move |_| {
        tree.is_some()
            && row_key
                .with_value(|key| full.with(|t| t.collection().children(key).any(Node::is_item)))
    });
    // A row without child rows is never expanded, even when its key is among the expanded ones.
    let is_expanded = Signal::derive(move || {
        has_child_rows.get()
            && tree.is_some_and(|tree| row_key.with_value(|key| tree.expansion.is_expanded(key)))
    });
    let direction = use_direction();
    let on_keydown_capture = EventHandler::new(move |e: KeyboardEvent| {
        let Some(tree) = tree else {
            return;
        };
        let key = row_key.get_value();
        if !untrack(|| selection.is_focused_key(&key)) {
            return;
        }
        let (expand, collapse) = if direction.get_untracked() == WritingDirection::Rtl {
            (KeyboardKey::ArrowLeft, KeyboardKey::ArrowRight)
        } else {
            (KeyboardKey::ArrowRight, KeyboardKey::ArrowLeft)
        };
        let pressed = e.typed_key();
        let expanded = untrack(|| tree.expansion.is_expanded(&key));
        let has_children = has_child_rows.get_untracked();
        if pressed == expand && has_children && !expanded {
            tree.expansion.toggle_key(key);
            e.stop_propagation();
        } else if pressed == collapse {
            if has_children && expanded {
                tree.expansion.toggle_key(key);
                e.stop_propagation();
            } else if !expanded
                && let Some(parent) = full.with_untracked(|t| {
                    let parent = t.collection().get(&key)?.parent_key.clone()?;
                    t.collection().get(&parent)?.is_item().then_some(parent)
                })
            {
                // A leaf or collapsed row: focus moves to the parent row.
                selection.set_focused_key(Some(parent), None);
                e.stop_propagation();
            }
        }
    });

    let expand_button = tree.map(|tree| {
        let button_id = use_id("table-expand");
        let strings = use_localized_strings::<TableStrings>();
        UseButtonInput {
            // Labelled by its own label ("Expand"/"Collapse") and the row.
            id: Some(button_id.clone()),
            aria_label: MaybeProp::derive(move || {
                let strings = strings.read();
                Some(if is_expanded.get() {
                    strings.collapse()
                } else {
                    strings.expand()
                })
            }),
            aria_labelledby: Signal::derive(move || {
                Some(format!("{button_id} {}", aria_labelledby.get()))
            }),
            is_disabled,
            exclude_from_tab_order: Signal::stored(true),
            prevent_focus_on_press: true,
            on_press: Some(Callback::new(move |_: PressEvent| {
                if is_disabled.get_untracked() {
                    return;
                }
                let key = row_key.get_value();
                tree.expansion.toggle_key(key.clone());
                selection.set_focused(true);
                selection.set_focused_key(Some(key), None);
            })),
            ..UseButtonInput::default()
        }
    });

    UseTableRowReturn {
        row_props: PropsWithStyles::new(
            UseTableRowProps {
                row,
                aria_labelledby,
                aria_expanded: Signal::derive(move || {
                    has_child_rows
                        .get()
                        .then(|| AriaExpanded::from(is_expanded.get()))
                }),
                aria_level: Signal::derive(move || position.get().map(|(level, _, _)| level)),
                aria_posinset: Signal::derive(move || position.get().map(|(_, index, _)| index)),
                aria_setsize: Signal::derive(move || position.get().map(|(_, _, size)| size)),
                on_keydown_capture,
            },
            styles,
        ),
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
        expand_button,
        expand_button_attrs: prevent_focus_attr(),
        is_expanded,
        has_child_rows: has_child_rows.into(),
        level: Signal::derive(move || position.get().map(|(level, _, _)| level)),
    }
}
