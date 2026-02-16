// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// This hook is based on React Aria's `usePopover`:
// https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/usePopover.ts
//
// ## OMITTED FEATURES
//
// - `arrowRef`/`arrowProps`: Arrow element positioning is not built-in. Users must
//   implement arrow positioning manually if needed.
//
// - `groupRef`: Submenu-style popover groups (where multiple popovers share a
//   trigger area) are not implemented.
//
// - `ariaHideOutside`/`keepVisible`: React Aria hides outside elements from
//   assistive technology when a popover is open. This is not implemented.
//
// ## IMPLEMENTED (previously omitted)
//
// - `placement` return value: Resolved placement after flipping is now returned
//   via `resolved_placement_x` and `resolved_placement_y`.
//
// ## API DIFFERENCES
//
// - `underlayProps` is renamed to `underlay_props` following Rust naming conventions.
//
// =============================================================================

use std::marker::PhantomData;

use educe::Educe;
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
    tachys::html::style::{style, Style},
};
use leptos_use::core::IntoElementMaybeSignal;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

use super::{
    use_overlay::{use_overlay, UseOverlayInput},
    use_overlay_position::{
        use_overlay_position, PhysicalPlacementX, PlacementX, PlacementY, UseOverlayPositionInput,
    },
};
use crate::{
    hooks::{
        interactions::use_prevent_scroll::{use_prevent_scroll, UsePreventScrollInput},
        IntoAttrs,
    },
    utils::{locale::WritingDirection, ElementCaptureAttr, EventHandler},
};

/// Input parameters for the `use_popover` hook.
#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct UsePopoverInput<Trigger, Popover, M>
where
    Trigger: IntoElementMaybeSignal<web_sys::Element, M> + Clone,
    Popover: IntoElementMaybeSignal<web_sys::Element, M> + Clone,
{
    /// The ref for the element which the popover positions itself with respect to.
    #[educe(Debug(ignore))]
    pub trigger_ref: Trigger,

    /// The ref for the popover element.
    #[educe(Debug(ignore))]
    pub popover_ref: Popover,

    /// Whether the popover is open.
    pub is_open: Signal<bool>,

    /// Called when the popover should close.
    pub on_close: Callback<()>,

    /// Horizontal placement of the popover relative to the trigger.
    pub placement_x: Signal<PlacementX>,

    /// Vertical placement of the popover relative to the trigger.
    pub placement_y: Signal<PlacementY>,

    /// Writing direction for logical placement.
    pub writing_direction: Signal<WritingDirection>,

    /// Additional offset along the main axis (pushes the popover away from the trigger).
    /// Default: 0.0
    pub offset: Signal<f64>,

    /// Additional offset along the cross axis.
    /// Default: 0.0
    pub cross_offset: Signal<f64>,

    /// Minimum padding between the popover and the viewport edge.
    /// Default: 12.0
    pub container_padding: Signal<f64>,

    /// Whether the popover should flip to the opposite side when there isn't enough space.
    /// Default: true
    pub should_flip: Signal<bool>,

    /// Whether the popover is non-modal (allows interaction with elements outside).
    pub is_non_modal: bool,

    /// Whether pressing Escape should be disabled.
    pub is_keyboard_dismiss_disabled: bool,

    /// When the user interacts with an element outside of the overlay,
    /// return `true` if `on_close` should be called. This gives you a chance to
    /// filter out interaction with elements that should not dismiss the popover.
    /// By default, `on_close` will always be called on interaction outside the popover.
    pub should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,

    pub phantom_data: PhantomData<M>,
}

/// The return value of the `use_popover` hook.
#[derive(Debug)]
pub struct UsePopoverReturn {
    /// Props for the popover element. Call `.into_attrs()` for view spreading.
    pub props: UsePopoverProps,

    /// Props for the underlay element (optional background layer behind the popover).
    /// Call `.into_attrs()` for view spreading.
    pub underlay_props: UsePopoverUnderlayProps,

    /// Unique ID for the overlay. Pass to `use_overlay_trigger` as `overlay_id`.
    pub id: Oco<'static, str>,

    /// Resolved horizontal placement after flipping.
    pub resolved_placement_x: Memo<PhysicalPlacementX>,

    /// Resolved vertical placement after flipping.
    pub resolved_placement_y: Memo<PlacementY>,
}

/// Props from `use_popover` for the popover element that can be extracted and merged programmatically.
///
/// These merge overlay props (from `use_overlay`) with position props (from `use_overlay_position`).
#[derive(Debug)]
pub struct UsePopoverProps {
    // From use_overlay
    pub id: String,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    // From use_overlay_position
    pub position: Signal<(&'static str, String)>,
    pub z_index: Signal<(&'static str, String)>,
    pub top: Signal<(&'static str, String)>,
    pub left: Signal<(&'static str, String)>,
    pub max_height: Signal<(&'static str, String)>,
}

impl IntoAttrs for UsePopoverProps {
    type Attrs = UsePopoverAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            // Overlay attrs
            Attr(attr::Id, self.id),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            // Position attrs
            style(self.position),
            style(self.z_index),
            style(self.top),
            style(self.left),
            style(self.max_height),
        )
    }
}

/// Props from `use_popover` for the underlay element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UsePopoverUnderlayProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl IntoAttrs for UsePopoverUnderlayProps {
    type Attrs = UsePopoverUnderlayAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.on_pointerdown.into_on(ev::pointerdown),)
    }
}

/// These attributes must be spread onto the popover element.
pub type UsePopoverAttrs = (
    // Overlay
    Attr<attr::Id, String>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    // Position
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
);

/// These attributes can be spread onto an underlay element.
/// The underlay captures pointer events behind the popover content.
pub type UsePopoverUnderlayAttrs = (On<ev::pointerdown, SharedEventCallback<PointerEvent>>,);

/// Provides the behavior and accessibility implementation for a popover component.
///
/// A popover is an overlay element positioned relative to a trigger. This hook delegates
/// to [`use_overlay`] for dismiss handling (Escape key, click outside, blur, overlay stacking)
/// and [`use_overlay_position`] for positioning.
///
/// # Example
///
/// ```ignore
/// let popover = use_popover(UsePopoverInput {
///     trigger_ref: trigger_el,
///     popover_ref: popover_el,
///     is_open: is_open.into(),
///     on_close: Callback::new(move |_| set_is_open.set(false)),
///     placement_x: Signal::derive(|| PlacementX::Center),
///     placement_y: Signal::derive(|| PlacementY::Below),
///     writing_direction: Signal::derive(|| WritingDirection::Ltr),
///     offset: 0.0.into(),
///     cross_offset: 0.0.into(),
///     container_padding: 12.0.into(),
///     should_flip: true.into(),
///     is_non_modal: false,
///     is_keyboard_dismiss_disabled: false,
///     should_close_on_interact_outside: None,
///     phantom_data: PhantomData,
/// });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div class="underlay" {..popover.underlay_props.into_attrs()}/>
///         <div class="popover" {..popover.props.into_attrs()}>
///             "Popover content"
///         </div>
///     </Show>
/// }
/// ```
pub fn use_popover<Trigger, Popover, M>(
    input: UsePopoverInput<Trigger, Popover, M>,
) -> UsePopoverReturn
where
    Trigger: IntoElementMaybeSignal<web_sys::Element, M> + Clone + 'static,
    Popover: IntoElementMaybeSignal<web_sys::Element, M> + Clone + 'static,
{
    let UsePopoverInput {
        trigger_ref,
        popover_ref,
        is_open,
        on_close,
        placement_x,
        placement_y,
        writing_direction,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        is_non_modal,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        phantom_data: _,
    } = input;

    // 1. Delegate all dismissal to use_overlay (overlay stack, escape, interact outside, blur).
    //    react-aria: isDismissable = !isNonModal || isSubmenu (no submenu support here).
    //    react-aria: shouldCloseOnBlur is always true for popovers.
    let overlay = use_overlay(UseOverlayInput {
        is_open,
        on_close,
        is_dismissable: !is_non_modal,
        should_close_on_blur: true,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
    });

    // 2. Positioning relative to the trigger.
    let position = use_overlay_position(UseOverlayPositionInput {
        overlay: popover_ref,
        target: trigger_ref,
        placement_x,
        placement_y,
        writing_direction,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height: None,
        is_open,
        phantom_data: PhantomData,
    });

    // 3. Scroll prevention (disabled when non-modal or not open).
    use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || is_non_modal || !is_open.get()),
    });

    // 4. Return merged props.
    let id = overlay.id;
    UsePopoverReturn {
        props: UsePopoverProps {
            // Overlay
            id: overlay.props.id,
            element_capture: overlay.props.element_capture,
            on_keydown: overlay.props.on_keydown,
            on_focusin: overlay.props.on_focusin,
            on_focusout: overlay.props.on_focusout,
            // Position
            position: position.props.position,
            z_index: position.props.z_index,
            top: position.props.top,
            left: position.props.left,
            max_height: position.props.max_height,
        },
        underlay_props: UsePopoverUnderlayProps {
            on_pointerdown: overlay.underlay_props.on_pointerdown,
        },
        id,
        resolved_placement_x: position.resolved_placement_x,
        resolved_placement_y: position.resolved_placement_y,
    }
}
