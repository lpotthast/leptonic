// Upstream: react-aria/src/tag/useTagGroup.ts @ 99e6102368
// Upstream: react-aria/test/tag/useTagGroup.test.js @ 99e6102368
// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/tag/TagGroup.test.js @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{
    CapturedElement, IntoAttrs, SlotProps,
    hooks::{
        collections::{
            CollectionOptions, Key, KeyboardDelegate, LinkBehavior, ListLayout, ListState,
            UseListKeyboardDelegateInput, use_list_keyboard_delegate,
        },
        focus::use_focus_within::{
            UseFocusWithinAttrs, UseFocusWithinInput, UseFocusWithinProps, use_focus_within,
        },
        form::{
            use_field::{UseFieldInput, UseFieldReturn, use_field},
            use_label::{LabelElementType, UseLabelProps},
        },
        gridlist::{
            GridListData, KeyboardNavigationBehavior, UseGridListAttrs, UseGridListInput,
            UseGridListProps, UseGridListReturn, use_grid_list,
        },
    },
    utils::{
        aria::{AriaAtomic, AriaLive, AriaRelevant, AriaRole},
        focus::focus_safely,
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Tags get the group's settings through the returned `TagGroupData` (react-aria: a `WeakMap`
//   keyed by the state), which the caller hands to `use_tag`.
// - `has_label` says whether a visible label is rendered.
//
// =============================================================================

/// Input of [`use_tag_group`].
#[derive(Clone, Debug)]
pub struct UseTagGroupInput {
    pub state: ListState,
    /// The group element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    /// Replaces the (horizontal) list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Called with the keys of tags to remove (Delete/Backspace, or a remove button). Without
    /// it, tags can't be removed.
    pub on_remove: Option<Callback<HashSet<Key>>>,
    /// Called with the key of an activated tag.
    pub on_action: Option<Callback<Key>>,
}

/// What tags need to know about their group. Pass it to `use_tag`.
#[derive(Debug, Clone)]
pub struct TagGroupData {
    pub list: GridListData,
    pub on_remove: Option<Callback<HashSet<Key>>>,
}

/// Return value of [`use_tag_group`].
#[derive(Debug)]
pub struct UseTagGroupReturn {
    pub grid_props: UseTagGroupProps,
    pub label_props: UseLabelProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    pub data: TagGroupData,
}

/// Props for the tag group element.
#[derive(Debug)]
pub struct UseTagGroupProps {
    pub grid: UseGridListProps,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_atomic: AriaAtomic,
    pub aria_relevant: AriaRelevant,
    /// Announces added tags while focus is in the group.
    pub aria_live: Signal<AriaLive>,
    pub focus_within: UseFocusWithinProps,
}

pub type UseTagGroupAttrs = (
    UseGridListAttrs,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaAtomic, AriaAtomic>,
    Attr<attr::AriaRelevant, AriaRelevant>,
    Attr<attr::AriaLive, Signal<AriaLive>>,
    UseFocusWithinAttrs,
);

impl IntoAttrs for UseTagGroupProps {
    type Attrs = UseTagGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.grid.into_attrs(),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaAtomic, self.aria_atomic),
            Attr(attr::AriaRelevant, self.aria_relevant),
            Attr(attr::AriaLive, self.aria_live),
            self.focus_within.into_attrs(),
        )
    }
}

/// A tag group: a list of tags (keywords, filters, recipients), navigated with arrow keys and
/// optionally removable with Delete/Backspace. Built on the grid list.
pub fn use_tag_group(input: UseTagGroupInput) -> UseTagGroupReturn {
    let UseTagGroupInput {
        state,
        element,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        keyboard_delegate,
        on_remove,
        on_action,
    } = input;

    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state,
            element,
            orientation: Orientation::Horizontal.into(),
            layout: ListLayout::Stack,
            layout_delegate: None,
        })
    });
    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        aria_describedby,
        ..UseFieldInput::default()
    });

    let UseGridListReturn {
        props: mut grid,
        data,
    } = use_grid_list(UseGridListInput {
        id: Some(field_props.id),
        aria_label: field_props.aria_label,
        aria_labelledby: field_props.aria_labelledby,
        keyboard_delegate: Some(delegate),
        options: CollectionOptions {
            should_focus_wrap: true,
            link_behavior: Signal::stored(LinkBehavior::Override),
            ..CollectionOptions::default()
        },
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Tab,
        on_action,
        state,
        element,
        layout: ListLayout::Stack,
        orientation: Orientation::Horizontal.into(),
        should_select_on_press_up: false,
        tree: None,
    });
    let is_empty = Signal::derive(move || state.collection.with(|c| c.is_empty()));
    grid.role = Signal::derive(move || {
        if is_empty.get() {
            AriaRole::Group
        } else {
            AriaRole::Grid
        }
    });

    let focus_within = use_focus_within(UseFocusWithinInput::default());
    let is_focus_within = focus_within.is_focus_within;

    // Removing the last tag keeps focus in the (now empty) group.
    let previous_count = StoredValue::new(untrack(|| state.collection.with(|c| c.size())));
    Effect::new(move |_| {
        let count = state.collection.with(|c| c.size());
        if previous_count.get_value() > 0
            && count == 0
            && untrack(|| is_focus_within.get())
            && let Some(el) = element.get_untracked()
        {
            focus_safely(&el);
        }
        previous_count.set_value(count);
    });

    UseTagGroupReturn {
        grid_props: UseTagGroupProps {
            grid,
            aria_describedby: field_props.aria_describedby,
            aria_atomic: AriaAtomic::False,
            aria_relevant: AriaRelevant::Additions,
            aria_live: Signal::derive(move || {
                if is_focus_within.get() {
                    AriaLive::Polite
                } else {
                    AriaLive::Off
                }
            }),
            focus_within: focus_within.props,
        },
        label_props,
        description_props,
        error_message_props,
        data: TagGroupData {
            list: data,
            on_remove,
        },
    }
}
