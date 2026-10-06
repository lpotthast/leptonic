use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    hooks::{
        IntoAttrs, KeyboardActivation, Orientation, TabListData, TabListItemData, UseTabInput,
        UseTabListInput, UseTabListReturn, UseTabListStateInput, UseTabPanelInput, UseTabReturn,
        collections::{CollectionMemo, Key},
        use_tab, use_tab_list, use_tab_list_state, use_tab_panel,
    },
    utils::data_attributes::flag,
    utils::{CapturedElement, ValueBinding, classes::Classes, styles::Styles},
};

/// The settings of a [`Tabs`] for its [`TabList`].
#[derive(Debug, Clone, Copy)]
struct TabsConfig {
    orientation: Orientation,
    keyboard_activation: KeyboardActivation,
}

/// Headless tabs: a [`TabList`] of [`Tab`]s, one of which is selected, and a [`TabPanel`] per
/// tab showing the selected tab's content.
///
/// The tabs come from `collection`: render one [`Tab`] per collection item, in collection order.
/// Exposes `data-orientation` for styling.
#[component]
#[allow(clippy::implicit_hasher)]
pub fn Tabs(
    /// The tabs.
    #[prop(into)]
    collection: CollectionMemo,
    /// The initially selected tab. Defaults to the first enabled tab. Ignored when `selected_key`
    /// is bound.
    #[prop(into, optional)]
    default_selected_key: Option<Key>,
    /// The selected tab as app state (e.g. an `RwSignal<Key>`), replacing `default_selected_key`.
    #[prop(into, optional)]
    selected_key: Option<ValueBinding<Key>>,
    /// Called with the key of the tab the user selects.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<Key>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    /// Disables all tabs.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    /// Whether focusing a tab with the arrow keys selects it.
    #[prop(optional)]
    keyboard_activation: KeyboardActivation,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let state = use_tab_list_state(UseTabListStateInput {
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys: disabled_keys.unwrap_or_default(),
        is_disabled,
        ..UseTabListStateInput::new(collection)
    });
    let data_orientation = orientation.as_str();

    view! {
        <Provider value=TabListData::new(state)>
            <Provider value=TabsConfig { orientation, keyboard_activation }>
                <div class=classes style=styles data-orientation=data_orientation>
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The tab list of [`Tabs`]: holds the [`Tab`]s.
#[component]
pub fn TabList(
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let tabs = expect_context::<TabListData>();
    let config = expect_context::<TabsConfig>();
    let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
        orientation: config.orientation,
        keyboard_activation: config.keyboard_activation,
        aria_label,
        aria_labelledby,
        ..UseTabListInput::new(tabs, CapturedElement::new())
    });

    view! {
        <Provider value=data>
            <div {..props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// A tab of a [`TabList`], for the collection item `key`. Disable tabs in the collection
/// (`ItemBuilder::disabled`) or with the `disabled_keys` of [`Tabs`], so that keyboard
/// navigation skips them.
///
/// Exposes `data-selected`, `data-focused`, `data-disabled` and `data-pressed` for styling.
#[component]
pub fn Tab(
    /// The tab's key in the collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let list = expect_context::<TabListItemData>();
    let UseTabReturn {
        tab_props,
        is_selected,
        is_disabled,
        is_pressed,
        is_focused,
    } = use_tab(UseTabInput::new(list, key));
    let (attrs, tab_styles) = tab_props.into_parts();
    let styles = tab_styles.merge(styles);

    view! {
        <div
            {..attrs}
            class=classes
            style=styles
            data-selected=flag(is_selected)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
            data-pressed=flag(is_pressed)
        >
            {children()}
        </div>
    }
}

/// The content of the tab `key` of [`Tabs`], rendered while that tab is selected.
#[component]
pub fn TabPanel(
    /// The panel's tab.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let tabs = expect_context::<TabListData>();
    let state = tabs.state;
    let panel_key = key.clone();
    let is_selected = move || state.selected_key().as_ref() == Some(&panel_key);
    view! {
        <Show when=is_selected>
            {
                let panel = use_tab_panel(UseTabPanelInput {
                    tabs: tabs.clone(),
                    key: Some(key.clone()),
                });
                view! {
                    <div
                        {..panel.tab_panel_props.into_attrs()}
                        class=classes.clone()
                        style=styles.clone()
                    >
                        {children()}
                    </div>
                }
            }
        </Show>
    }
}
