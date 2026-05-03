//! Hit-testing for resolving pointer coordinates to a [`DropTarget`] within a collection.
//!
//! Based on react-aria's `ListDropTargetDelegate` from
//! `@react-aria/dnd/src/ListDropTargetDelegate.ts`.

use super::types::{DropPosition, DropTarget};

/// The layout mode for a droppable collection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DropLayout {
    /// Items are stacked along the primary axis (default).
    #[default]
    Stack,
    /// Items are arranged in a grid.
    Grid,
}

/// The primary axis orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DropOrientation {
    /// Items flow vertically (default).
    #[default]
    Vertical,
    /// Items flow horizontally.
    Horizontal,
}

/// The text/layout direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DropDirection {
    /// Left-to-right (default).
    #[default]
    Ltr,
    /// Right-to-left.
    Rtl,
}

//
// ## API DIFFERENCES
//
// - Uses `dyn Fn` for `is_valid_drop_target` instead of a method on state.
//   Rationale: Avoids coupling to specific state type; allows flexible validation.
// - `ListDropTargetDelegate` takes items by `data-key` attribute lookup instead
//   of react-aria's collection/ref approach.
//   Rationale: Leptos doesn't have a unified collection abstraction yet.
//

/// Trait for resolving pointer coordinates to a drop target within a collection.
///
/// Implementations perform hit-testing against the collection's DOM elements
/// to determine which item (and position) a pointer is targeting.
pub trait DropTargetDelegate {
    /// Resolves a pointer position to a drop target.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate (client-relative).
    /// * `y` - The y coordinate (client-relative).
    /// * `is_valid_drop_target` - Predicate to check if a given target is valid.
    ///
    /// Returns `None` if no valid target can be determined.
    fn get_drop_target_from_point(
        &self,
        x: f64,
        y: f64,
        is_valid_drop_target: &dyn Fn(&DropTarget) -> bool,
    ) -> Option<DropTarget>;
}

/// Threshold in pixels from item edge for before/after detection.
const EDGE_THRESHOLD_PX: f64 = 5.0;

/// A [`DropTargetDelegate`] for flat (non-hierarchical) lists and grids.
///
/// Resolves pointer positions to drop targets by querying DOM elements
/// with `data-key` attributes within the collection container.
///
/// The delegate determines `Before`, `After`, or `On` position based on
/// pointer proximity to item edges:
/// - Within `EDGE_THRESHOLD_PX` of the start edge → `Before`
/// - Within `EDGE_THRESHOLD_PX` of the end edge → `After`
/// - Otherwise → `On` (falls back to `Before`/`After` if `On` is invalid)
///
/// Supports vertical/horizontal orientation, grid layout, and RTL direction.
pub struct ListDropTargetDelegate {
    /// The collection container element.
    collection_element: web_sys::Element,
    /// The layout mode (stack or grid).
    layout: DropLayout,
    /// The primary axis orientation.
    orientation: DropOrientation,
    /// The text/layout direction.
    direction: DropDirection,
}

impl ListDropTargetDelegate {
    /// Creates a new delegate for the given collection container.
    ///
    /// For simple vertical lists, use `horizontal: false`.
    /// For simple horizontal lists, use `horizontal: true`.
    #[must_use]
    pub fn new(collection_element: web_sys::Element, horizontal: bool) -> Self {
        Self {
            collection_element,
            layout: DropLayout::Stack,
            orientation: if horizontal {
                DropOrientation::Horizontal
            } else {
                DropOrientation::Vertical
            },
            direction: DropDirection::Ltr,
        }
    }

    /// Creates a delegate with full layout configuration.
    #[must_use]
    pub fn with_layout(
        collection_element: web_sys::Element,
        layout: DropLayout,
        orientation: DropOrientation,
        direction: DropDirection,
    ) -> Self {
        Self {
            collection_element,
            layout,
            orientation,
            direction,
        }
    }

    /// Whether the primary axis is horizontal.
    fn is_horizontal(&self) -> bool {
        self.orientation == DropOrientation::Horizontal
    }

    /// Finds all child elements with `data-key` attributes.
    fn get_keyed_elements(&self) -> Vec<(String, web_sys::Element)> {
        let mut result = Vec::new();
        let children = self.collection_element.children();
        for i in 0..children.length() {
            if let Some(child) = children.item(i) {
                // Check the child itself
                if let Some(key) = child.get_attribute("data-key") {
                    result.push((key, child.clone()));
                }
                // Also check nested children (one level deep for wrappers)
                let nested = child.query_selector_all("[data-key]");
                if let Ok(nodes) = nested {
                    for j in 0..nodes.length() {
                        if let Some(node) = nodes.item(j) {
                            use wasm_bindgen::JsCast;
                            if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                                if let Some(key) = el.get_attribute("data-key") {
                                    result.push((key, el.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Determines the drop position based on pointer proximity to the item's edges.
    fn resolve_position(
        &self,
        rect: &web_sys::DomRect,
        x: f64,
        y: f64,
        key: &str,
        is_valid: &dyn Fn(&DropTarget) -> bool,
    ) -> DropPosition {
        // For grid layout, use the primary axis for position detection.
        // For stack layout, use the orientation axis.
        let (offset, size) = if self.layout == DropLayout::Grid {
            // Grid: use horizontal axis for position (items flow left-to-right/RTL).
            (x - rect.left(), rect.width())
        } else if self.is_horizontal() {
            (x - rect.left(), rect.width())
        } else {
            (y - rect.top(), rect.height())
        };

        // RTL: flip the offset so "start" is on the right side.
        let offset = if self.direction == DropDirection::Rtl
            && (self.is_horizontal() || self.layout == DropLayout::Grid)
        {
            size - offset
        } else {
            offset
        };

        // Near the start edge → Before
        if offset < EDGE_THRESHOLD_PX {
            return DropPosition::Before;
        }

        // Near the end edge → After
        if offset > size - EDGE_THRESHOLD_PX {
            return DropPosition::After;
        }

        // Middle → On, but fall back if invalid
        let on_target = DropTarget::Item {
            key: key.to_owned(),
            position: DropPosition::On,
        };
        if is_valid(&on_target) {
            return DropPosition::On;
        }

        // Fall back to before/after based on which half of the item we're in
        if offset < size / 2.0 {
            DropPosition::Before
        } else {
            DropPosition::After
        }
    }
}

impl DropTargetDelegate for ListDropTargetDelegate {
    fn get_drop_target_from_point(
        &self,
        x: f64,
        y: f64,
        is_valid_drop_target: &dyn Fn(&DropTarget) -> bool,
    ) -> Option<DropTarget> {
        let keyed_elements = self.get_keyed_elements();
        if keyed_elements.is_empty() {
            // Empty collection → root drop
            return Some(DropTarget::Root);
        }

        // Find the item whose bounding rect contains the point, or the closest one.
        let mut closest_key: Option<String> = None;
        let mut closest_distance = f64::MAX;
        let mut closest_rect: Option<web_sys::DomRect> = None;

        for (key, element) in &keyed_elements {
            let rect = element.get_bounding_client_rect();

            // Check if point is inside the element
            if x >= rect.left() && x <= rect.right() && y >= rect.top() && y <= rect.bottom() {
                let position = self.resolve_position(&rect, x, y, key, is_valid_drop_target);
                let target = DropTarget::Item {
                    key: key.clone(),
                    position,
                };
                if is_valid_drop_target(&target) {
                    return Some(target);
                }
                // Try other positions as fallback
                for fallback_pos in [DropPosition::Before, DropPosition::After, DropPosition::On] {
                    if fallback_pos == position {
                        continue;
                    }
                    let fallback = DropTarget::Item {
                        key: key.clone(),
                        position: fallback_pos,
                    };
                    if is_valid_drop_target(&fallback) {
                        return Some(fallback);
                    }
                }
            }

            // Track closest element for gap handling
            let center_x = f64::midpoint(rect.left(), rect.right());
            let center_y = f64::midpoint(rect.top(), rect.bottom());
            let dist = if self.is_horizontal() || self.layout == DropLayout::Grid {
                // For horizontal or grid, use 2D distance
                let dx = x - center_x;
                let dy = y - center_y;
                dx * dx + dy * dy
            } else {
                let dy = y - center_y;
                dy * dy
            };

            if dist < closest_distance {
                closest_distance = dist;
                closest_key = Some(key.clone());
                closest_rect = Some(rect);
            }
        }

        // Point is in a gap between items → use closest item
        if let (Some(key), Some(rect)) = (closest_key, closest_rect) {
            let position = if self.is_horizontal() || self.layout == DropLayout::Grid {
                let in_start_half = if self.direction == DropDirection::Rtl {
                    x > f64::midpoint(rect.left(), rect.right())
                } else {
                    x < f64::midpoint(rect.left(), rect.right())
                };
                if in_start_half {
                    DropPosition::Before
                } else {
                    DropPosition::After
                }
            } else if y < f64::midpoint(rect.top(), rect.bottom()) {
                DropPosition::Before
            } else {
                DropPosition::After
            };

            let target = DropTarget::Item {
                key: key.clone(),
                position,
            };
            if is_valid_drop_target(&target) {
                return Some(target);
            }
        }

        // Nothing matched → root
        if is_valid_drop_target(&DropTarget::Root) {
            return Some(DropTarget::Root);
        }

        None
    }
}
