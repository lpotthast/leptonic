use educe::Educe;
use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::{attr, ev};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::aria::*;
use crate::utils::EventHandler;

use super::{
    focus::use_focus_ring::{use_focus_ring, UseFocusRingInput},
    interactions::{
        use_hover::{use_hover, UseHoverInput},
        use_press::{use_press, UsePressInput},
    },
    UseFocusRingReturn, UseHoverReturn, UsePressReturn,
};

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseButtonInput {
    pub disabled: Signal<bool>,
    pub aria_haspopup: Signal<AriaHasPopup>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,

    pub use_press_input: UsePressInput,
    pub use_hover_input: UseHoverInput,
    pub use_focus_ring_input: UseFocusRingInput,
}

#[derive(Debug, Clone)]
pub struct UseButtonReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseButtonProps,
    pub is_hovered: Signal<bool>,
    pub is_pressed: Signal<bool>,
    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_button` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseButtonProps {
    pub role: &'static str,
    pub tabindex: Signal<Option<&'static str>>,
    pub disabled: Signal<bool>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<AriaHasPopup>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_describedby: Option<&'static str>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl UseButtonProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseButtonAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseButtonAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseButtonAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<Option<&'static str>>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaHaspopup, Signal<AriaHasPopup>>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

pub fn use_button(input: UseButtonInput) -> UseButtonReturn {
    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(input.use_hover_input);

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(input.use_press_input);

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(input.use_focus_ring_input);

    // From https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded
    // A button that opens a widget should have aria-controls set to the id of the expandable widget and aria-expanded set to the current state of the widget.

    //props.insert(
    //    attr::AriaControls,
    //    initial_props.aria_controls.into_attribute(),
    //);
    //props.insert(
    //    attr::ArioPressed,
    //    initial_props.aria_pressed.into_attribute(),
    //);

    UseButtonReturn {
        props: UseButtonProps {
            role: "button",
            tabindex: Signal::derive(move || {
                if input.disabled.get() {
                    None
                } else {
                    Some("0")
                }
            }),
            disabled: Signal::derive(move || input.disabled.get().into_attribute_value()),
            aria_disabled: Signal::derive(move || {
                input.disabled.get().then_some(AriaDisabled::True)
            }),
            aria_haspopup: input.aria_haspopup,
            aria_expanded: input.aria_expanded,
            aria_describedby: press_props.aria_describedby,
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_keydown: press_props.on_keydown,
            on_click: press_props.on_click,
            on_pointerdown: press_props.on_pointerdown,
            on_dragstart: press_props.on_dragstart,
            on_pointerenter: hover_props.on_pointerenter,
            on_pointerleave: hover_props.on_pointerleave,
            on_focus: focus_ring_props.handle_focus,
            on_blur: focus_ring_props.handle_blur,
            on_focusin: focus_ring_props.handle_focusin,
            on_focusout: focus_ring_props.handle_focusout,
        },
        is_hovered,
        is_pressed,
        is_focus_visible,
    }
}
