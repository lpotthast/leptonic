use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::use_tabs::TabsActivationMode;
use crate::{
    hooks::{
        focus::{
            use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
            use_focusable::{use_focusable, UseFocusableInput},
        },
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaRole, AriaSelected},
        element_capture::ElementCaptureAttr,
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tabs/src/useTab.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
pub struct UseTabReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseTabProps,

    /// The ID of the tab.
    pub tab_id: String,

    /// The ID of the associated panel.
    pub panel_id: String,

    /// Whether the tab is selected.
    pub is_selected: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_tab` that can be extracted and merged programmatically.
#[derive(Clone)]
pub struct UseTabProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_controls: String,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: Signal<&'static str>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseTabProps {
    type Attrs = UseTabAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaControls, self.aria_controls),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.element_capture,
        )
    }
}

/// Attributes for the tab button element.
pub type UseTabAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
///     <button {..tab.props.into_attrs()}>
///         "Tab 1"
///     </button>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_tab(input: UseTabInput) -> UseTabReturn {
    let UseTabInput {
        tab_key,
        id_base,
        is_selected,
        is_disabled: disabled,
        is_focused,
        activation_mode,
        on_select,
        on_focus,
    } = input;

    let tab_id = format!("{id_base}-tab-{tab_key}");
    let panel_id = format!("{id_base}-panel-{tab_key}");

    // Use focusable to get element capture and focus handle
    let focusable = use_focusable(UseFocusableInput {
        disabled,
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
    let aria_selected = Signal::derive(move || Some(AriaSelected::from(is_selected.get())));

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

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
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(());
        }
    };

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
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
        disabled,
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: Some(Callback::new(move |_e| {
            if disabled.get_untracked() {
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
    UseTabReturn {
        props: UseTabProps {
            id: tab_id.clone(),
            role: AriaRole::Tab,
            aria_selected,
            aria_controls: panel_id.clone(),
            aria_disabled,
            tabindex,
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            element_capture: focusable.props.element_capture,
        },
        tab_id,
        panel_id,
        is_selected,
        is_focus_visible,
    }
}
