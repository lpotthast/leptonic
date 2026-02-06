use crate::hooks::{UseFocusRingProps, UseHoverProps};
use crate::utils::{EventHandler, MergeWith};
use leptos::attr::custom::CustomAttr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use web_sys::{FocusEvent, PointerEvent};

/// Combined props from `use_hover` and `use_focus_ring` hooks (without press).
///
/// This type includes:
/// - Hover event handlers: `on_pointerenter`, `on_pointerleave`
/// - Focus ring event handlers: `on_focus`, `on_blur`
/// - Focus ring data attribute: `data_focus_visible`
#[derive(Debug, Clone)]
pub struct MergedHoverFocusRingProps {
    // From hover.
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    // From focus ring.
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub data_focus_visible: CustomAttr<&'static str, leptos::prelude::Signal<Option<&'static str>>>,
}

/// Attribute tuple type for [`MergedHoverFocusRingProps`].
pub type MergedHoverFocusRingAttrs = (
    // From hover.
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    // From focus ring.
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, leptos::prelude::Signal<Option<&'static str>>>,
);

impl MergedHoverFocusRingProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> MergedHoverFocusRingAttrs {
        // Cloning self is an equal performance cost to cloning all fields individually.
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedHoverFocusRingAttrs {
        (
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.data_focus_visible,
        )
    }
}

impl MergeWith<UseFocusRingProps> for UseHoverProps {
    type Output = MergedHoverFocusRingProps;

    fn merge_with(self, focus_ring: UseFocusRingProps) -> Self::Output {
        let hover = self;
        MergedHoverFocusRingProps {
            on_pointerenter: hover.on_pointerenter,
            on_pointerleave: hover.on_pointerleave,
            on_focus: focus_ring.on_focus,
            on_blur: focus_ring.on_blur,
            data_focus_visible: focus_ring.data_focus_visible,
        }
    }
}

// Reverse: UseFocusRingProps + UseHoverProps
impl MergeWith<UseHoverProps> for UseFocusRingProps {
    type Output = MergedHoverFocusRingProps;

    fn merge_with(self, other: UseHoverProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        other.merge_with(self)
    }
}
