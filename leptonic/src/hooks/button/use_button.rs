use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;

use crate::hooks::{
    focus::use_focus_ring::{use_focus_ring, UseFocusRingInput},
    interactions::{
        use_hover::{use_hover, UseHoverInput},
        use_press::{use_press, UsePressInput},
    },
    MergedPressHoverFocusRingAttrs, MergedPressHoverFocusRingProps, UseFocusRingReturn,
    UseHoverReturn, UsePressReturn,
};
use crate::utils::aria::*;
use crate::utils::MergeWith;
use crate::hooks::IntoAttrs;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

#[derive(Debug, Clone)]
pub struct UseButtonInput {
    pub disabled: Signal<bool>,
    pub aria_haspopup: Signal<AriaHasPopup>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub use_press_input: UsePressInput,
    pub use_hover_input: UseHoverInput,
    pub use_focus_ring_input: UseFocusRingInput,
}

#[derive(Debug)]
pub struct UseButtonReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseButtonProps,
    pub is_hovered: Signal<bool>,
    pub is_pressed: Signal<bool>,
    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_button` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseButtonProps {
    pub role: &'static str,
    pub tabindex: Signal<Option<&'static str>>,
    pub disabled: Signal<bool>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<AriaHasPopup>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub other: MergedPressHoverFocusRingProps,
}

impl IntoAttrs for UseButtonProps {
    type Attrs = UseButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Role, self.role),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::AriaDisabled, self.aria_disabled),
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaExpanded, self.aria_expanded),
            ),
            self.other.into_attrs(),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseButtonAttrs = (
    (
        Attr<attr::Role, &'static str>,
        Attr<attr::Tabindex, Signal<Option<&'static str>>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
        Attr<attr::AriaHaspopup, Signal<AriaHasPopup>>,
        Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    ),
    MergedPressHoverFocusRingAttrs,
);

pub fn use_button(input: UseButtonInput) -> UseButtonReturn {
    let UseButtonInput {
        disabled,
        aria_haspopup,
        aria_expanded,
        use_press_input,
        use_hover_input,
        use_focus_ring_input,
    } = input;

    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(use_hover_input);

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(use_press_input);

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(use_focus_ring_input);

    let merged = hover_props.merge_with(press_props);
    let merged = merged.merge_with(focus_ring_props);

    UseButtonReturn {
        props: UseButtonProps {
            role: "button",
            tabindex: Signal::derive(move || if disabled.get() { None } else { Some("0") }),
            disabled: Signal::derive(move || disabled.get().into_attribute_value()),
            aria_disabled: Signal::derive(move || disabled.get().then_some(AriaDisabled::True)),
            aria_haspopup,
            aria_expanded,
            other: merged,
        },
        is_hovered,
        is_pressed,
        is_focus_visible,
    }
}
