// Upstream: react-stately/src/layout/ListLayout.ts @ 99e6102368
// Upstream: @react-spectrum/ai/src/ListLayout.ts @ 99e6102368
// (react-stately has no tests of `ListLayout`; the tests below follow its implementation.)
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

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
// - Row and heading sizes are an [`ItemSize`] each: fixed or estimated (react-stately: a
//   `rowSize` and an `estimatedRowSize`, where a fixed size silently wins over an estimate).
// - The anchoring of react-spectrum's AI `ListLayout` (`anchorTo: 'end'` + `scrollEndThreshold`,
//   vertical only) is part of this layout, as `anchor_to_end: Some(EndAnchor { threshold })`;
//   like that one, only a change of the cross-axis size (the width of a vertical list) throws
//   away measured sizes. Unlike that one, a change of `anchor_to_end` doesn't either: there it
//   is a fixed option, here `VirtualList` switches it whenever the user scrolls to or away from
//   the end, and dropping every measured size then made the content jump.
// - Loaders are always shown as loading (collections have no `isLoading` per loader node).
//
// ## DIFFERENT BEHAVIOR
// - Rows before the requested area are skipped only if they were never laid out. Upstream
//   checks whether their cached position is valid (and checks the section for section rows):
//   invalid positions must be rebuilt while keeping measured sizes, not replaced by estimates.
// - `buildSectionHeader` subtracts the header's position from the breadth twice: headers were
//   narrower than the rows by the padding.
// - Nodes are compared by content (everything but their position in the collection), not by
//   identity: our collections build new nodes on every change, and a log dropping its first
//   lines moves every node, which re-estimated (and re-measured) every row.
// - Nodes with children (sections) are laid out again whenever the collection changes (a
//   changed child doesn't change its parent node); their children are reused if valid.
// - The visible root nodes are found by binary search over their offsets, plus the always
//   visible ones (headers, loaders, sticky) and those of persisted keys (computed once per
//   call); react-stately checks every laid out root node, all of them once the requested area
//   covers the whole list (after Home/End, or a log that was at both ends).
// - When only the end of the requested area moves (scrolling down, laying out everything for a
//   far key), the build continues from its last laid out node instead of starting over.
// - The top-level node count and loaders are counted once per collection (react-stately: an
//   array of all nodes and a filter per build); deleted nodes are only searched if the build
//   didn't visit every cached node.
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

/// The size of rows or headings along a list's orientation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemSize {
    /// Every one has this size: the layout knows where each is without rendering it.
    Fixed(f64),
    /// Sizes vary: each starts at this estimate and is measured once rendered.
    Estimated(f64),
}

impl ItemSize {
    /// The fixed size, or the estimate.
    pub fn size(self) -> f64 {
        match self {
            Self::Fixed(size) | Self::Estimated(size) => size,
        }
    }

    /// The fixed size; `None` for estimated sizes.
    pub fn fixed(self) -> Option<f64> {
        match self {
            Self::Fixed(size) => Some(size),
            Self::Estimated(_) => None,
        }
    }
}

impl Default for ItemSize {
    /// Estimated at 48px (react-stately's default).
    fn default() -> Self {
        Self::Estimated(DEFAULT_SIZE)
    }
}

/// Keeps the viewport at the end of a vertical list while it is there and the content changes
/// (e.g. a log or chat appended at the end).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EndAnchor {
    /// Within this distance (px) from the end, the viewport counts as being at the end.
    pub threshold: f64,
}

/// Options of a [`ListLayout`].
#[derive(Debug, Clone, PartialEq)]
pub struct ListLayoutOptions {
    /// The direction the items stack in (and the collection usually scrolls).
    pub orientation: Orientation,
    /// The size of rows along the orientation. Default: estimated at 48px.
    pub row_size: ItemSize,
    /// The size of section headers along the orientation. Default: estimated at 48px.
    pub heading_size: ItemSize,
    /// The size of a loader ("load more"). Default: the row size.
    pub loader_size: Option<f64>,
    /// The gap between items.
    pub gap: f64,
    /// The padding around the list.
    pub padding: f64,
    /// Keeps the viewport at the end while it is there (vertical lists only: ignored, with a
    /// warning in debug builds, in horizontal ones, as upstream).
    pub anchor_to_end: Option<EndAnchor>,
}

impl Default for ListLayoutOptions {
    fn default() -> Self {
        Self {
            orientation: Orientation::Vertical,
            row_size: ItemSize::default(),
            heading_size: ItemSize::default(),
            loader_size: None,
            gap: 0.0,
            padding: 0.0,
            anchor_to_end: None,
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

/// How far the root nodes are laid out.
#[derive(Debug, Clone)]
enum BuiltTo {
    /// To the end of the collection.
    End,
    /// Up to the requested area's end: the rest is estimated.
    Partial(Resume),
    /// Sizes changed since: lay out again from the start.
    Unknown,
}

/// Where a partial build continues: at its last laid out root node, which is laid out again (it
/// may straddle the end of the requested area, e.g. a section laid out only partially).
#[derive(Debug, Clone)]
struct Resume {
    key: Key,
    /// Its index among the top-level nodes.
    index: usize,
    offset: f64,
    /// Its index in the root nodes (later root nodes are estimated loaders).
    root_index: usize,
}

/// The top-level nodes of a collection, counted once per collection: a partial build estimates
/// the nodes after the requested area and places the loaders among them.
#[derive(Debug, Clone)]
struct TopLevel {
    collection: Arc<Collection>,
    count: usize,
    /// The loaders, by index.
    loaders: Vec<(usize, Key)>,
}

/// Whether two versions of a node show the same thing: they differ at most in their position in
/// the collection (index, document position, siblings). react-stately compares node identity;
/// our collections build new nodes on every change, and a log dropping its first lines changes
/// every node's position.
fn same_content(a: &Node, b: &Node) -> bool {
    a.key == b.key
        && a.kind == b.kind
        && a.level == b.level
        && a.has_child_nodes == b.has_child_nodes
        && a.is_disabled == b.is_disabled
        && a.parent_key == b.parent_key
        && a.first_child_key == b.first_child_key
        && a.last_child_key == b.last_child_key
        && a.col_index == b.col_index
        && a.col_span == b.col_span
        && a.disabled_behavior == b.disabled_behavior
        && a.text_value == b.text_value
        && a.aria_label == b.aria_label
        && a.link == b.link
}

/// The top-level nodes from `key` on.
fn siblings_from<'a>(
    collection: &'a Collection,
    key: Option<&Key>,
) -> impl Iterator<Item = &'a Node> {
    std::iter::successors(key.and_then(|key| collection.get(key)), |node| {
        node.next_key.as_ref().and_then(|next| collection.get(next))
    })
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
    /// The laid out top-level nodes, in order (their offsets ascend).
    root_nodes: Vec<Arc<LayoutNode>>,
    /// The indices of the root nodes that are always visible (headers, loaders, sticky ones).
    always_visible: Vec<usize>,
    built_to: BuiltTo,
    top_level: Option<TopLevel>,
    invalidate_everything: bool,
    /// The collection changed in the current update: nodes with children are laid out again (a
    /// changed child doesn't change its parent node).
    collection_changed: bool,
    /// The nodes the current build laid out (or reused).
    visited: usize,
    /// The rectangle containing currently valid layout infos.
    valid_rect: Rect,
    /// The rectangle of requested layout infos so far.
    requested_rect: Rect,
}

/// Warns about options the layout ignores (as upstream).
fn warn_about_ignored(options: &ListLayoutOptions) {
    if options.anchor_to_end.is_some() && options.orientation == Orientation::Horizontal {
        crate::utils::dev_warn!(
            "ListLayout: `anchor_to_end` is only supported in vertical lists and is ignored in \
             horizontal ones."
        );
    }
}

impl ListLayout {
    pub fn new(options: ListLayoutOptions) -> Self {
        warn_about_ignored(&options);
        Self {
            options,
            layout_nodes: HashMap::new(),
            content_size: Size::default(),
            last_collection: None,
            root_nodes: Vec::new(),
            always_visible: Vec::new(),
            built_to: BuiltTo::Unknown,
            top_level: None,
            invalidate_everything: false,
            collection_changed: false,
            visited: 0,
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

    /// The size of a row not laid out yet, with the gap after it.
    fn row_size(&self) -> f64 {
        self.options.row_size.size() + self.options.gap
    }

    fn ensure_layout_info(&mut self, ctx: &VirtualizerContext<'_>, key: &Key) -> bool {
        // Outside the area laid out so far (e.g. Home/End): lay out everything.
        if !self.layout_nodes.contains_key(key)
            && self.requested_rect.area() < self.content_size.area()
            && self.last_collection.is_some()
        {
            self.request(ctx, Rect::new(0.0, 0.0, f64::INFINITY, f64::INFINITY));
            self.requested_rect =
                Rect::new(0.0, 0.0, self.content_size.width, self.content_size.height);
            return true;
        }
        false
    }

    /// Lays out `rect` too: continues the last build if only the end of the requested area moves
    /// (scrolling down a list), else lays out again.
    fn request(&mut self, ctx: &VirtualizerContext<'_>, rect: Rect) {
        let requested = self.requested_rect.union(&rect);
        let same_start = self.offset_of(&requested) == self.offset_of(&self.requested_rect);
        self.requested_rect = requested;
        match &self.built_to {
            BuiltTo::End if same_start => {}
            BuiltTo::Partial(resume) if same_start => {
                let resume = resume.clone();
                self.root_nodes.truncate(resume.root_index);
                self.always_visible
                    .retain(|index| *index < resume.root_index);
                self.build_from(ctx, Some(&resume.key), resume.index, resume.offset);
            }
            _ => self.build_collection(ctx),
        }
    }

    fn layout_if_needed(&mut self, ctx: &VirtualizerContext<'_>, rect: Rect) {
        if self.last_collection.is_none() {
            return;
        }
        if !self.requested_rect.contains_rect(&rect) {
            self.request(ctx, rect);
        }
        // The persisted keys must be available.
        for key in ctx.persisted_keys {
            if self.ensure_layout_info(ctx, key) {
                return;
            }
        }
    }

    /// The persisted keys and their ancestors (react-stately asks the virtualizer per node).
    fn persisted_with_ancestors(&self, ctx: &VirtualizerContext<'_>) -> HashSet<Key> {
        let mut keys = HashSet::new();
        for key in ctx.persisted_keys {
            let mut current = Some(key.clone());
            while let Some(key) = current {
                current = self
                    .layout_nodes
                    .get(&key)
                    .and_then(|node| node.layout_info.parent_key.clone());
                if !keys.insert(key) {
                    break;
                }
            }
        }
        keys
    }

    /// The index of the root node of `key`, found by its offset.
    fn root_index_of(&self, key: &Key) -> Option<usize> {
        let offset = self.offset_of(&self.layout_nodes.get(key)?.layout_info.rect);
        let first = self
            .root_nodes
            .partition_point(|node| self.offset_of(&node.layout_info.rect) < offset);
        self.root_nodes[first..]
            .iter()
            .take_while(|node| self.offset_of(&node.layout_info.rect) == offset)
            .position(|node| node.layout_info.key == *key)
            .map(|position| first + position)
    }

    fn is_visible(node: &LayoutNode, rect: &Rect, persisted: &HashSet<Key>) -> bool {
        node.layout_info.rect.intersects(rect)
            || node.layout_info.is_sticky
            || matches!(node.layout_info.kind, NodeKind::Header | NodeKind::Loader)
            || persisted.contains(&node.layout_info.key)
    }

    /// Adds `node` and its visible descendants, if visible.
    fn add_visible(
        node: &LayoutNode,
        rect: &Rect,
        persisted: &HashSet<Key>,
        result: &mut Vec<LayoutInfo>,
    ) {
        if Self::is_visible(node, rect, persisted) {
            result.push(node.layout_info.clone());
            for child in &node.children {
                Self::add_visible(child, rect, persisted, result);
            }
        }
    }

    /// The cached layout node of `node` at `offset`, if still valid.
    fn valid_cached(&self, node: &Node, offset: f64) -> Option<&Arc<LayoutNode>> {
        self.layout_nodes.get(&node.key).filter(|cached| {
            !(self.invalidate_everything || self.collection_changed && node.has_child_nodes)
                && offset == self.offset_of(&cached.layout_info.rect)
                && cached.layout_info.rect.intersects(&self.valid_rect)
                && cached
                    .valid_rect
                    .contains_rect(&cached.layout_info.rect.intersection(&self.requested_rect))
                && cached
                    .node
                    .as_ref()
                    .is_some_and(|cached| same_content(cached, node))
        })
    }

    /// Lays out the root nodes from the start.
    fn build_collection(&mut self, ctx: &VirtualizerContext<'_>) {
        self.root_nodes.clear();
        self.always_visible.clear();
        self.visited = 0;
        self.build_from(ctx, ctx.collection.first_key(), 0, self.options.padding);
    }

    /// Lays out the root nodes from `start` (the `index`th top-level node, at `offset`) to the end
    /// of the requested area, and estimates the rest.
    fn build_from(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        start: Option<&Key>,
        index: usize,
        mut offset: f64,
    ) {
        let collection = ctx.collection;
        let is_empty = collection.size() == 0;
        if is_empty {
            offset = 0.0;
        }
        let padding = self.options.padding;
        let gap = self.options.gap;
        let row_size = self.row_size();
        self.built_to = BuiltTo::End;
        // Sections may be laid out partially wherever the requested area ends: a later request
        // lays out from the start again.
        let mut has_sections = false;
        for (index, node) in (index..).zip(siblings_from(collection, start)) {
            // Rows before the requested area are skipped unless cached.
            if node.kind == NodeKind::Item
                && offset + row_size < self.offset_of(&self.requested_rect)
                && !self.layout_nodes.contains_key(&node.key)
            {
                offset += row_size;
                continue;
            }
            let node_offset = offset;
            has_sections |= node.kind == NodeKind::Section;
            let layout_node = if self.is_horizontal() {
                self.build_child(ctx, node, offset, padding, None)
            } else {
                self.build_child(ctx, node, padding, offset, None)
            };
            offset = self.max_offset_of(&layout_node.layout_info.rect) + gap;
            self.push_root(layout_node);

            // Past the requested area: place the remaining loaders at their estimated
            // positions (so they persist) and estimate the rest.
            if matches!(node.kind, NodeKind::Item | NodeKind::Loader)
                && offset > self.max_offset_of(&self.requested_rect)
            {
                self.built_to = BuiltTo::Partial(Resume {
                    key: node.key.clone(),
                    index,
                    offset: node_offset,
                    root_index: self.root_nodes.len() - 1,
                });
                let (count, loaders) = self.top_level(collection);
                let mut last_processed = index;
                for (loader_index, loader_key) in
                    loaders.iter().filter(|(loader, _)| *loader > index)
                {
                    #[allow(clippy::cast_precision_loss)]
                    {
                        offset += (loader_index - last_processed - 1) as f64 * row_size;
                    }
                    let Some(loader_node) = collection.get(loader_key) else {
                        continue;
                    };
                    let loader = if self.is_horizontal() {
                        self.build_child(ctx, loader_node, offset, padding, None)
                    } else {
                        self.build_child(ctx, loader_node, padding, offset, None)
                    };
                    offset = self.max_offset_of(&loader.layout_info.rect);
                    self.push_root(loader);
                    last_processed = *loader_index;
                }
                #[allow(clippy::cast_precision_loss)]
                {
                    offset += count.saturating_sub(last_processed + 1) as f64 * row_size;
                }
                break;
            }
        }
        if has_sections {
            self.built_to = BuiltTo::Unknown;
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
    }

    fn push_root(&mut self, layout_node: Arc<LayoutNode>) {
        let info = &layout_node.layout_info;
        if info.is_sticky || matches!(info.kind, NodeKind::Header | NodeKind::Loader) {
            self.always_visible.push(self.root_nodes.len());
        }
        self.root_nodes.push(layout_node);
    }

    /// The number of top-level nodes and the loaders among them (counted once per collection).
    fn top_level(&mut self, collection: &Arc<Collection>) -> (usize, Vec<(usize, Key)>) {
        if let Some(top_level) = &self.top_level
            && Arc::ptr_eq(&top_level.collection, collection)
        {
            return (top_level.count, top_level.loaders.clone());
        }
        let mut count = 0;
        let mut loaders = Vec::new();
        for node in collection.iter() {
            if node.kind == NodeKind::Loader {
                loaders.push((count, node.key.clone()));
            }
            count += 1;
        }
        self.top_level = Some(TopLevel {
            collection: Arc::clone(collection),
            count,
            loaders: loaders.clone(),
        });
        (count, loaders)
    }

    fn build_child(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        node: &Node,
        x: f64,
        y: f64,
        parent_key: Option<Key>,
    ) -> Arc<LayoutNode> {
        self.visited += 1;
        let offset = if self.is_horizontal() { x } else { y };
        if let Some(cached) = self.valid_cached(node, offset) {
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
            .unwrap_or_else(|| self.options.row_size.size());
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
        let mut children = Vec::new();
        let mut child_nodes = ctx.collection.children(&node.key);
        for child in child_nodes.by_ref() {
            // Cached rows retain their measurements even when their positions need rebuilding.
            if offset + row_size < self.offset_of(&self.requested_rect)
                && !self.layout_nodes.contains_key(&child.key)
            {
                offset += row_size;
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
                    offset += child_nodes.by_ref().count() as f64 * row_size;
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

    /// The length of a node of variable size: its previous one (estimated again if the breadth
    /// or the node's content changed), else the estimate.
    fn variable_length(&self, node: &Node, breadth: f64, estimate: f64) -> (f64, bool) {
        if let Some(previous) = self.layout_nodes.get(&node.key) {
            let changed = !previous
                .node
                .as_ref()
                .is_some_and(|previous| same_content(previous, node));
            (
                self.length_of(&previous.layout_info.rect),
                breadth != self.breadth_of(&previous.layout_info.rect)
                    || changed
                    || previous.layout_info.estimated_size,
            )
        } else {
            (estimate, true)
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
            ItemSize::Fixed(size) => (size, false),
            ItemSize::Estimated(estimate) => self.variable_length(node, breadth, estimate),
        };
        // As wide as the rows (react-stately subtracts the position twice: UPSTREAM BUG FIXED).
        let rect = if self.is_horizontal() {
            Rect::new(x, y, length, breadth)
        } else {
            Rect::new(x, y, breadth, length)
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
            ItemSize::Fixed(size) => (size, false),
            ItemSize::Estimated(estimate) => self.variable_length(node, breadth, estimate),
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
            || self.options.row_size.fixed() != options.row_size.fixed()
            || self.options.orientation != options.orientation
            || self.options.heading_size.fixed() != options.heading_size.fixed()
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

        // The root nodes overlapping the rectangle along the orientation (found by their
        // offsets), the ones always visible, and those of persisted keys (react-stately checks
        // every root node).
        let persisted = self.persisted_with_ancestors(ctx);
        let first = self.root_nodes.partition_point(|node| {
            self.max_offset_of(&node.layout_info.rect) < self.offset_of(&rect)
        });
        let end = first
            + self.root_nodes[first..].partition_point(|node| {
                self.offset_of(&node.layout_info.rect) <= self.max_offset_of(&rect)
            });
        let mut roots: Vec<usize> = self
            .always_visible
            .iter()
            .copied()
            .filter(|index| !(first..end).contains(index))
            .collect();
        roots.extend(
            persisted
                .iter()
                .filter(|key| {
                    self.layout_nodes
                        .get(*key)
                        .is_some_and(|node| node.layout_info.parent_key.is_none())
                })
                .filter_map(|key| self.root_index_of(key))
                .filter(|index| !(first..end).contains(index)),
        );
        roots.extend(first..end);
        roots.sort_unstable();
        roots.dedup();

        let mut result = Vec::with_capacity(roots.len());
        for index in roots {
            Self::add_visible(&self.root_nodes[index], &rect, &persisted, &mut result);
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
        if let Some(options) = &invalidation.layout_options
            && *options != self.options
        {
            warn_about_ignored(options);
            self.options = options.clone();
        }
        self.collection_changed = self
            .last_collection
            .as_ref()
            .is_some_and(|last| !Arc::ptr_eq(last, ctx.collection));
        self.build_collection(ctx);

        // Remove deleted nodes: only needed if the build didn't visit every cached node
        // (react-stately looks up every key of the previous collection).
        if self.collection_changed && self.visited != self.layout_nodes.len() {
            self.layout_nodes
                .retain(|key, _| ctx.collection.contains_key(key));
        }
        self.last_collection = Some(Arc::clone(ctx.collection));
        self.invalidate_everything = false;
        self.collection_changed = false;
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
        // The nodes after it move: the next build starts over.
        self.built_to = BuiltTo::Unknown;

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
        let anchor = options.anchor_to_end?;
        // End anchoring works for vertical lists only (as upstream, for now).
        if options.orientation == Orientation::Horizontal {
            return None;
        }
        Some(ScrollAnchorInfo {
            edge: ScrollAnchorEdge::End,
            axis: ScrollAnchorAxis::Y,
            threshold: anchor.threshold,
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
        /// The scroll position.
        scroll_y: f64,
    }

    impl Harness {
        fn new(options: ListLayoutOptions, collection: Arc<Collection>, size: Size) -> Self {
            let mut harness = Self {
                layout: ListLayout::new(options),
                collection,
                persisted_keys: HashSet::new(),
                size,
                scroll_y: 0.0,
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
                visible_rect: Rect::new(0.0, self.scroll_y, self.size.width, self.size.height),
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

        fn info(&mut self, key: impl Into<Key>) -> LayoutInfo {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            self.layout
                .layout_info(&ctx, &key.into())
                .expect("laid out")
        }

        /// Reports the measured height of `key` and lays out again, as the virtualizer does.
        fn measure(&mut self, key: impl Into<Key>, height: f64) -> bool {
            let collection = Arc::clone(&self.collection);
            let persisted_keys = self.persisted_keys.clone();
            let ctx = VirtualizerContext {
                collection: &collection,
                persisted_keys: &persisted_keys,
                ..self.ctx()
            };
            let changed =
                self.layout
                    .update_item_size(&ctx, &key.into(), Size::new(self.size.width, height));
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
                row_size: ItemSize::Fixed(30.0),
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
                row_size: ItemSize::Fixed(30.0),
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
                row_size: ItemSize::Estimated(20.0),
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
    /// `anchor_to_end` on and off as the user scrolls to and away from the end), but not a change of
    /// the width (rows wrap differently).
    #[test]
    fn anchoring_changes_keep_measured_sizes() {
        let options = ListLayoutOptions {
            row_size: ItemSize::Estimated(20.0),
            ..ListLayoutOptions::default()
        };
        let mut harness = Harness::new(options.clone(), rows(10), Size::new(200.0, 1000.0));
        harness.visible();
        harness.measure("row-0", 50.0);
        harness.measure("row-1", 70.0);

        for new_options in [
            ListLayoutOptions {
                anchor_to_end: Some(EndAnchor::default()),
                ..options.clone()
            },
            ListLayoutOptions {
                anchor_to_end: Some(EndAnchor { threshold: 10.0 }),
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
                row_size: ItemSize::Fixed(30.0),
                ..ListLayoutOptions::default()
            },
            rows(10),
            Size::new(200.0, 1000.0),
        );
        harness.visible();
        harness.update(InvalidationContext {
            layout_options_changed: true,
            layout_options: Some(ListLayoutOptions {
                row_size: ItemSize::Fixed(40.0),
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
                row_size: ItemSize::Fixed(30.0),
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

    /// Rows whose position in the collection changes (a log dropping its first lines) keep their
    /// measured sizes: only a change of their content estimates them again.
    #[test]
    fn rows_keep_their_measured_sizes_when_rows_before_them_go() {
        let rows_in = |range: std::ops::Range<usize>| {
            Arc::new(Collection::build(|b| {
                for i in range {
                    b.item(format!("row-{i}"), format!("Row {i}"));
                }
            }))
        };
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: ItemSize::Estimated(20.0),
                ..ListLayoutOptions::default()
            },
            rows_in(0..10),
            Size::new(200.0, 1000.0),
        );
        for info in harness.visible() {
            harness.measure(info.key, 30.0);
        }

        harness.collection = rows_in(1..11);
        harness.update(InvalidationContext::default());
        let visible = harness.visible();
        assert_that!(visible[0].key.to_string()).is_equal_to("row-1".to_owned());
        assert_that!(visible[0].rect).is_equal_to(Rect::new(0.0, 0.0, 200.0, 30.0));
        assert_that!(visible[8].estimated_size).is_false();
        // The new row only.
        assert_that!(visible[9].estimated_size).is_true();
        assert_that!(visible[9].rect.y).is_equal_to(270.0);
    }

    /// Section headers are as wide as the rows (react-stately subtracts the padding twice).
    #[test]
    fn section_headers_are_as_wide_as_the_rows() {
        let collection = Arc::new(Collection::build(|b| {
            b.section("section", |s| {
                s.header("header", "Header");
                s.item("row", "Row");
            });
        }));
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: ItemSize::Fixed(30.0),
                heading_size: ItemSize::Fixed(20.0),
                padding: 10.0,
                ..ListLayoutOptions::default()
            },
            collection,
            Size::new(200.0, 100.0),
        );
        harness.visible();
        assert_that!(harness.info("header").rect).is_equal_to(Rect::new(10.0, 10.0, 180.0, 20.0));
        assert_that!(harness.info("row").rect).is_equal_to(Rect::new(10.0, 30.0, 180.0, 30.0));
    }

    /// A section laid out again keeps its laid out rows before the requested area (with their
    /// measured sizes) instead of estimating them (react-stately checks the section instead of
    /// the row).
    #[test]
    fn sections_keep_laid_out_rows_before_the_requested_area() {
        let section = |count: usize| {
            Arc::new(Collection::build(|b| {
                b.section("section", |s| {
                    for i in 0..count {
                        s.item(format!("row-{i}"), format!("Row {i}"));
                    }
                });
            }))
        };
        let mut harness = Harness::new(
            ListLayoutOptions {
                row_size: ItemSize::Estimated(20.0),
                ..ListLayoutOptions::default()
            },
            section(20),
            Size::new(200.0, 100.0),
        );
        // Everything laid out, the first 5 rows measured at 50px.
        harness.info("row-19");
        for i in 0..5 {
            harness.measure(format!("row-{i}"), 50.0);
        }
        assert_that!(harness.layout.content_size().height).is_equal_to(550.0);

        // The requested area starts after them (as the view had moved there); a row is appended.
        harness.layout.requested_rect = Rect::new(0.0, 400.0, 200.0, 150.0);
        harness.collection = section(21);
        harness.update(InvalidationContext::default());
        assert_that!(harness.layout.content_size().height).is_equal_to(570.0);
        harness.collection = section(22);
        harness.update(InvalidationContext::default());
        assert_that!(harness.layout.content_size().height).is_equal_to(590.0);
    }

    #[test]
    fn end_anchoring_is_for_vertical_lists() {
        let layout = ListLayout::new(ListLayoutOptions {
            anchor_to_end: Some(EndAnchor { threshold: 12.0 }),
            ..ListLayoutOptions::default()
        });
        let info = layout.scroll_anchor_info(None).expect("anchored");
        assert_that!(info.threshold).is_equal_to(12.0);
        let horizontal = ListLayoutOptions {
            anchor_to_end: Some(EndAnchor { threshold: 12.0 }),
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

    /// A collection of `range`'s rows, keyed by number (as a log's lines).
    fn numbered(range: std::ops::Range<i64>) -> Arc<Collection> {
        Arc::new(Collection::build(|b| {
            for i in range {
                b.item(Key::from(i), "");
            }
        }))
    }

    /// The costs of a long log (20,000 rows of estimated size, as `VirtualList` lays them out),
    /// without building the collections: run with `cargo test --release -p leptonic --features
    /// full --lib timing_ -- --ignored --nocapture`.
    #[test]
    #[ignore = "timing"]
    fn timing_a_long_list() {
        const ROWS: i64 = 20_000;
        const ROUNDS: i64 = 20;
        let options = ListLayoutOptions {
            row_size: ItemSize::Estimated(20.0),
            ..ListLayoutOptions::default()
        };
        let size = Size::new(800.0, 600.0);
        // Rows are measured a pixel taller than estimated.
        let measure_visible = |harness: &mut Harness| {
            for info in harness.visible() {
                if info.estimated_size {
                    harness.measure(info.key, 21.0);
                }
            }
        };
        let time = |f: &mut dyn FnMut()| {
            let start = std::time::Instant::now();
            f();
            start.elapsed()
        };
        let appended: Vec<_> = (1..=ROUNDS)
            .map(|round| numbered(0..ROWS + round * 50))
            .collect();
        let build = time(&mut || {
            let _ = numbered(0..ROWS);
        });

        // Appends of 50 rows, the view at the top (only the start laid out).
        let mut harness = Harness::new(options.clone(), numbered(0..ROWS), size);
        measure_visible(&mut harness);
        let mut append_at_top = std::time::Duration::ZERO;
        for collection in &appended {
            harness.collection = Arc::clone(collection);
            append_at_top += time(&mut || {
                harness.update(InvalidationContext::default());
                measure_visible(&mut harness);
            });
        }

        // The view at the end after it was at the start (a log following its end): everything is
        // laid out.
        let mut harness = Harness::new(options.clone(), numbered(0..ROWS), size);
        measure_visible(&mut harness);
        harness.scroll_y = harness.layout.content_size().height - size.height;
        measure_visible(&mut harness);
        let mut append_at_end = std::time::Duration::ZERO;
        for collection in &appended {
            harness.collection = Arc::clone(collection);
            append_at_end += time(&mut || {
                harness.update(InvalidationContext::default());
                harness.scroll_y = harness.layout.content_size().height - size.height;
                measure_visible(&mut harness);
            });
        }
        // A ring buffer: 50 rows out, 50 rows in.
        let end = ROWS + ROUNDS * 50;
        let trimmed: Vec<_> = (1..=ROUNDS)
            .map(|round| numbered(round * 50..end + round * 50))
            .collect();
        let mut trim = std::time::Duration::ZERO;
        // The view stays on the same rows (in the middle of the log): the rows in it estimated
        // again after a trim.
        let middle = Key::from(ROWS / 2);
        harness.scroll_y = harness.info(middle.clone()).rect.y;
        measure_visible(&mut harness);
        let mut reestimated = 0;
        for collection in &trimmed {
            harness.collection = Arc::clone(collection);
            trim += time(&mut || {
                harness.update(InvalidationContext::default());
                harness.scroll_y = harness.info(middle.clone()).rect.y;
                reestimated = harness
                    .visible()
                    .iter()
                    .filter(|info| info.estimated_size)
                    .count();
                measure_visible(&mut harness);
            });
        }
        // Scroll frames over the laid out list, the first row persisted (e.g. focused).
        harness.persisted_keys = HashSet::from([Key::from(ROUNDS * 50)]);
        let scroll_frames = time(&mut || {
            for round in 1..=200 {
                harness.scroll_y = f64::from(round) * 1000.0;
                let _ = harness.visible();
            }
        });

        // Scrolling down a fresh list, a view at a time, measuring the rows.
        let mut harness = Harness::new(options, numbered(0..ROWS), size);
        let scroll_down = time(&mut || {
            for round in 1..=200 {
                harness.scroll_y = f64::from(round) * size.height;
                measure_visible(&mut harness);
            }
        });

        let rounds = u32::try_from(ROUNDS).expect("a few rounds");
        println!(
            "20,000 rows: collection build {build:?}; layout: append at the top {:?}, append at \
             the end {:?}, trim and append {:?} ({reestimated} rows in view estimated again), \
             scroll frame {:?}, scroll down a view {:?}",
            append_at_top / rounds,
            append_at_end / rounds,
            trim / rounds,
            scroll_frames / 200,
            scroll_down / 200,
        );
    }
}
