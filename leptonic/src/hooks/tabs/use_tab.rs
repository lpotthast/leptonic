use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::use_tabs::TabsActivationMode;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::hooks::focus::use_focusable::{use_focusable, UseFocusableInput};
use crate::utils::element_capture::ElementCaptureAttr;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tabs/src/useTab.ts

/// Input parameters for the `use_tab` hook.
#[derive(Debug, Clone)]
pub struct UseTabInput {
    /// The unique key for this tab.
    pub tab_key: String,

    /// The ID base from the parent tabs.
    pub id_base: String,

    /// Whether this tab is selected.
    pub is_selected: Signal<bool>,

    /// Whether this tab is disabled.
    pub is_disabled: Signal<bool>,

    /// The application's desired focus state for this tab.
    ///
    /// When this transitions to `true`, the hook will call `.focus()` on the
    /// DOM element to synchronize browser focus with the application state.
    /// This is typically managed by `use_tab_list` based on keyboard navigation.
    pub is_focused: Signal<bool>,

    /// The activation mode.
    pub activation_mode: TabsActivationMode,

    /// Callback when this tab is selected.
    pub on_select: Option<Callback<()>>,

    /// Callback when this tab receives focus.
    pub on_focus: Option<Callback<()>>,
}

/// The return value of the `use_tab` hook.
#[derive(Clone)]
pub struct UseTabReturn {
    /// Props for the tab button element.
    pub tab_props: UseTabAttrs,

    /// The ID of the tab.
    pub tab_id: String,

    /// The ID of the associated panel.
    pub panel_id: String,

    /// Whether the tab is selected.
    pub is_selected: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the tab button element.
pub type UseTabAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaSelected, Signal<&'static str>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    ElementCaptureAttr,
);

/// Provides the behavior and accessibility for a single tab.
///
/// A tab is a button that activates its associated panel when selected.
///
/// # Example
///
/// ```ignore
/// let tab = use_tab(UseTabInput {
///     tab_key: "tab1".to_string(),
///     id_base: tabs.id_base.clone(),
///     is_selected: Signal::derive(move || selected.get() == Some("tab1".to_string())),
///     is_disabled: Signal::derive(|| false),
///     is_focused: is_tab1_focused.into(),
///     activation_mode: TabsActivationMode::Automatic,
///     on_select: Some(Callback::new(|_| tabs.select_tab.run("tab1".to_string()))),
///     on_focus: None,
/// });
///
/// view! {
///     <button {..tab.tab_props}>
///         "Tab 1"
///     </button>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_tab(input: UseTabInput) -> UseTabReturn {
    let tab_id = format!("{}-tab-{}", input.id_base, input.tab_key);
    let panel_id = format!("{}-panel-{}", input.id_base, input.tab_key);

    let is_selected = input.is_selected;
    let is_disabled = input.is_disabled;
    let is_focused = input.is_focused;
    let activation_mode = input.activation_mode;
    let on_select = input.on_select;
    let on_focus = input.on_focus;

    // Use focusable to get element capture and focus handle
    let focusable = use_focusable(UseFocusableInput {
        disabled: is_disabled,
        auto_focus: false,
        exclude_from_tab_order: Signal::derive(|| true), // Tabs use roving tabindex
        on_focus: None, // We handle focus through focus_ring's on_focus
        on_blur: None,
        on_focus_change: None,
        on_key_down: None, // We handle keydown separately for tab-specific behavior
        on_key_up: None,
    });

    // Effect to focus the element when is_focused becomes true
    let focus_handle = focusable.focus_handle;
    Effect::new(move |prev_focused: Option<bool>| {
        let currently_focused = is_focused.get();
        let was_focused = prev_focused.unwrap_or(false);

        // Only focus if we just became focused (transition from false to true)
        if currently_focused && !was_focused {
            focus_handle.focus();
        }

        currently_focused
    });

    // Compute aria-selected
    let aria_selected = Signal::derive(move || if is_selected.get() { "true" } else { "false" });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Compute tabindex
    let tabindex = Signal::derive(move || {
        if is_selected.get() || is_focused.get() {
            "0"
        } else {
            "-1"
        }
    });

    // Handle click
    let handle_click = move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(());
        }
    };

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        if activation_mode == TabsActivationMode::Manual && (key == "Enter" || key == " ") {
            e.prevent_default();
            if let Some(on_select) = on_select {
                on_select.run(());
            }
        }
        // Arrow key navigation is handled by the tab list
    };

    // Use focus ring with custom on_focus callback that handles the tab's focus logic
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: Some(Callback::new(move |_e| {
            if is_disabled.get_untracked() {
                return;
            }

            if let Some(on_focus) = on_focus {
                on_focus.run(());
            }

            // For automatic activation, select on focus
            if activation_mode == TabsActivationMode::Automatic {
                if let Some(on_select) = on_select {
                    on_select.run(());
                }
            }
        })),
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    UseTabReturn {
        tab_props: (
            Attr(attr::Id, tab_id.clone()),
            Attr(attr::Role, "tab"),
            Attr(attr::AriaSelected, aria_selected),
            Attr(attr::AriaControls, panel_id.clone()),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, tabindex),
            data_focus_visible,
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
            focusable.props.element_capture,
        ),
        tab_id,
        panel_id,
        is_selected,
        is_focus_visible,
    }
}

/// Input parameters for the `use_tab_panel` hook.
#[derive(Debug, Clone)]
pub struct UseTabPanelInput {
    /// The unique key for this panel.
    pub panel_key: String,

    /// The ID base from the parent tabs.
    pub id_base: String,

    /// Whether this panel is visible (its tab is selected).
    pub is_selected: Signal<bool>,
}

/// The return value of the `use_tab_panel` hook.
pub struct UseTabPanelReturn {
    /// Props for the tab panel element.
    pub panel_props: UseTabPanelAttrs,

    /// The ID of the panel.
    pub panel_id: String,

    /// The ID of the associated tab.
    pub tab_id: String,

    /// Whether the panel is visible.
    pub is_selected: Signal<bool>,
}

/// Attributes for the tab panel element.
pub type UseTabPanelAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaHidden, Signal<&'static str>>,
);

/// Provides the behavior and accessibility for a tab panel.
///
/// A tab panel contains the content associated with a tab.
///
/// # Example
///
/// ```ignore
/// let panel = use_tab_panel(UseTabPanelInput {
///     panel_key: "tab1".to_string(),
///     id_base: tabs.id_base.clone(),
///     is_selected: Signal::derive(move || selected.get() == Some("tab1".to_string())),
/// });
///
/// view! {
///     <div {..panel.panel_props}>
///         "Panel 1 content"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_tab_panel(input: UseTabPanelInput) -> UseTabPanelReturn {
    let panel_id = format!("{}-panel-{}", input.id_base, input.panel_key);
    let tab_id = format!("{}-tab-{}", input.id_base, input.panel_key);

    let is_selected = input.is_selected;

    // Compute aria-hidden
    let aria_hidden = Signal::derive(move || if is_selected.get() { "false" } else { "true" });

    UseTabPanelReturn {
        panel_props: (
            Attr(attr::Id, panel_id.clone()),
            Attr(attr::Role, "tabpanel"),
            Attr(attr::AriaLabelledby, tab_id.clone()),
            Attr(attr::Tabindex, "0"),
            Attr(attr::AriaHidden, aria_hidden),
        ),
        panel_id,
        tab_id,
        is_selected,
    }
}
