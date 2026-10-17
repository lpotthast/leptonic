// Upstream: react-aria-components/src/OverlayArrow.tsx @ 99e6102368
// Upstream: react-aria-components/test/Popover.test.js @ 99e6102368
// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
use leptos::prelude::*;
use leptos_classes::Classes;

use crate::{
    IntoAttrs, Part, PropsWithStyles,
    hooks::overlay::{PlacementAxis, UseOverlayArrowProps},
    utils::{default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Render props become the `data-placement` attribute plus plain children (the arrow's shape).
//
// =============================================================================

/// Provided by overlays that position an arrow ([`Popover`](super::popover::Popover),
/// [`Tooltip`](super::tooltip::Tooltip)) for their [`OverlayArrow`].
#[derive(Clone)]
pub(crate) struct OverlayArrowContext {
    /// The arrow element: the overlay positions one arrow at a time.
    arrow: Part<UseOverlayArrowProps>,
    styles: Styles,
    placement: Signal<Option<PlacementAxis>>,
}

impl OverlayArrowContext {
    pub(crate) fn new(
        props: PropsWithStyles<UseOverlayArrowProps>,
        placement: Signal<Option<PlacementAxis>>,
    ) -> Self {
        let (UseOverlayArrowProps { element_capture }, styles) = props.into_inner();
        Self {
            arrow: Part::new(move || UseOverlayArrowProps {
                element_capture: element_capture.clone(),
            }),
            styles,
            placement,
        }
    }
}

/// An arrow pointing from its [`Popover`](super::popover::Popover) or
/// [`Tooltip`](super::tooltip::Tooltip) at the trigger: positioned at the overlay's edge facing the
/// trigger (`position: absolute`), as close to the trigger's center as the overlay allows. Its
/// children draw it (e.g. an SVG triangle pointing down; rotate it with `data-placement`). Hidden
/// from assistive technology.
///
/// The overlay measures the arrow's width to keep it within its edges; style it with a fixed size.
///
/// Data attributes: `data-placement` (`top`, `bottom`, `left` or `right`: the overlay's side of the
/// trigger).
///
/// Default class: `leptonic-OverlayArrow`.
#[component]
pub fn OverlayArrow(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// The arrow's shape, e.g. an SVG. Without children, style the element itself (CSS borders).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-OverlayArrow", classes);
    let Some(ctx) = use_context::<OverlayArrowContext>() else {
        crate::utils::dev_warn!("An <OverlayArrow> must be inside a <Popover> or <Tooltip>.");
        return None;
    };
    // One arrow per overlay (`Part`: a second one renders nothing). Rendered again each time the
    // overlay opens.
    let attrs = ctx.arrow.props()?.into_attrs();
    let placement = ctx.placement;
    // At the overlay's edge facing the trigger, centered on the offset the overlay computed.
    let edge = move |side: PlacementAxis| move || (placement.get() == Some(side)).then_some("100%");
    let position = Styles::builder()
        .with_unchecked("position", "absolute")
        .with_optional_unchecked("top", edge(PlacementAxis::Top))
        .with_optional_unchecked("bottom", edge(PlacementAxis::Bottom))
        .with_optional_unchecked("left", edge(PlacementAxis::Left))
        .with_optional_unchecked("right", edge(PlacementAxis::Right))
        .with_optional_unchecked("transform", move || {
            placement.get().map(|placement| match placement {
                PlacementAxis::Top | PlacementAxis::Bottom => "translateX(-50%)",
                PlacementAxis::Left | PlacementAxis::Right => "translateY(-50%)",
            })
        })
        .build();
    let styles = position.merge(ctx.styles).merge(styles);

    Some(view! {
        <div
            {..attrs}
            class=classes
            style=styles
            data-placement=move || placement.get().map(PlacementAxis::as_str)
        >
            {children.map(|children| children())}
        </div>
    })
}
