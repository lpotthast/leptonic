use crate::hooks::{UseHoverProps, UsePressProps};
use crate::utils::aria::AriaDescribedby;
use crate::utils::style::TouchActionStyle;
use crate::utils::{EventHandler, MergeWith};
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::tachys::html::style::Style;
use web_sys::{DragEvent, KeyboardEvent, MouseEvent, PointerEvent};

/// Combined props from `use_press` and `use_hover` hooks.
///
/// This type includes:
/// - Press event handlers: `on_keydown`, `on_click`, `on_pointerdown`
/// - Hover event handlers: `on_pointerenter`, `on_pointerleave`
///
/// All fields are distinct between the two hooks, so no chaining is needed.
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::MergeWith;
///
/// let press = use_press(press_input);
/// let hover = use_hover(hover_input);
/// let combined = press.props.merge_with(hover.props);
///
/// view! {
///     <button {..combined.into_attrs()}>
///         "Hover and click me"
///     </button>
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MergedPressHoverProps {
    // From press.
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub aria_describedby: Option<AriaDescribedby>,
    // From hover.
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

/// Attribute tuple type for [`MergedPressHoverProps`].
///
/// Spread this onto elements: `<button {..attrs}/>`
pub type MergedPressHoverAttrs = (
    // From press.
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    Style<(TouchActionStyle, &'static str)>,
    Attr<attr::AriaDescribedby, Option<AriaDescribedby>>,
    // From hover.
    On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
);

impl MergedPressHoverProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> MergedPressHoverAttrs {
        // Cloning self is an equal performance cost to cloning all fields individually.
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> MergedPressHoverAttrs {
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
        )
    }
}

impl MergeWith<UseHoverProps> for UsePressProps {
    type Output = MergedPressHoverProps;

    fn merge_with(self, hover: UseHoverProps) -> Self::Output {
        let UsePressProps {
            on_keydown: press_on_keydown,
            on_click: press_on_click,
            on_pointerdown: press_on_pointerdown,
            on_mousedown: press_on_mousedown,
            on_pointerup: press_on_pointerup,
            on_dragstart: press_on_dragstart,
            on_dblclick: press_on_dblclick,
            aria_describedby: press_aria_describedby,
        } = self;

        let UseHoverProps {
            on_pointerenter: hover_on_pointerenter,
            on_pointerleave: hover_on_pointerleave,
        } = hover;

        MergedPressHoverProps {
            // From press (distinct).
            on_keydown: press_on_keydown,
            on_click: press_on_click,
            on_pointerdown: press_on_pointerdown,
            on_pointerup: press_on_pointerup,
            on_mousedown: press_on_mousedown,
            on_dragstart: press_on_dragstart,
            on_dblclick: press_on_dblclick,
            aria_describedby: press_aria_describedby,
            // From hover (distinct).
            on_pointerenter: hover_on_pointerenter,
            on_pointerleave: hover_on_pointerleave,
        }
    }
}

impl MergeWith<UsePressProps> for UseHoverProps {
    type Output = MergedPressHoverProps;

    fn merge_with(self, other: UsePressProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        other.merge_with(self)
    }
}
