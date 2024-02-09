use educe::Educe;
use leptos::{html::ElementDescriptor, Attribute, IntoAttribute, MaybeSignal, NodeRef, Oco};

use crate::{
    utils::{aria::*, props::Attributes},
    MaybeSignalExt,
};

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseButtonProps<E: ElementDescriptor + 'static> {
    #[educe(Debug(ignore))]
    pub node_ref: NodeRef<E>,
    pub disabled: MaybeSignal<bool>,
    pub aria_haspopup: MaybeSignal<AriaHasPopup>,
    pub aria_expanded: MaybeSignal<AriaExpanded>,
}

#[derive(Debug, Clone)]
pub struct UseButtonReturn {
    /// Spread these props onto your button using the spread syntax: `<button {..props}>...`
    pub props: Attributes,
}

pub fn use_button<E: ElementDescriptor + 'static>(
    initial_props: UseButtonProps<E>,
) -> UseButtonReturn {
    let mut props = Attributes::new();
    props.insert("role", Attribute::String(Oco::Borrowed("button")));
    props.insert(
        "tabindex",
        initial_props
            .disabled
            .map(|it| match it {
                true => Attribute::Option(None),
                false => Attribute::String(Oco::Borrowed("0")),
            })
            .into_attribute(),
    );
    props.insert("disabled", initial_props.disabled.into_attribute());
    props.insert(
        "aria-disabled",
        initial_props
            .disabled
            .map(|it| match it {
                true => "true",
                false => "false",
            })
            .into_attribute(),
    );
    props.insert(
        "aria-haspopup",
        initial_props.aria_haspopup.into_attribute(),
    );

    // From https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded
    // A button that opens a widget should have aria-controls set to the id of the expandable widget and aria-expanded set to the current state of the widget.

    props.insert(
        "aria-expanded",
        initial_props.aria_expanded.into_attribute(),
    );
    //props.insert(
    //    "aria-controls",
    //    initial_props.aria_controls.into_attribute(),
    //);
    //props.insert(
    //    "aria-pressed",
    //    initial_props.aria_pressed.into_attribute(),
    //);
    UseButtonReturn { props }
}
