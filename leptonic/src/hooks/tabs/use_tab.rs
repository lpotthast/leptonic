// Upstream: react-aria/src/tabs/useTab.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TabListItemData;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            Key, LinkBehavior, SelectOnPressUp, UseSelectableItemAttrs, UseSelectableItemInput,
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
// - Link attributes (`href`, ...) on tabs that are links: they navigate on selection
//   (`LinkBehavior::Selection`), but don't render as links.
//
// =============================================================================

/// Input of [`use_tab`].
#[derive(Debug, Clone)]
pub struct UseTabInput {
    /// The tab list (from `use_tab_list`).
    pub list: TabListItemData,
    /// The tab's key in the tab list's collection.
    pub key: Key,
    pub is_disabled: Signal<bool>,
    /// Select when the press ends instead of when it starts. Defaults to `true` for links.
    pub should_select_on_press_up: SelectOnPressUp,
}

/// Return value of [`use_tab`].
#[derive(Debug)]
pub struct UseTabReturn {
    /// For the tab element. Call `.into_parts()` for spreading and styles.
    pub props: PropsWithStyles<UseTabProps>,
    pub is_selected: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_focused: Signal<bool>,
}

/// Props for the tab element.
#[derive(Debug)]
pub struct UseTabProps {
    pub role: AriaRole,
    pub aria_selected: Signal<AriaSelected>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    /// The selected tab controls its panel.
    pub aria_controls: Signal<Option<String>>,
    pub item: UseSelectableItemProps,
}

pub type UseTabAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaSelected, Signal<AriaSelected>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
    UseSelectableItemAttrs,
);

impl IntoAttrs for UseTabProps {
    type Attrs = UseTabAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaControls, self.aria_controls),
            self.item.into_attrs(),
        )
    }
}

/// A tab of a tab list: selected by press (and, with automatic activation, by focus).
pub fn use_tab(input: UseTabInput) -> UseTabReturn {
    let UseTabInput {
        list,
        key,
        is_disabled,
        should_select_on_press_up,
    } = input;
    let state = list.tabs.state;
    let selection = state.list.list.selection;
    let collection = state.list.list.collection;
    let tab_key = StoredValue::new(key.clone());
    let is_disabled = Signal::derive(move || {
        is_disabled.get()
            || state.is_disabled.get()
            || tab_key.with_value(|k| selection.is_disabled(k))
    });
    let is_link = untrack(|| collection.with(|c| c.get(&key).is_some_and(|n| n.link.is_some())));

    let UseSelectableItemReturn {
        props,
        is_pressed,
        is_selected,
        is_focused,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.list.list.item_elements,
        key: key.clone(),
        element: CapturedElement::new(),
        id: Some(list.tabs.tab_id(&key)),
        collection_id: list.collection_id,
        is_disabled,
        should_select_on_press_up: should_select_on_press_up.resolve(|| is_link),
        allows_different_press_origin: false,
        on_action: Signal::stored(None),
        link_behavior: LinkBehavior::Selection,
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (mut item, styles) = props.into_inner();
    let item_tabindex = item.tabindex;
    item.tabindex = Signal::derive(move || {
        if is_disabled.get() {
            None
        } else {
            item_tabindex.get()
        }
    });
    let panel_id = list.tabs.tab_panel_id(&key);

    UseTabReturn {
        props: PropsWithStyles::new(
            UseTabProps {
                role: AriaRole::Tab,
                aria_selected: Signal::derive(move || AriaSelected::from(is_selected.get())),
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
                aria_controls: Signal::derive(move || is_selected.get().then(|| panel_id.clone())),
                item,
            },
            styles,
        ),
        is_selected,
        is_disabled,
        is_pressed,
        is_focused,
    }
}
