use leptos::{
    prelude::*,
    tachys::html::style::{style, Style},
};
use leptos_use::{use_document, use_element_bounding};

use super::calculate_position::{calculate_position, CalculatePositionInput, Rect};
use crate::{
    hooks::IntoAttrs,
    utils::{locale::WritingDirection, CapturedElement, ElementCaptureAttr},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
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
// - **Visual viewport handling**: React Aria handles iOS virtual keyboard pushing
//   the visual viewport. Not yet implemented.
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
// - **Close-on-scroll in `use_overlay`**: React Aria puts close-on-scroll in
//   `useOverlayPosition` for historical reasons. We keep it in `use_overlay`
//   where other dismiss logic lives.
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
// =============================================================================

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

#[derive(Debug, Clone)]
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
    /// Props for the overlay element. Call `.into_attrs()` for view spreading.
    pub props: UseOverlayPositionProps,

    /// Resolved horizontal placement after flipping.
    pub resolved_placement_x: Memo<PhysicalPlacementX>,

    /// Resolved vertical placement after flipping.
    pub resolved_placement_y: Memo<PlacementY>,
}

/// Props from `use_overlay_position` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseOverlayPositionProps {
    pub element_capture: ElementCaptureAttr,
    pub position: Signal<(&'static str, String)>,
    pub z_index: Signal<(&'static str, String)>,
    pub top: Signal<(&'static str, String)>,
    pub left: Signal<(&'static str, String)>,
    pub max_height: Signal<(&'static str, String)>,
}

impl IntoAttrs for UseOverlayPositionProps {
    type Attrs = UseOverlayPositionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.element_capture,
            style(self.position),
            style(self.z_index),
            style(self.top),
            style(self.left),
            style(self.max_height),
        )
    }
}

pub type UseOverlayPositionAttrs = (
    ElementCaptureAttr,
    Style<Signal<(&'static str, String)>>, // position: fixed
    Style<Signal<(&'static str, String)>>, // z-index: 100000
    Style<Signal<(&'static str, String)>>, // top: Xpx
    Style<Signal<(&'static str, String)>>, // left: Xpx
    Style<Signal<(&'static str, String)>>, // max-height: Xpx
);

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

    let container_width = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => f64::from(body.client_width()),
            None => 0.0,
        },
        None => 0.0,
    };

    let container_height = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => f64::from(body.client_height()),
            None => 0.0,
        },
        None => 0.0,
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
                width: container_width(),
                height: container_height(),
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
        props: UseOverlayPositionProps {
            element_capture: overlay_element.attr(),
            position: Signal::derive(|| ("position", String::from("fixed"))),
            z_index: Signal::derive(|| ("z-index", String::from("100000"))),
            top: Signal::derive(move || ("top", format!("{}px", result.get().top))),
            left: Signal::derive(move || ("left", format!("{}px", result.get().left))),
            max_height: Signal::derive(move || {
                let mh = result.get().max_height;
                if mh >= f64::MAX / 2.0 {
                    // No effective constraint — don't set max-height
                    ("max-height", String::new())
                } else {
                    ("max-height", format!("{mh}px"))
                }
            }),
        },
        resolved_placement_x,
        resolved_placement_y,
    }
}
