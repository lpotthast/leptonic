use crate::hooks::{MergedPressHoverProps, UseFocusRingProps};
use crate::utils::aria::AriaDescribedby;
use crate::utils::style::TouchActionStyle;
use crate::utils::{EventHandler, MergeWith};
use crate::hooks::IntoAttrs;
use leptos::attr;
use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::tachys::html::style::Style;
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

/// Combined props from `use_press`, `use_hover`, and `use_focus_ring` hooks.
///
/// This type includes:
/// - Press event handlers: `on_keydown`, `on_click`, `on_pointerdown`
/// - Hover event handlers: `on_pointerenter`, `on_pointerleave`
/// - Focus ring event handlers: `on_focus`, `on_blur`
/// - Focus ring data attribute: `data_focus_visible`
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::MergeWith;
///
/// let press = use_press(press_input);
/// let hover = use_hover(hover_input);
/// let focus_ring = use_focus_ring(focus_ring_input);
///
/// let combined = press.props
///     .merge_with(hover.props)
///     .merge_with(focus_ring.props);
///
/// view! {
///     <button {..combined.into_attrs()}>
///         "Interactive button"
///     </button>
/// }
/// ```
#[derive(Debug)]
pub struct MergedPressHoverFocusRingProps {
    // From press
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub aria_describedby: Option<AriaDescribedby>,
    // From hover
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    // From focus ring
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

/// Attribute tuple type for [`MergedPressHoverFocusRingProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedPressHoverFocusRingAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    Style<(TouchActionStyle, &'static str)>,
    Attr<attr::AriaDescribedby, Option<AriaDescribedby>>,
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

impl IntoAttrs for MergedPressHoverFocusRingProps {
    type Attrs = MergedPressHoverFocusRingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_pointerup.into_on(ev::pointerup),
            TouchActionStyle::with_value("pan-x pan-y pinch-zoom"),
            Attr(attr::AriaDescribedby, self.aria_describedby),
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

impl MergeWith<UseFocusRingProps> for MergedPressHoverProps {
    type Output = MergedPressHoverFocusRingProps;

    fn merge_with(self, other: UseFocusRingProps) -> Self::Output {
        MergedPressHoverFocusRingProps {
            // From press+hover (all distinct)
            on_keydown: self.on_keydown,
            on_click: self.on_click,
            on_pointerdown: self.on_pointerdown,
            on_dragstart: self.on_dragstart,
            on_mousedown: self.on_mousedown,
            on_pointerup: self.on_pointerup,
            aria_describedby: self.aria_describedby,
            on_pointerenter: self.on_pointerenter,
            on_pointerleave: self.on_pointerleave,
            // From focus ring (distinct)
            on_focus: other.on_focus,
            on_blur: other.on_blur,
            on_focusin: other.on_focusin,
            on_focusout: other.on_focusout,
            data_focus_visible: other.data_focus_visible,
        }
    }
}
