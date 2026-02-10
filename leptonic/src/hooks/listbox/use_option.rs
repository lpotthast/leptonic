use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use std::hash::Hash;
use uuid::Uuid;
use web_sys::FocusEvent;

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::hooks::focus::use_focusable::{use_focusable, UseFocusableInput};
use crate::hooks::selection::use_selection_state::{Selection, UseSelectionStateReturn};
use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::element_capture::ElementCaptureAttr;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useOption.ts

/// Input parameters for the `use_option` hook.
#[derive(Clone)]
pub struct UseOptionInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The unique key for this option.
    pub key: K,

    /// The selection state from the parent listbox.
    pub state: UseSelectionStateReturn<K>,

    /// Whether this option is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the option should be selected on press up instead of press down.
    pub should_select_on_press_up: bool,

    /// Whether the option should use virtual focus (for combobox).
    pub should_use_virtual_focus: bool,

    /// Callback when the option is focused.
    pub on_focus: Option<Callback<()>>,

    /// Callback when the option is pressed.
    pub on_press: Option<Callback<()>>,

    /// Text value for accessibility (screen reader announcement).
    pub text_value: Option<String>,

    /// The currently focused key from the parent listbox.
    /// Used to derive whether this option is focused and to manage DOM focus.
    pub focused_key: Signal<Option<K>>,
}

// Note: No Default implementation for UseOptionInput because `state` must be provided

/// The return value of the `use_option` hook.
#[derive(Clone)]
pub struct UseOptionReturn {
    /// Props for the option element.
    pub option_props: UseOptionAttrs,

    /// Props for the label element inside the option.
    pub label_props: UseOptionLabelProps,

    /// Props for the description element inside the option.
    pub description_props: UseOptionDescriptionProps,

    /// Whether this option is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this option is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether this option is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether this option is pressed.
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the option element.
pub type UseOptionAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::pointerup, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    ElementCaptureAttr,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseOptionLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseOptionDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Provides the behavior and accessibility implementation for an option in a listbox.
///
/// Options are the selectable items within a listbox.
///
/// # Example
///
/// ```ignore
/// let option = use_option(UseOptionInput {
///     key: "apple".to_string(),
///     state: listbox.state.collection.selection_state,
///     is_disabled: Signal::derive(|| false),
///     text_value: Some("Apple".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <li {..option.option_props}>
///         <span id=option.label_props.id>"Apple"</span>
///     </li>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_option<K>(input: UseOptionInput<K>) -> UseOptionReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let base_id = Uuid::new_v4();
    let option_id = format!("option-{base_id}");
    let label_id = format!("option-label-{base_id}");
    let description_id = format!("option-description-{base_id}");

    let key = input.key.clone();
    let state = input.state;
    let local_disabled = input.is_disabled;
    let focused_key = input.focused_key;
    let should_use_virtual_focus = input.should_use_virtual_focus;

    let (is_pressed, set_is_pressed) = signal(false);

    // Check if this option is selected - derive directly from selected_keys for proper reactivity
    let key_for_selected = key.clone();
    let selected_keys = state.selected_keys;
    let is_selected = Signal::derive(move || match selected_keys.get() {
        Selection::Keys(keys) => keys.contains(&key_for_selected),
        Selection::All => true,
    });

    // Derive is_focused from parent's focused_key for proper reactivity
    let key_for_focus = key.clone();
    let is_focused = Signal::derive(move || focused_key.get().as_ref() == Some(&key_for_focus));

    // Combine local and global disabled state
    let is_disabled = Signal::derive(move || local_disabled.get() || state.is_disabled.get());

    // Use focusable to get element capture and focus handle (for real focus mode)
    let focusable = use_focusable(UseFocusableInput {
        disabled: is_disabled,
        auto_focus: false,
        exclude_from_tab_order: Signal::derive(|| true), // Roving tabindex
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
    });

    // Effect to focus the element when is_focused becomes true (real focus mode only)
    if !should_use_virtual_focus {
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
    }

    // Handle click
    let key_for_click = key.clone();
    let on_press_click = input.on_press;
    let handle_click = move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        state.toggle.run(key_for_click.clone());

        if let Some(on_press) = on_press_click {
            on_press.run(());
        }
    };

    // Handle pointer down (for immediate feedback)
    let handle_pointer_down = move |_e: web_sys::PointerEvent| {
        if !is_disabled.get_untracked() {
            set_is_pressed.set(true);
        }
    };

    // Handle pointer up
    let handle_pointer_up = move |_e: web_sys::PointerEvent| {
        set_is_pressed.set(false);
    };

    // Compute tabindex - roving tabindex pattern
    let tabindex = Signal::derive(move || {
        if should_use_virtual_focus {
            "-1"
        } else if is_focused.get() {
            "0"
        } else {
            "-1"
        }
    });

    // Compute aria-selected
    let aria_selected = Signal::derive(move || Some(AriaSelected::from(is_selected.get())));

    // Compute aria-disabled (reactive)
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // Use focus ring for keyboard focus visibility with user callback
    let on_focus_input = input.on_focus;
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: on_focus_input.map(|cb| Callback::new(move |_: FocusEvent| cb.run(()))),
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    UseOptionReturn {
        option_props: (
            Attr(attr::Id, option_id),
            Attr(attr::Role, "option"),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaSelected, aria_selected),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::AriaLabel, input.text_value),
            Attr(attr::AriaDescribedby, None),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::pointerdown, handle_pointer_down).into_cloneable(),
            on(ev::pointerup, handle_pointer_up).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
            data_focus_visible,
            focusable.props.element_capture,
        ),
        label_props: UseOptionLabelProps { id: label_id },
        description_props: UseOptionDescriptionProps { id: description_id },
        is_selected,
        is_focused,
        is_disabled,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
