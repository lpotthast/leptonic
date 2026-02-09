use leptos::attr::Attribute;
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/toggle/src/useToggle.ts

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
#[derive(Debug, Clone)]
pub struct UseToggleReturn {
    /// Props for the toggle element.
    pub toggle_props: UseToggleAttrs,

    /// Whether the toggle is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the toggle is pressed.
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the toggle element.
pub type UseToggleAttrs = (
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    leptos::attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
    let is_selected = input.is_selected;
    let on_change = input.on_change;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

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
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    UseToggleReturn {
        toggle_props: (
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
            data_focus_visible,
        ),
        is_selected,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}

/// State for managing toggle state with internal signal.
#[derive(Clone, Copy)]
pub struct UseToggleStateReturn {
    /// Whether the toggle is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}

/// Creates internal state for a toggle component.
///
/// Use this when you want uncontrolled toggle state.
///
/// # Example
///
/// ```ignore
/// let state = use_toggle_state(false);
///
/// let toggle = use_toggle(UseToggleInput {
///     is_selected: state.is_selected,
///     on_change: Some(Callback::new(move |selected| {
///         state.set_selected.run(selected);
///     })),
///     ..Default::default()
/// });
/// ```
pub fn use_toggle_state(default_selected: bool) -> UseToggleStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseToggleStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}
