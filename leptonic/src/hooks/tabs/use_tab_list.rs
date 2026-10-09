// Upstream: react-aria/src/tabs/useTabList.ts @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{TabListState, TabsKeyboardDelegate};
use crate::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::collections::{
        CollectionOptions, KeyboardDelegate, LinkBehavior, UseSelectableCollectionAttrs,
        UseSelectableCollectionInput, UseSelectableCollectionProps, keyboard_delegate_memo,
        use_selectable_collection,
    },
    labels,
    utils::{
        aria::{AriaOrientation, AriaRole},
        i18n::use_direction,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `keyboard_activation` is not a signal yet: the collection's `select_on_focus` it configures
//   is fixed when the collection is created.
//
// =============================================================================

/// When arrow keys select tabs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum KeyboardActivation {
    /// Moving focus to a tab selects it.
    #[default]
    Automatic,
    /// Enter or Space selects the focused tab.
    Manual,
}

/// Input of [`use_tab_list`].
#[derive(Debug, Clone)]
pub struct UseTabListInput {
    /// The tab list's state.
    pub state: TabListState,
    /// The tab list element; the hook's props capture it.
    pub element: CapturedElement,
    /// The axis of the arrow keys (react-aria's default: horizontal).
    pub orientation: Signal<Orientation>,
    /// Whether focusing a tab with the arrow keys selects it.
    pub keyboard_activation: KeyboardActivation,
    /// Names the tab list. Next to `aria_labelledby`, the list labels itself too.
    pub aria_label: MaybeProp<String>,
    /// The ids of the elements naming the tab list.
    pub aria_labelledby: Option<String>,
}

/// Return value of [`use_tab_list`].
#[derive(Debug)]
pub struct UseTabListReturn {
    pub props: UseTabListProps,
}

/// Props for the tab list element.
#[derive(Debug)]
pub struct UseTabListProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_orientation: Signal<AriaOrientation>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    /// Keyboard navigation and focus handling (`use_selectable_collection`).
    pub collection: UseSelectableCollectionProps,
}

pub type UseTabListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaOrientation, Signal<AriaOrientation>>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    UseSelectableCollectionAttrs,
);

impl IntoAttrs for UseTabListProps {
    type Attrs = UseTabListAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            self.collection.into_attrs(),
        )
    }
}

/// A tab list: arrow keys move between the tabs (selecting them with automatic activation),
/// one of them is always selected. Render the tabs with `use_tab` and the selected tab's
/// content with `use_tab_panel`.
pub fn use_tab_list(input: UseTabListInput) -> UseTabListReturn {
    let UseTabListInput {
        state,
        element,
        orientation,
        keyboard_activation,
        aria_label,
        aria_labelledby,
    } = input;
    let list = state.list.list;
    let direction = use_direction();
    let delegate = keyboard_delegate_memo(move || {
        Arc::new(TabsKeyboardDelegate::new(
            list.collection,
            list.selection,
            direction.get(),
            orientation.get(),
        )) as Arc<dyn KeyboardDelegate>
    });
    let mut collection = use_selectable_collection(UseSelectableCollectionInput {
        selection: list.selection,
        item_elements: list.item_elements,
        delegate,
        element,
        options: CollectionOptions {
            select_on_focus: (keyboard_activation == KeyboardActivation::Automatic).into(),
            disallow_empty_selection: true,
            link_behavior: Signal::stored(LinkBehavior::Selection),
            ..CollectionOptions::default()
        },
    })
    .props;
    // The tabs are the tab stops, not the list.
    collection.tabindex = Signal::stored(None);
    state.set_collection_id(collection.collection_id.clone());
    let id = state.id();
    let labelling = {
        let id = id.clone();
        Signal::derive(move || labels(&id, aria_label.get(), aria_labelledby.as_deref()))
    };

    UseTabListReturn {
        props: UseTabListProps {
            id,
            role: AriaRole::Tablist,
            aria_orientation: Signal::derive(move || match orientation.get() {
                Orientation::Horizontal => AriaOrientation::Horizontal,
                Orientation::Vertical => AriaOrientation::Vertical,
            }),
            aria_label: Signal::derive(move || labelling.with(|l| l.aria_label.clone())),
            aria_labelledby: Signal::derive(move || labelling.with(|l| l.aria_labelledby.clone())),
            collection,
        },
    }
}
