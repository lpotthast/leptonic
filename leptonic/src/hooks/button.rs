use std::fmt::{Debug, Formatter};
use educe::Educe;
use leptos::ev::On;
use leptos::html::ElementType;
use leptos::prelude::*;
use leptos::{attr, ev};
use leptos::attr::Attr;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::aria::*;

use super::{focus::use_focus::{use_focus, UseFocusInput}, interactions::{
    use_hover::{use_hover, UseHoverInput},
    use_press::{use_press, UsePressInput},
}, UseFocusReturn, UseHoverReturn, UsePressReturn};

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseButtonInput<E: ElementType + 'static> {
    #[educe(Debug(ignore))]
    pub node_ref: NodeRef<E>,
    pub disabled: Signal<bool>,
    pub aria_haspopup: Signal<AriaHasPopup>,
    pub aria_expanded: Signal<AriaExpanded>,

    pub use_press_input: UsePressInput,
    pub use_hover_input: UseHoverInput,
    pub use_focus_input: UseFocusInput,
}

pub struct UseButtonReturn {
    /// Spread these props onto your button using the spread syntax: `<button {..props}>...`
    pub attrs: UseButtonAttrs,
    pub is_hovered: Signal<bool>,
    pub is_pressed: Signal<bool>,
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseButtonAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<Option<&'static str>>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::AriaHaspopup, Signal<&'static str>>,
    Attr<attr::AriaExpanded, Signal<&'static str>>,
    On<ev::keydown, Box<dyn Fn(KeyboardEvent) + Send + Sync + 'static>>,
    On<ev::click, Box<dyn Fn(MouseEvent) + Send + Sync + 'static>>,
    On<ev::pointerdown, Box<dyn Fn(PointerEvent) + Send + Sync + 'static>>,
    On<ev::pointerenter, Box<dyn Fn(PointerEvent) + Send + Sync + 'static>>,
    On<ev::pointerleave, Box<dyn Fn(PointerEvent) + Send + Sync + 'static>>,
    On<ev::focus, Box<dyn Fn(FocusEvent) + Send + Sync + 'static>>,
    On<ev::blur, Box<dyn Fn(FocusEvent) + Send + Sync + 'static>>,
);

impl Debug for UseButtonReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseButtonReturn")
            .field("attrs", &"...")
            .field("is_hovered", &self.is_hovered)
            .field("is_pressed", &self.is_pressed)
            .finish()
    }
}

pub fn use_button<E: ElementType + 'static>(input: UseButtonInput<E>) -> UseButtonReturn {
    let UseHoverReturn { attrs: (on_pointerenter, on_pointerleave), is_hovered } = use_hover(input.use_hover_input);

    let UsePressReturn { attrs: (on_keydown, on_click, on_pointerdown), is_pressed, } = use_press(input.use_press_input);

    let UseFocusReturn { attrs: (on_focus, on_blur) } = use_focus(input.use_focus_input);

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
        attrs: (
            Attr(attr::Role, "button"),
            Attr(attr::Tabindex, Signal::derive(move || match input
                .disabled
                .get() {
                true => None,
                false => Some("0"),
            })),
            Attr(attr::Disabled, Signal::derive(move || input.disabled.get().into_attribute_value())),
            Attr(attr::AriaDisabled, Signal::derive(move || match input
                .disabled
                .get() {
                true => "true",
                false => "false",
            })),
            Attr(attr::AriaHaspopup, Signal::derive(move || input.aria_haspopup.get().into_attribute_value())),
            Attr(attr::AriaExpanded, Signal::derive(move || input.aria_expanded.get().into_attribute_value())),
            on_keydown,
            on_click,
            on_pointerdown,
            on_pointerenter,
            on_pointerleave,
            on_focus,
            on_blur,
        ),
        is_hovered,
        is_pressed,
    }
}
