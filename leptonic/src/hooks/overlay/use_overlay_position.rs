// Upstream: react-aria/src/overlays/useOverlayPosition.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

pub use super::calculate_position::{Placement, PlacementAxis, PositionResult, Rect};
use super::use_close_on_scroll::{UseCloseOnScrollInput, use_close_on_scroll};
use crate::{
    hooks::{IntoAttrs, PropsWithStyles},
    utils::{
        CapturedElement, ElementCaptureAttr,
        aria::AriaHidden,
        css::{LengthPercentageAuto, MaxSize, NonNegativeLengthPercentage, ZIndex, try_px},
        i18n::use_direction,
        point::Point,
        style::{
            BottomProperty, LeftProperty, MaxHeightProperty, RightProperty, TopProperty,
            ZIndexProperty,
        },
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `placement` is the typed `Placement` enum; the result's side is `PlacementAxis`.
// - Elements are `CapturedElement`s: the overlay and arrow elements are captured by the
//   returned props, the target (and optional boundary and scroll elements) are passed in.
// - `getTargetRect` is the reactive `target_rect`: a rectangle replacing the target's bounding
//   rectangle (e.g. a point for context menus).
// - `updatePosition()` is the returned `PositionUpdater`.
//
// ## DIFFERENT BEHAVIOR
// - The position is computed in an effect after rendering (react-aria: a layout effect) and
//   written to the overlay's style at once; the returned styles hold the same values.
//
// ## OMITTED FEATURES
// - Repositioning on scroll events of shadow roots (react-aria: `getPropagationTargets`).
//
// =============================================================================

/// Input of [`use_overlay_position`].
#[derive(Debug, Clone, Copy)]
pub struct UseOverlayPositionInput {
    /// The element the overlay is positioned at.
    pub target: CapturedElement,
    /// Whether the overlay is open; positioning only happens while it is.
    pub is_open: Signal<bool>,
    /// Where the overlay goes relative to the target. Default: [`Placement::Bottom`].
    pub placement: Signal<Placement>,
    /// The minimum distance between the overlay and the boundary's edges. Default: 12.
    pub container_padding: Signal<f64>,
    /// The distance from the target, along the main axis. Default: 0.
    pub offset: Signal<f64>,
    /// The shift along the target's side. Default: 0.
    pub cross_offset: Signal<f64>,
    /// Whether the overlay flips to the other side when there is more room there. Default: `true`.
    pub should_flip: Signal<bool>,
    /// The element the overlay must stay within. Default: the document body.
    pub boundary: Option<CapturedElement>,
    /// The overlay's maximum height. Default: the room available.
    pub max_height: Signal<Option<f64>>,
    /// The arrow's size across the main axis. Default: the width of the element captured by
    /// `arrow_props` (0 without one).
    pub arrow_size: Signal<Option<f64>>,
    /// The minimum distance between the arrow and the overlay's edges. Default: 0.
    pub arrow_boundary_offset: Signal<f64>,
    /// Whether the position follows changes (resizes, ...). Default: `true`.
    pub should_update_position: Signal<bool>,
    /// Replaces the target's bounding rectangle (viewport coordinates), e.g. a point for a
    /// context menu. Default: none.
    pub target_rect: Signal<Option<Rect>>,
    /// The scrollable element inside the overlay whose focused content keeps its place when the
    /// overlay moves. Default: the overlay.
    pub scroll: Option<CapturedElement>,
    /// Called when an ancestor of the target scrolls (not while the visual viewport resizes, as when
    /// a virtual keyboard opens). Default: none (the overlay stays open).
    pub on_close: Option<Callback<()>>,
}

impl UseOverlayPositionInput {
    /// Positioning below `target` while `is_open`, with the defaults above.
    pub fn new(target: CapturedElement, is_open: Signal<bool>) -> Self {
        Self {
            target,
            is_open,
            placement: Signal::stored(Placement::Bottom),
            container_padding: Signal::stored(12.0),
            offset: Signal::stored(0.0),
            cross_offset: Signal::stored(0.0),
            should_flip: Signal::stored(true),
            boundary: None,
            max_height: Signal::stored(None),
            arrow_size: Signal::stored(None),
            arrow_boundary_offset: Signal::stored(0.0),
            should_update_position: Signal::stored(true),
            target_rect: Signal::stored(None),
            scroll: None,
            on_close: None,
        }
    }
}

/// Repositions the overlay on request (react-aria: `updatePosition`), e.g. after its content
/// changed size in a way no observer sees.
#[derive(Debug, Clone, Copy)]
pub struct PositionUpdater(Callback<()>);

impl PositionUpdater {
    /// Computes the position again and applies it at once: the overlay's style and the returned
    /// styles hold the new position when this returns. Does nothing while the overlay is closed or
    /// `should_update_position` is `false`.
    pub fn update(&self) {
        self.0.run(());
    }
}

/// Return value of [`use_overlay_position`].
#[derive(Debug)]
pub struct UseOverlayPositionReturn {
    /// Props for the overlay element: its capture and position styles.
    pub props: PropsWithStyles<UseOverlayPositionProps>,
    /// Props for an arrow element inside the overlay: hidden from assistive technology and
    /// positioned along the overlay's edge (pin it to the edge facing the target yourself).
    pub arrow_props: PropsWithStyles<UseOverlayArrowProps>,
    /// The side of the target the overlay is on, once positioned.
    pub placement: Signal<Option<PlacementAxis>>,
    /// The point of the overlay closest to the target in its own coordinates, once positioned
    /// (e.g. as `transform-origin` for animations).
    pub trigger_anchor_point: Signal<Option<Point>>,
    /// The overlay element, once rendered.
    pub overlay_element: CapturedElement,
    pub updater: PositionUpdater,
}

/// Props for the overlay element.
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

/// Props for the arrow element.
#[derive(Debug)]
pub struct UseOverlayArrowProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseOverlayArrowProps {
    type Attrs = UseOverlayArrowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.element_capture,
            Attr(attr::AriaHidden, AriaHidden::True),
            Attr(attr::Role, "presentation"),
        )
    }
}

pub type UseOverlayArrowAttrs = (
    ElementCaptureAttr,
    Attr<attr::AriaHidden, AriaHidden>,
    Attr<attr::Role, &'static str>,
);

fn px(value: f64) -> Option<LengthPercentageAuto> {
    try_px(value).ok().map(LengthPercentageAuto::from)
}

/// Positions an overlay (popover, menu, tooltip, ...) next to its target and keeps it there: when
/// the window, the overlay or the target resizes, and when a virtual keyboard opens. The overlay
/// flips to the other side when it doesn't fit, stays within the boundary and gets a maximum
/// height for the room available.
///
/// ```ignore
/// let position = use_overlay_position(UseOverlayPositionInput {
///     placement: Signal::stored(Placement::Top),
///     offset: Signal::stored(8.0),
///     ..UseOverlayPositionInput::new(trigger, is_open)
/// });
/// let (attrs, styles) = position.props.into_parts();
/// view! { <div {..attrs} style=styles>"Content"</div> }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_overlay_position(input: UseOverlayPositionInput) -> UseOverlayPositionReturn {
    let UseOverlayPositionInput {
        target,
        is_open,
        placement,
        container_padding,
        offset,
        cross_offset,
        should_flip,
        boundary,
        max_height,
        arrow_size,
        arrow_boundary_offset,
        should_update_position,
        target_rect,
        scroll,
        on_close,
    } = input;
    let direction = use_direction();
    let overlay_element = CapturedElement::new();
    let arrow_element = CapturedElement::new();
    let position = RwSignal::new(None::<PositionResult>);
    let update = Trigger::new();
    // While the visual viewport resizes (a virtual keyboard opening), scrolling repositions the
    // overlay instead of closing it.
    let is_resizing = StoredValue::new(false);
    #[cfg(feature = "ssr")]
    let updater = {
        let _ = (
            placement,
            container_padding,
            offset,
            cross_offset,
            should_flip,
            boundary,
            max_height,
            arrow_size,
            arrow_boundary_offset,
            should_update_position,
            target_rect,
            scroll,
            direction,
            update,
        );
        PositionUpdater(Callback::new(|()| {}))
    };

    #[cfg(not(feature = "ssr"))]
    let updater = {
        use leptos::ev;
        use leptos_use::{use_event_listener, use_resize_observer, use_window};

        use super::calculate_position::dom::{PositionOptions, calculate_position, get_rect};

        // The visual viewport's scale when the overlay opened: positioning pauses while pinch
        // zoomed differently, so the overlay doesn't jump around.
        let visual_viewport_scale = move || {
            use_window()
                .as_ref()
                .and_then(web_sys::Window::visual_viewport)
                .map(|vv| vv.scale())
        };
        let last_scale = StoredValue::new(visual_viewport_scale());
        Effect::new(move |_| {
            if is_open.get() {
                last_scale.set_value(visual_viewport_scale());
            }
        });

        let update_position = move || {
            if !should_update_position.get_untracked() || !is_open.get_untracked() {
                return;
            }
            let (Some(overlay), Some(target_el)) =
                (overlay_element.get_untracked(), target.get_untracked())
            else {
                return;
            };
            let boundary_el: Option<web_sys::Element> = match boundary {
                Some(boundary) => boundary.get_untracked().map(|el| (*el).clone()),
                None => leptos_use::use_document()
                    .as_ref()
                    .and_then(web_sys::Document::body)
                    .map(Into::into),
            };
            let Some(boundary_el) = boundary_el else {
                return;
            };
            if visual_viewport_scale() != last_scale.get_value() {
                return;
            }
            let Some(overlay_html) =
                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&*overlay)
            else {
                return;
            };
            let scroll_el = scroll
                .and_then(|scroll| scroll.get_untracked())
                .unwrap_or_else(|| overlay.clone());

            // Scroll anchoring: keep the focused element in place within the scroll element.
            let document = leptos_use::use_document();
            let active = document
                .as_ref()
                .and_then(crate::utils::shadow_dom::get_active_element);
            let anchor = active
                .as_ref()
                .filter(|active| scroll_el.contains(Some(active)))
                .map(|active| {
                    let anchor_rect = active.get_bounding_client_rect();
                    let scroll_rect = scroll_el.get_bounding_client_rect();
                    let top_offset = anchor_rect.top() - scroll_rect.top();
                    if top_offset > scroll_rect.height() / 2.0 {
                        (false, anchor_rect.bottom() - scroll_rect.bottom())
                    } else {
                        (true, top_offset)
                    }
                });

            // Measure the overlay's natural height: reset a max height not set by the user.
            let user_max_height = max_height.get_untracked();
            let style = overlay_html.style();
            if user_max_height.is_none() {
                let viewport_height = use_window().as_ref().map_or(0.0, |window| {
                    window.visual_viewport().map_or_else(
                        || {
                            window
                                .inner_height()
                                .ok()
                                .and_then(|h| h.as_f64())
                                .unwrap_or(0.0)
                        },
                        |vv| vv.height(),
                    )
                });
                let _ = style.set_property("top", "0px");
                let _ = style.set_property("bottom", "");
                let _ = style.set_property("max-height", &format!("{viewport_height}px"));
            }

            let arrow_size = arrow_size.get_untracked().unwrap_or_else(|| {
                arrow_element
                    .get_untracked()
                    .map_or(0.0, |arrow| get_rect(&arrow, true).width)
            });
            let Some(result) = calculate_position(&PositionOptions {
                placement: placement.get_untracked().parse(direction.get_untracked()),
                target: &target_el,
                overlay: &overlay,
                boundary: &boundary_el,
                padding: container_padding.get_untracked(),
                should_flip: should_flip.get_untracked(),
                offset: offset.get_untracked(),
                cross_offset: cross_offset.get_untracked(),
                max_height: user_max_height,
                arrow_size,
                arrow_boundary_offset: arrow_boundary_offset.get_untracked(),
                target_rect: target_rect.get_untracked(),
            }) else {
                return;
            };

            // Apply at once, so focus and scroll prevention see the final position.
            let side = |value: Option<f64>| value.map_or_else(String::new, |v| format!("{v}px"));
            let _ = style.set_property("position", "absolute");
            let _ = style.set_property("top", &side(result.position.top));
            let _ = style.set_property("bottom", &side(result.position.bottom));
            let _ = style.set_property("left", &side(result.position.left));
            let _ = style.set_property("right", &side(result.position.right));
            let _ = style.set_property("max-height", &format!("{}px", result.max_height));

            if let (Some((from_top, anchor_offset)), Some(active)) = (anchor, active) {
                let anchor_rect = active.get_bounding_client_rect();
                let scroll_rect = scroll_el.get_bounding_client_rect();
                let new_offset = if from_top {
                    anchor_rect.top() - scroll_rect.top()
                } else {
                    anchor_rect.bottom() - scroll_rect.bottom()
                };
                scroll_el.set_scroll_top(scroll_el.scroll_top() + new_offset - anchor_offset);
            }

            position.set(Some(result));
        };

        // Reposition whenever an input or an element changes.
        Effect::new(move |_| {
            update.track();
            should_update_position.track();
            placement.track();
            container_padding.track();
            should_flip.track();
            offset.track();
            cross_offset.track();
            is_open.track();
            direction.track();
            max_height.track();
            arrow_size.track();
            arrow_boundary_offset.track();
            target_rect.track();
            let _ = overlay_element.get();
            let _ = target.get();
            let _ = arrow_element.get();
            if let Some(boundary) = boundary {
                let _ = boundary.get();
            }
            if let Some(scroll) = scroll {
                let _ = scroll.get();
            }
            update_position();
        });

        // Window resizes, and size changes of the overlay or the target (may need a flip).
        let _ = use_event_listener(use_window(), ev::resize, move |_| update.notify());
        let _ = use_resize_observer(
            Signal::derive(move || overlay_element.get()),
            move |_, _| update.notify(),
        );
        let _ = use_resize_observer(Signal::derive(move || target.get()), move |_, _| {
            update.notify();
        });

        // The visual viewport resizing (e.g. a virtual keyboard): reposition, and for a moment
        // also on the scrolling it causes.
        let resize_timeout = StoredValue::new(None::<TimeoutHandle>);
        let on_viewport_resize = move || {
            is_resizing.set_value(true);
            if let Some(handle) = resize_timeout.get_value() {
                handle.clear();
            }
            resize_timeout.set_value(
                set_timeout_with_handle(
                    move || {
                        let _ = is_resizing.try_set_value(false);
                    },
                    std::time::Duration::from_millis(500),
                )
                .ok(),
            );
            update.notify();
        };
        let on_scroll = move || {
            if is_resizing.get_value() {
                on_viewport_resize();
            }
        };
        let visual_viewport = Signal::derive(|| {
            use_window()
                .as_ref()
                .and_then(web_sys::Window::visual_viewport)
                .map(send_wrapper::SendWrapper::new)
        });
        let _ = use_event_listener(visual_viewport, ev::resize, move |_| on_viewport_resize());
        let _ = use_event_listener(visual_viewport, ev::scroll, move |_| on_scroll());
        let _ = use_event_listener(use_window(), ev::scroll, move |_| on_scroll());
        on_cleanup(move || {
            if let Some(Some(handle)) = resize_timeout.try_get_value() {
                handle.clear();
            }
        });

        PositionUpdater(Callback::new(move |()| update_position()))
    };

    // Close when an ancestor of the target scrolls, unless the viewport is resizing.
    if let Some(on_close) = on_close {
        use_close_on_scroll(UseCloseOnScrollInput {
            is_open,
            trigger_element: target,
            on_close: Callback::new(move |()| {
                if !is_resizing.get_value() {
                    on_close.run(());
                }
            }),
        });
    }

    let positioned = move || position.with(Option::is_some);
    let side = move |get: fn(&PositionResult) -> Option<f64>| {
        move || position.with(|p| p.as_ref().and_then(get))
    };
    let top = side(|p| p.position.top);
    let left = side(|p| p.position.left);
    let bottom = side(|p| p.position.bottom);
    let right = side(|p| p.position.right);
    let styles = Styles::builder()
        .with_optional_unchecked("position", move || {
            Some(if positioned() { "absolute" } else { "fixed" })
        })
        .with(ZIndexProperty.declare(ZIndex::Integer(100_000)))
        // Until positioned: at the viewport's origin.
        .with_optional(move || {
            let top = if positioned() { top() } else { Some(0.0) };
            top.and_then(px).map(|v| TopProperty.declare(v))
        })
        .with_optional(move || {
            let left = if positioned() { left() } else { Some(0.0) };
            left.and_then(px).map(|v| LeftProperty.declare(v))
        })
        .with_optional(move || bottom().and_then(px).map(|v| BottomProperty.declare(v)))
        .with_optional(move || right().and_then(px).map(|v| RightProperty.declare(v)))
        .with_optional(move || {
            let max_height = position.with(|p| p.as_ref().map(|p| p.max_height))?;
            let max_height =
                NonNegativeLengthPercentage::try_from(try_px(max_height.max(0.0)).ok()?).ok()?;
            Some(MaxHeightProperty.declare(MaxSize::from(max_height)))
        })
        .with_optional_unchecked("max-height", move || (!positioned()).then_some("100vh"))
        .build();

    let arrow_styles = Styles::builder()
        .with_optional(move || {
            position
                .with(|p| p.as_ref().and_then(|p| p.arrow_offset_left))
                .and_then(px)
                .map(|v| LeftProperty.declare(v))
        })
        .with_optional(move || {
            position
                .with(|p| p.as_ref().and_then(|p| p.arrow_offset_top))
                .and_then(px)
                .map(|v| TopProperty.declare(v))
        })
        .build();

    UseOverlayPositionReturn {
        props: PropsWithStyles::new(
            UseOverlayPositionProps {
                element_capture: overlay_element.attr(),
            },
            styles,
        ),
        arrow_props: PropsWithStyles::new(
            UseOverlayArrowProps {
                element_capture: arrow_element.attr(),
            },
            arrow_styles,
        ),
        placement: Signal::derive(move || position.with(|p| p.as_ref().map(|p| p.placement))),
        trigger_anchor_point: Signal::derive(move || {
            position.with(|p| p.as_ref().map(|p| p.trigger_anchor_point))
        }),
        overlay_element,
        updater,
    }
}
