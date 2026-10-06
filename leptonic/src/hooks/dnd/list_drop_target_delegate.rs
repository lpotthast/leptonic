// Upstream: react-aria/src/dnd/ListDropTargetDelegate.ts @ 99e6102368
use leptos::prelude::*;

use super::types::{DropPosition, DropTarget, ItemDropTarget};
use crate::{
    hooks::{
        Orientation,
        collections::{CollectionMemo, ItemElements, ListLayout},
    },
    utils::{CapturedElement, locale::WritingDirection},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Finds the items' elements in the list's `ItemElements` registry (react-aria queries
//   `[data-key]` elements in the collection element).
//
// =============================================================================

/// Finds the drop target at a point of a collection (native drags).
pub trait DropTargetDelegate: Send + Sync {
    /// The drop target at `x`, `y` (relative to the collection element), preferring targets for
    /// which `is_valid_drop_target` holds.
    fn drop_target_from_point(
        &self,
        x: f64,
        y: f64,
        is_valid_drop_target: &dyn Fn(&DropTarget) -> bool,
    ) -> Option<DropTarget>;
}

/// The drop targets of a list or grid of items: before, on or after each item, depending on the
/// position within it (the edges are before/after when the item accepts drops on it).
#[derive(Debug, Clone, Copy)]
pub struct ListDropTargetDelegate {
    collection: CollectionMemo,
    item_elements: ItemElements,
    element: CapturedElement,
    layout: ListLayout,
    orientation: Orientation,
    direction: WritingDirection,
}

/// How close (in px) to an item's edge a drop is before/after rather than on it.
const EDGE: f64 = 5.0;

impl ListDropTargetDelegate {
    /// A vertical stack in left-to-right text.
    pub fn new(
        collection: CollectionMemo,
        item_elements: ItemElements,
        element: CapturedElement,
    ) -> Self {
        Self {
            collection,
            item_elements,
            element,
            layout: ListLayout::Stack,
            orientation: Orientation::Vertical,
            direction: WritingDirection::Ltr,
        }
    }

    #[must_use]
    pub fn with_layout(mut self, layout: ListLayout) -> Self {
        self.layout = layout;
        self
    }

    #[must_use]
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn with_direction(mut self, direction: WritingDirection) -> Self {
        self.direction = direction;
        self
    }

    fn horizontal(&self) -> bool {
        self.orientation == Orientation::Horizontal
    }

    fn primary_start(&self, r: &web_sys::DomRect) -> f64 {
        if self.horizontal() { r.left() } else { r.top() }
    }

    fn primary_end(&self, r: &web_sys::DomRect) -> f64 {
        if self.horizontal() {
            r.right()
        } else {
            r.bottom()
        }
    }

    fn secondary_start(&self, r: &web_sys::DomRect) -> f64 {
        if self.horizontal() { r.top() } else { r.left() }
    }

    fn secondary_end(&self, r: &web_sys::DomRect) -> f64 {
        if self.horizontal() {
            r.bottom()
        } else {
            r.right()
        }
    }

    fn flow_start(&self, r: &web_sys::DomRect) -> f64 {
        if self.layout == ListLayout::Stack {
            self.primary_start(r)
        } else {
            self.secondary_start(r)
        }
    }

    fn flow_end(&self, r: &web_sys::DomRect) -> f64 {
        if self.layout == ListLayout::Stack {
            self.primary_end(r)
        } else {
            self.secondary_end(r)
        }
    }
}

impl DropTargetDelegate for ListDropTargetDelegate {
    #[allow(clippy::too_many_lines)]
    fn drop_target_from_point(
        &self,
        x: f64,
        y: f64,
        is_valid_drop_target: &dyn Fn(&DropTarget) -> bool,
    ) -> Option<DropTarget> {
        let items: Vec<_> = self
            .collection
            .with_untracked(|c| c.items().map(|n| n.key.clone()).collect());
        let Some(element) = self.element.get_untracked() else {
            return Some(DropTarget::Root);
        };
        if items.is_empty() {
            return Some(DropTarget::Root);
        }

        let rect = element.get_bounding_client_rect();
        let mut primary = if self.horizontal() { x } else { y };
        let mut secondary = if self.horizontal() { y } else { x };
        primary += self.primary_start(&rect);
        secondary += self.secondary_start(&rect);
        let flow = if self.layout == ListLayout::Stack {
            primary
        } else {
            secondary
        };
        let rtl = self.direction == WritingDirection::Rtl;
        let primary_rtl = self.horizontal() && rtl;
        let secondary_rtl = self.layout == ListLayout::Grid && !self.horizontal() && rtl;
        let flow_rtl = if self.layout == ListLayout::Stack {
            primary_rtl
        } else {
            secondary_rtl
        };
        let (before, after) = if flow_rtl {
            (DropPosition::After, DropPosition::Before)
        } else {
            (DropPosition::Before, DropPosition::After)
        };
        let target = |key, drop_position| DropTarget::Item(ItemDropTarget { key, drop_position });

        let mut low = 0;
        let mut high = items.len();
        while low < high {
            let mid = usize::midpoint(low, high);
            let key = &items[mid];
            let Some(item) = self.item_elements.get(key) else {
                break;
            };
            let r = item.get_bounding_client_rect();
            let mut update = |greater: bool| {
                if greater {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            };
            if primary < self.primary_start(&r) {
                update(primary_rtl);
            } else if primary > self.primary_end(&r) {
                update(!primary_rtl);
            } else if secondary < self.secondary_start(&r) {
                update(secondary_rtl);
            } else if secondary > self.secondary_end(&r) {
                update(!secondary_rtl);
            } else {
                let on = target(key.clone(), DropPosition::On);
                let valid = |position| is_valid_drop_target(&target(key.clone(), position));
                let position = if is_valid_drop_target(&on) {
                    if flow <= self.flow_start(&r) + EDGE && valid(DropPosition::Before) {
                        before
                    } else if flow >= self.flow_end(&r) - EDGE && valid(DropPosition::After) {
                        after
                    } else {
                        DropPosition::On
                    }
                } else {
                    let middle = f64::midpoint(self.flow_start(&r), self.flow_end(&r));
                    if flow <= middle && valid(DropPosition::Before) {
                        before
                    } else if flow >= middle && valid(DropPosition::After) {
                        after
                    } else {
                        DropPosition::On
                    }
                };
                return Some(target(key.clone(), position));
            }
        }

        let key = items[low.min(items.len() - 1)].clone();
        let r = self
            .item_elements
            .get(&key)
            .map(|e| e.get_bounding_client_rect());
        if let Some(r) = r
            && (primary < self.primary_start(&r)
                || (flow - self.flow_start(&r)).abs() < (flow - self.flow_end(&r)).abs())
        {
            return Some(target(key, before));
        }
        Some(target(key, after))
    }
}
