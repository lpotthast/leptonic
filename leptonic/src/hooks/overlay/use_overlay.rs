use educe::Educe;
use leptos::attr;
use leptos::attr::Attr;
use leptos::oco::Oco;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct UseOverlayInput {
    /// Disables the handling overlay events when true.
    pub disabled: Signal<bool>,
}

#[derive(Debug)]
pub struct UseOverlayReturn {
    pub attrs: UseOverlayAttrs,

    pub id: Oco<'static, str>,

    /// Whether the overlay should be shown.
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseOverlayAttrs = (
    Attr<attr::Id, String>,
);

pub fn use_overlay(input: UseOverlayInput) -> UseOverlayReturn {
    let (state, set_state) = signal(false);
    
    let id = uuid::Uuid::new_v4();

    UseOverlayReturn {
        attrs: (
            Attr(attr::Id, id.to_string()),
        ),
        id: Oco::Owned(id.to_string()),
        state,
        set_state,
    }
}
