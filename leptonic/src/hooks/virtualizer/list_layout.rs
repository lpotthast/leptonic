// Upstream: react-stately/src/layout/ListLayout.ts @ 99e6102368
// Upstream: @react-spectrum/ai/src/ListLayout.ts @ 99e6102368
use std::{collections::HashMap, sync::Arc};

use super::{
    InvalidationContext, Layout, LayoutInfo, ScrollAnchorAxis, ScrollAnchorEdge, ScrollAnchorInfo,
    VirtualizerContext,
};
use crate::{
    hooks::collections::{Collection, Key, Node, NodeKind, Rect, Size},
    utils::orientation::Orientation,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The anchoring of react-spectrum's AI `ListLayout` (`anchor_to: Some(End)` +
//   `scroll_end_threshold`, vertical only) is part of this layout; like that one, only a change
//   of the cross-axis size (the width of a vertical list) throws away measured sizes. Unlike
//   that one, a change of `anchor_to` doesn't either: there it is a fixed option, here
//   `VirtualList` switches it whenever the user scrolls to or away from the end, and dropping
//   every measured size then made the content jump.
// - Loaders are always shown as loading (collections have no `isLoading` per loader node).
//
// ## OMITTED FEATURES
// - The AI layout's bottom alignment of content shorter than the viewport (chat-style).
// - Drop targets (`getDropTargetFromPoint`, `getDropTargetLayoutInfo`): with DnD on the atoms.
//   Until then there is no `dropIndicatorThickness` option either (only drop indicator layout
//   infos read it).
// - The deprecated `*Height` options.
//
// =============================================================================

/// The size of rows, headings and loaders without a fixed or estimated size.
const DEFAULT_SIZE: f64 = 48.0;

/// Options of a [`ListLayout`].
#[derive(Debug, Clone, PartialEq)]
pub struct ListLayoutOptions {
    /// The direction the items stack in (and the collection usually scrolls).
    pub orientation: Orientation,
    /// The fixed size of a row along the orientation. `None`: estimated, then measured.
    pub row_size: Option<f64>,
    /// The estimated size of a row with variable sizes. Default: 48.
    pub estimated_row_size: Option<f64>,
    /// The fixed size of a section header. `None`: estimated, then measured.
    pub heading_size: Option<f64>,
    pub estimated_heading_size: Option<f64>,
    /// The size of a loader ("load more"). Default: the row size, else 48.
    pub loader_size: Option<f64>,
    /// The gap between items.
    pub gap: f64,
    /// The padding around the list.
    pub padding: f64,
    /// Keeps the viewport at this edge while it is there and the content changes (e.g. a log or
    /// chat appended at the end). Vertical lists only.
    pub anchor_to: Option<ScrollAnchorEdge>,
    /// Within this distance (px) from the end, the viewport counts as being at the end.
    pub scroll_end_threshold: f64,
}

impl Default for ListLayoutOptions {
    fn default() -> Self {
        Self {
            orientation: Orientation::Vertical,
            row_size: None,
            estimated_row_size: None,
            heading_size: None,
            estimated_heading_size: None,
            loader_size: None,
            gap: 0.0,
            padding: 0.0,
            anchor_to: None,
            scroll_end_threshold: 0.0,
        }
    }
}

/// A laid out node: its layout info, children, and the part of it that is valid. Shared between
/// the cache and the laid out tree (a valid node is reused without copying it).
#[derive(Debug, Clone)]
struct LayoutNode {
    node: Option<Node>,
    layout_info: LayoutInfo,
    children: Vec<Arc<LayoutNode>>,
    valid_rect: Rect,
}

/// Arranges items in a stack along its orientation, with fixed or variable (estimated, then
/// measured) sizes (react-stately's `ListLayout`). Lays out lazily: the rows before and after the
/// requested area are only estimated.
#[derive(Debug, Clone)]
pub struct ListLayout {
    options: ListLayoutOptions,
    layout_nodes: HashMap<Key, Arc<LayoutNode>>,
    content_size: Size,
    last_collection: Option<Arc<Collection>>,
    root_nodes: Vec<Arc<LayoutNode>>,
    invalidate_everything: bool,
    /// The rectangle containing currently valid layout infos.
    valid_rect: Rect,
    /// The rectangle of requested layout infos so far.
    requested_rect: Rect,
}

impl ListLayout {
    pub fn new(options: ListLayoutOptions) -> Self {
        Self {
            options,
            layout_nodes: HashMap::new(),
            content_size: Size::default(),
            last_collection: None,
            root_nodes: Vec::new(),
            invalidate_everything: false,
            valid_rect: Rect::default(),
            requested_rect: Rect::default(),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.options.orientation == Orientation::Horizontal
    }

    /// The position along the orientation.
    fn offset_of(&self, rect: &Rect) -> f64 {
        if self.is_horizontal() { rect.x } else { rect.y }
    }

    fn max_offset_of(&self, rect: &Rect) -> f64 {
        if self.is_horizontal() {
            rect.max_x()
        } else {
            rect.max_y()
        }
    }

    /// The extent along the orientation.
    fn length_of(&self, rect: &Rect) -> f64 {
        if self.is_horizontal() {
            rect.width
        } else {
            rect.height
        }
    }

    fn set_length(&self, rect: &mut Rect, length: f64) {
        if self.is_horizontal() {
            rect.width = length;
        } else {
            rect.height = length;
        }
    }

    /// The extent across the orientation.
    fn breadth_of(&self, rect: &Rect) -> f64 {
        if self.is_horizontal() {
            rect.height
        } else {
            rect.width
        }
    }

    fn row_size(&self) -> f64 {
        self.options
            .row_size
            .or(self.options.estimated_row_size)
            .unwrap_or(DEFAULT_SIZE)
            + self.options.gap
    }

    fn ensure_layout_info(&mut self, ctx: &VirtualizerContext<'_>, key: &Key) -> bool {
        // Outside the area laid out so far (e.g. Home/End): lay out everything.
        if !self.layout_nodes.contains_key(key)
            && self.requested_rect.area() < self.content_size.area()
            && self.last_collection.is_some()
        {
            self.requested_rect = Rect::new(0.0, 0.0, f64::INFINITY, f64::INFINITY);
            self.root_nodes = self.build_collection(ctx, self.options.padding);
            self.requested_rect =
                Rect::new(0.0, 0.0, self.content_size.width, self.content_size.height);
            return true;
        }
        false
    }

    fn layout_if_needed(&mut self, ctx: &VirtualizerContext<'_>, rect: Rect) {
        if self.last_collection.is_none() {
            return;
        }
        if !self.requested_rect.contains_rect(&rect) {
            self.requested_rect = self.requested_rect.union(&rect);
            self.root_nodes = self.build_collection(ctx, self.options.padding);
        }
        // The persisted keys must be available.
        for key in ctx.persisted_keys {
            if self.ensure_layout_info(ctx, key) {
                return;
            }
        }
    }

    /// Whether `key` or a descendant of it is persisted.
    fn is_persisted_key(&self, ctx: &VirtualizerContext<'_>, key: &Key) -> bool {
        if ctx.persisted_keys.contains(key) {
            return true;
        }
        ctx.persisted_keys.iter().any(|persisted| {
            let mut current = persisted.clone();
            while let Some(parent) = self
                .layout_nodes
                .get(&current)
                .and_then(|node| node.layout_info.parent_key.clone())
            {
                if parent == *key {
                    return true;
                }
                current = parent;
            }
            false
        })
    }

    fn is_visible(&self, ctx: &VirtualizerContext<'_>, node: &LayoutNode, rect: &Rect) -> bool {
        node.layout_info.rect.intersects(rect)
            || node.layout_info.is_sticky
            || matches!(node.layout_info.kind, NodeKind::Header | NodeKind::Loader)
            || self.is_persisted_key(ctx, &node.layout_info.key)
    }

    fn is_valid(&self, node: &Node, offset: f64) -> bool {
        self.layout_nodes.get(&node.key).is_some_and(|cached| {
            !self.invalidate_everything
                && cached.node.as_ref() == Some(node)
                && offset == self.offset_of(&cached.layout_info.rect)
                && cached.layout_info.rect.intersects(&self.valid_rect)
                && cached
                    .valid_rect
                    .contains_rect(&cached.layout_info.rect.intersection(&self.requested_rect))
        })
    }

    fn build_collection(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        mut offset: f64,
    ) -> Vec<Arc<LayoutNode>> {
        let collection = ctx.collection;
        let collection_nodes: Vec<&Node> = collection.iter().collect();
        let mut loader_nodes: Vec<usize> = collection_nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.kind == NodeKind::Loader)
            .map(|(index, _)| index)
            .collect();
        let mut nodes = Vec::new();
        let is_empty = collection.size() == 0;
        if is_empty {
            offset = 0.0;
        }
        let padding = self.options.padding;
        let gap = self.options.gap;
        let row_size = self.row_size();
        for (index, node) in collection_nodes.iter().enumerate() {
            // Rows before the requested area are skipped unless cached.
            if node.kind == NodeKind::Item
                && offset + row_size < self.offset_of(&self.requested_rect)
                && !self.is_valid(node, offset)
            {
                offset += row_size;
                continue;
            }
            let layout_node = if self.is_horizontal() {
                self.build_child(ctx, node, offset, padding, None)
            } else {
                self.build_child(ctx, node, padding, offset, None)
            };
            offset = self.max_offset_of(&layout_node.layout_info.rect) + gap;
            nodes.push(layout_node);
            loader_nodes.retain(|loader| *loader != index);

            // Past the requested area: place the remaining loaders at their estimated
            // positions (so they persist) and estimate the rest.
            if matches!(node.kind, NodeKind::Item | NodeKind::Loader)
                && offset > self.max_offset_of(&self.requested_rect)
            {
                let mut last_processed = index;
                for loader_index in std::mem::take(&mut loader_nodes) {
                    #[allow(clippy::cast_precision_loss)]
                    {
                        offset += (loader_index - last_processed - 1) as f64 * row_size;
                    }
                    let loader_node = collection_nodes[loader_index];
                    let loader = if self.is_horizontal() {
                        self.build_child(ctx, loader_node, offset, padding, None)
                    } else {
                        self.build_child(ctx, loader_node, padding, offset, None)
                    };
                    offset = self.max_offset_of(&loader.layout_info.rect);
                    nodes.push(loader);
                    last_processed = loader_index;
                }
                #[allow(clippy::cast_precision_loss)]
                {
                    offset += (collection_nodes.len() - last_processed - 1) as f64 * row_size;
                }
                break;
            }
        }
        offset = (offset - gap).max(0.0);
        if !is_empty {
            offset += padding;
        }
        self.content_size = if self.is_horizontal() {
            Size::new(offset, ctx.size.height)
        } else {
            Size::new(ctx.size.width, offset)
        };
        nodes
    }

    fn build_child(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
        parent_key: Option<Key>,
    ) -> Arc<LayoutNode> {
        let offset = if self.is_horizontal() { x } else { y };
        if self.is_valid(node, offset)
            && let Some(cached) = self.layout_nodes.get(&node.key)
        {
            return Arc::clone(cached);
        }
        let mut layout_node = self.build_node(ctx, node, x, y);
        layout_node.layout_info.parent_key = parent_key;
        layout_node.layout_info.allow_overflow = true;
        let layout_node = Arc::new(layout_node);
        self.layout_nodes
            .insert(node.key.clone(), Arc::clone(&layout_node));
        layout_node
    }

    fn build_node(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
    ) -> LayoutNode {
        match node.kind {
            NodeKind::Section => self.build_section(ctx, node, x, y),
            NodeKind::Header => self.build_section_header(ctx, node, x, y),
            NodeKind::Loader => self.build_loader(ctx, node, x, y),
            _ => self.build_item(ctx, node, x, y),
        }
    }

    fn build_loader(
        &self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
    ) -> LayoutNode {
        let padding = self.options.padding;
        let size = self
            .options
            .loader_size
            .or(self.options.row_size)
            .or(self.options.estimated_row_size)
            .unwrap_or(DEFAULT_SIZE);
        let rect = if self.is_horizontal() {
            Rect::new(x, y, size, ctx.content_size.height - padding - y)
        } else {
            Rect::new(x, y, ctx.content_size.width - padding - x, size)
        };
        LayoutNode {
            node: Some(node.clone()),
            layout_info: LayoutInfo::new(node.kind, node.key.clone(), rect),
            children: Vec::new(),
            valid_rect: rect.intersection(&self.requested_rect),
        }
    }

    fn build_section(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
    ) -> LayoutNode {
        let padding = self.options.padding;
        let gap = self.options.gap;
        let mut rect = if self.is_horizontal() {
            Rect::new(x, y, 0.0, ctx.size.height - padding - y)
        } else {
            Rect::new(x, y, ctx.size.width - padding - x, 0.0)
        };
        let start = if self.is_horizontal() { x } else { y };
        let mut offset = start;
        let row_size = self.row_size();
        let child_nodes: Vec<&Node> = ctx.collection.children(&node.key).collect();
        let mut skipped = 0;
        let mut children = Vec::new();
        for child in child_nodes.iter().copied() {
            // Rows before the requested area are skipped unless cached.
            if offset + row_size < self.offset_of(&self.requested_rect)
                && !self.is_valid(node, offset)
            {
                offset += row_size;
                skipped += 1;
                continue;
            }
            let layout_node = if self.is_horizontal() {
                self.build_child(ctx, child, offset, y, Some(node.key.clone()))
            } else {
                self.build_child(ctx, child, x, offset, Some(node.key.clone()))
            };
            offset = self.max_offset_of(&layout_node.layout_info.rect) + gap;
            children.push(layout_node);
            if offset > self.max_offset_of(&self.requested_rect) {
                // Estimate the rows not laid out now.
                #[allow(clippy::cast_precision_loss)]
                {
                    offset += (child_nodes.len() - (children.len() + skipped)) as f64 * row_size;
                }
                break;
            }
        }
        offset -= gap;
        self.set_length(&mut rect, offset - start);
        let layout_info = LayoutInfo::new(node.kind, node.key.clone(), rect);
        LayoutNode {
            node: Some(node.clone()),
            valid_rect: layout_info.rect.intersection(&self.requested_rect),
            layout_info,
            children,
        }
    }

    /// The length of a node of variable size: its previous one (estimated if the breadth or the
    /// node changed), else the estimate.
    fn variable_length(
        &self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        breadth: f64,
        estimate: Option<f64>,
    ) -> (f64, bool) {
        if let Some(previous) = self.layout_nodes.get(&node.key) {
            let last = self
                .last_collection
                .as_ref()
                .and_then(|collection| collection.get(&node.key));
            let changed =
                previous.node.as_ref() != Some(node) || last != ctx.collection.get(&node.key);
            (
                self.length_of(&previous.layout_info.rect),
                breadth != self.breadth_of(&previous.layout_info.rect)
                    || changed
                    || previous.layout_info.estimated_size,
            )
        } else {
            (estimate.unwrap_or(DEFAULT_SIZE), true)
        }
    }

    fn build_section_header(
        &self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
    ) -> LayoutNode {
        let breadth =
            self.cross_size(ctx) - self.options.padding - if self.is_horizontal() { y } else { x };
        let (length, estimated) = match self.options.heading_size {
            Some(size) => (size, false),
            None => self.variable_length(ctx, node, breadth, self.options.estimated_heading_size),
        };
        let rect = if self.is_horizontal() {
            Rect::new(x, y, length, breadth - y)
        } else {
            Rect::new(x, y, breadth - x, length)
        };
        let mut header = LayoutInfo::new(NodeKind::Header, node.key.clone(), rect);
        header.estimated_size = estimated;
        LayoutNode {
            node: Some(node.clone()),
            valid_rect: header.rect.intersection(&self.requested_rect),
            layout_info: header,
            children: Vec::new(),
        }
    }

    fn build_item(&self, ctx: &VirtualizerContext<'_>, node: &Node, x: f64, y: f64) -> LayoutNode {
        let breadth =
            self.cross_size(ctx) - self.options.padding - if self.is_horizontal() { y } else { x };
        let (length, estimated) = match self.options.row_size {
            Some(size) => (size, false),
            None => self.variable_length(ctx, node, breadth, self.options.estimated_row_size),
        };
        let rect = if self.is_horizontal() {
            Rect::new(x, y, length, breadth)
        } else {
            Rect::new(x, y, breadth, length)
        };
        let mut layout_info = LayoutInfo::new(node.kind, node.key.clone(), rect);
        layout_info.estimated_size = estimated;
        LayoutNode {
            node: Some(node.clone()),
            valid_rect: layout_info.rect.intersection(&self.requested_rect),
            layout_info,
            children: Vec::new(),
        }
    }

    /// The scroll view's size across the orientation.
    fn cross_size(&self, ctx: &VirtualizerContext<'_>) -> f64 {
        if self.is_horizontal() {
            ctx.size.height
        } else {
            ctx.size.width
        }
    }

    fn update_layout_node(&mut self, key: &Key, old: &LayoutInfo, new: &LayoutInfo) {
        let valid_rect = self.valid_rect;
        if let Some(node) = self.layout_nodes.get_mut(key) {
            // A copy if the laid out tree shares it (react-stately mutates it in place).
            let node = Arc::make_mut(node);
            // Invalidate it by intersecting its valid rectangle with the overall one.
            node.valid_rect = node.valid_rect.intersection(&valid_rect);
            if node.layout_info == *old {
                node.layout_info = new.clone();
            }
        }
    }

    fn should_invalidate_everything(
        &self,
        invalidation: &InvalidationContext<ListLayoutOptions>,
    ) -> bool {
        let options = invalidation
            .layout_options
            .as_ref()
            .unwrap_or(&self.options);
        // Only the cross-axis size (the width of a vertical list) changes how items wrap.
        let cross_axis_changed = if options.orientation == Orientation::Horizontal {
            invalidation.height_changed
        } else {
            invalidation.width_changed
        };
        cross_axis_changed
            || self.options.row_size != options.row_size
            || self.options.orientation != options.orientation
            || self.options.heading_size != options.heading_size
            || self.options.loader_size != options.loader_size
            || self.options.gap != options.gap
            || self.options.padding != options.padding
    }
}

impl Layout for ListLayout {
    type Options = ListLayoutOptions;

    fn visible_layout_infos(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        rect: Rect,
    ) -> Vec<LayoutInfo> {
        // Keep the number of visible rows consistent: snap the rectangle to whole rows (only for
        // real areas, not single points).
        let mut rect = rect;
        if self.length_of(&rect) > 1.0 {
            let row_size = self.row_size();
            let offset = (self.offset_of(&rect) / row_size).floor() * row_size;
            let length = self.length_of(&rect) + self.offset_of(&rect) - offset;
            if self.is_horizontal() {
                rect.x = offset;
            } else {
                rect.y = offset;
            }
            self.set_length(&mut rect, (length / row_size).ceil() * row_size);
        }
        self.layout_if_needed(ctx, rect);

        let mut result = Vec::new();
        let mut stack: Vec<&Arc<LayoutNode>> = self.root_nodes.iter().rev().collect();
        while let Some(node) = stack.pop() {
            if self.is_visible(ctx, node, &rect) {
                result.push(node.layout_info.clone());
                stack.extend(node.children.iter().rev());
            }
        }
        result
    }

    fn layout_info(&mut self, ctx: &VirtualizerContext<'_>, key: &Key) -> Option<LayoutInfo> {
        self.ensure_layout_info(ctx, key);
        self.layout_nodes
            .get(key)
            .map(|node| node.layout_info.clone())
    }

    fn content_size(&self) -> Size {
        self.content_size
    }

    fn should_invalidate_layout_options(
        &self,
        new: &ListLayoutOptions,
        old: &ListLayoutOptions,
    ) -> bool {
        new != old
    }

    fn update(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        invalidation: &InvalidationContext<ListLayoutOptions>,
    ) {
        // Unless everything is invalid, cached layout infos outside the visible area are reused.
        self.invalidate_everything = self.should_invalidate_everything(invalidation);
        if self.invalidate_everything {
            self.requested_rect = ctx.visible_rect;
            self.layout_nodes.clear();
        }
        if let Some(options) = &invalidation.layout_options {
            self.options = options.clone();
        }
        self.root_nodes = self.build_collection(ctx, self.options.padding);

        // Remove deleted nodes.
        if let Some(last) = &self.last_collection
            && !Arc::ptr_eq(last, ctx.collection)
        {
            let removed: Vec<Key> = last
                .keys()
                .filter(|key| ctx.collection.get(key).is_none())
                .cloned()
                .collect();
            for key in removed {
                self.layout_nodes.remove(&key);
            }
        }
        self.last_collection = Some(Arc::clone(ctx.collection));
        self.invalidate_everything = false;
        self.valid_rect = self.requested_rect;
    }

    fn update_item_size(&mut self, ctx: &VirtualizerContext<'_>, key: &Key, size: Size) -> bool {
        let new_length = if self.is_horizontal() {
            size.width
        } else {
            size.height
        };
        let Some(layout_node) = self.layout_nodes.get_mut(key) else {
            // Deleted.
            return false;
        };
        let layout_node = Arc::make_mut(layout_node);
        layout_node.layout_info.estimated_size = false;
        let layout_info = layout_node.layout_info.clone();
        let is_item = layout_node
            .node
            .as_ref()
            .is_some_and(|node| node.kind == NodeKind::Item);
        if self.length_of(&layout_info.rect) == new_length {
            return false;
        }
        // A copy, so later caches are invalidated.
        let mut new_layout_info = layout_info.clone();
        self.set_length(&mut new_layout_info.rect, new_length);
        if let Some(layout_node) = self.layout_nodes.get_mut(key) {
            Arc::make_mut(layout_node).layout_info = new_layout_info.clone();
        }

        // The items after it move: only the ones above stay valid.
        let valid_length = self
            .length_of(&self.valid_rect)
            .min(self.offset_of(&layout_info.rect) - self.offset_of(&self.valid_rect));
        let mut valid_rect = self.valid_rect;
        self.set_length(&mut valid_rect, valid_length);
        self.valid_rect = valid_rect;
        // The requested area grows or shrinks with the item.
        if is_item {
            let mut requested = self.requested_rect;
            let length =
                self.length_of(&requested) + new_length - self.length_of(&layout_info.rect);
            self.set_length(&mut requested, length);
            self.requested_rect = requested;
        }

        // Invalidate the node and its parents.
        self.update_layout_node(key, &layout_info, &new_layout_info);
        let mut parent = layout_info.parent_key.clone();
        while let Some(parent_key) = parent {
            self.update_layout_node(&parent_key, &layout_info, &new_layout_info);
            parent = ctx
                .collection
                .get(&parent_key)
                .and_then(|node| node.parent_key.clone());
        }
        true
    }

    fn scroll_anchor_info(&self, options: Option<&ListLayoutOptions>) -> Option<ScrollAnchorInfo> {
        let options = options.unwrap_or(&self.options);
        // End anchoring works for vertical lists only (as upstream, for now).
        if options.anchor_to != Some(ScrollAnchorEdge::End)
            || options.orientation == Orientation::Horizontal
        {
            return None;
        }
        Some(ScrollAnchorInfo {
            edge: ScrollAnchorEdge::End,
            axis: ScrollAnchorAxis::Y,
            threshold: options.scroll_end_threshold,
            is_anchorable: Some(Arc::new(|info: &LayoutInfo| info.kind != NodeKind::Loader)),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use assertr::prelude::*;

    use super::*;

    fn rows(count: usize) -> Arc<Collection> {
        Arc::new(Collection::build(|b| {
            for i in 0..count {
                b.item(format!("row-{i}"), format!("Row {i}"));
            }
        }))
    }

    /// A layout with the virtualizer's state around it.
    struct Harness {
        layout: ListLayout,
        collection: Arc<Collection>,
        persisted_keys: HashSet<Key>,
        size: Size,
    }

    impl Harness {
        fn new(options: ListLayoutOptions, collection: Arc<Collection>, size: Size) -> Self {
            let mut harness = Self {
                layout: ListLayout::new(options),
                collection,
                persisted_keys: HashSet::new(),
                size,
            };
            // The first pass: the scroll view got its size.
            harness.update(InvalidationContext {
                size_changed: true,
                width_changed: true,
                height_changed: true,
                ..InvalidationContext::default()
            });
            harness
        }

        fn ctx(&self) -> VirtualizerContext<'_> {
            VirtualizerContext {
                collection: &self.collection,
                persisted_keys: &self.persisted_keys,
                size: self.size,
                visible_rect: Rect::new(0.0, 0.0, self.size.width, self.size.height),
                content_size: self.layout.content_size(),
            }
        }

        fn update(&mut self, invalidation: InvalidationContext<ListLayoutOptions>) {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            self.layout.update(&ctx, &invalidation);
        }

        fn visible(&mut self) -> Vec<LayoutInfo> {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            let rect = ctx.visible_rect;
            self.layout.visible_layout_infos(&ctx, rect)
        }

        fn info(&mut self, key: &str) -> LayoutInfo {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            self.layout
                .layout_info(&ctx, &Key::from(key))
                .expect("laid out")
        }

        /// Reports the measured height of `key` and lays out again, as the virtualizer does.
        fn measure(&mut self, key: &str, height: f64) -> bool {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            let changed = self.layout.update_item_size(
                &ctx,
                &Key::from(key),
                Size::new(self.size.width, height),
            );
            if changed {
                self.update(InvalidationContext {
                    item_size_changed: true,
                    ..InvalidationContext::default()
                });
            }
            changed
        }
    }

    fn keys(infos: &[LayoutInfo]) -> Vec<String> {
        infos.iter().map(|info| info.key.to_string()).collect()
    }

    #[test]
    fn stacks_fixed_rows_with_gap_and_padding() {
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: Some(30.0),
                gap: 5.0,
                padding: 10.0,
                ..ListLayoutOptions::default()
            },
            rows(10),
            Size::new(200.0, 100.0),
        );
        // Padding, 10 rows, 9 gaps, padding.
        assert_that!(harness.layout.content_size()).is_equal_to(Size::new(200.0, 365.0));
        let visible = harness.visible();
        // The visible rectangle snaps to whole rows (35px with the gap): 0..105.
        assert_that!(keys(&visible)).is_equal_to(vec![
            "row-0".to_owned(),
            "row-1".to_owned(),
            "row-2".to_owned(),
        ]);
        assert_that!(visible[0].rect).is_equal_to(Rect::new(10.0, 10.0, 180.0, 30.0));
        assert_that!(visible[1].rect).is_equal_to(Rect::new(10.0, 45.0, 180.0, 30.0));
        assert_that!(visible[0].estimated_size).is_false();
        // Far rows are laid out on demand.
        assert_that!(harness.info("row-9").rect.y).is_equal_to(10.0 + 9.0 * 35.0);
    }

    #[test]
    fn an_empty_collection_has_no_content() {
        let harness = Harness::new(
            ListLayoutOptions {
                row_size: Some(30.0),
                padding: 10.0,
                ..ListLayoutOptions::default()
            },
            rows(0),
            Size::new(200.0, 100.0),
        );
        assert_that!(harness.layout.content_size()).is_equal_to(Size::new(200.0, 0.0));
    }

    #[test]
    fn measured_rows_move_the_rows_after_them() {
        let mut harness = Harness::new(
            ListLayoutOptions {
                estimated_row_size: Some(20.0),
                ..ListLayoutOptions::default()
            },
            rows(10),
            Size::new(200.0, 1000.0),
        );
        let visible = harness.visible();
        assert_that!(visible.len()).is_equal_to(10);
        assert_that!(visible.iter().all(|info| info.estimated_size)).is_true();
        assert_that!(harness.layout.content_size().height).is_equal_to(200.0);

        assert_that!(harness.measure("row-0", 50.0)).is_true();
        // The same size again changes nothing.
        assert_that!(harness.measure("row-0", 50.0)).is_false();
        let visible = harness.visible();
        assert_that!(visible[0].rect.height).is_equal_to(50.0);
        assert_that!(visible[0].estimated_size).is_false();
        assert_that!(visible[1].rect.y).is_equal_to(50.0);
        assert_that!(visible[9].rect.y).is_equal_to(50.0 + 8.0 * 20.0);
        assert_that!(harness.layout.content_size().height).is_equal_to(230.0);
    }

    /// Measured sizes survive changes of the anchoring options (a `VirtualList` turns
    /// `anchor_to` on and off as the user scrolls to and away from the end), but not a change of
    /// the width (rows wrap differently).
    #[test]
    fn anchoring_changes_keep_measured_sizes() {
        let options = ListLayoutOptions {
            estimated_row_size: Some(20.0),
            ..ListLayoutOptions::default()
        };
        let mut harness = Harness::new(options.clone(), rows(10), Size::new(200.0, 1000.0));
        harness.visible();
        harness.measure("row-0", 50.0);
        harness.measure("row-1", 70.0);

        for new_options in [
            ListLayoutOptions {
                anchor_to: Some(ScrollAnchorEdge::End),
                ..options.clone()
            },
            ListLayoutOptions {
                anchor_to: Some(ScrollAnchorEdge::End),
                scroll_end_threshold: 10.0,
                ..options.clone()
            },
            options.clone(),
        ] {
            harness.update(InvalidationContext {
                layout_options_changed: true,
                layout_options: Some(new_options),
                ..InvalidationContext::default()
            });
            let visible = harness.visible();
            assert_that!(visible[0].rect.height).is_equal_to(50.0);
            assert_that!(visible[0].estimated_size).is_false();
            assert_that!(visible[1].rect.height).is_equal_to(70.0);
            assert_that!(visible[2].rect.y).is_equal_to(120.0);
        }

        // A new width: every row is estimated again.
        harness.size = Size::new(150.0, 1000.0);
        harness.update(InvalidationContext {
            size_changed: true,
            width_changed: true,
            ..InvalidationContext::default()
        });
        let visible = harness.visible();
        assert_that!(visible[0].rect.height).is_equal_to(20.0);
        assert_that!(visible[0].estimated_size).is_true();
        assert_that!(visible[0].rect.width).is_equal_to(150.0);

        // A new height alone (more or less of the list visible) keeps them.
        harness.measure("row-0", 50.0);
        harness.size = Size::new(150.0, 500.0);
        harness.update(InvalidationContext {
            size_changed: true,
            height_changed: true,
            ..InvalidationContext::default()
        });
        assert_that!(harness.info("row-0").rect.height).is_equal_to(50.0);
    }

    #[test]
    fn a_changed_fixed_row_size_invalidates_everything() {
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: Some(30.0),
                ..ListLayoutOptions::default()
            },
            rows(10),
            Size::new(200.0, 1000.0),
        );
        harness.visible();
        harness.update(InvalidationContext {
            layout_options_changed: true,
            layout_options: Some(ListLayoutOptions {
                row_size: Some(40.0),
                ..ListLayoutOptions::default()
            }),
            ..InvalidationContext::default()
        });
        assert_that!(harness.layout.content_size().height).is_equal_to(400.0);
        assert_that!(harness.visible()[1].rect.y).is_equal_to(40.0);
    }

    #[test]
    fn removed_rows_are_forgotten() {
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: Some(30.0),
                ..ListLayoutOptions::default()
            },
            rows(10),
            Size::new(200.0, 1000.0),
        );
        harness.visible();
        harness.collection = rows(5);
        harness.update(InvalidationContext::default());
        assert_that!(keys(&harness.visible()).len()).is_equal_to(5);
        assert_that!(harness.layout.content_size().height).is_equal_to(150.0);
        let collection = Arc::clone(&harness.collection);
        let persisted_keys = HashSet::new();
        let ctx = VirtualizerContext {
            collection: &collection,
            persisted_keys: &persisted_keys,
            ..harness.ctx()
        };
        assert_that!(harness.layout.layout_info(&ctx, &Key::from("row-7"))).is_none();
    }

    #[test]
    fn end_anchoring_is_for_vertical_lists() {
        let layout = ListLayout::new(ListLayoutOptions {
            anchor_to: Some(ScrollAnchorEdge::End),
            scroll_end_threshold: 12.0,
            ..ListLayoutOptions::default()
        });
        let info = layout.scroll_anchor_info(None).expect("anchored");
        assert_that!(info.threshold).is_equal_to(12.0);
        let horizontal = ListLayoutOptions {
            anchor_to: Some(ScrollAnchorEdge::End),
            orientation: Orientation::Horizontal,
            ..ListLayoutOptions::default()
        };
        assert_that!(layout.scroll_anchor_info(Some(&horizontal)).is_none()).is_true();
        assert_that!(
            layout
                .scroll_anchor_info(Some(&ListLayoutOptions::default()))
                .is_none()
        )
        .is_true();
    }

    /// The cost of an append to a long log (as `VirtualList` rebuilds its collection): run with
    /// `cargo test --release -p leptonic --features full --lib timing_ -- --ignored --nocapture`.
    #[test]
    #[ignore = "timing"]
    fn timing_appends_to_a_long_list() {
        let build = |count: usize| {
            Arc::new(Collection::build(|b| {
                for i in 0..count {
                    b.item(Key::from(i64::try_from(i).expect("a small count")), "");
                }
            }))
        };
        let mut harness = Harness::new(
            ListLayoutOptions {
                estimated_row_size: Some(20.0),
                ..ListLayoutOptions::default()
            },
            build(20_000),
            Size::new(800.0, 600.0),
        );
        let _ = harness.visible();
        let mut build_total = std::time::Duration::ZERO;
        let mut layout_total = std::time::Duration::ZERO;
        for round in 1..=20 {
            let start = std::time::Instant::now();
            let collection = build(20_000 + round * 50);
            build_total += start.elapsed();
            let start = std::time::Instant::now();
            harness.collection = collection;
            harness.update(InvalidationContext::default());
            let _ = harness.visible();
            layout_total += start.elapsed();
        }
        println!(
            "per append: build {:?}, layout {:?}",
            build_total / 20,
            layout_total / 20
        );
    }
}
