// Upstream: react-aria/src/overlays/usePopover.ts @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger and popover elements are captured (`trigger_props`, `props`) instead of passing
//   refs; `trigger` lets a caller that captures the trigger already (`DialogTrigger`) pass it.
// - Positioning options are those of `use_overlay_position` (typed `Placement`); the arrow
//   element is captured by the returned `arrow_props`.
//
// ## OMITTED FEATURES
// - `useFocusWithin` props.
//
// =============================================================================

use leptos::{oco::Oco, prelude::*};

use super::{
    use_overlay::{UseOverlayInput, use_overlay},
    use_overlay_position::{
        Placement, PlacementAxis, Rect, UseOverlayArrowProps, UseOverlayPositionInput,
        use_overlay_position,
    },
};
use crate::{
    hooks::{
        IntoAttrs, MergedOverlayOverlayPositionAttrs, OverlayState, OverlayTriggerState,
        PropsWithStyles,
        interactions::use_prevent_scroll::{UsePreventScrollInput, use_prevent_scroll},
        merged::MergedOverlayOverlayPositionProps,
    },
    utils::{CapturedElement, ElementCaptureAttr, MergeWith, point::Point},
};

/// Input parameters for the `use_popover` hook.
#[derive(Debug, Clone)]
pub struct UsePopoverInput<S: OverlayState = OverlayTriggerState> {
    /// Whether the popover is open; dismissing (Escape, outside interaction, blur) closes it. An
    /// `OverlayTriggerState`, or a component state with its own closing logic (a select's).
    pub state: S,

    /// The trigger the popover is positioned at, captured by `trigger_props` (or by the caller).
    pub trigger: CapturedElement,

    /// Where the popover goes relative to the trigger. Default: [`Placement::Bottom`].
    pub placement: Signal<Placement>,

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

    /// The popover's maximum height. Default: the room available.
    pub max_height: Signal<Option<f64>>,

    /// The arrow's size across the main axis. Default: the width of the arrow element.
    pub arrow_size: Signal<Option<f64>>,

    /// The minimum distance between the arrow and the popover's edges. Default: 0.
    pub arrow_boundary_offset: Signal<f64>,

    /// The element the popover must stay within. Default: the document body.
    pub boundary: Option<CapturedElement>,

    /// Replaces the trigger's bounding rectangle (viewport coordinates). Default: the state's
    /// `point` (where a context menu opened), else none.
    pub target_rect: Signal<Option<Rect>>,

    /// Whether the popover takes over the page while open. Default: modal.
    pub modality: PopoverModality,

    /// Whether pressing Escape should be disabled.
    pub is_keyboard_dismiss_disabled: Signal<bool>,

    /// When the user interacts with an element outside of the overlay,
    /// return `true` if `on_close` should be called. This gives you a chance to
    /// filter out interaction with elements that should not dismiss the popover.
    /// By default, `on_close` will always be called on interaction outside the popover.
    pub should_close_on_interact_outside: Option<crate::hooks::InteractOutsideFilter>,

    /// The group the popover belongs to: a root popover's container, which also holds the
    /// popovers of its submenus. The overlay stack, outside interactions and hiding the rest of the
    /// page work on the group. Default: the popover alone.
    pub group: Option<CapturedElement>,

    /// Whether this is a submenu's popover: dismissable by outside interaction although non-modal.
    pub is_submenu: bool,
}

/// Whether a popover takes over the page while open (react-aria: `isNonModal`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverModality {
    /// The rest of the page is inert (hidden from assistive technology) and focus stays in the
    /// popover; an outside press closes it.
    #[default]
    Modal,
    /// The page stays usable: focus can leave the popover (which closes it), and scrolling the
    /// page closes it.
    NonModal,
}

impl PopoverModality {
    /// Whether the popover is [`Modal`](Self::Modal).
    pub fn is_modal(self) -> bool {
        self == Self::Modal
    }
}

/// The return value of the `use_popover` hook.
#[derive(Debug)]
pub struct UsePopoverReturn {
    /// Props for the popover element. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UsePopoverProps>,

    /// Props for the trigger element. Call `.into_attrs()` for view spreading.
    /// Spread these onto the trigger element so the hook can capture it for positioning.
    pub trigger_props: UsePopoverTriggerProps,

    /// Unique ID for the overlay. Pass to `use_overlay_trigger` as `overlay_id`.
    pub id: Oco<'static, str>,

    /// The popover element, once rendered.
    pub popover_element: CapturedElement,

    /// Props for an arrow element inside the popover (see `use_overlay_position`).
    pub arrow_props: PropsWithStyles<UseOverlayArrowProps>,

    /// The side of the trigger the popover is on, once positioned (after flipping).
    pub placement: Signal<Option<PlacementAxis>>,

    /// The point of the popover closest to the trigger, in its own coordinates, once positioned.
    pub trigger_anchor_point: Signal<Option<Point>>,
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

/// These attributes must be spread onto the popover element.
pub type UsePopoverAttrs = MergedOverlayOverlayPositionAttrs;

/// Provides the behavior and accessibility implementation for a popover component.
///
/// A popover is an overlay element positioned relative to a trigger. This hook delegates
/// to [`use_overlay`] for dismiss handling (Escape key, click outside, blur, overlay stacking)
/// and [`use_overlay_position`] for positioning.
///
/// # Example
///
/// ```ignore
/// let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
/// let popover = use_popover(UsePopoverInput {
///     state,
///     trigger: CapturedElement::new(),
///     placement: Signal::stored(Placement::Bottom),
///     offset: Signal::stored(8.0),
///     cross_offset: Signal::stored(0.0),
///     container_padding: Signal::stored(12.0),
///     should_flip: Signal::stored(true),
///     max_height: Signal::stored(None),
///     arrow_size: Signal::stored(None),
///     arrow_boundary_offset: Signal::stored(0.0),
///     boundary: None,
///     target_rect: Signal::stored(None),
///     modality: PopoverModality::Modal,
///     is_keyboard_dismiss_disabled: Signal::stored(false),
///     should_close_on_interact_outside: None,
///     group: None,
///     is_submenu: false,
/// });
///
/// let trigger_attrs = StoredValue::new(popover.trigger_props.into_attrs());
/// let popover_props = StoredValue::new(popover.props.into_attrs());
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
///             // A modal popover's underlay covers the page (an outside press lands on it)
///             <div style="position: fixed; inset: 0; z-index: 999;" />
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
pub fn use_popover<S: OverlayState>(input: UsePopoverInput<S>) -> UsePopoverReturn {
    let UsePopoverInput {
        state,
        trigger: trigger_element,
        placement,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height,
        arrow_size,
        arrow_boundary_offset,
        boundary,
        target_rect,
        modality,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        group,
        is_submenu,
    } = input;
    let is_open = Signal::derive(move || state.is_open());
    let on_close = Callback::new(move |()| state.close());

    // 1. Delegate all dismissal to use_overlay (overlay stack, escape, interact outside, blur).
    //    As react-aria: isDismissable = !isNonModal || isSubmenu.
    //    react-aria: shouldCloseOnBlur is always true for popovers.
    let overlay = use_overlay(UseOverlayInput {
        is_open,
        on_close,
        is_dismissable: Signal::stored(modality.is_modal() || is_submenu),
        should_close_on_blur: Signal::stored(true),
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        group,
    });

    // 2. Positioning relative to the trigger (or the point the state opened at). A non-modal
    //    popover closes when the page scrolls; a modal one prevents scrolling.
    let point = state.point();
    let position = use_overlay_position(UseOverlayPositionInput {
        placement,
        container_padding,
        offset,
        cross_offset,
        should_flip,
        boundary,
        max_height,
        arrow_size,
        arrow_boundary_offset,
        target_rect: Signal::derive(move || {
            target_rect.get().or_else(|| {
                point.get().map(|point| Rect {
                    top: point.y,
                    left: point.x,
                    width: 0.0,
                    height: 0.0,
                })
            })
        }),
        // A submenu's popover stays open (as react-aria): its menu may scroll the trigger item.
        on_close: (!modality.is_modal() && !is_submenu).then_some(on_close),
        target: trigger_element,
        is_open,
        should_update_position: Signal::stored(true),
        scroll: None,
    });

    // 3. Scroll prevention (disabled when non-modal or not open).
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !modality.is_modal() || !is_open.get()),
    });

    // 4. Hide the rest of the page from assistive technology (modal), or stay visible (non-modal).
    let popover_element = overlay.overlay_element;
    use_popover_visibility(is_open, group.unwrap_or(popover_element), modality);

    // 5. Return merged props.
    let id = overlay.id;
    let (position_props, position_styles) = position.props.into_inner();
    UsePopoverReturn {
        props: PropsWithStyles::new(
            UsePopoverProps {
                other: overlay.props.merge_with(position_props),
            },
            position_styles,
        ),
        trigger_props: UsePopoverTriggerProps {
            element_capture: trigger_element.attr(),
        },
        id,
        popover_element,
        arrow_props: position.arrow_props,
        placement: position.placement,
        trigger_anchor_point: position.trigger_anchor_point,
    }
}

/// While the popover is open and rendered: a modal popover makes the rest of the page inert (hidden
/// from assistive technology), a non-modal one stays visible even if another modal overlay hid
/// the page. Does nothing until the popover element is captured, so the popover itself is never
/// made inert.
pub(crate) fn use_popover_visibility(
    is_open: Signal<bool>,
    overlay_element: CapturedElement,
    modality: PopoverModality,
) {
    #[cfg(feature = "ssr")]
    {
        let _ = (is_open, overlay_element, modality);
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::LocalStorage;

        use crate::utils::aria_hide_outside::{
            AriaHideOutsideOptions, HideMode, aria_hide_outside, keep_visible,
        };

        let undo: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);
        let restore = move || {
            undo.update_value(|undo| {
                if let Some(undo) = undo.take() {
                    undo();
                }
            });
        };

        Effect::new(move |_| {
            restore();
            if !is_open.get() {
                return;
            }
            let Some(popover) = overlay_element.get() else {
                return;
            };
            let popover: web_sys::Element = (*popover).clone();
            let new_undo = match modality {
                PopoverModality::Modal => Some(aria_hide_outside(
                    &[popover],
                    AriaHideOutsideOptions {
                        mode: HideMode::Inert,
                        ..AriaHideOutsideOptions::default()
                    },
                )),
                PopoverModality::NonModal => keep_visible(&popover),
            };
            undo.set_value(new_undo);
        });

        on_cleanup(restore);
    }
}
