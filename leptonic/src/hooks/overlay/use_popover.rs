// Upstream: react-aria/src/overlays/usePopover.ts @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger and popover elements are captured (`trigger_props`, `props`) instead of passing
//   refs; `trigger` lets a caller that captures the trigger already (`DialogTrigger`) pass it.
// - Placement is two typed axes (`placement_x`, `placement_y`) instead of a placement string;
//   the resolved placement after flipping is returned per axis.
//
// ## OMITTED FEATURES
// - `arrowRef`/`arrowProps`, `groupRef` (submenu groups), `getTargetRect` and anchoring at
//   `state.point`, `useFocusWithin` props.
//
// =============================================================================

use leptos::{oco::Oco, prelude::*};

use super::{
    use_overlay::{UseOverlayInput, use_overlay},
    use_overlay_position::{
        PhysicalPlacementX, PlacementX, PlacementY, UseOverlayPositionInput, use_overlay_position,
    },
};
use crate::{
    hooks::{
        IntoAttrs, MergedOverlayOverlayPositionAttrs, OverlayState, OverlayTriggerState,
        PropsWithStyles, UseCloseOnScrollInput, UsePreventScrollProps, UsePreventScrollReturn,
        interactions::use_prevent_scroll::{UsePreventScrollInput, use_prevent_scroll},
        merged::MergedOverlayOverlayPositionProps,
        use_close_on_scroll,
    },
    utils::{CapturedElement, ElementCaptureAttr, MergeWith},
};

/// Input parameters for the `use_popover` hook.
#[derive(Debug, Clone, Copy)]
pub struct UsePopoverInput<S: OverlayState = OverlayTriggerState> {
    /// Whether the popover is open; dismissing (Escape, outside interaction, blur) closes it. An
    /// `OverlayTriggerState`, or a component state with its own closing logic (a select's).
    pub state: S,

    /// The trigger the popover is positioned at, captured by `trigger_props` (or by the caller).
    pub trigger: CapturedElement,

    /// Horizontal placement of the popover relative to the trigger.
    pub placement_x: Signal<PlacementX>,

    /// Vertical placement of the popover relative to the trigger.
    pub placement_y: Signal<PlacementY>,

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

    /// Whether the popover takes over the page while open. Default: modal.
    pub modality: PopoverModality,

    /// Whether pressing Escape should be disabled.
    pub is_keyboard_dismiss_disabled: bool,

    /// When the user interacts with an element outside of the overlay,
    /// return `true` if `on_close` should be called. This gives you a chance to
    /// filter out interaction with elements that should not dismiss the popover.
    /// By default, `on_close` will always be called on interaction outside the popover.
    pub should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,
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

impl<S: OverlayState> UsePopoverInput<S> {
    /// A modal popover for `state`, below its trigger and centered, flipping when there is no
    /// room, 12px from the viewport edges.
    pub fn new(state: S) -> Self {
        Self {
            state,
            trigger: CapturedElement::new(),
            placement_x: Signal::stored(PlacementX::Center),
            placement_y: Signal::stored(PlacementY::Below),
            offset: Signal::stored(0.0),
            cross_offset: Signal::stored(0.0),
            container_padding: Signal::stored(12.0),
            should_flip: Signal::stored(true),
            modality: PopoverModality::Modal,
            is_keyboard_dismiss_disabled: false,
            should_close_on_interact_outside: None,
        }
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
///     offset: Signal::stored(8.0),
///     ..UsePopoverInput::new(state)
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
        placement_x,
        placement_y,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        modality,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
    } = input;
    let is_open = Signal::derive(move || state.is_open());
    let on_close = Callback::new(move |()| state.close());

    // 1. Delegate all dismissal to use_overlay (overlay stack, escape, interact outside, blur).
    //    react-aria: isDismissable = !isNonModal || isSubmenu (no submenu support here).
    //    react-aria: shouldCloseOnBlur is always true for popovers.
    let overlay = use_overlay(UseOverlayInput {
        is_open,
        on_close,
        is_dismissable: modality.is_modal(),
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
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height: None,
        is_open,
    });

    // A non-modal popover closes when the page scrolls (react-aria: `useOverlayPosition`'s
    // `onClose`); a modal one prevents scrolling.
    if !modality.is_modal() {
        use_close_on_scroll(UseCloseOnScrollInput {
            is_open,
            trigger_element,
            on_close,
        });
    }

    // 3. Scroll prevention (disabled when non-modal or not open).
    let UsePreventScrollReturn {
        props: UsePreventScrollProps { /* Empty, no further prop merge required. */ },
    } = use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !modality.is_modal() || !is_open.get()),
    });

    // 4. Hide the rest of the page from assistive technology (modal), or stay visible (non-modal).
    let popover_element = overlay.overlay_element;
    use_popover_visibility(is_open, popover_element, modality);

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
        resolved_placement_x: position.resolved_placement_x,
        resolved_placement_y: position.resolved_placement_y,
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
