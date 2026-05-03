use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::{FocusEvent, MouseEvent};

use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        focus::{
            use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
            use_focusable::{UseFocusableInput, use_focusable},
        },
        interactions::use_press::UsePressAttrs,
        selection::{
            SelectionKey,
            use_selectable_item::{UseSelectableItemInput, use_selectable_item},
            use_selection_state::UseSelectionStateReturn,
        },
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaRole, AriaSelected},
        element_capture::ElementCaptureAttr,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useOption.ts

// REACT-ARIA DEVIATIONS
//
// No intentional deviations from the react-aria implementation.
//
// Like react-aria's useOption, this hook delegates selection behavior to
// use_selectable_item (which internally uses use_press).

/// Input parameters for the `use_option` hook.
#[derive(Clone)]
pub struct UseOptionInput<K>
where
    K: SelectionKey,
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

    /// Whether the option should receive focus on mouse hover.
    pub should_focus_on_hover: bool,

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
    pub option_props: PropsWithStyles<UseOptionProps>,

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
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_mouseenter: EventHandler<MouseEvent>,
    pub element_capture: ElementCaptureAttr,
    /// Press-related event handlers from `use_press` (via `use_selectable_item`).
    pub press_attrs: UsePressAttrs,
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
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_mouseenter.into_on(ev::mouseenter),
            self.element_capture,
            self.press_attrs,
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
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
    ElementCaptureAttr,
    UsePressAttrs,
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
/// Options are the selectable items within a listbox. This hook delegates selection
/// behavior to `use_selectable_item` (which internally uses `use_press`), and
/// composes with `use_focusable` and `use_focus_ring` for focus management.
///
/// # Example
///
/// ```ignore
/// let option = use_option(UseOptionInput {
///     key: "apple".to_string(),
///     state: listbox.state.collection.selection_state,
///     is_disabled: Signal::derive(|| false),
///     text_value: Some("Apple".to_string()),
///     should_select_on_press_up: false,
///     should_use_virtual_focus: false,
///     should_focus_on_hover: false,
///     on_focus: None,
///     on_press: None,
///     focused_key: listbox.state.collection.selection_state.focused_key,
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
    K: SelectionKey,
{
    let UseOptionInput {
        key,
        state,
        is_disabled: local_disabled,
        should_select_on_press_up,
        should_use_virtual_focus,
        should_focus_on_hover,
        on_focus,
        on_press,
        text_value,
        focused_key,
    } = input;

    let base_id = Uuid::new_v4();
    let option_id = format!("option-{base_id}");
    let label_id = format!("option-label-{base_id}");
    let description_id = format!("option-description-{base_id}");

    // Combine local and global disabled state
    let is_disabled = Signal::derive(move || local_disabled.get() || state.is_disabled.get());

    // --- Use focusable for element capture and focus handle (real focus mode) ---
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

    // Focus callback for DOM synchronization (for use_selectable_item).
    let focus_fn = if should_use_virtual_focus {
        None
    } else {
        let focus_handle = focusable.focus_handle;
        Some(Callback::new(move |()| focus_handle.focus()))
    };

    // Adapter: use_selectable_item expects Callback<Option<K>> for on_focus.
    let set_focused_key_adapter = Callback::new(move |key_opt: Option<K>| {
        state.set_focused_key.run((key_opt, None));
    });

    // --- Delegate selection to use_selectable_item ---
    let selectable = use_selectable_item(UseSelectableItemInput {
        key: key.clone(),
        selection_mode: state.selection_mode,
        selection_behavior: state.selection_behavior,
        selected_keys: state.selected_keys,
        focused_key,
        is_collection_focused: state.is_focused,
        is_disabled,
        disabled_behavior: state.disabled_behavior,
        disallow_empty_selection: state.disallow_empty_selection,
        on_toggle: state.toggle,
        on_replace: state.select,
        on_extend: None, // Listbox options don't extend-select
        on_double_click: None,
        on_focus: set_focused_key_adapter,
        should_select_on_press_up,
        should_focus_on_hover,
        allow_drag: false,
        allows_different_press_origin: false,
        on_action: on_press,
        on_selection_behavior_change: None,
        focus: focus_fn,
        data_key: Some(format!("{key}")),
    });

    let is_selected = selectable.is_selected;
    let is_focused = selectable.is_focused;

    // Tabindex
    let tabindex = Signal::derive(move || {
        if should_use_virtual_focus {
            "-1"
        } else if is_focused.get() {
            "0"
        } else {
            "-1"
        }
    });

    // ARIA attributes
    let aria_selected = Signal::derive(move || Some(AriaSelected::from(is_selected.get())));
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // --- Focus ring for keyboard focus visibility ---
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

    // Chain selectable_item's focus handler with focus_ring's focus handler.
    let (selectable_props, selectable_styles) = selectable.props.into_inner();
    let on_focus_merged = selectable_props.on_focus.chain(focus_ring_props.on_focus);

    // Convert press props to attrs for embedding in the option's attrs tuple.
    let press_attrs = selectable_props.press.into_attrs();

    UseOptionReturn {
        option_props: PropsWithStyles::new(
            UseOptionProps {
                id: option_id,
                role: AriaRole::Option,
                tabindex,
                aria_selected,
                aria_disabled,
                aria_label: text_value,
                aria_describedby: None,
                on_focus: on_focus_merged,
                on_blur: focus_ring_props.on_blur,
                on_focusin: focus_ring_props.on_focusin,
                on_focusout: focus_ring_props.on_focusout,
                data_focus_visible: focus_ring_props.data_focus_visible,
                on_mouseenter: selectable_props.on_mouseenter,
                element_capture: focusable.props.element_capture,
                press_attrs,
            },
            selectable_styles,
        ),
        label_props: UseOptionLabelProps { id: label_id },
        description_props: UseOptionDescriptionProps { id: description_id },
        is_selected,
        is_focused,
        is_disabled,
        is_pressed: selectable.is_pressed,
        is_focus_visible,
    }
}
