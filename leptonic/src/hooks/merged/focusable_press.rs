use crate::hooks::{UseFocusableProps, UsePressProps};
use crate::utils::aria::AriaDescribedby;
use crate::utils::style::TouchActionStyle;
use crate::utils::{ElementCaptureAttr, EventHandler, MergeWith};
use crate::hooks::IntoAttrs;
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::tachys::html::style::Style;
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

/// Combined props from `use_focusable` and `use_press` hooks.
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::MergeWith;
///
/// let focusable = use_focusable(focusable_input);
/// let press = use_press(press_input);
/// let combined = focusable.props.merge_with(press.props);
///
/// view! {
///     <button {..combined.into_attrs()}>
///         "Click me"
///     </button>
/// }
/// ```
#[derive(Debug)]
pub struct MergedFocusablePressProps {
    pub tabindex: Signal<Option<i32>>,
    pub aria_describedby: Option<AriaDescribedby>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
}

/// Attribute tuple type for [`MergedFocusablePressProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedFocusablePressAttrs = (
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    Attr<attr::AriaDescribedby, Option<AriaDescribedby>>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    Style<(TouchActionStyle, &'static str)>,
);

impl IntoAttrs for MergedFocusablePressProps {
    type Attrs = MergedFocusablePressAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_dragstart.into_on(ev::dragstart),
            TouchActionStyle::with_value("pan-x pan-y pinch-zoom"),
        )
    }
}

impl MergeWith<UsePressProps> for UseFocusableProps {
    type Output = MergedFocusablePressProps;

    fn merge_with(self, press: UsePressProps) -> Self::Output {
        let UseFocusableProps {
            tabindex: focusable_tabindex,
            on_focus: focusable_on_focus,
            on_blur: focusable_on_blur,
            on_keydown: focusable_on_keydown,
            on_keyup: focusable_on_keyup,
            element_capture: focusable_element_capture,
        } = self;

        let UsePressProps {
            on_keydown: press_on_keydown,
            on_click: press_on_click,
            on_pointerdown: press_on_pointerdown,
            on_mousedown: press_on_mousedown,
            on_pointerup: press_on_pointerup,
            on_dragstart: press_on_dragstart,
            on_dblclick: press_on_dblclick,
            aria_describedby: press_aria_describedby,
        } = press;

        MergedFocusablePressProps {
            tabindex: focusable_tabindex,
            aria_describedby: press_aria_describedby,
            element_capture: focusable_element_capture,
            on_keydown: focusable_on_keydown.chain(press_on_keydown),
            on_keyup: focusable_on_keyup,
            on_focus: focusable_on_focus,
            on_blur: focusable_on_blur,
            on_click: press_on_click,
            on_dblclick: press_on_dblclick,
            on_pointerdown: press_on_pointerdown,
            on_mousedown: press_on_mousedown,
            on_pointerup: press_on_pointerup,
            on_dragstart: press_on_dragstart,
        }
    }
}

impl MergeWith<UseFocusableProps> for UsePressProps {
    type Output = MergedFocusablePressProps;

    fn merge_with(self, focusable: UseFocusableProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        focusable.merge_with(self)
    }
}
