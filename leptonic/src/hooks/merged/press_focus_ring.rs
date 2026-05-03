use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::{
    hooks::{IntoAttrs, UseFocusRingProps, UsePressProps},
    utils::{EventHandler, MergeWith, aria::AriaDescribedby},
};

/// Combined props from `use_press` and `use_focus_ring` hooks (without hover).
///
/// This type includes:
/// - Press event handlers: `on_keydown`, `on_click`, `on_pointerdown`
/// - Focus ring event handlers: `on_focus`, `on_blur`
/// - Focus ring data attribute: `data_focus_visible`
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::MergeWith;
///
/// let press = use_press(press_input);
/// let focus_ring = use_focus_ring(focus_ring_input);
///
/// let combined = press.props.merge_with(focus_ring.props);
///
/// view! {
///     <button {..combined.into_attrs()}>
///         "Click me"
///     </button>
/// }
/// ```
#[derive(Debug)]
pub struct MergedPressFocusRingProps {
    // From press.
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub aria_describedby: Option<AriaDescribedby>,
    // From focus ring.
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

/// Attribute tuple type for [`MergedPressFocusRingProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedPressFocusRingAttrs = (
    // From press.
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    Attr<attr::AriaDescribedby, Option<AriaDescribedby>>,
    // From focus ring.
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

impl IntoAttrs for MergedPressFocusRingProps {
    type Attrs = MergedPressFocusRingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_pointerup.into_on(ev::pointerup),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

impl MergeWith<UseFocusRingProps> for UsePressProps {
    type Output = MergedPressFocusRingProps;

    fn merge_with(self, focus_ring: UseFocusRingProps) -> Self::Output {
        let press = self;
        MergedPressFocusRingProps {
            // From press.
            on_keydown: press.on_keydown,
            on_click: press.on_click,
            on_pointerdown: press.on_pointerdown,
            on_dragstart: press.on_dragstart,
            on_mousedown: press.on_mousedown,
            on_pointerup: press.on_pointerup,
            aria_describedby: press.aria_describedby,
            // From focus ring.
            on_focus: focus_ring.on_focus,
            on_blur: focus_ring.on_blur,
            on_focusin: focus_ring.on_focusin,
            on_focusout: focus_ring.on_focusout,
            data_focus_visible: focus_ring.data_focus_visible,
        }
    }
}

impl MergeWith<UsePressProps> for UseFocusRingProps {
    type Output = MergedPressFocusRingProps;

    fn merge_with(self, other: UsePressProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        other.merge_with(self)
    }
}
