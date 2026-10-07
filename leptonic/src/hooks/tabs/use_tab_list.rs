// Upstream: react-aria/src/tabs/useTabList.ts @ 99e6102368
// Upstream: react-aria/src/tabs/utils.ts @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{TabListState, TabsKeyboardDelegate};
use crate::{
    hooks::{
        IntoAttrs, Orientation,
        collections::{
            CollectionOptions, Key, KeyboardDelegate, LinkBehavior, UseSelectableCollectionAttrs,
            UseSelectableCollectionInput, UseSelectableCollectionProps, use_selectable_collection,
        },
    },
    utils::{
        CapturedElement,
        aria::{AriaOrientation, AriaRole},
        i18n::use_direction,
        id::use_id,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Tabs and panels get the tab list through the returned `TabListData` (react-aria: a
//   `WeakMap` keyed by the state). Create it up front with `TabListData::new` when a panel is
//   rendered before the tab list.
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

/// What tabs and tab panels need to know about their tab list.
#[derive(Debug, Clone)]
pub struct TabListData {
    pub state: TabListState,
    /// The base of the tab and panel ids.
    pub id: String,
}

impl TabListData {
    /// Tab list data with a generated id.
    pub fn new(state: TabListState) -> Self {
        Self {
            state,
            id: use_id("tabs"),
        }
    }

    /// The id of the tab `key`.
    pub fn tab_id(&self, key: &Key) -> String {
        format!("{}-tab-{}", self.id, normalize_key(key))
    }

    /// The id of the panel of the tab `key`.
    pub fn tab_panel_id(&self, key: &Key) -> String {
        format!("{}-tabpanel-{}", self.id, normalize_key(key))
    }
}

/// A key as an id fragment: without whitespace.
fn normalize_key(key: &Key) -> String {
    key.to_string().split_whitespace().collect()
}

/// Input of [`use_tab_list`].
#[derive(Debug, Clone)]
pub struct UseTabListInput {
    /// The tab list (see [`TabListData::new`]).
    pub tabs: TabListData,
    /// The tab list element; the hook's props capture it.
    pub element: CapturedElement,
    pub orientation: Orientation,
    pub keyboard_activation: KeyboardActivation,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
}

/// Return value of [`use_tab_list`].
#[derive(Debug)]
pub struct UseTabListReturn {
    pub props: UseTabListProps,
    /// Pass to `use_tab` (and `use_tab_panel`).
    pub data: TabListItemData,
}

/// What tabs need to know about their tab list (see [`use_tab_list`]).
#[derive(Debug, Clone)]
pub struct TabListItemData {
    pub tabs: TabListData,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: String,
}

/// Props for the tab list element.
#[derive(Debug)]
pub struct UseTabListProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_orientation: AriaOrientation,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// Keyboard navigation and focus handling (`use_selectable_collection`).
    pub collection: UseSelectableCollectionProps,
}

pub type UseTabListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
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
        tabs,
        element,
        orientation,
        keyboard_activation,
        aria_label,
        aria_labelledby,
    } = input;
    let list = tabs.state.list.list;
    let direction = use_direction();
    let delegate = Signal::derive(move || {
        Arc::new(TabsKeyboardDelegate::new(
            list.collection,
            list.selection,
            direction.get(),
            orientation,
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
            link_behavior: LinkBehavior::Selection,
            ..CollectionOptions::default()
        },
    })
    .props;
    // The tabs are the tab stops, not the list.
    collection.tabindex = Signal::stored(None);

    UseTabListReturn {
        data: TabListItemData {
            tabs: tabs.clone(),
            collection_id: collection.collection_id.clone(),
        },
        props: UseTabListProps {
            id: tabs.id,
            role: AriaRole::Tablist,
            aria_orientation: match orientation {
                Orientation::Horizontal => AriaOrientation::Horizontal,
                Orientation::Vertical => AriaOrientation::Vertical,
            },
            aria_label,
            aria_labelledby,
            collection,
        },
    }
}
