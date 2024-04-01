use educe::Educe;
use leptos::{html::ElementDescriptor, Attribute, IntoAttribute, NodeRef};
use leptos_reactive::{MaybeSignal, Oco};
use typed_builder::TypedBuilder;

use crate::utils::{
    aria::*, attributes::Attributes, event_handlers::EventHandlers, signals::MaybeSignalExt,
};

use super::{
    focus::use_focus::{use_focus, UseFocusInput},
    interactions::{
        use_hover::{use_hover, UseHoverInput},
        use_press::{use_press, UsePressInput},
    },
};

#[derive(Clone, Copy, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseButtonInput<E: ElementDescriptor + 'static> {
    #[educe(Debug(ignore))]
    pub(crate) node_ref: NodeRef<E>,

    /// Whether this button is disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    #[builder(default = AriaHasPopup::default().into(), setter(into))]
    pub(crate) aria_haspopup: MaybeSignal<AriaHasPopup>,

    #[builder(default = AriaExpanded::default().into(), setter(into))]
    pub(crate) aria_expanded: MaybeSignal<AriaExpanded>,

    /// Press behavior of this button.
    pub(crate) use_press_input: UsePressInput,

    /// Optional hover behavior. Omit this if you are not interested in programmatically handling hover events.
    #[builder(default = None, setter(strip_option))]
    pub(crate) use_hover_input: Option<UseHoverInput>,

    /// Optional focus behavior. Omit this if you are not interested in programmatically handling focus events.
    #[builder(default = None,setter(strip_option))]
    pub(crate) use_focus_input: Option<UseFocusInput>,
}

#[derive(Debug)]
pub struct UseButtonReturn {
    /// Spread these props onto your button using the spread syntax: `<button {..props}>...`
    pub props: UseButtonProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseButtonProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

pub fn use_button<E: ElementDescriptor + 'static>(input: UseButtonInput<E>) -> UseButtonReturn {
    let press = use_press(input.use_press_input);

    let focus = use_focus(input.use_focus_input.unwrap_or_else(|| {
        UseFocusInput::builder()
            .disabled(input.disabled.clone())
            .build()
    }));

    let hover = input.use_hover_input.map(use_hover);

    let attrs = Attributes::new()
        .insert("role", Attribute::String(Oco::Borrowed("button")))
        .insert(
            "tabindex",
            input
                .disabled
                .map(|it| match it {
                    true => Attribute::Option(None),
                    false => Attribute::String(Oco::Borrowed("0")),
                })
                .into_attribute(),
        )
        .insert("disabled", input.disabled.into_attribute())
        .insert(
            "aria-disabled",
            input
                .disabled
                .map(|it| match it {
                    true => "true",
                    false => "false",
                })
                .into_attribute(),
        )
        .insert("aria-haspopup", input.aria_haspopup.into_attribute())
        // From https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded
        // A button that opens a widget should have aria-controls set to the id of the expandable widget and aria-expanded set to the current state of the widget.
        .insert("aria-expanded", input.aria_expanded.into_attribute())
        //props.insert(
        //    "aria-controls",
        //    initial_props.aria_controls.into_attribute(),
        //);
        //props.insert(
        //    "aria-pressed",
        //    initial_props.aria_pressed.into_attribute(),
        //);
        // Merge attributes
        .merge(press.props.attrs)
        .merge(focus.props.attrs);

    UseButtonReturn {
        props: UseButtonProps {
            attrs,
            handlers: EventHandlers::builder()
                .build()
                .merge(press.props.handlers)
                .merge(focus.props.handlers)
                .merge_opt(hover.map(|it| it.props.handlers)),
        },
    }
}
