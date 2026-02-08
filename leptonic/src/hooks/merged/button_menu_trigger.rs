//! Merged Props types for combining `use_button` with `use_menu_trigger`.
//!
//! This module provides types for creating accessible menu buttons by combining
//! button semantics with menu trigger behavior.

use leptos::attr;
use leptos::attr::custom::CustomAttr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::hooks::button::UseButtonProps;
use crate::hooks::menu::use_menu_trigger::{UseMenuTriggerMenuProps, UseMenuTriggerProps};
use crate::utils::{EventHandler, MergeWith};

/// Return type from merging `UseButtonProps` with `UseMenuTriggerProps`.
///
/// Contains the combined props from both hooks, along with state signals and menu props.
///
/// # Example
///
/// ```ignore
/// let button = use_button(...);
/// let menu_trigger = use_menu_trigger(...);
/// let merged = button.props.merge_with(menu_trigger.props);
/// // or use the convenience method:
/// // let merged = menu_trigger.merge_with_button(button);
///
/// view! {
///     <button {..merged.props.into_attrs()}>
///         "Actions"
///     </button>
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MergedButtonMenuTriggerReturn {
    /// Combined props from both hooks. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub trigger_props: MergedButtonMenuTriggerProps,

    /// Props to pass to the menu (from menu trigger hook).
    pub menu_props: UseMenuTriggerMenuProps,

    /// Whether the button is currently hovered (from button hook).
    pub is_hovered: Signal<bool>,

    /// Whether the button is currently pressed (from button hook).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (from button hook).
    pub is_focus_visible: Signal<bool>,
}

/// Combined props from `use_button` and `use_menu_trigger` hooks.
///
/// This type includes:
/// - Button semantics: `role="button"`, `tabindex`, `disabled`, `aria_disabled`
/// - Menu ARIA (takes precedence): `id`, `aria_haspopup`, `aria_expanded`, `aria_controls`
/// - Button's focus-visible data attribute
/// - Chained event handlers: `on_keydown`, `on_click`, `on_pointerdown` (both run in sequence)
/// - Button's distinct handlers: `on_pointerenter`, `on_pointerleave`, `on_focus`, `on_blur`
#[derive(Debug, Clone)]
pub struct MergedButtonMenuTriggerProps {
    // Button semantics
    /// The role of the element ("button").
    pub role: &'static str,
    /// The tabindex of the element.
    pub tabindex: Signal<Option<&'static str>>,
    /// Whether the element is disabled.
    pub disabled: Signal<bool>,
    /// The aria-disabled state.
    pub aria_disabled: Signal<&'static str>,

    // Menu ARIA (takes precedence over button)
    /// Unique identifier for the trigger element (from menu trigger).
    pub id: String,
    /// The type of popup this trigger opens (from menu trigger).
    pub aria_haspopup: &'static str,
    /// Whether the popup is currently expanded (from menu trigger).
    pub aria_expanded: Signal<&'static str>,
    /// ID of the controlled popup element (from menu trigger).
    pub aria_controls: Signal<Option<String>>,

    /// Accessibility description for long press action (from button's press hook).
    pub aria_describedby: Option<&'static str>,

    // Button's focus-visible data attribute
    /// Data attribute for focus-visible styling.
    pub data_focus_visible: CustomAttr<&'static str, Signal<Option<&'static str>>>,

    // Chained event handlers (button first, then menu trigger)
    /// Keyboard event handler (chained: button, then menu trigger).
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Click event handler (chained: button, then menu trigger).
    pub on_click: EventHandler<MouseEvent>,
    /// Pointer down event handler (chained: button, then menu trigger).
    pub on_pointerdown: EventHandler<PointerEvent>,
    /// Drag start event handler (from button, for Safari workaround).
    pub on_dragstart: EventHandler<DragEvent>,

    // Button's distinct handlers
    /// Pointer enter event handler (from button).
    pub on_pointerenter: EventHandler<PointerEvent>,
    /// Pointer leave event handler (from button).
    pub on_pointerleave: EventHandler<PointerEvent>,
    /// Focus event handler (from button).
    pub on_focus: EventHandler<FocusEvent>,
    /// Blur event handler (from button).
    pub on_blur: EventHandler<FocusEvent>,
}

impl MergedButtonMenuTriggerProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> MergedButtonMenuTriggerAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedButtonMenuTriggerAttrs {
        (
            // Button semantics
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaDisabled, self.aria_disabled),
            // Menu ARIA
            Attr(attr::Id, self.id),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
            // Long press accessibility
            Attr(attr::AriaDescribedby, self.aria_describedby),
            // Focus visible
            self.data_focus_visible,
            // Chained event handlers
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            // Button's distinct handlers
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// Attribute tuple type for [`MergedButtonMenuTriggerProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedButtonMenuTriggerAttrs = (
    // Button semantics
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<Option<&'static str>>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    // Menu ARIA
    Attr<attr::Id, String>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<&'static str>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
    // Long press accessibility
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    // Focus visible
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    // Chained event handlers
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    // Button's distinct handlers
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

impl MergeWith<UseMenuTriggerProps> for UseButtonProps {
    type Output = MergedButtonMenuTriggerProps;

    /// Merge `UseButtonProps` with `UseMenuTriggerProps`.
    ///
    /// # Handler Chaining
    ///
    /// Overlapping event handlers are chained (both run in sequence):
    /// - `on_keydown`: button first, then menu trigger
    /// - `on_click`: button first, then menu trigger
    /// - `on_pointerdown`: button first, then menu trigger
    ///
    /// # ARIA Precedence
    ///
    /// Menu trigger ARIA attributes take precedence over button's:
    /// - `id`: from menu trigger
    /// - `aria-haspopup`: from menu trigger (knows the popup type)
    /// - `aria-expanded`: from menu trigger (tracks menu state)
    /// - `aria-controls`: from menu trigger
    ///
    /// Button provides:
    /// - `role`: "button"
    /// - `tabindex`: based on disabled state
    /// - `disabled`: disabled state
    /// - `aria-disabled`: disabled state as string
    /// - `data-focus-visible`: focus ring visibility
    /// - `on_pointerenter`, `on_pointerleave`: hover handlers
    /// - `on_focus`, `on_blur`: focus handlers
    fn merge_with(self, menu_trigger: UseMenuTriggerProps) -> Self::Output {
        let button = self;
        MergedButtonMenuTriggerProps {
            // Button semantics
            role: button.role,
            tabindex: button.tabindex,
            disabled: button.disabled,
            aria_disabled: button.aria_disabled,

            // Menu trigger ARIA (takes precedence)
            id: menu_trigger.id,
            aria_haspopup: menu_trigger.aria_haspopup,
            aria_expanded: menu_trigger.aria_expanded,
            aria_controls: menu_trigger.aria_controls,

            // Long press accessibility (from button's press hook)
            aria_describedby: button.aria_describedby,

            // Button's focus-visible attribute
            data_focus_visible: button.data_focus_visible,

            // Chain overlapping handlers (button first, then menu trigger)
            on_keydown: button.on_keydown.chain(menu_trigger.on_keydown),
            on_click: button.on_click.chain(menu_trigger.on_click),
            on_pointerdown: button.on_pointerdown.chain(menu_trigger.on_pointerdown),
            on_dragstart: button.on_dragstart,

            // Button's distinct handlers
            on_pointerenter: button.on_pointerenter,
            on_pointerleave: button.on_pointerleave,
            on_focus: button.on_focus,
            on_blur: button.on_blur,
        }
    }
}

impl MergeWith<UseButtonProps> for UseMenuTriggerProps {
    type Output = MergedButtonMenuTriggerProps;

    fn merge_with(self, button: UseButtonProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        button.merge_with(self)
    }
}
