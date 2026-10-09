// Upstream: react-aria-components/src/Tabs.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{
    attr::{self, Attr},
    context::Provider,
    prelude::*,
};
use leptos_classes::Classes;
use wasm_bindgen::JsCast;

use crate::{
    CapturedElement, IntoAttrs, Orientation, Out, ValueBinding,
    hooks::{
        animation::{
            UseEnterAnimationInput, UseEnterAnimationReturn, UseExitAnimationInput,
            use_enter_animation, use_exit_animation,
        },
        collections::{CollectionMemo, Key, SelectOnPressUp},
        focus::{FocusRingTarget, UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
        interactions::{HoverEndEvent, HoverStartEvent, UseHoverInput, use_hover},
        tabs::{
            KeyboardActivation, TabListState, UseTabInput, UseTabListInput, UseTabListReturn,
            UseTabListStateInput, UseTabPanelInput, UseTabReturn, use_tab, use_tab_list,
            use_tab_list_state, use_tab_panel,
        },
    },
    utils::{
        aria::AriaRole, data_attributes::flag, default_class::with_default_class, dev_warn,
        styles::Styles,
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
// - Selected key (C4): `default_selected_key` + `on_selected_key_change`, or `selected_key` +
//   `set_selected_key`.
// - Render props become `data-*` attributes plus plain children. The parts read the tab list's
//   `TabListState` from the context (react-aria-components' `TabListStateContext`).
// - `Tab` has no press callbacks (`onPress*`): the collection item hook doesn't take an item's
//   own press handlers yet.
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
// - Slots and the `render` prop.
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
/// The parts (and custom ones) read the [`TabListState`] from the context.
///
/// Data attributes: `data-orientation`, `data-focused` (focus within), `data-focus-visible`
/// (keyboard focus within), `data-disabled`.
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
    on_selected_key_change: Option<Callback<Key>>,
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
        ValueBinding::from_state_props(selected_key, set_selected_key, on_selected_key_change);
    let disabled_tabs = RwSignal::new(HashSet::new());
    let disabled_keys = disabled_keys.unwrap_or_default();
    let state = use_tab_list_state(UseTabListStateInput {
        default_selected_key,
        selected_key,
        on_selection_change,
        disabled_keys: Memo::new(move |_| {
            let mut keys = disabled_keys.get();
            keys.extend(disabled_tabs.get());
            keys
        })
        .into(),
        is_disabled,
        collection,
    });
    let data_orientation = move || orientation.get().as_str();
    let config = TabsConfig {
        orientation,
        keyboard_activation,
        disabled_tabs,
    };
    // As react-aria-components' `Tabs`: focus (and keyboard focus) within.
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });

    view! {
        <Provider value=state>
            <Provider value=config>
                <div
                    {..focus_ring.into_attrs()}
                    class=classes
                    style=styles
                    data-orientation=data_orientation
                    data-focused=flag(is_focused)
                    data-focus-visible=flag(is_focus_visible)
                    data-disabled=flag(is_disabled)
                >
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The tab list's state and the settings of the surrounding [`Tabs`], or `None` (with a warning
/// in debug builds) outside of one.
fn tabs_context(part: &str) -> Option<(TabListState, TabsConfig)> {
    let context = use_context::<TabListState>().zip(use_context::<TabsConfig>());
    if context.is_none() {
        dev_warn!("A <{part}> must be inside <Tabs>.");
    }
    context
}

/// The tab list of [`Tabs`]: holds the [`Tab`]s. Name it with `aria_label` or `aria_labelledby`
/// (with both, it is labelled by itself and the referenced elements).
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
    let (state, config) = tabs_context("TabList")?;
    let orientation = config.orientation;
    let data_orientation = move || orientation.get().as_str();
    let UseTabListReturn { props } = use_tab_list(UseTabListInput {
        orientation: config.orientation,
        keyboard_activation: config.keyboard_activation,
        aria_label,
        aria_labelledby,
        state,
        element: CapturedElement::new(),
    });

    Some(view! {
        <div {..props.into_attrs()} class=classes style=styles data-orientation=data_orientation>
            {children()}
        </div>
    })
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
    /// Called when the pointer starts hovering the (enabled) tab.
    #[prop(into, optional)]
    on_hover_start: Option<Callback<HoverStartEvent>>,
    /// Called when the pointer stops hovering the tab.
    #[prop(into, optional)]
    on_hover_end: Option<Callback<HoverEndEvent>>,
    /// Called when the hover state changes.
    #[prop(into, optional)]
    on_hover_change: Option<Callback<bool>>,
    /// Called when the tab receives focus.
    #[prop(into, optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when the tab loses focus.
    #[prop(into, optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Called when the tab's focus changes.
    #[prop(into, optional)]
    on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Tab", classes);
    let (state, config) = tabs_context("Tab")?;
    // Disabled tabs join the tab list's disabled keys, so that the keyboard skips them.
    let disabled_tabs = config.disabled_tabs;
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
        state,
        key,
        is_disabled,
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus,
        on_blur,
        on_focus_change,
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
        on_hover_start,
        on_hover_end,
        on_hover_change,
    });

    Some(view! {
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
    })
}

/// The content of the tab `key` of [`Tabs`], rendered while that tab is selected (and while its
/// exit animation runs), or always with `should_force_mount`. While its tab isn't selected, the
/// panel is inert and no tab panel (no role, id, label or tab stop), as in
/// react-aria-components; style `[data-inert]` to hide a force-mounted one.
///
/// Data attributes: `data-focused`, `data-focus-visible` (the panel is focusable without
/// focusable content), `data-inert`, `data-entering` (while the enter animation of a panel
/// mounted by a selection change runs), `data-exiting` (while the exit animation of a panel whose
/// tab was deselected runs; it stays mounted until it finished).
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
    /// Names the panel, next to its tab.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_details: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TabPanel", classes);
    let Some(state) = use_context::<TabListState>() else {
        dev_warn!("A <TabPanel> must be inside <Tabs>.");
        return None;
    };
    let panel_key = key.clone();
    let is_selected = Memo::new(move |_| state.selected_key().as_ref() == Some(&panel_key)).into();
    let element = CapturedElement::new();
    let is_exiting = use_exit_animation(UseExitAnimationInput {
        element,
        is_open: is_selected,
        on_exit: None,
    })
    .is_exiting;
    // Selected when first rendered: no enter animation (react-aria-components).
    let initially_selected = StoredValue::new(is_selected.get_untracked());
    Effect::new(move || {
        if !is_selected.get() {
            initially_selected.set_value(false);
        }
    });
    let panel = move || {
        let panel = use_tab_panel(UseTabPanelInput {
            state,
            key: Some(key.clone()),
            aria_label,
            aria_describedby: aria_describedby.clone(),
            aria_details: aria_details.clone(),
        });
        let focus_ring = use_focus_ring(UseFocusRingInput::default());
        let UseEnterAnimationReturn {
            is_entering,
            styles: hiding,
        } = use_enter_animation(UseEnterAnimationInput {
            element,
            is_ready: Signal::stored(true),
            on_enter: None,
        });
        let animates_entry = !initially_selected.get_value();
        let is_entering = Signal::derive(move || animates_entry && is_entering.get());
        let props = panel.props;
        // While its tab isn't selected (force-mounted or exiting), the panel is no tab panel.
        let selected = move |value| Signal::derive(move || is_selected.get().then_some(value));
        let (id, label, labelled_by, tabindex) = (
            props.id,
            props.aria_label,
            props.aria_labelledby,
            props.tabindex,
        );
        let attrs = (
            Attr(
                attr::Id,
                Signal::derive(move || id.get().filter(|_| is_selected.get())),
            ),
            Attr(attr::Role, selected(AriaRole::Tabpanel)),
            Attr(
                attr::AriaLabel,
                Signal::derive(move || label.get().filter(|_| is_selected.get())),
            ),
            Attr(
                attr::AriaLabelledby,
                Signal::derive(move || labelled_by.get().filter(|_| is_selected.get())),
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
        let inert = move || (!is_selected.get()).then_some("");
        view! {
            <div
                {..attrs}
                {..focus_ring.props.into_attrs()}
                {..element.attr()}
                class=classes.clone()
                style=hiding.merge(styles.clone())
                inert=inert
                data-inert=flag(Signal::derive(move || !is_selected.get()))
                data-focused=flag(is_focused)
                data-focus-visible=flag(is_focus_visible)
                data-entering=flag(is_entering)
                data-exiting=flag(is_exiting)
            >
                {children()}
            </div>
        }
    };

    Some(view! {
        <Show when=move || should_force_mount || is_selected.get() || is_exiting.get()>
            {panel()}
        </Show>
    })
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
    let Some(state) = use_context::<TabListState>() else {
        dev_warn!("A <TabPanels> must be inside <Tabs>.");
        return None;
    };
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

    Some(view! {
        <div {..element.attr()} class=classes style=styles>
            {children()}
        </div>
    })
}
