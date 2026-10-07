// Upstream: react-stately/src/virtualizer/useVirtualizerState.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use super::{InvalidationContext, Layout, LayoutInfo, RenderInput, Virtualizer};
use crate::hooks::collections::{Collection, Key, LayoutDelegate, Rect, Size};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The virtualizer renders in an effect (react-stately: during render), as rendering may move
//   the viewport (scroll anchoring, clamping): the visible layout infos are a signal. Nothing is
//   visible during server-side rendering (as upstream: the scroll view has no size there).
//
// =============================================================================

/// Input of [`use_virtualizer_state`].
pub struct UseVirtualizerStateInput<L: Layout> {
    pub layout: L,
    pub collection: Signal<Arc<Collection>>,
    /// Keys that stay rendered while not visible (e.g. the focused item).
    pub persisted_keys: Signal<HashSet<Key>>,
    /// Options for the layout, replacing the ones it was created with.
    pub layout_options: Signal<Option<L::Options>>,
    /// Called when the virtualizer moves the viewport (scroll the element there).
    pub on_visible_rect_change: Callback<Rect>,
}

/// The state of a virtualized collection (react-stately's `useVirtualizerState`).
pub struct VirtualizerState<L: Layout> {
    virtualizer: StoredValue<Virtualizer<L>>,
    visible_rect: RwSignal<Rect>,
    size: RwSignal<Size>,
    is_scrolling: RwSignal<bool>,
    /// Counts the item size changes (the render effect compares it with the last run's).
    invalidation: RwSignal<u64>,
    /// The layout infos to render, parents before children.
    visible: RwSignal<Vec<LayoutInfo>>,
    /// The size of the scrollable content.
    content_size: RwSignal<Size>,
}

// Derived, they would require `L: Copy`.
impl<L: Layout> Clone for VirtualizerState<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Layout> Copy for VirtualizerState<L> {}

impl<L: Layout> std::fmt::Debug for VirtualizerState<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualizerState").finish_non_exhaustive()
    }
}

impl<L: Layout> VirtualizerState<L> {
    /// The scroll view moved or resized its visible area.
    pub fn set_visible_rect(&self, rect: Rect) {
        if self.visible_rect.get_untracked() != rect {
            self.visible_rect.set(rect);
        }
    }

    /// The scroll view's size changed.
    pub fn set_size(&self, size: Size) {
        if self.size.get_untracked() != size {
            self.size.set(size);
        }
    }

    pub fn start_scrolling(&self) {
        self.is_scrolling.set(true);
    }

    pub fn end_scrolling(&self) {
        self.is_scrolling.set(false);
    }

    pub fn is_scrolling(&self) -> Signal<bool> {
        self.is_scrolling.into()
    }

    /// The visible area of the content.
    pub fn visible_rect(&self) -> Signal<Rect> {
        self.visible_rect.into()
    }

    /// The layout infos to render (the visible ones and the persisted ones), parents before
    /// children.
    pub fn visible(&self) -> Signal<Vec<LayoutInfo>> {
        self.visible.into()
    }

    /// The size of the scrollable content (for the scroll view's content box).
    pub fn content_size(&self) -> Signal<Size> {
        self.content_size.into()
    }

    /// The measured size of a rendered item.
    pub fn update_item_size(&self, key: &Key, size: Size) {
        let changed = self
            .virtualizer
            .try_update_value(|virtualizer| virtualizer.update_item_size(key, size))
            .unwrap_or(false);
        if changed {
            self.invalidation.update(|generation| *generation += 1);
        }
    }

    /// The layout info of `key` (laying out more of the collection if needed).
    pub fn layout_info(&self, key: &Key) -> Option<LayoutInfo> {
        self.virtualizer
            .try_update_value(|virtualizer| virtualizer.layout_info(key))
            .flatten()
    }

    /// The key of the item at `point` (content coordinates).
    pub fn key_at_point(&self, point: crate::utils::point::Point) -> Option<Key> {
        self.virtualizer
            .try_update_value(|virtualizer| virtualizer.key_at_point(point))
            .flatten()
    }

    /// The layout as the collection's [`LayoutDelegate`] (react-stately's layouts are one):
    /// keyboard navigation reaches items that aren't rendered.
    pub fn layout_delegate(&self) -> Arc<dyn LayoutDelegate> {
        Arc::new(VirtualizerLayoutDelegate { state: *self })
    }
}

/// The virtualizer's layout as a [`LayoutDelegate`].
struct VirtualizerLayoutDelegate<L: Layout> {
    state: VirtualizerState<L>,
}

impl<L: Layout> LayoutDelegate for VirtualizerLayoutDelegate<L> {
    fn item_rect(&self, key: &Key) -> Option<Rect> {
        self.state.layout_info(key).map(|info| info.rect)
    }

    fn visible_rect(&self) -> Rect {
        self.state
            .visible_rect
            .try_get_untracked()
            .unwrap_or_default()
    }

    fn content_size(&self) -> Size {
        self.state
            .content_size
            .try_get_untracked()
            .unwrap_or_default()
    }
}

fn now() -> f64 {
    #[cfg(not(feature = "ssr"))]
    {
        js_sys::Date::now()
    }
    #[cfg(feature = "ssr")]
    {
        0.0
    }
}

/// Lays out a collection and tracks what is visible (react-stately's `useVirtualizerState`).
pub fn use_virtualizer_state<L: Layout>(input: UseVirtualizerStateInput<L>) -> VirtualizerState<L> {
    let UseVirtualizerStateInput {
        layout,
        collection,
        persisted_keys,
        layout_options,
        on_visible_rect_change,
    } = input;
    let state = VirtualizerState {
        virtualizer: StoredValue::new(Virtualizer::new(layout)),
        visible_rect: RwSignal::new(Rect::default()),
        size: RwSignal::new(Size::default()),
        is_scrolling: RwSignal::new(false),
        invalidation: RwSignal::new(0),
        visible: RwSignal::new(Vec::new()),
        content_size: RwSignal::new(Size::default()),
    };
    // A new value of the options is an invalidation.
    let options_generation = Memo::new(move |previous: Option<&u64>| {
        layout_options.track();
        previous.map_or(0, |generation| generation + 1)
    });

    // Returns the item size generation it rendered.
    Effect::new(move |rendered_item_generation: Option<u64>| {
        let collection = collection.get();
        let persisted_keys = persisted_keys.get();
        let visible_rect = state.visible_rect.get();
        let size = state.size.get();
        let is_scrolling = state.is_scrolling.get();
        let item_generation = state.invalidation.get();
        // Items were measured since the last render (not: ever).
        let item_size_changed =
            rendered_item_generation.is_some_and(|rendered| rendered != item_generation);
        let options = layout_options.get();
        let options_generation = options_generation.get();
        let result = state.virtualizer.try_update_value(|virtualizer| {
            virtualizer.render(RenderInput {
                collection,
                persisted_keys,
                visible_rect,
                size,
                invalidation: InvalidationContext {
                    item_size_changed,
                    layout_options: options,
                    ..InvalidationContext::default()
                },
                invalidation_generation: item_generation + options_generation,
                is_scrolling,
                now: now(),
            })
        });
        let Some(result) = result else {
            return item_generation;
        };
        let content_size = state.virtualizer.with_value(Virtualizer::content_size);
        if state.content_size.get_untracked() != content_size {
            state.content_size.set(content_size);
        }
        if let Some(target) = result.scroll_to {
            // Render again there; the element follows.
            state.visible_rect.set(target);
            on_visible_rect_change.run(target);
            return item_generation;
        }
        if state
            .visible
            .with_untracked(|visible| *visible != result.visible)
        {
            state.visible.set(result.visible);
        }
        item_generation
    });

    state
}
