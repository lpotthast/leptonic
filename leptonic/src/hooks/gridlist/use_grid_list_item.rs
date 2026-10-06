// Upstream: react-aria/src/gridlist/useGridListItem.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent};

use super::{GridListData, KeyboardNavigationBehavior, grid_list_row_id};
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            Key, Node, NodeKind, SelectionMode, UseSelectableItemAttrs, UseSelectableItemInput,
            UseSelectableItemProps, UseSelectableItemReturn, use_selectable_item,
        },
        focus::use_focus_visible::{
            Modality, UseFocusVisibleInput, get_modality, use_focus_visible,
        },
    },
    utils::{
        CapturedElement, EventAccessors, EventHandler, SlotProps,
        aria::{AriaDisabled, AriaExpanded, AriaRole, AriaSelected},
        focus::focus_safely,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        i18n::use_direction,
        key::{KeyboardEventKey, KeyboardKey},
        locale::WritingDirection,
        node_contains,
        owner_alive::OwnerAlive,
        scroll::{ScrollIntoViewportOpts, get_scroll_parent, scroll_into_viewport},
        shadow_dom::get_active_element,
        use_slot,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rows read their settings from the grid list (`GridListData`) and their label, disabled
//   state and link from the collection node.
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`) and synthetic link attributes.
//
// =============================================================================

/// What gets focus when a row is focused.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FocusMode {
    /// The row itself.
    #[default]
    Row,
    /// The row's first focusable child (e.g. the remove button of a tag).
    Child,
}

/// Input of [`use_grid_list_item`].
#[derive(Debug, Clone)]
pub struct UseGridListItemInput {
    /// The grid list (from `use_grid_list`).
    pub list: GridListData,
    /// The row's key in the grid list's collection.
    pub key: Key,
    pub focus_mode: FocusMode,
    /// Let ArrowLeft/ArrowRight move between the row's children even with
    /// `KeyboardNavigationBehavior::Tab`.
    pub allows_arrow_navigation: bool,
}

impl UseGridListItemInput {
    /// The row `key` of `list`, focused as a whole.
    pub fn new(list: GridListData, key: Key) -> Self {
        Self {
            list,
            key,
            focus_mode: FocusMode::Row,
            allows_arrow_navigation: false,
        }
    }
}

/// Return value of [`use_grid_list_item`].
pub struct UseGridListItemReturn {
    pub row_props: PropsWithStyles<UseGridListItemRowProps>,
    pub grid_cell_props: UseGridListItemCellProps,
    /// For an element describing the row (referenced only while rendered).
    pub description_props: SlotProps,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub allows_selection: Signal<bool>,
    pub has_action: Signal<bool>,
}

/// Props for the row element.
#[derive(Debug)]
pub struct UseGridListItemRowProps {
    pub role: AriaRole,
    pub aria_label: Option<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    /// Tree rows: whether expanded (rows with children), level and position among siblings.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_level: Signal<Option<usize>>,
    pub aria_posinset: Signal<Option<usize>>,
    pub aria_setsize: Signal<Option<usize>>,
    pub item: UseSelectableItemProps,
    /// Navigation between the row's focusable children.
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
}

pub type UseGridListItemRowAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    (
        Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
        leptos::attr::custom::CustomAttr<&'static str, Signal<Option<usize>>>,
        Attr<attr::AriaPosinset, Signal<Option<usize>>>,
        Attr<attr::AriaSetsize, Signal<Option<usize>>>,
    ),
    UseSelectableItemAttrs,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

impl IntoAttrs for UseGridListItemRowProps {
    type Attrs = UseGridListItemRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            (
                Attr(attr::AriaExpanded, self.aria_expanded),
                leptos::attr::custom::custom_attribute("aria-level", self.aria_level),
                Attr(attr::AriaPosinset, self.aria_posinset),
                Attr(attr::AriaSetsize, self.aria_setsize),
            ),
            self.item.into_attrs(),
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
            self.on_focus.into_on(ev::focus),
        )
    }
}

/// Props for the row's single cell.
#[derive(Debug)]
pub struct UseGridListItemCellProps {
    pub role: AriaRole,
    pub aria_colindex: u32,
}

pub type UseGridListItemCellAttrs = (Attr<attr::Role, AriaRole>, Attr<attr::AriaColindex, u32>);

impl IntoAttrs for UseGridListItemCellProps {
    type Attrs = UseGridListItemCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
        )
    }
}

/// A row of a grid list: selection and actions on press (via `use_selectable_item`), and
/// keyboard navigation into the row's interactive children (ArrowLeft/ArrowRight, or Tab with
/// `KeyboardNavigationBehavior::Tab`).
#[allow(clippy::too_many_lines)]
pub fn use_grid_list_item(input: UseGridListItemInput) -> UseGridListItemReturn {
    crate::hooks::track_interaction_modality();
    let UseGridListItemInput {
        list,
        key,
        focus_mode,
        allows_arrow_navigation,
    } = input;
    let GridListData {
        state,
        id: list_id,
        collection_id,
        on_action,
        link_behavior,
        keyboard_navigation_behavior,
        should_select_on_press_up,
        tree,
    } = list;
    let selection = state.selection;
    let direction = use_direction();
    let row_id = grid_list_row_id(&list_id, &key);

    let (label, has_label) = untrack(|| {
        state.collection.with(|c| {
            let node = c.get(&key);
            let label = node.and_then(|n| {
                n.aria_label
                    .as_deref()
                    .map(str::to_owned)
                    .or_else(|| (!n.text_value.is_empty()).then(|| n.text_value.to_string()))
            });
            let has_label = label.is_some();
            (label, has_label)
        })
    });

    // -- Tree rows --
    let has_child_rows = tree.is_some()
        && untrack(|| {
            state
                .collection
                .with(|c| c.get(&key).is_some_and(|n| n.has_child_nodes))
        });
    let tree_key = StoredValue::new(key.clone());
    let is_expanded = move || tree.is_some_and(|tree| tree_key.with_value(|k| tree.is_expanded(k)));
    // Without an action, link or selection, pressing a parent row toggles it.
    let on_action = on_action.map(|on_action| {
        let key = key.clone();
        Callback::new(move |()| on_action.run(key.clone()))
    });
    let on_action = on_action.or_else(|| {
        let tree = tree?;
        let toggles = has_child_rows
            && !untrack(|| tree_key.with_value(|k| selection.is_link(k)))
            && untrack(|| selection.selection_mode()) == SelectionMode::None;
        toggles.then(|| Callback::new(move |()| tree.toggle_key(tree_key.get_value())))
    });
    let position = Signal::derive(move || {
        tree?;
        state.collection.with(|c| {
            let node = c.get(&tree_key.get_value())?;
            let siblings: Vec<&Key> = match &node.parent_key {
                Some(parent) => Box::new(c.children(parent)) as Box<dyn Iterator<Item = &Node>>,
                None => Box::new(c.iter()),
            }
            .filter(|n| n.kind == NodeKind::Item)
            .map(|n| &n.key)
            .collect::<Vec<_>>();
            let index = siblings.iter().position(|k| **k == node.key)?;
            Some((node.level + 1, index + 1, siblings.len()))
        })
    });

    let element = CapturedElement::new();
    let key_when_focused: StoredValue<Option<Key>> = StoredValue::new(None);
    let row_key = StoredValue::new(key.clone());

    // Focuses the row (or its first focusable child), unless focus already is within it.
    let focus_row = move || {
        let Some(row) = element.get_untracked() else {
            return;
        };
        let row: web_sys::Element = (*row).clone();
        let active = row.owner_document().as_ref().and_then(get_active_element);
        let focus_within = active
            .as_ref()
            .is_some_and(|a| row.contains(Some(a.unchecked_ref())));
        if focus_mode == FocusMode::Child {
            if focus_within && active.as_ref() != Some(&row) {
                return;
            }
            if let Some(mut walker) = get_focusable_tree_walker(
                &row,
                FocusableTreeWalkerOptions {
                    tabbable: true,
                    ..FocusableTreeWalkerOptions::default()
                },
            ) && let Some(child) = walker
                .first_child()
                .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
            {
                focus_safely(&child);
                let container = get_scroll_parent(&child, false);
                scroll_into_viewport(
                    Some(&child),
                    &ScrollIntoViewportOpts {
                        containing_element: Some(container),
                    },
                );
                return;
            }
        }
        let moved = key_when_focused
            .get_value()
            .is_some_and(|k| row_key.with_value(|key| *key != k));
        if moved || !focus_within {
            focus_safely(&row);
        }
    };

    let UseSelectableItemReturn {
        props: item_props,
        is_pressed,
        is_selected,
        is_focused,
        is_disabled,
        allows_selection,
        has_action,
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.item_elements,
        key: key.clone(),
        element,
        id: Some(row_id.clone()),
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up,
        allows_different_press_origin: false,
        on_action,
        link_behavior,
        focus: Some(Callback::new(move |()| focus_row())),
        should_use_virtual_focus: false,
    });
    let (mut item_props, item_styles) = item_props.into_inner();

    // Moves focus to `target` and scrolls it into view.
    let focus_and_reveal = move |target: &web_sys::Element| {
        focus_safely(target);
        if let Some(row) = element.get_untracked() {
            let container = get_scroll_parent(&row, false);
            scroll_into_viewport(
                Some(target),
                &ScrollIntoViewportOpts {
                    containing_element: Some(container),
                },
            );
        }
    };

    // ArrowRight expands a collapsed parent; ArrowLeft collapses it, or moves to the parent row
    // (mirrored in right-to-left). Returns whether the key was handled.
    let handle_tree_expansion_keys = move |e: &KeyboardEvent, active: &web_sys::Element| {
        let Some(tree) = tree else {
            return false;
        };
        let on_row = element.get_untracked().is_some_and(|row| *row == *active);
        if !allows_arrow_navigation && !on_row {
            return false;
        }
        let is_focused_row = untrack(|| tree_key.with_value(|k| selection.is_focused_key(k)));
        if !is_focused_row {
            return false;
        }
        let rtl = direction.get_untracked() == WritingDirection::Rtl;
        let (expand_key, collapse_key) = if rtl {
            ("ArrowLeft", "ArrowRight")
        } else {
            ("ArrowRight", "ArrowLeft")
        };
        let key = e.key();
        let expanded = untrack(is_expanded);
        if key == expand_key && has_child_rows && !expanded {
            tree.toggle_key(tree_key.get_value());
            e.stop_propagation();
            return true;
        }
        if key == collapse_key {
            if has_child_rows && expanded {
                tree.toggle_key(tree_key.get_value());
                e.stop_propagation();
                return true;
            }
            let parent = untrack(|| {
                state.collection.with(|c| {
                    let parent = c.get(&tree_key.get_value())?.parent_key.clone()?;
                    (c.get(&parent)?.kind == NodeKind::Item).then_some(parent)
                })
            });
            if !expanded && let Some(parent) = parent {
                selection.set_focused_key(Some(parent), None);
                e.stop_propagation();
                return true;
            }
        }
        false
    };

    let on_keydown_capture = move |e: KeyboardEvent| {
        if keyboard_navigation_behavior != KeyboardNavigationBehavior::Arrow
            && !allows_arrow_navigation
        {
            return;
        }
        let Some(row) = element.get_untracked() else {
            return;
        };
        let row: web_sys::Element = (*row).clone();
        let target = e.expect_target().dyn_into::<web_sys::Node>().ok();
        if !node_contains(Some(row.unchecked_ref()), target.as_ref()).unwrap_or(false) {
            return;
        }
        let Some(active) = row.owner_document().as_ref().and_then(get_active_element) else {
            return;
        };
        let Some(mut walker) =
            get_focusable_tree_walker(&row, FocusableTreeWalkerOptions::default())
        else {
            return;
        };
        walker.set_current_node(active.unchecked_ref());
        if handle_tree_expansion_keys(&e, &active) {
            return;
        }
        let rtl = direction.get_untracked() == WritingDirection::Rtl;

        let key = e.key();
        match key.as_str() {
            "ArrowLeft" | "ArrowRight" => {
                if keyboard_navigation_behavior != KeyboardNavigationBehavior::Arrow {
                    return;
                }
                // "Forward" is the reading direction.
                let forward = (key == "ArrowRight") != rtl;
                let next = if forward {
                    walker.next_node()
                } else {
                    walker.previous_node()
                };
                e.prevent_default();
                e.stop_propagation();
                match next.and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                    Some(next) => focus_and_reveal(&next),
                    // Past the last child: back to the row. Before the row: its last child.
                    None if forward => focus_and_reveal(&row),
                    None => {
                        walker.set_current_node(row.unchecked_ref());
                        let mut last = None;
                        while let Some(node) = walker.last_child() {
                            last = Some(node);
                        }
                        if let Some(last) = last.and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                        {
                            focus_and_reveal(&last);
                        }
                    }
                }
            }
            // Up/down from a child: let the grid move to the neighboring row.
            "ArrowUp" | "ArrowDown" if !e.alt_key() => {
                if active == row {
                    return;
                }
                e.stop_propagation();
                e.prevent_default();
                if let Some(grid) = row.parent_element() {
                    let _ = grid.dispatch_event(&clone_keyboard_event(&e));
                }
            }
            _ => {}
        }
    };

    let tab_navigation = move |e: &KeyboardEvent| {
        if keyboard_navigation_behavior != KeyboardNavigationBehavior::Tab {
            return;
        }
        let Some(row) = element.get_untracked() else {
            return;
        };
        let row: web_sys::Element = (*row).clone();
        let on_row = e
            .target()
            .is_some_and(|t| t.unchecked_ref::<web_sys::Element>() == &row);
        if !on_row && e.typed_key() != KeyboardKey::Tab {
            // Keys typed into a child (e.g. a text field) stay there.
            e.stop_propagation();
            return;
        }
        if let Some(active) = row.owner_document().as_ref().and_then(get_active_element)
            && handle_tree_expansion_keys(e, &active)
        {
            return;
        }
        if e.typed_key() == KeyboardKey::Tab
            && let Some(active) = row.owner_document().as_ref().and_then(get_active_element)
            && let Some(mut walker) = get_focusable_tree_walker(
                &row,
                FocusableTreeWalkerOptions {
                    tabbable: true,
                    ..FocusableTreeWalkerOptions::default()
                },
            )
        {
            walker.set_current_node(active.unchecked_ref());
            let next = if e.shift_key() {
                walker.previous_node()
            } else {
                walker.next_node()
            };
            // Tab moves between the row's children before leaving the grid.
            if next.is_some() {
                e.stop_propagation();
            }
        }
    };
    // The row's own key handling (press) only runs if tab navigation didn't take the key.
    let press_keydown = item_props.press.on_keydown;
    item_props.press.on_keydown = EventHandler::new(move |e: KeyboardEvent| {
        tab_navigation(&e);
        if !e.cancel_bubble() {
            press_keydown.call(e);
        }
    });

    let alive = OwnerAlive::new();
    let on_focus = move |e: FocusEvent| {
        key_when_focused.set_value(Some(row_key.get_value()));
        let Some(row) = element.get_untracked() else {
            return;
        };
        let on_row = e
            .target()
            .is_some_and(|t| t.unchecked_ref::<web_sys::Element>() == &*row);
        if !on_row {
            // A child got focus (e.g. by clicking it): the row becomes the focused key.
            if get_modality() == Modality::Pointer {
                selection.set_focused_key(Some(row_key.get_value()), None);
            }
            return;
        }
        if focus_mode == FocusMode::Child {
            let from_child = e
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                .is_some_and(|related| row.contains(Some(&related)));
            if from_child {
                return;
            }
            let alive = alive.clone();
            request_animation_frame(move || {
                // The row may be gone by the next frame (filtering, removal).
                if !alive.get() {
                    return;
                }
                let still_on_row = element.get_untracked().is_some_and(|row| {
                    row.owner_document()
                        .as_ref()
                        .and_then(get_active_element)
                        .is_some_and(|a| a == *row)
                });
                if still_on_row {
                    focus_row();
                }
            });
        }
    };

    let description = use_slot("description");
    let description_id = description.referenced_id;
    let focus_visible = use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible;
    let key = StoredValue::new(key);

    UseGridListItemReturn {
        row_props: PropsWithStyles::new(
            UseGridListItemRowProps {
                role: AriaRole::Row,
                aria_label: label,
                aria_labelledby: Signal::derive(move || {
                    let description = description_id.get()?;
                    has_label.then(|| format!("{row_id} {description}"))
                }),
                aria_selected: Signal::derive(move || {
                    key.with_value(|k| selection.can_select_item(k))
                        .then(|| AriaSelected::from(is_selected.get()))
                }),
                aria_expanded: Signal::derive(move || {
                    has_child_rows.then(|| AriaExpanded::from(is_expanded()))
                }),
                aria_level: Signal::derive(move || position.get().map(|(level, _, _)| level)),
                aria_posinset: Signal::derive(move || position.get().map(|(_, index, _)| index)),
                aria_setsize: Signal::derive(move || position.get().map(|(_, _, size)| size)),
                aria_disabled: Signal::derive(move || {
                    key.with_value(|k| selection.is_disabled(k))
                        .then_some(AriaDisabled::True)
                }),
                item: item_props,
                on_keydown_capture: EventHandler::new(on_keydown_capture),
                on_focus: EventHandler::new(on_focus),
            },
            item_styles,
        ),
        grid_cell_props: UseGridListItemCellProps {
            role: AriaRole::Gridcell,
            aria_colindex: 1,
        },
        description_props: description.props,
        is_selected,
        is_focused,
        is_focus_visible: Signal::derive(move || is_focused.get() && focus_visible.get()),
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    }
}

/// A copy of `e` (same key and modifiers) that bubbles, for re-dispatching it elsewhere.
fn clone_keyboard_event(e: &KeyboardEvent) -> KeyboardEvent {
    let init = web_sys::KeyboardEventInit::new();
    init.set_key(&e.key());
    init.set_code(&e.code());
    init.set_location(e.location());
    init.set_repeat(e.repeat());
    init.set_shift_key(e.shift_key());
    init.set_ctrl_key(e.ctrl_key());
    init.set_alt_key(e.alt_key());
    init.set_meta_key(e.meta_key());
    init.set_bubbles(true);
    init.set_cancelable(true);
    KeyboardEvent::new_with_keyboard_event_init_dict(&e.type_(), &init)
        .expect("KeyboardEvent creation should not fail")
}
