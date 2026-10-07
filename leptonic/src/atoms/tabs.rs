use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        IntoAttrs, KeyboardActivation, Orientation, TabListData, TabListItemData,
        UseFocusRingInput, UseFocusRingReturn, UseHoverInput, UseTabInput, UseTabListInput,
        UseTabListReturn, UseTabListStateInput, UseTabPanelInput, UseTabReturn,
        collections::{CollectionMemo, Key, SelectOnPressUp},
        use_focus_ring, use_hover, use_tab, use_tab_list, use_tab_list_state, use_tab_panel,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
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
///
/// Default class: `leptonic-Tabs`.
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
    /// The selected tab (controlled): a value or any signal.
    #[prop(into, optional)]
    selected_key: Option<Signal<Key>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected_key: Option<Out<Key>>,
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
    let classes = with_default_class("leptonic-Tabs", classes);
    let (selected_key, on_selection_change) =
        ValueBinding::from_state_props(selected_key, set_selected_key, on_selection_change);
    let state = use_tab_list_state(UseTabListStateInput {
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys: disabled_keys.unwrap_or_default(),
        is_disabled,
        collection,
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

/// The tab list of [`Tabs`]: holds the [`Tab`]s. Exposes `data-orientation` for styling.
///
/// Default class: `leptonic-TabList`.
#[component]
pub fn TabList(
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TabList", classes);
    let tabs = expect_context::<TabListData>();
    let config = expect_context::<TabsConfig>();
    let data_orientation = config.orientation.as_str();
    let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
        orientation: config.orientation,
        keyboard_activation: config.keyboard_activation,
        aria_label,
        aria_labelledby,
        tabs,
        element: CapturedElement::new(),
    });

    view! {
        <Provider value=data>
            <div
                {..props.into_attrs()}
                class=classes
                style=styles
                data-orientation=data_orientation
            >
                {children()}
            </div>
        </Provider>
    }
}

/// A tab of a [`TabList`], for the collection item `key`. Disable tabs in the collection
/// (`ItemBuilder::disabled`) or with the `disabled_keys` of [`Tabs`], so that keyboard
/// navigation skips them.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-hovered`, `data-disabled`
/// and `data-pressed` for styling.
///
/// Default class: `leptonic-Tab`.
#[component]
pub fn Tab(
    /// The tab's key in the collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Tab", classes);
    let list = expect_context::<TabListItemData>();
    let UseTabReturn {
        tab_props,
        is_selected,
        is_disabled,
        is_pressed,
        ..
    } = use_tab(UseTabInput {
        list,
        key,
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
    });
    let (attrs, tab_styles) = tab_props.into_parts();
    let styles = tab_styles.merge(styles);
    // As react-aria-components' `Tab`: a focus ring and hover on the tab.
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput::default());
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });

    view! {
        <div
            {..attrs}
            {..focus_ring.into_attrs()}
            {..hover.props.into_attrs()}
            class=classes
            style=styles
            data-selected=flag(is_selected)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-hovered=flag(hover.is_hovered)
            data-disabled=flag(is_disabled)
            data-pressed=flag(is_pressed)
        >
            {children()}
        </div>
    }
}

/// The content of the tab `key` of [`Tabs`], rendered while that tab is selected, or always with
/// `should_force_mount` (then inert while unselected: style `[data-inert]` to hide it). Exposes
/// `data-focused` and `data-focus-visible` (the panel is focusable without focusable content).
///
/// Default class: `leptonic-TabPanel`.
#[component]
pub fn TabPanel(
    /// The panel's tab.
    #[prop(into)]
    key: Key,
    /// Keep the panel mounted while its tab isn't selected (e.g. to keep its state), inert and
    /// with `data-inert` (react-aria-components' `shouldForceMount`).
    #[prop(optional)]
    should_force_mount: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TabPanel", classes);
    let tabs = expect_context::<TabListData>();
    let state = tabs.state;
    let panel_key = key.clone();
    let is_selected = Signal::derive(move || state.selected_key().as_ref() == Some(&panel_key));
    if should_force_mount {
        let panel = use_tab_panel(UseTabPanelInput {
            tabs,
            key: Some(key),
        });
        let inert = move || (!is_selected.get()).then_some("");
        let focus_ring = use_focus_ring(UseFocusRingInput::default());
        return view! {
            <div
                {..panel.tab_panel_props.into_attrs()}
                {..focus_ring.props.into_attrs()}
                class=classes
                style=styles
                inert=inert
                data-inert=flag(Signal::derive(move || !is_selected.get()))
                data-focused=flag(focus_ring.is_focused)
                data-focus-visible=flag(focus_ring.is_focus_visible)
            >
                {children()}
            </div>
        }
        .into_any();
    }
    view! {
        <Show when=move || is_selected.get()>
            {
                let panel = use_tab_panel(UseTabPanelInput {
                    tabs: tabs.clone(),
                    key: Some(key.clone()),
                });
                let focus_ring = use_focus_ring(UseFocusRingInput::default());
                view! {
                    <div
                        {..panel.tab_panel_props.into_attrs()}
                        {..focus_ring.props.into_attrs()}
                        class=classes.clone()
                        style=styles.clone()
                        data-focused=flag(focus_ring.is_focused)
                        data-focus-visible=flag(focus_ring.is_focus_visible)
                    >
                        {children()}
                    </div>
                }
            }
        </Show>
    }
    .into_any()
}
