// Upstream: react-aria/src/listbox/useListBox.ts @ 99e6102368
use std::sync::Arc;

use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    hooks::{
        IntoAttrs, Orientation,
        collections::{
            CollectionOptions, Key, KeyboardDelegate, LinkBehavior, ListLayout, ListState,
            SelectionBehavior, SelectionMode, UseSelectableCollectionAttrs,
            UseSelectableCollectionProps, UseSelectableListInput, use_selectable_list,
        },
        focus::use_focus_within::{FocusWithinEvent, UseFocusWithinInput, use_focus_within},
    },
    utils::{
        CapturedElement,
        aria::{AriaMultiselectable, AriaOrientation, AriaRole},
        id::use_id,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Options get the listbox's settings through the returned `ListBoxData` (react-aria: a
//   `WeakMap` keyed by the state), which the caller hands to `use_option`.
// - No built-in visible label (`label` prop): render the label yourself and pass its id as
//   `aria_labelledby`.
//
// ## OMITTED FEATURES
// - Virtualization.
//
// =============================================================================

/// Input of [`use_listbox`].
#[derive(Clone)]
pub struct UseListBoxInput {
    pub state: ListState,
    /// The listbox element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`. Set this when another element references the
    /// listbox, e.g. a select trigger's `aria-controls`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub orientation: Orientation,
    /// Items stacked (one per row/column) or wrapping in a grid.
    pub layout: ListLayout,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Keyboard and focus behavior. `link_behavior` defaults to `Override` in `Toggle` selection
    /// behavior (pressing a link item opens it, selection needs a checkbox).
    pub options: CollectionOptions,
    /// Select when the press ends instead of when it starts (select popovers).
    pub should_select_on_press_up: bool,
    /// Focus options when the pointer moves over them (select popovers).
    pub should_focus_on_hover: bool,
    /// Called with the key of an option that is activated (pressed without a selection mode,
    /// double-clicked or Enter in `Replace` selection behavior).
    pub on_action: Option<Callback<Key>>,
    /// Called when focus moves into the listbox from outside.
    pub on_focus: Option<Callback<FocusWithinEvent>>,
    /// Called when focus leaves the listbox.
    pub on_blur: Option<Callback<FocusWithinEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

impl UseListBoxInput {
    /// A vertical listbox for `state`, with all other settings at their defaults.
    pub fn new(state: ListState, element: CapturedElement) -> Self {
        Self {
            state,
            element,
            id: None,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            orientation: Orientation::Vertical,
            layout: ListLayout::Stack,
            keyboard_delegate: None,
            options: CollectionOptions::default(),
            should_select_on_press_up: false,
            should_focus_on_hover: false,
            on_action: None,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        }
    }
}

/// What options need to know about their listbox. Pass it to `use_option` (atoms provide it as
/// context).
#[derive(Debug, Clone)]
pub struct ListBoxData {
    pub state: ListState,
    /// The listbox element id; option ids derive from it.
    pub id: String,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: String,
    pub should_select_on_press_up: bool,
    pub should_focus_on_hover: bool,
    pub link_behavior: LinkBehavior,
    pub on_action: Option<Callback<Key>>,
    /// Options are focused virtually (`CollectionOptions::should_use_virtual_focus`).
    pub should_use_virtual_focus: bool,
}

/// Return value of [`use_listbox`].
#[derive(Debug)]
pub struct UseListBoxReturn {
    pub props: UseListBoxProps,
    pub data: ListBoxData,
}

/// Props for the listbox element.
#[derive(Debug)]
pub struct UseListBoxProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_multiselectable: Signal<Option<AriaMultiselectable>>,
    pub aria_orientation: AriaOrientation,
    /// Focus, keyboard and tab-index handling (`use_selectable_list`), including focus-within
    /// tracking.
    pub collection: UseSelectableCollectionProps,
}

pub type UseListBoxAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaMultiselectable, Signal<Option<AriaMultiselectable>>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    UseSelectableCollectionAttrs,
);

impl IntoAttrs for UseListBoxProps {
    type Attrs = UseListBoxAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaOrientation, self.aria_orientation),
            self.collection.into_attrs(),
        )
    }
}

/// A listbox: a list of options to select one or more from, navigated with arrow keys and
/// type-ahead.
///
/// The options come from the state's collection; render one `use_option` per item, in
/// collection order.
///
/// ```ignore
/// let collection = use_list_collection(fruits, |f| Key::from(f.id), |f| f.name.clone());
/// let state = use_list_state(UseListStateInput { collection, selection: SelectionOptions {
///     selection_mode: Signal::stored(SelectionMode::Multiple), ..SelectionOptions::default() } });
/// let element = CapturedElement::new();
/// let listbox = use_listbox(UseListBoxInput {
///     aria_label: Some("Fruits".to_owned()),
///     ..UseListBoxInput::new(state, element)
/// });
/// let data = listbox.data.clone();
/// view! {
///     <ul {..listbox.props.into_attrs()}>
///         <For each=move || fruits.get() key=|f| f.id let:fruit>
///             {
///                 let option = use_option(UseOptionInput { list: data.clone(), key: fruit.id.into() });
///                 view! { <li {..option.props.into_attrs()}>{fruit.name}</li> }
///             }
///         </For>
///     </ul>
/// }
/// ```
pub fn use_listbox(input: UseListBoxInput) -> UseListBoxReturn {
    let UseListBoxInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        orientation,
        layout,
        keyboard_delegate,
        mut options,
        should_select_on_press_up,
        should_focus_on_hover,
        on_action,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;

    let id = id.unwrap_or_else(|| use_id("listbox"));

    // Pressing a link in a toggle-selection list opens it; selecting it needs a checkbox.
    if options.link_behavior == LinkBehavior::Action
        && untrack(|| state.selection.selection_behavior()) == SelectionBehavior::Toggle
    {
        options.link_behavior = LinkBehavior::Override;
    }

    let mut collection = use_selectable_list(UseSelectableListInput {
        state,
        element,
        orientation,
        layout,
        keyboard_delegate,
        options,
    })
    .props;

    let focus_within = use_focus_within(UseFocusWithinInput {
        on_focus_within: on_focus,
        on_blur_within: on_blur,
        on_focus_within_change: on_focus_change,
        ..UseFocusWithinInput::default()
    })
    .props;
    collection.on_focusin = collection.on_focusin.chain(focus_within.on_focusin);
    collection.on_focusout = collection.on_focusout.chain(focus_within.on_focusout);

    let data = ListBoxData {
        state,
        id: id.clone(),
        collection_id: collection.collection_id.clone(),
        should_select_on_press_up,
        should_focus_on_hover,
        link_behavior: options.link_behavior,
        on_action,
        should_use_virtual_focus: options.should_use_virtual_focus,
    };

    UseListBoxReturn {
        props: UseListBoxProps {
            id,
            role: AriaRole::Listbox,
            aria_label,
            aria_labelledby,
            aria_multiselectable: Signal::derive(move || {
                (state.selection.selection_mode() == SelectionMode::Multiple)
                    .then_some(AriaMultiselectable::True)
            }),
            aria_orientation: match orientation {
                Orientation::Horizontal => AriaOrientation::Horizontal,
                Orientation::Vertical => AriaOrientation::Vertical,
            },
            collection,
        },
        data,
    }
}
