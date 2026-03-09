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

use leptos::{oco::Oco, prelude::*};

use super::{
    use_overlay::{use_overlay, UseOverlayInput, UseOverlayUnderlayAttrs, UseOverlayUnderlayProps},
    use_overlay_position::{
        use_overlay_position, PhysicalPlacementX, PlacementX, PlacementY, UseOverlayPositionInput,
    },
};
use crate::{
    hooks::{
        interactions::use_prevent_scroll::{use_prevent_scroll, UsePreventScrollInput},
        merged::MergedOverlayOverlayPositionProps,
        IntoAttrs, MergedOverlayOverlayPositionAttrs, UsePreventScrollProps,
        UsePreventScrollReturn,
    },
    utils::{locale::WritingDirection, CapturedElement, ElementCaptureAttr, MergeWith},
};

/// Input parameters for the `use_popover` hook.
#[derive(Debug, Clone)]
pub struct UsePopoverInput {
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
}

/// The return value of the `use_popover` hook.
#[derive(Debug)]
pub struct UsePopoverReturn {
    /// Props for the popover element. Call `.into_attrs()` for view spreading.
    pub props: UsePopoverProps,

    /// Props for the trigger element. Call `.into_attrs()` for view spreading.
    /// Spread these onto the trigger element so the hook can capture it for positioning.
    pub trigger_props: UsePopoverTriggerProps,

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
/// These merge overlay props (from `use_overlay`) with position props (from `use_overlay_position`)
/// via `MergeWith`, stored in the `other` field.
#[derive(Debug)]
pub struct UsePopoverProps {
    pub other: MergedOverlayOverlayPositionProps,
}

impl IntoAttrs for UsePopoverProps {
    type Attrs = UsePopoverAttrs;

    fn into_attrs(self) -> Self::Attrs {
        self.other.into_attrs()
    }
}

/// Props from `use_popover` for the trigger element.
///
/// Contains the `ElementCaptureAttr` that captures the trigger element for positioning.
#[derive(Debug)]
pub struct UsePopoverTriggerProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UsePopoverTriggerProps {
    type Attrs = UsePopoverTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.element_capture,)
    }
}

/// These attributes must be spread onto the trigger element.
pub type UsePopoverTriggerAttrs = (ElementCaptureAttr,);

/// Props from `use_popover` for the underlay element, delegated from `use_overlay`.
///
/// Call `.into_attrs()` to get spreadable attributes.
#[derive(Debug)]
pub struct UsePopoverUnderlayProps(pub UseOverlayUnderlayProps);

impl IntoAttrs for UsePopoverUnderlayProps {
    type Attrs = UsePopoverUnderlayAttrs;

    fn into_attrs(self) -> Self::Attrs {
        self.0.into_attrs()
    }
}

/// These attributes must be spread onto the popover element.
pub type UsePopoverAttrs = MergedOverlayOverlayPositionAttrs;

/// These attributes can be spread onto an underlay element.
/// The underlay captures pointer events behind the popover content.
pub type UsePopoverUnderlayAttrs = UseOverlayUnderlayAttrs;

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
/// });
///
/// let trigger_attrs = StoredValue::new(popover.trigger_props.into_attrs());
/// let popover_props = StoredValue::new(popover.props.into_attrs());
/// let underlay_props = StoredValue::new(popover.underlay_props.into_attrs());
///
/// view! {
///     <button
///         {..trigger_attrs.get_value()}
///         on:click=move |_| set_is_open.set(!is_open.get())
///     >
///         "Toggle Popover"
///     </button>
///
///     <Portal>
///         <Show when=move || is_open.get()>
///             // Underlay captures pointer events to close (modal only)
///             <div
///                 {..underlay_props.get_value()}
///                 style="position: fixed; inset: 0; z-index: 999;"
///             />
///             // Popover content — positioned automatically
///             <div
///                 {..popover_props.get_value()}
///                 style="z-index: 1000;"
///             >
///                 "Popover content"
///             </div>
///         </Show>
///     </Portal>
/// }
/// ```
pub fn use_popover(input: UsePopoverInput) -> UsePopoverReturn {
    let UsePopoverInput {
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
    } = input;

    // Create a CapturedElement for the trigger — the caller will spread
    // trigger_props onto their trigger element.
    let trigger_element = CapturedElement::new();

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
    //    The overlay element is captured internally by use_overlay_position.
    let position = use_overlay_position(UseOverlayPositionInput {
        target: trigger_element,
        placement_x,
        placement_y,
        writing_direction,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height: None,
        is_open,
    });

    // 3. Scroll prevention (disabled when non-modal or not open).
    let UsePreventScrollReturn {
        props: UsePreventScrollProps { /* Empty, no further prop merge required. */ },
    } = use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || is_non_modal || !is_open.get()),
    });

    // 4. Return merged props.
    let id = overlay.id;
    UsePopoverReturn {
        props: UsePopoverProps {
            other: overlay.props.merge_with(position.props),
        },
        trigger_props: UsePopoverTriggerProps {
            element_capture: trigger_element.attr(),
        },
        underlay_props: UsePopoverUnderlayProps(overlay.underlay_props),
        id,
        resolved_placement_x: position.resolved_placement_x,
        resolved_placement_y: position.resolved_placement_y,
    }
}
