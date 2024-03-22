use educe::Educe;
use leptos::{IntoAttribute, Oco};
use leptos_reactive::{create_signal, MaybeSignal, ReadSignal, WriteSignal};

use crate::utils::attributes::Attributes;

#[derive(Debug, Clone, Copy)]
pub struct UseOverlayInput {
    /// Disables the handling overlay events when true.
    pub disabled: MaybeSignal<bool>,
}

#[derive(Debug)]
pub struct UseOverlayReturn {
    pub props: UseOverlayProps,

    pub id: Oco<'static, str>,

    /// Whether the overlay should be shown.
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
}

#[derive(Educe)]
#[educe(Debug, Clone)]
pub struct UseOverlayProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

pub fn use_overlay(input: UseOverlayInput) -> UseOverlayReturn {
    let (state, set_state) = create_signal(false);
    let id = uuid::Uuid::new_v4();

    let attrs = Attributes::new().insert("id", id.to_string().into_attribute());

    UseOverlayReturn {
        props: UseOverlayProps { attrs },
        id: Oco::Owned(id.to_string()),
        state,
        set_state,
    }
}
