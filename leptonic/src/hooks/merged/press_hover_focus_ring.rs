use crate::hooks::{MergedPressHoverProps, UseFocusRingProps};
use crate::utils::{EventHandler, MergeWith};
use leptos::attr;
use leptos::attr::custom::CustomAttr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
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
#[derive(Debug, Clone)]
pub struct MergedPressHoverFocusRingProps {
    // From press
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub aria_describedby: Option<&'static str>,
    // From hover
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    // From focus ring
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub data_focus_visible: CustomAttr<&'static str, leptos::prelude::Signal<Option<&'static str>>>,
}

/// Attribute tuple type for [`MergedPressHoverFocusRingProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedPressHoverFocusRingAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    Attr<attr::AriaDescribedby, Option<&'static str>>,
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, leptos::prelude::Signal<Option<&'static str>>>,
);

impl MergedPressHoverFocusRingProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> MergedPressHoverFocusRingAttrs {
        // Cloning self is an equal performance cost to cloning all fields individually.
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedPressHoverFocusRingAttrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_dragstart.into_on(ev::dragstart),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.data_focus_visible,
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
            aria_describedby: self.aria_describedby,
            on_pointerenter: self.on_pointerenter,
            on_pointerleave: self.on_pointerleave,
            // From focus ring (distinct)
            on_focus: other.on_focus,
            on_blur: other.on_blur,
            data_focus_visible: other.data_focus_visible,
        }
    }
}
