// Upstream: react-aria/src/tabs/useTab.ts @ 99e6102368
// Upstream: react-aria-components/test/Tabs.test.js @ 99e6102368
use leptos::prelude::*;
use web_sys::FocusEvent;

use super::TabListState;
use crate::{
    CapturedElement, Id, IntoAttrs, PropsWithStyles,
    hooks::{
        collections::{
            Key, LinkBehavior, SelectOnPressUp, UseSelectableItemAttrs, UseSelectableItemInput,
            UseSelectableItemProps, UseSelectableItemReturn, use_selectable_item,
        },
        focus::{UseFocusableInput, UseFocusableItemAttrs, UseFocusableItemProps, use_focusable},
    },
    utils::{
        aria::{AriaDisabled, AriaRole, AriaSelected, aria_true},
        dev_warn,
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Of `useFocusable`, the tab takes the focus handlers, element capture and a
//   `FocusableContext`'s description and attributes (e.g. a `TooltipTrigger`'s); its tab stop
//   is the collection's (react-aria merges the selectable item's `tabIndex` last too).
//
// ## OMITTED FEATURES
// - Link attributes (`href`, ...) on tabs that are links: they navigate on selection
//   (`LinkBehavior::Selection`), but don't render as links.
//
// =============================================================================

/// Input of [`use_tab`].
#[derive(Debug, Clone)]
pub struct UseTabInput {
    /// The tab list's state.
    pub state: TabListState,
    /// The tab's key in the tab list's collection.
    pub key: Key,
    pub is_disabled: Signal<bool>,
    /// Select when the press ends instead of when it starts. Defaults to `true` for links.
    pub should_select_on_press_up: SelectOnPressUp,
    /// Called when the tab receives focus.
    pub on_focus: Option<Callback<FocusEvent>>,
    /// Called when the tab loses focus.
    pub on_blur: Option<Callback<FocusEvent>>,
    /// Called when the tab's focus changes.
    pub on_focus_change: Option<Callback<bool>>,
}

/// The return value of [`use_tab`].
#[derive(Debug)]
pub struct UseTabReturn {
    /// The tab element: `let (attrs, styles) = tab.attrs();`.
    pub tab: Part<PropsWithStyles<UseTabProps>>,
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
    pub aria_controls: Signal<Option<Id>>,
    pub item: UseSelectableItemProps,
    /// A `FocusableContext`'s handlers, description and attributes (e.g. a `TooltipTrigger`'s).
    pub focusable: UseFocusableItemProps,
}

impl Remount for UseTabProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_selected: self.aria_selected,
            aria_disabled: self.aria_disabled,
            aria_controls: self.aria_controls,
            item: self.item.remount(),
            focusable: self.focusable.remount(),
        }
    }
}

pub type UseTabAttrs = (
    FixedAttr,
    SignalAttr<AriaSelected>,
    SignalAttr<Option<AriaDisabled>>,
    SignalAttr<Option<Id>>,
    UseSelectableItemAttrs,
    UseFocusableItemAttrs,
);

impl IntoAttrs for UseTabProps {
    type Attrs = UseTabAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-selected", self.aria_selected),
            signal_attr("aria-disabled", self.aria_disabled),
            signal_attr("aria-controls", self.aria_controls),
            self.item.into_attrs(),
            self.focusable.into_attrs(),
        )
    }
}

/// A tab of a tab list: selected by press (and, with automatic activation, by focus).
pub fn use_tab(input: UseTabInput) -> UseTabReturn {
    crate::metrics::subject!();
    let UseTabInput {
        state,
        key,
        is_disabled,
        should_select_on_press_up,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;
    let selection = state.list.list.selection;
    let collection = state.list.list.collection;
    let tab_key = StoredValue::new(key.clone());
    let is_disabled: Signal<bool> = Memo::new(move |_| {
        is_disabled.get()
            || state.is_disabled.get()
            || tab_key.with_value(|k| selection.is_disabled(k))
    })
    .into();
    let is_link = untrack(|| collection.with(|c| c.get(&key).is_some_and(|n| n.link.is_some())));

    let UseSelectableItemReturn {
        item: props,
        is_pressed,
        is_selected,
        is_focused,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.list.list.item_elements,
        key: key.clone(),
        element: CapturedElement::new(),
        id: Some(state.tab_id(&key)),
        collection_id: state.collection_id().unwrap_or_else(|| {
            dev_warn!("use_tab: call `use_tab_list` before rendering its tabs");
            // The tab list's own id names no collection: the tab belongs to none.
            state.id()
        }),
        is_disabled,
        should_select_on_press_up: should_select_on_press_up.resolve(|| is_link),
        allows_different_press_origin: false,
        on_action: Signal::stored(None),
        link_behavior: Signal::stored(LinkBehavior::Selection),
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (item, styles) = props.into_props().into_inner();
    let panel_id = state.tab_panel_id(&key);
    let focusable: UseFocusableItemProps = use_focusable(UseFocusableInput {
        is_disabled,
        on_focus,
        on_blur,
        on_focus_change,
        auto_focus: false,
        exclude_from_tab_order: Signal::stored(false),
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        allow_shortcut_repeats: false,
        mark: None,
    })
    .target
    .into_props()
    .into();

    UseTabReturn {
        tab: Part::stored(PropsWithStyles::new(
            UseTabProps {
                role: AriaRole::Tab,
                aria_selected: Signal::derive(move || AriaSelected::from(is_selected.get())),
                aria_disabled: aria_true(is_disabled),
                aria_controls: Signal::derive(move || is_selected.get().then(|| panel_id.clone())),
                item,
                focusable,
            },
            styles,
        )),
        is_selected,
        is_disabled,
        is_pressed,
        is_focused,
    }
}
