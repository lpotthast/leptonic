use educe::Educe;
use leptos_reactive::{MaybeSignal, Oco};
use leptos::{html::ElementDescriptor, Attribute, IntoAttribute, NodeRef};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::{aria::*, props::Attributes, signals::MaybeSignalExt};

use super::{
    focus::{use_focus, UseFocusInput},
    prelude::{use_hover, use_press, UseHoverInput, UsePressInput},
};

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseButtonInput<E: ElementDescriptor + 'static> {
    #[educe(Debug(ignore))]
    pub node_ref: NodeRef<E>,
    pub disabled: MaybeSignal<bool>,
    pub aria_haspopup: MaybeSignal<AriaHasPopup>,
    pub aria_expanded: MaybeSignal<AriaExpanded>,

    pub use_press_input: UsePressInput,
    pub use_hover_input: UseHoverInput,
    pub use_focus_input: UseFocusInput,
}

#[derive(Debug)]
pub struct UseButtonReturn {
    /// Spread these props onto your button using the spread syntax: `<button {..props}>...`
    pub props: UseButtonProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseButtonProps {
    pub attrs: Attributes,

    /// This handler must be attached to the target element: `<foo on:keydown=on_key_down />`
    #[educe(Debug(ignore))]
    pub on_key_down: Box<dyn Fn(KeyboardEvent)>,
    /// This handler must be attached to the target element: `<foo on:click=on_click />`
    #[educe(Debug(ignore))]
    pub on_click: Box<dyn Fn(MouseEvent)>,
    /// This handler must be attached to the target element: `<foo on:pointerdown=on_pointer_down />`
    #[educe(Debug(ignore))]
    pub on_pointer_down: Box<dyn Fn(PointerEvent)>,
    /// This handler must be attached to the target element: `<foo on:pointerdown=on_pointer_down />`
    #[educe(Debug(ignore))]
    pub on_pointer_enter: Box<dyn Fn(PointerEvent)>,
    /// This handler must be attached to the target element: `<foo on:pointerdown=on_pointer_down />`
    #[educe(Debug(ignore))]
    pub on_pointer_leave: Box<dyn Fn(PointerEvent)>,
    /// This handler must be attached to the target element: `<foo on:focus=on_focus />`
    #[educe(Debug(ignore))]
    pub on_focus: Box<dyn Fn(FocusEvent)>,
    /// This handler must be attached to the target element: `<foo on:blur=on_blur />`
    #[educe(Debug(ignore))]
    pub on_blur: Box<dyn Fn(FocusEvent)>,
}

pub fn use_button<E: ElementDescriptor + 'static>(input: UseButtonInput<E>) -> UseButtonReturn {
    let press = use_press(input.use_press_input);

    let hover = use_hover(input.use_hover_input);

    let focus = use_focus(input.use_focus_input);

    let mut attrs = Attributes::new();
    attrs.insert("role", Attribute::String(Oco::Borrowed("button")));
    attrs.insert(
        "tabindex",
        input
            .disabled
            .map(|it| match it {
                true => Attribute::Option(None),
                false => Attribute::String(Oco::Borrowed("0")),
            })
            .into_attribute(),
    );
    attrs.insert("disabled", input.disabled.into_attribute());
    attrs.insert(
        "aria-disabled",
        input
            .disabled
            .map(|it| match it {
                true => "true",
                false => "false",
            })
            .into_attribute(),
    );
    attrs.insert("aria-haspopup", input.aria_haspopup.into_attribute());

    // From https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded
    // A button that opens a widget should have aria-controls set to the id of the expandable widget and aria-expanded set to the current state of the widget.

    attrs.insert("aria-expanded", input.aria_expanded.into_attribute());
    //props.insert(
    //    "aria-controls",
    //    initial_props.aria_controls.into_attribute(),
    //);
    //props.insert(
    //    "aria-pressed",
    //    initial_props.aria_pressed.into_attribute(),
    //);

    // Merge attributes
    attrs.merge(press.props.attrs);
    attrs.merge(focus.props.attrs);

    // Merge event handlers
    let on_key_down = press.props.on_key_down;
    let on_click = press.props.on_click;
    let on_pointer_down = press.props.on_pointer_down;
    let on_pointer_enter = hover.props.on_pointer_enter;
    let on_pointer_leave = hover.props.on_pointer_leave;
    let on_focus = focus.props.on_focus;
    let on_blur = focus.props.on_blur;

    UseButtonReturn {
        props: UseButtonProps {
            attrs,
            on_key_down,
            on_click,
            on_pointer_down,
            on_pointer_enter,
            on_pointer_leave,
            on_focus,
            on_blur,
        },
    }
}
