use leptos::prelude::*;
use leptos_use::{use_document, use_element_bounding, use_window};

use super::calculate_position::{CalculatePositionInput, Rect, calculate_position};
use crate::{
    hooks::{IntoAttrs, PropsWithStyles},
    utils::{CapturedElement, ElementCaptureAttr, locale::WritingDirection, styles::Styles},
};

//
// This hook is based on React Aria's `useOverlayPosition` and `calculatePosition`:
// https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useOverlayPosition.ts
// https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/calculatePosition.ts
//
// ## OMITTED FEATURES
//
// - **Arrow positioning**: `arrowSize`, `arrowBoundaryOffset`, and arrow position
//   outputs are not implemented. Arrow positioning adds significant complexity and
//   is not yet needed by any consumer.
//
// - **Scroll anchoring**: React Aria recalculates position on scroll/resize via
//   `useResize` + `useCloseOnScroll`. We rely on `use_element_bounding` (from
//   `leptos-use`) which uses `ResizeObserver` and provides reactive bounding rects.
//   Close-on-scroll is handled by `use_overlay`, not here (see below).
//
// - **Trigger anchor point**: React Aria's `placementAxis` concept (anchor point
//   on the trigger for different physical placements) is not needed because we use
//   separate `PlacementX`/`PlacementY` enums that already encode this.
//
// - **Visual viewport advanced features**: React Aria freezes overlay positioning
//   during pinch-zoom (scale detection) and tracks `visualViewport.offsetTop/Left`
//   for coordinate transformation on iOS. Not implemented. Basic `visualViewport`
//   support (width/height for boundary dimensions, resize event) IS implemented.
//
// ## DIFFERENT BEHAVIOR
//
// - **Separate `PlacementX`/`PlacementY` enums**: React Aria uses a single
//   `Placement` enum like `"top"`, `"bottom start"`. We use orthogonal X/Y enums
//   for more flexible combination.
//
// - **`position: fixed` instead of `absolute`**: React Aria uses `position: absolute`
//   with containing-block detection. Leptos `<Portal>` appends to `<body>`, making
//   `position: fixed` (relative to the viewport) correct and avoiding ~200 lines
//   of containing-block detection logic.
//
// - **Close-on-scroll in `use_close_on_scroll`**: React Aria puts close-on-scroll
//   in `useOverlayPosition` for historical reasons. Leptonic provides it as a
//   separate `use_close_on_scroll` hook for composability. The `Popover` atom
//   calls it from `PopoverContent`.
//
// ## LEPTOS ADAPTATIONS
//
// - `use_element_bounding` (from `leptos-use`) provides reactive bounding rects
//   instead of imperative `getBoundingClientRect()` calls.
//
// - Position is computed via a `Memo<PositionResult>` rather than imperative DOM
//   style manipulation.
//
// - **Element capture**: The overlay element is captured internally via
//   `CapturedElement` / `ElementCaptureAttr` (spread onto the overlay element
//   via the returned props). The target element is accepted as a `CapturedElement`
//   from the caller because the overlay position props are spread onto the overlay
//   element, not the target.
//

// TODO: Serialize, Deserialize, Display, FormStr ???

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlacementX {
    OuterLeft,
    OuterStart,
    Start,
    Left,
    Center,
    Right,
    End,
    OuterEnd,
    OuterRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalPlacementX {
    OuterLeft,
    Left,
    Center,
    Right,
    OuterRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlacementY {
    Above,
    Top,
    Center,
    Bottom,
    Below,
}

impl PlacementX {
    pub(crate) fn direction_aware(self, direction: WritingDirection) -> PhysicalPlacementX {
        match self {
            Self::OuterLeft => PhysicalPlacementX::OuterLeft,
            Self::OuterStart => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::OuterLeft,
                WritingDirection::Rtl => PhysicalPlacementX::OuterRight,
            },
            Self::Start => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::Left,
                WritingDirection::Rtl => PhysicalPlacementX::Right,
            },
            Self::Left => PhysicalPlacementX::Left,
            Self::Center => PhysicalPlacementX::Center,
            Self::Right => PhysicalPlacementX::Right,
            Self::End => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::Right,
                WritingDirection::Rtl => PhysicalPlacementX::Left,
            },
            Self::OuterEnd => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::OuterRight,
                WritingDirection::Rtl => PhysicalPlacementX::OuterLeft,
            },
            Self::OuterRight => PhysicalPlacementX::OuterRight,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UseOverlayPositionInput {
    /// Element to which the overlay should be positioned relative to.
    /// This is a `CapturedElement` from the caller because the overlay position
    /// props are spread onto the overlay element, not the target.
    pub target: CapturedElement,

    pub placement_x: Signal<PlacementX>,
    pub placement_y: Signal<PlacementY>,

    pub writing_direction: Signal<WritingDirection>,

    /// Additional offset along the main axis (pushes the overlay away from the target).
    /// Default: 0.0
    pub offset: Signal<f64>,

    /// Additional offset along the cross axis.
    /// Default: 0.0
    pub cross_offset: Signal<f64>,

    /// Minimum padding between the overlay and the viewport edge.
    /// Default: 12.0
    pub container_padding: Signal<f64>,

    /// Whether the overlay should flip to the opposite side when there isn't enough space.
    /// Default: true
    pub should_flip: Signal<bool>,

    /// Optional maximum height override. If `None`, max height is computed from available space.
    pub max_height: Option<Signal<f64>>,

    /// Whether the overlay is currently open. When false, position computation is skipped.
    pub is_open: Signal<bool>,
}

#[derive(Debug)]
pub struct UseOverlayPositionReturn {
    /// Props for the overlay element. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UseOverlayPositionProps>,

    /// Resolved horizontal placement after flipping.
    pub resolved_placement_x: Memo<PhysicalPlacementX>,

    /// Resolved vertical placement after flipping.
    pub resolved_placement_y: Memo<PlacementY>,
}

/// Props from `use_overlay_position` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseOverlayPositionProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseOverlayPositionProps {
    type Attrs = UseOverlayPositionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.element_capture,)
    }
}

pub type UseOverlayPositionAttrs = (ElementCaptureAttr,);

pub fn use_overlay_position(input: UseOverlayPositionInput) -> UseOverlayPositionReturn {
    let UseOverlayPositionInput {
        target,
        placement_x,
        placement_y,
        writing_direction,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height: user_max_height,
        is_open,
    } = input;

    // Capture the overlay element internally.
    let overlay_element = CapturedElement::new();

    // Bridge CapturedElement -> Signal for use_element_bounding.
    // Signal<Option<SendWrapper<Element>>> implements IntoElementMaybeSignal
    // via leptos-use's OptionSendWrapperSignalMarker impl.
    let overlay_signal = Signal::derive(move || overlay_element.get());
    let target_signal = Signal::derive(move || target.get());

    let overlay_bounding = use_element_bounding(overlay_signal);
    let target_bounding = use_element_bounding(target_signal);

    // Recompute positioning when the viewport resizes.
    // `use_element_bounding` handles overlay/target resize via ResizeObserver, but
    // viewport size changes (window resize, virtual keyboard) are not tracked by it.
    let viewport_resize = Trigger::new();
    #[cfg(not(feature = "ssr"))]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::{JsCast, prelude::Closure};

        let cleanup: StoredValue<Option<SendWrapper<Box<dyn FnOnce()>>>, LocalStorage> =
            StoredValue::new_local(None);

        if let Some(window) = use_window().as_ref().cloned() {
            let resize_handler = Closure::<dyn Fn()>::new(move || viewport_resize.notify());
            let _ = window.add_event_listener_with_callback(
                "resize",
                resize_handler.as_ref().unchecked_ref(),
            );

            // Also listen to visualViewport resize events (virtual keyboard, zoom).
            #[cfg(web_sys_unstable_apis)]
            let vv_state = window.visual_viewport().map(|vv| {
                let vv_handler = Closure::<dyn Fn()>::new(move || viewport_resize.notify());
                let _ = vv.add_event_listener_with_callback(
                    "resize",
                    vv_handler.as_ref().unchecked_ref(),
                );
                (vv, vv_handler)
            });

            let cleanup_fn: Box<dyn FnOnce()> = Box::new(move || {
                let _ = window.remove_event_listener_with_callback(
                    "resize",
                    resize_handler.as_ref().unchecked_ref(),
                );
                #[cfg(web_sys_unstable_apis)]
                if let Some((vv, vv_handler)) = vv_state {
                    let _ = vv.remove_event_listener_with_callback(
                        "resize",
                        vv_handler.as_ref().unchecked_ref(),
                    );
                }
            });
            cleanup.set_value(Some(SendWrapper::new(cleanup_fn)));
        }

        on_cleanup(move || {
            cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f.take()();
                }
            });
        });
    }

    // Viewport dimensions for the boundary rect.
    //
    // Uses `visualViewport` when available (accounts for pinch-zoom and virtual
    // keyboards), falling back to `documentElement.clientWidth/clientHeight`
    // (viewport dimensions excluding scrollbar). Never uses `body` dimensions,
    // which return the content size and can be much larger than the viewport on
    // scrollable pages.
    let viewport_width = move || {
        viewport_resize.track();
        #[cfg(web_sys_unstable_apis)]
        if let Some(window) = use_window().as_ref() {
            if let Some(vv) = window.visual_viewport() {
                return vv.width();
            }
        }
        match use_document().as_ref() {
            Some(document) => match document.document_element() {
                Some(root) => f64::from(root.client_width()),
                None => 0.0,
            },
            None => 0.0,
        }
    };

    let viewport_height = move || {
        viewport_resize.track();
        #[cfg(web_sys_unstable_apis)]
        if let Some(window) = use_window().as_ref() {
            if let Some(vv) = window.visual_viewport() {
                return vv.height();
            }
        }
        match use_document().as_ref() {
            Some(document) => match document.document_element() {
                Some(root) => f64::from(root.client_height()),
                None => 0.0,
            },
            None => 0.0,
        }
    };

    let result = Memo::new(move |_| {
        if !is_open.get() {
            return super::calculate_position::PositionResult::default();
        }

        let phys_x = placement_x.get().direction_aware(writing_direction.get());

        calculate_position(&CalculatePositionInput {
            target: Rect {
                top: target_bounding.top.get(),
                left: target_bounding.left.get(),
                width: target_bounding.width.get(),
                height: target_bounding.height.get(),
            },
            overlay: Rect {
                top: 0.0,
                left: 0.0,
                width: overlay_bounding.width.get(),
                height: overlay_bounding.height.get(),
            },
            boundary: Rect {
                top: 0.0,
                left: 0.0,
                width: viewport_width(),
                height: viewport_height(),
            },
            placement_x: phys_x,
            placement_y: placement_y.get(),
            offset: offset.get(),
            cross_offset: cross_offset.get(),
            container_padding: container_padding.get(),
            should_flip: should_flip.get(),
            max_height: user_max_height.map(|s| s.get()),
        })
    });

    let resolved_placement_x = Memo::new(move |_| result.get().placement_x);
    let resolved_placement_y = Memo::new(move |_| result.get().placement_y);

    UseOverlayPositionReturn {
        props: PropsWithStyles::new(
            UseOverlayPositionProps {
                element_capture: overlay_element.attr(),
            },
            Styles::builder()
                .with("position", "fixed")
                .with("z-index", "100000")
                .with("top", move || format!("{}px", result.get().top))
                .with("left", move || format!("{}px", result.get().left))
                .with_optional("max-height", move || {
                    let mh = result.get().max_height;
                    if mh >= f64::MAX / 2.0 {
                        None
                    } else {
                        Some(format!("{mh}px"))
                    }
                })
                .build(),
        ),
        resolved_placement_x,
        resolved_placement_y,
    }
}
