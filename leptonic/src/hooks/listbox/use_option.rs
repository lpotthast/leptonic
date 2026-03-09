use std::hash::Hash;

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
use uuid::Uuid;
use web_sys::FocusEvent;

use crate::{
    hooks::{
        focus::{
            use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
            use_focusable::{use_focusable, UseFocusableInput},
        },
        selection::use_selection_state::{Selection, UseSelectionStateReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaRole, AriaSelected},
        element_capture::ElementCaptureAttr,
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useOption.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
pub struct UseOptionReturn {
    /// Props for the option element.
    pub option_props: UseOptionProps,

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

/// Props from `use_option` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseOptionProps {
    pub id: String,
    pub role: AriaRole,
    pub tabindex: Signal<&'static str>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
    pub on_pointerdown: EventHandler<web_sys::PointerEvent>,
    pub on_pointerup: EventHandler<web_sys::PointerEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseOptionProps {
    type Attrs = UseOptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.element_capture,
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseOptionAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
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
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    ElementCaptureAttr,
);

/// Props for the label element.
#[derive(Debug)]
pub struct UseOptionLabelProps {
    /// The id of the label element.
    pub id: String,
}

impl IntoAttrs for UseOptionLabelProps {
    type Attrs = UseOptionLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// These attributes must be spread onto the label element: `<span {..attrs} />`
pub type UseOptionLabelAttrs = (Attr<attr::Id, String>,);

/// Props for the description element.
#[derive(Debug)]
pub struct UseOptionDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

impl IntoAttrs for UseOptionDescriptionProps {
    type Attrs = UseOptionDescriptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// These attributes must be spread onto the description element: `<span {..attrs} />`
pub type UseOptionDescriptionAttrs = (Attr<attr::Id, String>,);

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
///     <li {..option.option_props.into_attrs()}>
///         <span {..option.label_props.into_attrs()}>"Apple"</span>
///     </li>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_option<K>(input: UseOptionInput<K>) -> UseOptionReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseOptionInput {
        key,
        state,
        is_disabled: local_disabled,
        should_select_on_press_up,
        should_use_virtual_focus,
        on_focus,
        on_press,
        text_value,
        focused_key,
    } = input;

    let base_id = Uuid::new_v4();
    let option_id = format!("option-{base_id}");
    let label_id = format!("option-label-{base_id}");
    let description_id = format!("option-description-{base_id}");

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
    let on_press_click = on_press;
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
    let on_focus_input = on_focus;
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: on_focus_input.map(|cb| Callback::new(move |_: FocusEvent| cb.run(()))),
        on_blur: None,
        on_focus_change: None,
    });

    UseOptionReturn {
        option_props: UseOptionProps {
            id: option_id,
            role: AriaRole::Option,
            tabindex,
            aria_selected,
            aria_disabled,
            aria_label: text_value,
            aria_describedby: None,
            on_click: EventHandler::new(handle_click),
            on_pointerdown: EventHandler::new(handle_pointer_down),
            on_pointerup: EventHandler::new(handle_pointer_up),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
            element_capture: focusable.props.element_capture,
        },
        label_props: UseOptionLabelProps { id: label_id },
        description_props: UseOptionDescriptionProps { id: description_id },
        is_selected,
        is_focused,
        is_disabled,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
