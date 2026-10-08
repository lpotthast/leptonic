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

/// The time for the overscan's scroll velocity. The JS clock exists only in WebAssembly; natively
/// (server-side rendering, native tests) nothing scrolls: a constant.
fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
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

#[cfg(test)]
mod tests {
    // Upstream has no tests of `useVirtualizerState` itself (react-aria-components'
    // VirtualizedMenu.test.tsx renders through it in a DOM); these follow its implementation:
    // render on every change of the inputs, invalidate on new layout options and measured sizes,
    // report the virtualizer's viewport moves. `Virtualizer`'s own tests cover the layout.
    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::virtualizer::{ListLayout, ListLayoutOptions, ScrollAnchorEdge},
        testing::{flush_effects, with_owner},
        utils::point::Point,
    };

    fn rows(count: usize) -> Arc<Collection> {
        Arc::new(Collection::build(|b| {
            for i in 0..count {
                b.item(format!("row-{i}"), format!("Row {i}"));
            }
        }))
    }

    fn rows_of(size: f64) -> ListLayoutOptions {
        ListLayoutOptions {
            row_size: Some(size),
            ..ListLayoutOptions::default()
        }
    }

    struct Fixture {
        state: VirtualizerState<ListLayout>,
        collection: RwSignal<Arc<Collection>>,
        persisted_keys: RwSignal<HashSet<Key>>,
        layout_options: RwSignal<Option<ListLayoutOptions>>,
        /// The rectangles `on_visible_rect_change` was called with.
        moves: RwSignal<Vec<Rect>>,
    }

    /// A list of `count` rows laid out with `options`, in a 400x480 viewport scrolled to `y`.
    fn list(count: usize, options: ListLayoutOptions, y: f64) -> Fixture {
        list_with(count, options, None, y)
    }

    /// [`list`], with `layout_options` replacing `options` from the first render on.
    fn list_with(
        count: usize,
        options: ListLayoutOptions,
        layout_options: Option<ListLayoutOptions>,
        y: f64,
    ) -> Fixture {
        let collection = RwSignal::new(rows(count));
        let persisted_keys = RwSignal::new(HashSet::new());
        let layout_options = RwSignal::new(layout_options);
        let moves = RwSignal::new(Vec::new());
        let state = use_virtualizer_state(UseVirtualizerStateInput {
            layout: ListLayout::new(options),
            collection: collection.into(),
            persisted_keys: persisted_keys.into(),
            layout_options: layout_options.into(),
            on_visible_rect_change: Callback::new(move |rect| moves.update(|m| m.push(rect))),
        });
        state.set_size(Size::new(400.0, 480.0));
        state.set_visible_rect(Rect::new(0.0, y, 400.0, 480.0));
        flush_effects();
        Fixture {
            state,
            collection,
            persisted_keys,
            layout_options,
            moves,
        }
    }

    fn keys(state: &VirtualizerState<ListLayout>) -> Vec<String> {
        state
            .visible()
            .get_untracked()
            .iter()
            .map(|info| info.key.to_string())
            .collect()
    }

    fn scroll_to(state: &VirtualizerState<ListLayout>, y: f64) {
        state.set_visible_rect(Rect::new(0.0, y, 400.0, 480.0));
        flush_effects();
    }

    #[test]
    fn renders_nothing_before_the_effect_runs() {
        with_owner(|| {
            let state = use_virtualizer_state(UseVirtualizerStateInput {
                layout: ListLayout::new(rows_of(48.0)),
                collection: Signal::stored(rows(10)),
                persisted_keys: Signal::default(),
                layout_options: Signal::default(),
                on_visible_rect_change: Callback::new(|_| {}),
            });
            // As during server-side rendering.
            assert_that!(state.visible().get_untracked()).is_empty();
            assert_that!(state.content_size().get_untracked()).is_equal_to(Size::default());
        });
    }

    #[test]
    fn renders_the_rows_in_the_visible_rect() {
        with_owner(|| {
            let Fixture { state, moves, .. } = list(1000, rows_of(48.0), 0.0);
            assert_that!(state.content_size().get_untracked())
                .is_equal_to(Size::new(400.0, 48_000.0));
            // The viewport plus the overscan (see `Virtualizer`'s tests).
            assert_that!(keys(&state).len()).is_equal_to(15);
            assert_that!(keys(&state).first().cloned()).is_equal_to(Some("row-0".to_owned()));

            scroll_to(&state, 4800.0);
            assert_that!(keys(&state).get(1).cloned()).is_equal_to(Some("row-100".to_owned()));
            // Nothing moved the viewport.
            assert_that!(moves.get_untracked()).is_empty();
        });
    }

    #[test]
    fn renders_again_when_the_collection_changes() {
        with_owner(|| {
            let Fixture {
                state, collection, ..
            } = list(1000, rows_of(48.0), 0.0);
            collection.set(rows(3));
            flush_effects();
            assert_that!(keys(&state)).is_equal_to(vec![
                "row-0".to_owned(),
                "row-1".to_owned(),
                "row-2".to_owned(),
            ]);
            assert_that!(state.content_size().get_untracked().height).is_equal_to(144.0);
        });
    }

    #[test]
    fn keeps_persisted_keys_rendered() {
        with_owner(|| {
            let Fixture {
                state,
                persisted_keys,
                ..
            } = list(1000, rows_of(48.0), 0.0);
            persisted_keys.set(HashSet::from([Key::from("row-900")]));
            flush_effects();
            assert_that!(keys(&state).contains(&"row-900".to_owned())).is_true();
        });
    }

    #[test]
    fn new_layout_options_invalidate_the_layout() {
        with_owner(|| {
            let Fixture {
                state,
                layout_options,
                ..
            } = list_with(100, rows_of(32.0), Some(rows_of(48.0)), 0.0);
            // The first render takes its options.
            assert_that!(state.content_size().get_untracked().height).is_equal_to(4800.0);

            // Options replacing options (as upstream: not the first ones replacing none).
            layout_options.set(Some(rows_of(20.0)));
            flush_effects();
            assert_that!(state.content_size().get_untracked().height).is_equal_to(2000.0);
            assert_that!(
                state
                    .layout_info(&Key::from("row-1"))
                    .map(|info| info.rect.y)
            )
            .is_equal_to(Some(20.0));
        });
    }

    #[test]
    fn measured_item_sizes_move_the_rows_after_them() {
        with_owner(|| {
            let Fixture { state, .. } = list(
                100,
                ListLayoutOptions {
                    estimated_row_size: Some(20.0),
                    ..ListLayoutOptions::default()
                },
                0.0,
            );
            assert_that!(state.content_size().get_untracked().height).is_equal_to(2000.0);
            assert_that!(state.visible().get_untracked()[0].estimated_size).is_true();

            state.update_item_size(&Key::from("row-0"), Size::new(400.0, 50.0));
            flush_effects();
            let visible = state.visible().get_untracked();
            assert_that!(visible[0].estimated_size).is_false();
            assert_that!(visible[1].rect.y).is_equal_to(50.0);
            assert_that!(state.content_size().get_untracked().height).is_equal_to(2030.0);

            // The same size again: no invalidation, nothing changes.
            state.update_item_size(&Key::from("row-0"), Size::new(400.0, 50.0));
            flush_effects();
            assert_that!(state.visible().get_untracked()).is_equal_to(visible);
        });
    }

    #[test]
    fn reports_where_the_virtualizer_moves_the_viewport() {
        with_owner(|| {
            // Anchored at the end: the first layout snaps there.
            let Fixture { state, moves, .. } = list(
                100,
                ListLayoutOptions {
                    anchor_to: Some(ScrollAnchorEdge::End),
                    ..rows_of(48.0)
                },
                0.0,
            );
            let end = Rect::new(0.0, 100.0 * 48.0 - 480.0, 400.0, 480.0);
            assert_that!(moves.get_untracked()).is_equal_to(vec![end]);
            assert_that!(state.visible_rect().get_untracked()).is_equal_to(end);
            // Rendered there.
            assert_that!(keys(&state).last().cloned()).is_equal_to(Some("row-99".to_owned()));
        });
    }

    #[test]
    fn finds_items_for_keyboard_navigation_and_pointers() {
        with_owner(|| {
            let Fixture { state, .. } = list(1000, rows_of(48.0), 480.0);
            // Far beyond the rendered rows (e.g. End): laid out on demand.
            let delegate = state.layout_delegate();
            assert_that!(delegate.item_rect(&Key::from("row-999"))).is_equal_to(Some(Rect::new(
                0.0,
                999.0 * 48.0,
                400.0,
                48.0,
            )));
            assert_that!(delegate.visible_rect()).is_equal_to(Rect::new(0.0, 480.0, 400.0, 480.0));
            assert_that!(delegate.content_size()).is_equal_to(Size::new(400.0, 48_000.0));
            assert_that!(delegate.is_scrollable()).is_true();

            assert_that!(state.key_at_point(Point::new(10.0, 500.0)))
                .is_equal_to(Some(Key::from("row-10")));
        });
    }

    #[test]
    fn tracks_scrolling() {
        with_owner(|| {
            let Fixture { state, .. } = list(10, rows_of(48.0), 0.0);
            assert_that!(state.is_scrolling().get_untracked()).is_false();
            state.start_scrolling();
            flush_effects();
            assert_that!(state.is_scrolling().get_untracked()).is_true();
            state.end_scrolling();
            flush_effects();
            assert_that!(state.is_scrolling().get_untracked()).is_false();
        });
    }
}
