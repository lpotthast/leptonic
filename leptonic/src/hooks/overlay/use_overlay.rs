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
    /// Props for the overlay element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseOverlayProps,

    pub id: Oco<'static, str>,

    /// Whether the overlay should be shown.
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
}

/// Props from `use_overlay` that can be converted to spreadable attributes.
#[derive(Debug, Clone)]
pub struct UseOverlayProps {
    pub id: String,
}

impl UseOverlayProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseOverlayAttrs {
        (Attr(attr::Id, self.id.clone()),)
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseOverlayAttrs {
        (Attr(attr::Id, self.id),)
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseOverlayAttrs = (Attr<attr::Id, String>,);

pub fn use_overlay(_input: UseOverlayInput) -> UseOverlayReturn {
    let (state, set_state) = signal(false);

    let id = uuid::Uuid::new_v4();

    UseOverlayReturn {
        props: UseOverlayProps { id: id.to_string() },
        id: Oco::Owned(id.to_string()),
        state,
        set_state,
    }
}
