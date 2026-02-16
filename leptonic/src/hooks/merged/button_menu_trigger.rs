//! Merged Props types for combining `use_button` with `use_menu_trigger`.
//!
//! This module provides types for creating accessible menu buttons by combining
//! button semantics with menu trigger behavior.

use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    hooks::{
        button::UseButtonProps,
        menu::use_menu_trigger::{UseMenuTriggerMenuProps, UseMenuTriggerProps},
        IntoAttrs, MergedPressHoverFocusRingAttrs, MergedPressHoverFocusRingProps,
    },
    utils::{
        aria::{AriaDisabled, AriaExpanded, AriaHasPopup},
        MergeWith,
    },
};

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
#[derive(Debug)]
pub struct MergedButtonMenuTriggerReturn {
    /// Combined props from both hooks. Call `.into_attrs()` for view spreading.
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
#[derive(Debug)]
pub struct MergedButtonMenuTriggerProps {
    /// Unique identifier for the trigger element (from menu trigger).
    pub id: String,
    /// The role of the element ("button").
    pub role: &'static str,
    /// The tabindex of the element.
    pub tabindex: Signal<Option<&'static str>>,
    /// Whether the element is disabled.
    pub disabled: Signal<bool>,
    /// The type of popup this trigger opens (from menu trigger).
    pub aria_haspopup: Option<AriaHasPopup>,
    /// Whether the popup is currently expanded (from menu trigger).
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// ID of the controlled popup element (from menu trigger).
    pub aria_controls: Signal<Option<String>>,
    /// The aria-disabled state.
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub other: MergedPressHoverFocusRingProps,
}

impl IntoAttrs for MergedButtonMenuTriggerProps {
    type Attrs = MergedButtonMenuTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Role, self.role),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaExpanded, self.aria_expanded),
                Attr(attr::AriaControls, self.aria_controls),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            self.other.into_attrs(),
        )
    }
}

/// Attribute tuple type for [`MergedButtonMenuTriggerProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedButtonMenuTriggerAttrs = (
    (
        Attr<attr::Id, String>,
        Attr<attr::Role, &'static str>,
        Attr<attr::Tabindex, Signal<Option<&'static str>>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::AriaHaspopup, Option<AriaHasPopup>>,
        Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
        Attr<attr::AriaControls, Signal<Option<String>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    MergedPressHoverFocusRingAttrs,
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
        let UseButtonProps {
            role: button_role,
            tabindex: button_tabindex,
            disabled: button_disabled,
            aria_disabled: button_aria_disabled,
            aria_haspopup: _button_aria_haspopup, // menu_trigger props take precedence!
            aria_expanded: _button_aria_expanded, // menu_trigger props take precedence!
            other:
                MergedPressHoverFocusRingProps {
                    on_keydown: button_on_keydown,
                    on_click: button_on_click,
                    on_pointerdown: button_on_pointerdown,
                    on_dragstart: button_on_dragstart,
                    on_mousedown: button_on_mousedown,
                    on_pointerup: button_on_pointerup,
                    aria_describedby: button_aria_describedby,
                    on_pointerenter: button_on_pointerenter,
                    on_pointerleave: button_on_pointerleave,
                    on_focus: button_on_focus,
                    on_blur: button_on_blur,
                    on_focusin: button_on_focusin,
                    on_focusout: button_on_focusout,
                    data_focus_visible: button_data_focus_visible,
                },
        } = self;

        MergedButtonMenuTriggerProps {
            id: menu_trigger.id,
            role: button_role,
            tabindex: button_tabindex,
            disabled: button_disabled,
            aria_haspopup: menu_trigger.aria_haspopup,
            aria_expanded: menu_trigger.aria_expanded,
            aria_controls: menu_trigger.aria_controls,
            aria_disabled: button_aria_disabled,

            other: MergedPressHoverFocusRingProps {
                on_keydown: button_on_keydown.chain(menu_trigger.on_keydown),
                on_click: button_on_click.chain(menu_trigger.on_click),
                on_pointerdown: button_on_pointerdown.chain(menu_trigger.on_pointerdown),
                on_dragstart: button_on_dragstart,
                on_mousedown: button_on_mousedown,
                on_pointerup: button_on_pointerup,
                aria_describedby: button_aria_describedby,
                on_pointerenter: button_on_pointerenter,
                on_pointerleave: button_on_pointerleave,
                on_focus: button_on_focus,
                on_blur: button_on_blur,
                on_focusin: button_on_focusin,
                on_focusout: button_on_focusout,
                data_focus_visible: button_data_focus_visible,
            },
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
