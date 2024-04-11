use educe::Educe;
use leptos::on_cleanup;
use leptos_reactive::{MaybeSignal, SignalGetUntracked};
use leptos_use::{
    core::ElementMaybeSignal, use_event_listener_with_options, UseEventListenerOptions,
};
use typed_builder::TypedBuilder;

use crate::{
    prelude::AriaHasPopup,
    state::menu::MenuTriggerState,
    utils::{attributes::Attributes, event_handlers::EventHandlers},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenuTrigger.ts

#[derive(Debug, Clone, Copy)]
pub enum MenuTriggerType {
    Press,
    LongPress,
}

#[derive(Clone, Copy, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseMenuTriggerInput {
    /// Wether the trigger should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// How the menu is supposed to be triggered.
    pub trigger: MenuTriggerType,

    /// The type of overlay opened by this trigger.
    /// Using the variants `False` or `True` will result in a runtime warning on debug builds!
    /// Prefer `AriaHasPopup::Menu` if you are unsure what to use otherwise.
    #[builder(default = AriaHasPopup::Menu, setter(into))]
    pub(crate) overlay_type: AriaHasPopup, // TODO: Should we only support "menu" and "listbox" instead?
}

#[derive(Debug)]
pub struct UseMenuTriggerProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseMenuTriggerReturn {
    pub props: UseMenuTriggerProps,
}

pub fn use_menu_trigger<El, T>(
    el: El,
    state: MenuTriggerState,
    input: UseMenuTriggerInput,
) -> UseMenuTriggerReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>> + 'static,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    todo!();

    UseMenuTriggerReturn {
        props: UseMenuTriggerProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder().build(),
        },
    }
}
