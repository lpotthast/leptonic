// Upstream: react-aria/src/listbox/useOption.ts @ 99e6102368
// Upstream: react-aria/src/listbox/utils.ts @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/listbox/ListBox.test.js @ 99e6102368
use leptos::prelude::*;

use super::ListBoxData;
use crate::{
    CapturedElement, Id, IntoAttrs, Part, PropsWithStyles, SlotProps,
    hooks::{
        collections::{
            ItemLink, Key, SelectionMode, UseNodeAriaLabelInput, UseSelectableItemAttrs,
            UseSelectableItemInput, UseSelectableItemProps, UseSelectableItemReturn,
            use_node_aria_label, use_selectable_item,
        },
        focus::use_focus_visible::{Modality, get_modality, is_focus_visible},
        interactions::use_hover::{UseHoverAttrs, UseHoverInput, UseHoverProps, use_hover},
    },
    sharing_one_effect, use_slot,
    utils::{
        aria::{AriaDisabled, AriaRole, AriaSelected, aria_true},
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Options read their settings from the listbox (`ListBoxData`) and their text, label,
//   disabled state and link from the collection node. react-aria's per-option overrides
//   (`isDisabled`, `shouldSelectOnPressUp`, ...) are deprecated upstream and not offered.
// - The label and description ids are only referenced while those elements are rendered
//   (react-aria: `useSlotId`), detected through element capture.
// - Link attributes are returned as `link` instead of being merged into the props: render the
//   option as `<a>` with them, or keep another element (links then open through a temporary
//   `<a>`).
//
// ## DIFFERENT BEHAVIOR
// - Option ids escape the key's whitespace (`Key::id_fragment`), so different keys never share
//   an id. react-aria removes the whitespace and prints integer and string keys alike (`"Ice
//   cream"` and `"Icecream"`, `1` and `"1"` get the same id).
//
// =============================================================================

/// Input of [`use_option`].
#[derive(Debug, Clone)]
pub struct UseOptionInput {
    /// The listbox (from `use_listbox`).
    pub list: ListBoxData,
    /// The option's key in the listbox's collection.
    pub key: Key,
    /// Called when a context menu is requested on the option (right click, Shift+F10, the context
    /// menu key; a long press on iOS unless it selects).
    pub on_context_menu: Option<Callback<crate::hooks::interactions::ContextMenuEvent>>,
}

/// The return value of [`use_option`].
pub struct UseOptionReturn {
    /// The option element: `let (attrs, styles) = option.attrs();`.
    pub option: Part<PropsWithStyles<UseOptionProps>>,
    /// For the element holding the option's main text.
    pub label: Part<SlotProps>,
    /// For the element holding secondary text.
    pub description: Part<SlotProps>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    /// Focused, and focus should be shown (keyboard navigation).
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    /// Whether the pointer is over the option (options that can be selected or have an action,
    /// or that take focus on hover).
    pub is_hovered: Signal<bool>,
    /// Whether pressing the option can select it.
    pub allows_selection: Signal<bool>,
    /// Whether the option has an action (or link) to perform.
    pub has_action: Signal<bool>,
    /// The option's link, if the collection item has one (follows the collection).
    pub link: Signal<Option<ItemLink>>,
}

/// Props for the option element.
#[derive(Debug)]
pub struct UseOptionProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    /// In a virtualized listbox: the option's position (from 1) ...
    pub aria_posinset: Signal<Option<usize>>,
    /// ... and the number of options.
    pub aria_setsize: Signal<Option<usize>>,
    pub item: UseSelectableItemProps,
    pub hover: UseHoverProps,
}

impl Remount for UseOptionProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_disabled: self.aria_disabled,
            aria_selected: self.aria_selected,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            aria_posinset: self.aria_posinset,
            aria_setsize: self.aria_setsize,
            item: self.item.remount(),
            hover: self.hover.remount(),
        }
    }
}

pub type UseOptionAttrs = (
    FixedAttr,
    SignalAttr<Option<AriaDisabled>>,
    SignalAttr<Option<AriaSelected>>,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<usize>>,
    SignalAttr<Option<usize>>,
    UseSelectableItemAttrs,
    UseHoverAttrs,
);

impl IntoAttrs for UseOptionProps {
    type Attrs = UseOptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-disabled", self.aria_disabled),
            signal_attr("aria-selected", self.aria_selected),
            signal_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-describedby", self.aria_describedby),
            signal_attr("aria-posinset", self.aria_posinset),
            signal_attr("aria-setsize", self.aria_setsize),
            self.item.into_attrs(),
            self.hover.into_attrs(),
        )
    }
}

/// The element id of the option `key` in the listbox `list_id` (see [`Key::id_fragment`]).
pub fn option_id(list_id: &str, key: &Key) -> Id {
    Id::derived(format_args!("{list_id}-option-{}", key.id_fragment()))
}

/// An option of a listbox: selection on press (as configured by the listbox), focus handling,
/// and `role="option"` with its ARIA state.
pub fn use_option(input: UseOptionInput) -> UseOptionReturn {
    crate::metrics::subject!();
    // `use_selectable_item`'s resets and `use_hover`'s react to the disabled state in one effect.
    sharing_one_effect(|| option(input))
}

/// The body of [`use_option`].
fn option(input: UseOptionInput) -> UseOptionReturn {
    crate::hooks::focus::use_focus_visible::track_interaction_modality();
    let UseOptionInput {
        list,
        key,
        on_context_menu,
    } = input;
    let ListBoxData {
        state,
        id: list_id,
        collection_id,
        should_select_on_press_up,
        should_focus_on_hover,
        link_behavior,
        on_action,
        should_use_virtual_focus,
        is_virtualized,
    } = list;
    let selection = state.selection;

    if !untrack(|| state.collection.with(|c| c.contains_key(&key))) {
        crate::utils::dev_warn!("use_option: the key {key:?} is not in the listbox's collection");
    }
    let aria_label = use_node_aria_label(UseNodeAriaLabelInput {
        selection: state.selection,
        key: key.clone(),
    });
    let link = {
        let key = key.clone();
        let collection = state.collection;
        // Only this option's changes reach it, not every change of the collection. A derive: read
        // once by the element rendering the option (`performance.md` R5).
        Signal::derive(move || {
            selection.track_node(&key);
            collection.with_untracked(|c| c.get(&key).and_then(|n| n.link.clone()))
        })
    };

    let position_key = key.clone();
    let label = use_slot("label");
    let description = use_slot("description");

    let element = CapturedElement::new();
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
        item_elements: state.item_elements,
        key: key.clone(),
        element,
        id: Some(option_id(&list_id, &key)),
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up,
        allows_different_press_origin: should_select_on_press_up && should_focus_on_hover,
        on_action: Signal::stored(on_action),
        link_behavior,
        focus: None,
        should_use_virtual_focus,
        on_context_menu,
    });
    let props = props.into_props();

    let hover_key = key.clone();
    // Hovering is tracked for interactive options (react-aria-components' `ListBoxItem`
    // `isHovered`), and moves focus with `should_focus_on_hover` (react-aria's `useOption`).
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || {
            is_disabled.get()
                || !(should_focus_on_hover || allows_selection.get() || has_action.get())
        }),
        on_hover_start: should_focus_on_hover.then(|| {
            Callback::new(move |_| {
                // Unless the keyboard is in use: hovering moves focus.
                if get_modality() == Some(Modality::Pointer) {
                    selection.set_focused(true);
                    selection.set_focused_key(Some(hover_key.clone()), None);
                }
            })
        }),
        on_hover_end: None,
        on_hover_change: None,
    });
    let hover_props = hover.target.into_props();
    let is_hovered = hover.is_hovered;
    let hover = hover_props;

    let (item_props, styles) = props.into_inner();

    UseOptionReturn {
        option: Part::stored(PropsWithStyles::new(
            UseOptionProps {
                role: AriaRole::Option,
                aria_disabled: aria_true(is_disabled),
                aria_selected: Signal::derive(move || {
                    (selection.selection_mode() != SelectionMode::None)
                        .then(|| AriaSelected::from(is_selected.get()))
                }),
                aria_label: aria_label.into(),
                aria_labelledby: label.referenced_id,
                aria_describedby: description.referenced_id,
                aria_posinset: Signal::derive(move || {
                    is_virtualized
                        .then(|| {
                            state
                                .collection
                                .with(|c| c.get(&position_key).map(|node| node.index + 1))
                        })
                        .flatten()
                }),
                aria_setsize: Signal::derive(move || {
                    is_virtualized.then(|| state.collection.with(|c| c.size()))
                }),
                item: item_props,
                hover,
            },
            styles,
        )),
        label: label.part,
        description: description.part,
        is_selected,
        is_focused,
        // Only the focused option tracks the modality.
        is_focus_visible: Signal::derive(move || is_focused.get() && is_focus_visible()),
        is_disabled,
        is_pressed,
        is_hovered,
        allows_selection,
        has_action,
        link,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn option_ids_tell_keys_apart() {
        assert_that!(option_id("lb", &Key::from("Ice cream")))
            .is_equal_to(Id::from("lb-option-Ice%20;cream"));
        assert_that!(option_id("lb", &Key::from(7)))
            .is_not_equal_to(option_id("lb", &Key::from("7")));
    }
}
