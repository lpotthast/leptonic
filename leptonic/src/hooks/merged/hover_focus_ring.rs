use crate::hooks::{UseFocusRingProps, UseHoverProps};
use crate::utils::{EventHandler, MergeWith};
use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, PointerEvent};

/// Combined props from `use_hover` and `use_focus_ring` hooks (without press).
///
/// This type includes:
/// - Hover event handlers: `on_pointerenter`, `on_pointerleave`
/// - Focus ring event handlers: `on_focus`, `on_blur`
/// - Focus ring data attribute: `data_focus_visible`
#[derive(Debug)]
pub struct MergedHoverFocusRingProps {
    // From hover.
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    // From focus ring.
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

/// Attribute tuple type for [`MergedHoverFocusRingProps`].
pub type MergedHoverFocusRingAttrs = (
    // From hover.
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    // From focus ring.
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, leptos::prelude::Signal<Option<&'static str>>>,
);

impl MergedHoverFocusRingProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedHoverFocusRingAttrs {
        (
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
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
            on_focusin: focus_ring.on_focusin,
            on_focusout: focus_ring.on_focusout,
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
