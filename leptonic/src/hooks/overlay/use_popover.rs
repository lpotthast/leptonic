// Upstream: react-aria/src/overlays/usePopover.ts @ 99e6102368
// Upstream: react-aria/test/overlays/usePopover.test.tsx @ 99e6102368
// Upstream: react-aria/test/overlays/usePopover.shadow.test.tsx @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger and popover elements are captured (`trigger_props`, `props`) instead of passing
//   refs; `trigger` lets a caller that captures the trigger already (`DialogTrigger`) pass it.
// - Positioning options are one `OverlayPositionOptions`, shared with `use_overlay_position`
//   (react-aria: the `AriaPositionProps` spread into the props; typed `Placement`); the arrow
//   element is captured by the returned `arrow_props`, the scroll element (`scrollRef`) is a
//   `CapturedElement`.
//
// ## OMITTED FEATURES
// - `useFocusWithin` props.
//
// =============================================================================

use leptos::prelude::*;

use super::{
    use_overlay::{UseOverlayAttrs, UseOverlayInput, UseOverlayProps, use_overlay},
    use_overlay_position::{
        OverlayPositionOptions, PlacementAxis, Rect, UseOverlayArrowProps, UseOverlayPositionAttrs,
        UseOverlayPositionInput, UseOverlayPositionProps, use_overlay_position,
    },
};
use crate::{
    CapturedElement, ElementCaptureAttr, IntoAttrs, PropsWithStyles,
    hooks::overlay::{
        OverlayState, OverlayTriggerState,
        use_prevent_scroll::{UsePreventScrollInput, use_prevent_scroll},
    },
    utils::point::Point,
};

/// Input parameters for the `use_popover` hook.
#[derive(Debug, Clone)]
pub struct UsePopoverInput<S: OverlayState = OverlayTriggerState> {
    /// Whether the popover is open; dismissing (Escape, outside interaction, blur) closes it. An
    /// `OverlayTriggerState`, or a component state with its own closing logic (a select's).
    pub state: S,

    /// The trigger the popover is positioned at, captured by `trigger_props` (or by the caller).
    pub trigger: CapturedElement,

    /// Where and how the popover is placed.
    pub position: OverlayPositionOptions,

    /// Replaces the trigger's bounding rectangle (viewport coordinates). Default: the state's
    /// `point` (where a context menu opened), else none.
    pub target_rect: Signal<Option<Rect>>,

    /// The scrollable element inside the popover (a list box, a menu) whose focused content keeps
    /// its place when the popover moves. Default: the popover.
    pub scroll: Option<CapturedElement>,

    /// Whether the popover takes over the page while open. Default: modal.
    pub modality: PopoverModality,

    /// Whether pressing Escape should be disabled.
    pub is_keyboard_dismiss_disabled: Signal<bool>,

    /// When the user interacts with an element outside of the overlay,
    /// return `true` if `on_close` should be called. This gives you a chance to
    /// filter out interaction with elements that should not dismiss the popover.
    /// By default, `on_close` will always be called on interaction outside the popover.
    pub should_close_on_interact_outside: Option<crate::hooks::overlay::InteractOutsideFilter>,

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
    pub id: String,

    /// The popover element, once rendered.
    pub popover_element: CapturedElement,

    /// Props for an arrow element inside the popover (see `use_overlay_position`).
    pub arrow_props: PropsWithStyles<UseOverlayArrowProps>,

    /// The side of the trigger the popover is on, once positioned (after flipping).
    pub placement: Signal<Option<PlacementAxis>>,

    /// The point of the popover closest to the trigger, in its own coordinates, once positioned.
    pub trigger_anchor_point: Signal<Option<Point>>,
}

/// Props from `use_popover` for the popover element: those of `use_overlay` (dismissal) and of
/// `use_overlay_position` (positioning), both spread onto it.
#[derive(Debug)]
pub struct UsePopoverProps {
    pub overlay: UseOverlayProps,
    pub position: UseOverlayPositionProps,
}

impl IntoAttrs for UsePopoverProps {
    type Attrs = UsePopoverAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.overlay.into_attrs(), self.position.into_attrs())
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
pub type UsePopoverAttrs = (UseOverlayAttrs, UseOverlayPositionAttrs);

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
///     position: OverlayPositionOptions {
///         offset: Signal::stored(8.0),
///         ..OverlayPositionOptions::default()
///     },
///     target_rect: Signal::stored(None),
///     scroll: None,
///     modality: PopoverModality::Modal,
///     is_keyboard_dismiss_disabled: Signal::stored(false),
///     should_close_on_interact_outside: None,
///     group: None,
///     is_submenu: false,
/// });
///
/// let trigger_attrs = popover.trigger_props.into_attrs();
/// let (popover_attrs, popover_styles) = popover.props.into_parts();
/// let popover_attrs = StoredValue::new(popover_attrs);
///
/// view! {
///     <button {..trigger_attrs} on:click=move |_| state.toggle()>
///         "Toggle Popover"
///     </button>
///
///     <Portal>
///         <Show when=move || state.is_open.get()>
///             // A modal popover's underlay covers the page (an outside press lands on it)
///             <div style="position: fixed; inset: 0; z-index: 999;" />
///             // Popover content — positioned automatically
///             <div {..popover_attrs.get_value()} style=popover_styles.clone()>
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
        position: position_options,
        target_rect,
        scroll,
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
        position: position_options,
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
        scroll,
    });

    // 3. Scroll prevention (disabled when non-modal or not open).
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !modality.is_modal() || !is_open.get()),
    });

    // 4. Hide the rest of the page from assistive technology (modal), or stay visible (non-modal).
    let popover_element = overlay.overlay_element;
    use_popover_visibility(is_open, group.unwrap_or(popover_element), modality);

    // 5. Return the props.
    let id = overlay.id;
    let (position_props, position_styles) = position.props.into_inner();
    UsePopoverReturn {
        props: PropsWithStyles::new(
            UsePopoverProps {
                overlay: overlay.props,
                position: position_props,
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
