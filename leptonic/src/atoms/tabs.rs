// Upstream: react-aria-components/src/Tabs.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{
    attr::{self, Attr},
    context::Provider,
    prelude::*,
};
use wasm_bindgen::JsCast;

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
        CapturedElement, ValueBinding, aria::AriaRole, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The tabs come from a `collection` prop (react-aria-components builds the collection from
//   the `Tab` children). Render one `Tab` per collection item, in collection order: tabs without
//   an item aren't reachable by the keyboard, items without a tab are skipped invisibly. Reason:
//   leptonic's collections are built before rendering (the keyboard delegate, the default
//   selection and the server-rendered selection need them up front).
// - Selected key (C4): `default_selected_key` + `on_selection_change`, or `selected_key` +
//   `set_selected_key`.
// - Render props become `data-*` attributes plus plain children.
//
// ## DIFFERENT BEHAVIOR
// - A `Tab`'s `is_disabled` is known only once the tab renders. The first enabled tab selected
//   by default skips it as the tabs render (on the server too), but a `TabPanel` rendered before
//   the `TabList` doesn't know it yet. Disable tabs in the collection or with `disabled_keys` to
//   have them skipped from the start.
// - `TabPanels` animates from the size it measured after the previous selection change (or
//   mount), not from the size right before the change, and measures the new size in the next
//   animation frame (before it is painted). Reason: React renders allow measuring before the DOM
//   update; Leptos effects run after it, in no fixed order with the effects updating the panels.
//
// ## OMITTED FEATURES
// - `SelectionIndicator` (an indicator sliding between the tabs, built on
//   `SharedElementTransition`) and tabs as links (`href` on `Tab`): not ported yet.
// - Slots, the `render` prop and `TabListStateContext`.
//
// =============================================================================

/// The settings of a [`Tabs`] for its [`TabList`] and [`Tab`]s.
#[derive(Debug, Clone, Copy)]
struct TabsConfig {
    orientation: Signal<Orientation>,
    keyboard_activation: KeyboardActivation,
    /// The keys of the tabs disabled by their `is_disabled`.
    disabled_tabs: RwSignal<HashSet<Key>>,
}

/// Headless tabs: a [`TabList`] of [`Tab`]s, one of which is selected, and a [`TabPanel`] per
/// tab showing the selected tab's content.
///
/// The tabs come from `collection`: render one [`Tab`] per collection item, in collection order.
///
/// Data attributes: `data-orientation`.
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
    /// The axis of the arrow keys. Default: horizontal.
    #[prop(into, default = Orientation::Horizontal.into())]
    orientation: Signal<Orientation>,
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
    let disabled_tabs = RwSignal::new(HashSet::new());
    let disabled_keys = disabled_keys.unwrap_or_default();
    let state = use_tab_list_state(UseTabListStateInput {
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys: Signal::derive(move || {
            let mut keys = disabled_keys.get();
            keys.extend(disabled_tabs.get());
            keys
        }),
        is_disabled,
        collection,
    });
    let data_orientation = move || orientation.get().as_str();
    let config = TabsConfig {
        orientation,
        keyboard_activation,
        disabled_tabs,
    };

    view! {
        <Provider value=TabListData::new(state)>
            <Provider value=config>
                <div class=classes style=styles data-orientation=data_orientation>
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The tab list of [`Tabs`]: holds the [`Tab`]s.
///
/// Data attributes: `data-orientation`.
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
    let orientation = config.orientation;
    let data_orientation = move || orientation.get().as_str();
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

/// A tab of a [`TabList`], for the collection item `key`. Disable it with `is_disabled`, in the
/// collection (`ItemBuilder::disabled`) or with the `disabled_keys` of [`Tabs`]; keyboard
/// navigation skips it either way.
///
/// Data attributes: `data-selected`, `data-focused`, `data-focus-visible`, `data-hovered`,
/// `data-disabled`, `data-pressed`.
///
/// Default class: `leptonic-Tab`.
#[component]
pub fn Tab(
    /// The tab's key in the collection.
    #[prop(into)]
    key: Key,
    /// Whether the tab is disabled.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Tab", classes);
    let list = expect_context::<TabListItemData>();
    // Disabled tabs join the tab list's disabled keys, so that the keyboard skips them.
    let disabled_tabs = expect_context::<TabsConfig>().disabled_tabs;
    let tab_key = StoredValue::new(key.clone());
    let set_disabled = move |disabled: bool| {
        let changed = tab_key
            .with_value(|key| disabled_tabs.with_untracked(|keys| keys.contains(key) != disabled));
        if changed {
            disabled_tabs.update(|keys| {
                tab_key.with_value(|key| {
                    if disabled {
                        keys.insert(key.clone());
                    } else {
                        keys.remove(key);
                    }
                });
            });
        }
    };
    set_disabled(is_disabled.get_untracked());
    Effect::new(move || set_disabled(is_disabled.get()));
    on_cleanup(move || {
        if let Some(key) = tab_key.try_get_value() {
            disabled_tabs.try_update(|keys| keys.remove(&key));
        }
    });
    let UseTabReturn {
        props,
        is_selected,
        is_disabled,
        is_pressed,
        ..
    } = use_tab(UseTabInput {
        list,
        key,
        is_disabled,
        should_select_on_press_up: SelectOnPressUp::Auto,
    });
    let (attrs, tab_styles) = props.into_parts();
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
/// `should_force_mount`: then, while its tab isn't selected, it is inert and no tab panel (no
/// role, id, label or tab stop), as in react-aria-components; style `[data-inert]` to hide it.
///
/// Data attributes: `data-focused`, `data-focus-visible` (the panel is focusable without
/// focusable content), `data-inert`.
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
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_details: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TabPanel", classes);
    let tabs = expect_context::<TabListData>();
    let state = tabs.state;
    let panel_key = key.clone();
    let is_selected = Signal::derive(move || state.selected_key().as_ref() == Some(&panel_key));
    let panel = move || {
        let panel = use_tab_panel(UseTabPanelInput {
            tabs: tabs.clone(),
            key: Some(key.clone()),
            aria_describedby: aria_describedby.clone(),
            aria_details: aria_details.clone(),
        });
        let focus_ring = use_focus_ring(UseFocusRingInput::default());
        let props = panel.props;
        // While its tab isn't selected (force-mounted), the panel is no tab panel.
        let selected = move |value| Signal::derive(move || is_selected.get().then_some(value));
        let (id, labelled_by, tabindex) = (props.id, props.aria_labelledby, props.tabindex);
        let attrs = (
            Attr(
                attr::Id,
                Signal::derive(move || is_selected.get().then(|| id.get())),
            ),
            Attr(attr::Role, selected(AriaRole::Tabpanel)),
            Attr(
                attr::AriaLabelledby,
                Signal::derive(move || is_selected.get().then(|| labelled_by.get())),
            ),
            Attr(
                attr::Tabindex,
                Signal::derive(move || tabindex.get().filter(|_| is_selected.get())),
            ),
            Attr(attr::AriaDescribedby, props.aria_describedby),
            Attr(attr::AriaDetails, props.aria_details),
            props.tabbable_child.into_attrs(),
        );
        let is_focused = focus_ring.is_focused;
        let is_focus_visible = focus_ring.is_focus_visible;
        (
            attrs,
            focus_ring.props.into_attrs(),
            is_focused,
            is_focus_visible,
        )
    };
    if should_force_mount {
        let (attrs, focus_attrs, is_focused, is_focus_visible) = panel();
        let inert = move || (!is_selected.get()).then_some("");
        return view! {
            <div
                {..attrs}
                {..focus_attrs}
                class=classes
                style=styles
                inert=inert
                data-inert=flag(Signal::derive(move || !is_selected.get()))
                data-focused=flag(is_focused)
                data-focus-visible=flag(is_focus_visible)
            >
                {children()}
            </div>
        }
        .into_any();
    }
    view! {
        <Show when=move || is_selected.get()>
            {
                let (attrs, focus_attrs, is_focused, is_focus_visible) = panel();
                view! {
                    <div
                        {..attrs}
                        {..focus_attrs}
                        class=classes.clone()
                        style=styles.clone()
                        data-focused=flag(is_focused)
                        data-focus-visible=flag(is_focus_visible)
                    >
                        {children()}
                    </div>
                }
            }
        </Show>
    }
    .into_any()
}

/// Groups the [`TabPanel`]s of [`Tabs`] and animates its size when the selected tab changes:
/// while it changes, `--tab-panel-width` and `--tab-panel-height` hold its size in pixels (from
/// the old panel's to the new one's), otherwise `auto`. Give it a CSS transition on `width`/
/// `height` (or `block-size`/`inline-size`) set to these variables to animate.
///
/// Default class: `leptonic-TabPanels`.
#[component]
pub fn TabPanels(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TabPanels", classes);
    let state = expect_context::<TabListData>().state;
    let element = CapturedElement::new();
    // The size after the last selection change (or mount), to animate from.
    let size = StoredValue::new(None::<(f64, f64)>);
    let has_transition = StoredValue::new(None::<bool>);
    // Measures the panels at their natural size and, when the selection changed (`animate`) and
    // the size differs from the stored one, animates from the stored size to the new one.
    let update = move |el: web_sys::HtmlElement, animate: bool| {
        // Unmounted before the frame.
        if size.is_disposed() || has_transition.is_disposed() {
            return;
        }
        let window = leptos_use::use_window();
        let Some(window) = window.as_ref() else {
            return;
        };
        let has_transition = has_transition.get_value().unwrap_or_else(|| {
            let transition = window
                .get_computed_style(&el)
                .ok()
                .flatten()
                .and_then(|style| style.get_property_value("transition").ok())
                .unwrap_or_default();
            let found = ["width", "height", "block-size", "inline-size", "all"]
                .iter()
                .any(|property| transition.contains(property));
            has_transition.set_value(Some(found));
            found
        });
        let style = el.style();
        let set_size = move |width: &str, height: &str| {
            let _ = style.set_property("--tab-panel-width", width);
            let _ = style.set_property("--tab-panel-height", height);
        };
        set_size("auto", "auto");
        let rect = el.get_bounding_client_rect();
        let (width, height) = (rect.width(), rect.height());
        if has_transition
            && animate
            && let Some((old_width, old_height)) = size.get_value()
            && (old_width, old_height) != (width, height)
        {
            // From the old size (forcing a style calculation) to the new one, then back to auto.
            set_size(&format!("{old_width}px"), &format!("{old_height}px"));
            if let Ok(Some(computed)) = window.get_computed_style(&el) {
                let _ = computed.get_property_value("height");
            }
            set_size(&format!("{width}px"), &format!("{height}px"));
            let promises: js_sys::Array = el
                .get_animations()
                .iter()
                .filter_map(|animation| animation.dyn_into::<web_sys::Animation>().ok())
                .filter_map(|animation| animation.finished().ok())
                .collect();
            let el = el.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if wasm_bindgen_futures::JsFuture::from(js_sys::Promise::all(&promises))
                    .await
                    .is_ok()
                {
                    let style = el.style();
                    let _ = style.set_property("--tab-panel-width", "auto");
                    let _ = style.set_property("--tab-panel-height", "auto");
                }
            });
        }
        size.set_value(Some((width, height)));
    };
    Effect::new(move |previous: Option<Option<Key>>| {
        let selected = state.selected_key();
        let Some(el) = element
            .get()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlElement>().cloned())
        else {
            return selected;
        };
        let changed = previous
            .as_ref()
            .is_some_and(|previous| previous.is_some() && *previous != selected);
        if changed {
            // The panels change in effects too, which may run after this one: measure the new
            // panel in the next frame, after every effect, before it is painted.
            request_animation_frame(move || update(el, true));
        } else {
            update(el, false);
        }
        selected
    });

    view! {
        <div {..element.attr()} class=classes style=styles>
            {children()}
        </div>
    }
}
