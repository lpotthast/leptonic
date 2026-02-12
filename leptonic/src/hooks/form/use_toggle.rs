use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/toggle/src/useToggle.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_toggle` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseToggleInput {
    /// Whether the toggle is selected (controlled).
    pub is_selected: Signal<bool>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<bool>>,

    /// Whether the toggle is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the toggle is read-only.
    pub is_read_only: Signal<bool>,

    /// The value of the toggle when used in a form.
    pub value: Option<&'static str>,
}

impl Default for UseToggleInput {
    fn default() -> Self {
        Self {
            is_selected: Signal::derive(|| false),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            value: None,
        }
    }
}

/// The return value of the `use_toggle` hook.
#[derive(Debug)]
pub struct UseToggleReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseToggleProps,

    /// Whether the toggle is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the toggle is pressed.
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_toggle` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseToggleProps {
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl UseToggleProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseToggleAttrs {
        (
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseToggleAttrs = (
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior for toggle elements like checkboxes and switches.
///
/// This hook handles the toggle interaction pattern - clicking or pressing
/// Space/Enter toggles the selected state.
///
/// # Example
///
/// ```ignore
/// let (is_selected, set_is_selected) = signal(false);
///
/// let toggle = use_toggle(UseToggleInput {
///     is_selected: is_selected.into(),
///     on_change: Some(Callback::new(move |selected| {
///         set_is_selected.set(selected);
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <button
///         role="switch"
///         aria-checked=move || toggle.is_selected.get().to_string()
///         {..toggle.toggle_props}
///     >
///         "Toggle"
///     </button>
/// }
/// ```
pub fn use_toggle(input: UseToggleInput) -> UseToggleReturn {
    let UseToggleInput {
        is_selected,
        on_change,
        is_disabled,
        is_read_only,
        value,
    } = input;

    let (is_pressed, _set_is_pressed) = signal(false);

    // Toggle the state
    let toggle = move || {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let new_value = !is_selected.get_untracked();
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    // Handle click
    let toggle_click = toggle;
    let handle_click = move |_e: MouseEvent| {
        toggle_click();
    };

    // Handle keydown
    let toggle_key = toggle;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        match e.key().as_str() {
            " " | "Enter" => {
                e.prevent_default();
                toggle_key();
            }
            _ => {}
        }
    };

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    UseToggleReturn {
        props: UseToggleProps {
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        is_selected,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
