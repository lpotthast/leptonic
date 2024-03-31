use std::rc::Rc;

use educe::Educe;
use leptos::Oco;
use leptos_reactive::{Signal, SignalGet};
use typed_builder::TypedBuilder;

use crate::{
    prelude::{AriaExpanded, AriaHasPopup},
    utils::{
        aria::{AriaAttribute, AriaControls, GenericAttribute},
        attributes::Attributes,
    },
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct UseOverlayTriggerInput {
    /// Whether the overlay is currently shown.
    #[builder(setter(into))]
    pub(crate) show: Signal<bool>,

    #[builder(setter(into))]
    pub(crate) overlay_id: Oco<'static, str>,

    /// The type of overlay opened by this trigger.
    /// Using the variants `False` or `True` will result in a runtime warning on debug builds!
    /// Prefer `AriaHasPopup::Menu` if you are unsure what to use otherwise.
    #[builder(default = AriaHasPopup::Menu, setter(into))]
    pub(crate) overlay_type: AriaHasPopup,
}

#[derive(Debug)]
pub struct UseOverlayTriggerReturn {
    /// Props for the trigger.
    pub props: UseOverlayTriggerProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayTriggerProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayTriggerOverlayProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    #[cfg(debug_assertions)]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        match input.overlay_type {
            unexpected @ AriaHasPopup::False | unexpected @ AriaHasPopup::True => {
                tracing::warn!(?unexpected, "use_overlay_trigger received unexpected AriaHasPopup variant. Do not use `False` or `True`.");
                unexpected
            }
            other => other,
        }
    }
    #[cfg(not(debug_assertions))]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        input.overlay_type
    }
    let aria_has_popup = get_overlay_type(&input);

    let overlay_id = input.overlay_id;

    let trigger_attrs = Attributes::new()
        .insert_entry(AriaAttribute::HasPopup(GenericAttribute::Static(
            aria_has_popup,
        )))
        .insert_entry(AriaAttribute::Expanded(GenericAttribute::Fn(Rc::new(
            move || AriaExpanded::from(input.show.get()),
        ))))
        .insert_entry(AriaAttribute::Controls(GenericAttribute::Fn(Rc::new(
            move || match input.show.get() {
                true => AriaControls::Id(vec![overlay_id.to_string()]),
                false => AriaControls::Undefined,
            },
        ))));

    UseOverlayTriggerReturn {
        props: UseOverlayTriggerProps {
            attrs: trigger_attrs,
        },
    }
}
