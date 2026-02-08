use crate::hooks::{UseHoverProps, UsePressProps};
use crate::utils::{EventHandler, MergeWith};
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
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
    pub on_dragstart: EventHandler<DragEvent>,
    pub aria_describedby: Option<&'static str>,
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
    Attr<attr::AriaDescribedby, Option<&'static str>>,
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
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_pointerenter.into_on(ev::pointerenter),
            self.on_pointerleave.into_on(ev::pointerleave),
        )
    }
}

impl MergeWith<UseHoverProps> for UsePressProps {
    type Output = MergedPressHoverProps;

    fn merge_with(self, hover: UseHoverProps) -> Self::Output {
        let press = self;
        MergedPressHoverProps {
            // From press (distinct).
            on_keydown: press.on_keydown,
            on_click: press.on_click,
            on_pointerdown: press.on_pointerdown,
            on_dragstart: press.on_dragstart,
            aria_describedby: press.aria_describedby,
            // From hover (distinct).
            on_pointerenter: hover.on_pointerenter,
            on_pointerleave: hover.on_pointerleave,
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
