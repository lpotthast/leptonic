// Upstream: react-aria/src/listbox/useOption.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::ListBoxData;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            ItemLink, Key, SelectionMode, UseSelectableItemAttrs, UseSelectableItemInput,
            UseSelectableItemProps, UseSelectableItemReturn, use_node_aria_label, use_selectable_item,
        },
        focus::use_focus_visible::{
            Modality, UseFocusVisibleInput, get_modality, use_focus_visible,
        },
        interactions::use_hover::{UseHoverAttrs, UseHoverInput, UseHoverProps, use_hover},
    },
    utils::{
        CapturedElement, SlotProps,
        aria::{AriaDisabled, AriaRole, AriaSelected},
        use_slot,
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
    pub on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
}

/// Return value of [`use_option`].
pub struct UseOptionReturn {
    pub props: PropsWithStyles<UseOptionProps>,
    /// For the element holding the option's main text.
    pub label_props: SlotProps,
    /// For the element holding secondary text.
    pub description_props: SlotProps,
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
    /// The option's link, if the collection item has one.
    pub link: Option<ItemLink>,
}

/// Props for the option element.
#[derive(Debug)]
pub struct UseOptionProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    /// In a virtualized listbox: the option's position (from 1) ...
    pub aria_posinset: Signal<Option<usize>>,
    /// ... and the number of options.
    pub aria_setsize: Signal<Option<usize>>,
    pub item: UseSelectableItemProps,
    pub hover: UseHoverProps,
}

pub type UseOptionAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaPosinset, Signal<Option<usize>>>,
    Attr<attr::AriaSetsize, Signal<Option<usize>>>,
    UseSelectableItemAttrs,
    UseHoverAttrs,
);

impl IntoAttrs for UseOptionProps {
    type Attrs = UseOptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaPosinset, self.aria_posinset),
            Attr(attr::AriaSetsize, self.aria_setsize),
            self.item.into_attrs(),
            self.hover.into_attrs(),
        )
    }
}

/// The element id of the option `key` in the listbox `list_id` (whitespace removed from the
/// key).
pub fn option_id(list_id: &str, key: &Key) -> String {
    let key: String = key
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    format!("{list_id}-option-{key}")
}

/// An option of a listbox: selection on press (as configured by the listbox), focus handling,
/// and `role="option"` with its ARIA state.
pub fn use_option(input: UseOptionInput) -> UseOptionReturn {
    crate::hooks::track_interaction_modality();
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

    let node = untrack(|| state.collection.with(|c| c.get(&key).cloned()));
    if node.is_none() {
        crate::utils::dev_warn!("use_option: the key {key:?} is not in the listbox's collection");
    }
    let aria_label = use_node_aria_label(state.collection, key.clone());
    let link = node.and_then(|n| n.link);

    let position_key = StoredValue::new(key.clone());
    let label = use_slot("label");
    let description = use_slot("description");

    let element = CapturedElement::new();
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
        item_elements: state.item_elements,
        key: key.clone(),
        element,
        id: Some(option_id(&list_id, &key)),
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up,
        allows_different_press_origin: should_select_on_press_up && should_focus_on_hover,
        on_action: Signal::stored(on_action.map(|on_action| {
            let key = key.clone();
            Callback::new(move |()| on_action.run(key.clone()))
        })),
        link_behavior,
        focus: None,
        should_use_virtual_focus,
        on_context_menu,
    });

    let hover_key = key.clone();
    // Hovering is tracked for interactive options (react-aria-components' `ListBoxItem`
    // `isHovered`), and moves focus with `should_focus_on_hover` (react-aria's `useOption`).
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || {
            is_disabled.get()
                || !(should_focus_on_hover || allows_selection.get() || has_action.get())
        }),
        on_hover_start: Some(Callback::new(move |_| {
            // Unless the keyboard is in use: hovering moves focus.
            if should_focus_on_hover && get_modality() == Modality::Pointer {
                selection.set_focused(true);
                selection.set_focused_key(Some(hover_key.clone()), None);
            }
        })),
        ..UseHoverInput::default()
    });
    let is_hovered = hover.is_hovered;
    let hover = hover.props;

    let focus_visible = use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible;
    let (item_props, styles) = props.into_inner();

    UseOptionReturn {
        props: PropsWithStyles::new(
            UseOptionProps {
                role: AriaRole::Option,
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
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
                            position_key.with_value(|key| {
                                state
                                    .collection
                                    .with(|c| c.get(key).map(|node| node.index + 1))
                            })
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
        ),
        label_props: label.props,
        description_props: description.props,
        is_selected,
        is_focused,
        is_focus_visible: Signal::derive(move || is_focused.get() && focus_visible.get()),
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
    fn option_ids_drop_whitespace() {
        assert_that!(option_id("lb", &Key::from("Ice cream")))
            .is_equal_to("lb-option-Icecream".to_owned());
        assert_that!(option_id("lb", &Key::from(7))).is_equal_to("lb-option-7".to_owned());
    }
}
