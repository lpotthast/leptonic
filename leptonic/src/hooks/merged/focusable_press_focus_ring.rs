use crate::hooks::{MergedFocusablePressProps, UseFocusRingProps};
use crate::utils::{ElementCaptureAttr, EventHandler, MergeWith};
use leptos::attr;
use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

/// Combined props from `use_focusable`, `use_press` and `use_focus_ring` hooks.
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
#[derive(Debug, Clone)]
pub struct MergedFocusablePressFocusRingProps {
    pub tabindex: Signal<Option<i32>>,
    pub aria_describedby: Option<&'static str>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
}

/// Attribute tuple type for [`MergedFocusablePressFocusRingProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedFocusablePressFocusRingAttrs = (
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
);

impl MergedFocusablePressFocusRingProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> MergedFocusablePressFocusRingAttrs {
        // Cloning self is an equal performance cost to cloning all fields individually.
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedFocusablePressFocusRingAttrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
        )
    }
}

impl MergeWith<UseFocusRingProps> for MergedFocusablePressProps {
    type Output = MergedFocusablePressFocusRingProps;

    fn merge_with(self, focus_ring: UseFocusRingProps) -> Self::Output {
        let MergedFocusablePressProps {
            tabindex: merged_tabindex,
            aria_describedby: merged_aria_describedby,
            element_capture: merged_element_capture,
            on_keydown: merged_on_keydown,
            on_keyup: merged_on_keyup,
            on_focus: merged_on_focus,
            on_blur: merged_on_blur,
            on_click: merged_on_click,
            on_dblclick: merged_on_dblclick,
            on_pointerdown: merged_on_pointerdown,
            on_dragstart: merged_on_dragstart,
        } = self;

        let UseFocusRingProps {
            on_focus: focus_ring_on_focus,
            on_blur: focus_ring_on_blur,
            on_focusin: focus_ring_on_focusin,
            on_focusout: focus_ring_on_focusout,
            data_focus_visible: focus_ring_data_focus_visible,
        } = focus_ring;

        MergedFocusablePressFocusRingProps {
            tabindex: merged_tabindex,
            aria_describedby: merged_aria_describedby,
            data_focus_visible: focus_ring_data_focus_visible,
            element_capture: merged_element_capture,
            on_keydown: merged_on_keydown,
            on_keyup: merged_on_keyup,
            on_focus: merged_on_focus.chain(focus_ring_on_focus),
            on_blur: merged_on_blur.chain(focus_ring_on_blur),
            on_focusin: focus_ring_on_focusin,
            on_focusout: focus_ring_on_focusout,
            on_click: merged_on_click,
            on_dblclick: merged_on_dblclick,
            on_pointerdown: merged_on_pointerdown,
            on_dragstart: merged_on_dragstart,
        }
    }
}

impl MergeWith<MergedFocusablePressProps> for UseFocusRingProps {
    type Output = MergedFocusablePressFocusRingProps;

    fn merge_with(self, focusable_press: MergedFocusablePressProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        focusable_press.merge_with(self)
    }
}
