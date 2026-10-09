//! Headless virtualization of collection atoms, and of plain lists.
// Upstream: react-aria-components/src/Virtualizer.tsx @ 99e6102368
// Upstream: react-aria/src/virtualizer/Virtualizer.tsx @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/GridList.browser.test.tsx @ 99e6102368
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptos::prelude::*;
use leptos_classes::Classes;

use crate::{
    CapturedElement, IntoAttrs, Out, ValueBinding,
    hooks::{
        collections::{Collection, Key, LayoutDelegate, Rect, use_collection},
        focus::{UseFocusRingInput, use_focus_ring},
        virtualizer::{
            EndAnchor, ItemMeasurer, ItemSize, ItemSizeChange, Layout, LayoutInfo, ListLayout,
            ListLayoutOptions, ScrollDirection, ScrollViewScroller, UseItemMeasurerInput,
            UseScrollViewInput, UseVirtualizerStateInput, VirtualizerState, use_item_measurer,
            use_scroll_view, use_virtualizer_state,
        },
    },
    utils::{data_attributes::flag, default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The collection atom renders its items through its iterator (`ListBoxItems`): the iterator
//   renders only the visible ones (react-aria-components: a `CollectionRenderer` context whose
//   `CollectionRoot` renders the visible views of any collection). Items written out one by one
//   aren't virtualized.
// - `layout` is a layout value (cloned per collection); `layout_options` a signal.
//
// ## ADDITIONS
// - `VirtualList`: a virtualized list of plain, app-rendered rows (no collection semantics: no
//   roles, focus or selection handling, so the rows' text stays selectable), e.g. a log. Rows
//   holding the text selection stay rendered; "anchored to the end" is state (C4).
//
// ## OMITTED FEATURES
// - Sections and headers in virtualized list boxes (only items are rendered), drop indicators.
//
// =============================================================================

/// What a collection atom inside a [`Virtualizer`] creates its virtualized root from.
pub(crate) struct VirtualizedRootInput {
    pub(crate) collection: Signal<Arc<Collection>>,
    pub(crate) persisted_keys: Signal<HashSet<Key>>,
    /// The collection element, which scrolls.
    pub(crate) element: CapturedElement,
}

/// A virtualized collection: what its element and its iterator need.
#[derive(Clone, Copy)]
// The items are rendered on the client only.
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) struct VirtualizedRoot {
    pub(crate) layout_delegate: StoredValue<Arc<dyn LayoutDelegate>>,
    /// The collection element's styles (the scroll view).
    pub(crate) scroll_view_styles: StoredValue<Styles>,
    /// The content box around the items.
    pub(crate) content_styles: StoredValue<Styles>,
    pub(crate) visible: Signal<Vec<LayoutInfo>>,
    /// The visible layout infos by key (rows look theirs up on every scroll frame).
    pub(crate) layout_infos: Memo<HashMap<Key, LayoutInfo>>,
    /// Measures the rendered items.
    pub(crate) measurer: ItemMeasurer,
    /// The wrappers of the rendered items (each with a token identifying the wrapper).
    pub(crate) rendered: StoredValue<HashMap<Key, (Arc<()>, CapturedElement)>>,
}

/// Provided by a [`Virtualizer`] to the collection atom inside.
#[derive(Clone)]
pub(crate) struct VirtualizerRenderer {
    pub(crate) create_root: Arc<dyn Fn(VirtualizedRootInput) -> VirtualizedRoot + Send + Sync>,
}

impl VirtualizerRenderer {
    /// The virtualized root of the collection atom calling it, inside a `Virtualizer`.
    pub(crate) fn root_for(input: VirtualizedRootInput) -> Option<VirtualizedRoot> {
        use_context::<Self>().map(|renderer| (renderer.create_root)(input))
    }
}

/// The virtualizer state, root and scroll view of a virtualized element.
struct VirtualizedParts<L: Layout> {
    root: VirtualizedRoot,
    state: VirtualizerState<L>,
    /// Scrolls the view (to its end, as laid out then).
    scroller: ScrollViewScroller,
    /// The user scrolls the view itself (not the page, not `scroll_to`).
    is_user_scrolling: Signal<bool>,
}

fn create_virtualized<L: Layout>(
    layout: L,
    layout_options: Signal<Option<L::Options>>,
    should_observe_item_size: bool,
    input: VirtualizedRootInput,
) -> VirtualizedParts<L> {
    let VirtualizedRootInput {
        collection,
        persisted_keys,
        element,
    } = input;
    // The scroll view's scroller, created after the state.
    let scroller = StoredValue::new(None::<ScrollViewScroller>);
    let state = use_virtualizer_state(UseVirtualizerStateInput {
        layout,
        collection,
        persisted_keys,
        layout_options,
        // The virtualizer moved the viewport (e.g. to keep an anchor in place).
        on_visible_rect_change: Callback::new(move |rect: Rect| {
            if let Some(scroller) = scroller.get_value() {
                scroller.scroll_to(rect);
            }
        }),
    });
    let scroll_view = use_scroll_view(UseScrollViewInput {
        element,
        content_size: state.content_size(),
        on_visible_rect_change: Callback::new(move |rect| state.set_visible_rect(rect)),
        on_size_change: Some(Callback::new(move |size| state.set_size(size))),
        on_scroll_start: Some(Callback::new(move |()| state.start_scrolling())),
        on_scroll_end: Some(Callback::new(move |()| state.end_scrolling())),
        scroll_direction: Signal::stored(ScrollDirection::Both),
        allows_window_scrolling: Signal::stored(true),
    });
    scroller.set_value(Some(scroll_view.scroller));
    let visible = state.visible();
    let root = VirtualizedRoot {
        layout_delegate: StoredValue::new(state.layout_delegate()),
        scroll_view_styles: StoredValue::new(scroll_view.scroll_view_styles),
        content_styles: StoredValue::new(scroll_view.content_styles),
        visible,
        layout_infos: Memo::new(move |_| {
            visible.with(|infos| {
                infos
                    .iter()
                    .map(|info| (info.key.clone(), info.clone()))
                    .collect()
            })
        }),
        measurer: use_item_measurer(UseItemMeasurerInput {
            update_item_size: Callback::new(move |change: ItemSizeChange| {
                state.update_item_size(&change.key, change.size);
            }),
            should_observe_item_size,
        }),
        rendered: StoredValue::new(HashMap::new()),
    };
    VirtualizedParts {
        root,
        state,
        scroller: scroll_view.scroller,
        is_user_scrolling: scroll_view.is_user_scrolling,
    }
}

/// Renders only the visible items of the collection atom inside (a `ListBox` with
/// `ListBoxItems`), positioned by `layout` (react-aria-components' `Virtualizer`): very long
/// collections stay fast. The collection element scrolls (give it a height); the focused item
/// stays rendered, so keyboard navigation and screen readers reach every item.
///
/// ```ignore
/// <Virtualizer layout=ListLayout::new(ListLayoutOptions { row_size: Some(32.0), ..ListLayoutOptions::default() })>
///     <ListBox collection=items aria_label="Items" styles=...height...>
///         <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
///     </ListBox>
/// </Virtualizer>
/// ```
#[component]
pub fn Virtualizer<L: Layout + Clone>(
    /// Positions the items (e.g. a `ListLayout`).
    layout: L,
    /// Replaces the layout's options, e.g. to change sizes at runtime.
    #[prop(into, optional)]
    layout_options: Option<Signal<L::Options>>,
    /// Re-measure items whenever their content resizes (variable sizes that change after the
    /// first measurement).
    #[prop(optional)]
    should_observe_item_size: bool,
    children: Children,
) -> impl IntoView {
    let layout = StoredValue::new(layout);
    let create_root = Arc::new(move |input: VirtualizedRootInput| {
        create_virtualized(
            layout.get_value(),
            Signal::derive(move || layout_options.map(|options| options.get())),
            should_observe_item_size,
            input,
        )
        .root
    });
    // Only for the children: a context provided in the body would reach later siblings too.
    let renderer = VirtualizerRenderer { create_root };
    crate::utils::scoped_context::scoped_view(move || provide_context(renderer), children)
}

/// The layout of a [`VirtualList`]: a vertical stack of rows.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VirtualListOptions {
    /// The rows' size. Default: estimated at 48px.
    pub row_size: ItemSize,
    /// The gap between rows.
    pub gap: f64,
    /// The padding around the rows.
    pub padding: f64,
    /// Within this distance (px) from the end, the list counts as being at its end (scrolling
    /// back there anchors it again). At least 1px.
    pub end_threshold: f64,
}

impl VirtualListOptions {
    /// The list layout's options, anchored to the end or not.
    fn layout_options(self, is_anchored_to_end: bool) -> ListLayoutOptions {
        ListLayoutOptions {
            row_size: self.row_size,
            gap: self.gap,
            padding: self.padding,
            anchor_to_end: is_anchored_to_end.then_some(EndAnchor {
                threshold: self.end_threshold,
            }),
            ..ListLayoutOptions::default()
        }
    }
}

/// A virtualized list of plain rows, e.g. a log: only the visible rows are rendered, positioned
/// by a [`ListLayout`]. Unlike a virtualized `ListBox`, the list has no roles, focus or
/// selection handling: rows are whatever `children` renders for an item, and their text stays
/// selectable (rows holding the text selection stay rendered while scrolled away).
///
/// The list element scrolls: give it a height. `is_focusable` puts it in the tab order (to scroll
/// it with the keyboard). Attributes go to it (`attr:role`, `attr:aria-label`). Each item's key must be unique and
/// stable; a row renders once per key (a changed item needs a new key).
///
/// With `is_anchored_to_end`, the end stays in view while items are added (a log's "follow"):
/// scrolling away from the end turns it off, scrolling back to the end (within
/// `end_threshold` of the layout options) turns it on, and turning it on scrolls to the end.
///
/// Default class: `leptonic-VirtualList`.
///
/// ```ignore
/// <VirtualList
///     items=lines
///     key=|line: &LogLine| Key::from(line.id)
///     layout_options=VirtualListOptions { row_size: ItemSize::Estimated(20.0), ..VirtualListOptions::default() }
///     is_anchored_to_end=follow
///     set_anchored_to_end=set_follow
///     is_focusable=true
///     let:line
/// >
///     <pre>{line.text}</pre>
/// </VirtualList>
/// ```
#[component]
#[allow(clippy::too_many_lines)]
pub fn VirtualList<T, KF, R, IV>(
    #[prop(into)] items: Signal<Vec<T>>,
    /// The key of an item: unique and stable.
    key: KF,
    /// The rows' size, gap and padding, and the distance from the end that counts as the end.
    #[prop(into, optional)]
    layout_options: Signal<VirtualListOptions>,
    /// Re-measure rows whenever their content resizes (rows of estimated size are measured once
    /// rendered).
    #[prop(optional)]
    should_observe_item_size: bool,
    /// Whether the list starts at its end, keeping it in view. Ignored with `is_anchored_to_end`.
    #[prop(optional)]
    default_anchored_to_end: bool,
    /// Whether the end stays in view (controlled), replacing `default_anchored_to_end`.
    #[prop(into, optional)]
    is_anchored_to_end: Option<Signal<bool>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_anchored_to_end: Option<Out<bool>>,
    #[prop(into, optional)] on_anchored_to_end_change: Option<Callback<bool>>,
    /// Puts the list in the tab order (`tabindex="0"`), to scroll it with the keyboard; it then
    /// has `data-focused` and `data-focus-visible`.
    #[prop(optional)]
    is_focusable: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// Renders an item's row.
    children: R,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    KF: Fn(&T) -> Key + Send + Sync + 'static,
    R: Fn(T) -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    // The anchoring state (C4).
    let (binding, on_change) = ValueBinding::from_state_props(
        is_anchored_to_end,
        set_anchored_to_end,
        on_anchored_to_end_change,
    );
    let binding =
        binding.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_anchored_to_end)));
    let is_anchored = binding.value;
    let set_anchored = move |new: bool| {
        if is_anchored.get_untracked() == new {
            return;
        }
        binding.set(new);
        if let Some(on_change) = on_change {
            on_change.run(new);
        }
    };

    let collection = use_collection(move |b| {
        items.with(|items| {
            for item in items {
                b.item(key(item), "");
            }
        });
    });
    let element = CapturedElement::new();
    // The rows holding the text selection's ends.
    let selected_rows = RwSignal::new(HashSet::<Key>::new());
    let VirtualizedParts {
        root,
        state,
        scroller,
        is_user_scrolling,
    } = create_virtualized(
        ListLayout::new(
            layout_options
                .get_untracked()
                .layout_options(is_anchored.get_untracked()),
        ),
        Signal::derive(move || Some(layout_options.get().layout_options(is_anchored.get()))),
        should_observe_item_size,
        VirtualizedRootInput {
            collection: collection.into(),
            persisted_keys: selected_rows.into(),
            element,
        },
    );

    // The list's own scroll position (the visible rectangle is clipped to the window, which a
    // list taller than the window never fully shows).
    let distance_from_end = move || {
        element.get_untracked().and_then(|view| {
            use wasm_bindgen::JsCast;
            let view = view.dyn_ref::<web_sys::HtmlElement>()?.clone();
            Some(
                f64::from(view.scroll_height())
                    - f64::from(view.client_height())
                    - view.scroll_top(),
            )
        })
    };
    let is_at_end = move || {
        let threshold = layout_options.with_untracked(|o| o.end_threshold).max(1.0);
        distance_from_end().is_none_or(|distance| distance <= threshold)
    };
    // The user scrolled the list: anchored when they stopped at the end. Scrolls of the page or an
    // ancestor (e.g. a click scrolling its target into view) don't count: the list didn't move.
    Effect::new(move |was_scrolling: Option<bool>| {
        let is_scrolling = is_user_scrolling.get();
        if was_scrolling == Some(true) && !is_scrolling {
            set_anchored(is_at_end());
        }
        is_scrolling
    });
    // While anchored, the end stays in view: when anchoring turns on and whenever the content
    // changes (e.g. rows measured after the layout's anchoring settled), unless the user is
    // scrolling the list (anchoring then turns off when the scroll ends away from the end).
    Effect::new(move |_| {
        if !is_anchored.get() {
            return;
        }
        // Runs again whenever the content changes.
        state.content_size().track();
        if is_user_scrolling.get_untracked() {
            return;
        }
        if state.visible_rect().get_untracked().area() > 0.0
            && untrack(distance_from_end).is_some_and(|distance| distance > 1.0)
        {
            // To the end as laid out when it scrolls (the next frame): the visible rectangle may
            // already be where the virtualizer moves the view, the view not yet there.
            scroller.scroll_to_end();
        }
    });

    #[cfg(not(feature = "ssr"))]
    {
        // Removed with the list (leptos-use registers its cleanup).
        let _ = leptos_use::use_event_listener(
            leptos_use::use_document(),
            leptos::ev::selectionchange,
            move |_| {
                let Some(window) = leptos_use::use_window().as_ref().cloned() else {
                    return;
                };
                let Ok(Some(selection)) = window.get_selection() else {
                    return;
                };
                let ends = [selection.anchor_node(), selection.focus_node()];
                let keys: HashSet<Key> = if selection.is_collapsed() {
                    HashSet::new()
                } else {
                    root.rendered.with_value(|rendered| {
                        rendered
                            .iter()
                            .filter(|(_, (_, row))| {
                                row.get_untracked().is_some_and(|row| {
                                    let row: &web_sys::Element = &row;
                                    ends.iter().flatten().any(|end| {
                                        crate::utils::shadow_dom::node_contains(row.as_ref(), end)
                                    })
                                })
                            })
                            .map(|(key, _)| key.clone())
                            .collect()
                    })
                };
                if selected_rows.with_untracked(|selected| *selected != keys) {
                    selected_rows.set(keys);
                }
            },
        );
    }

    let children = Arc::new(children);
    let render = Arc::new(move |key: Key| {
        let index = collection.with_untracked(|c| c.get(&key).map(|node| node.index));
        let item = index.and_then(|index| items.with_untracked(|items| items.get(index).cloned()));
        match item {
            Some(item) => children(item).into_any(),
            None => ().into_any(),
        }
    });
    let styles = root.scroll_view_styles.get_value().merge(styles);
    let focus_ring = use_focus_ring(UseFocusRingInput {
        is_disabled: Signal::stored(!is_focusable),
        ..UseFocusRingInput::default()
    });
    view! {
        <div
            {..element.attr()}
            {..focus_ring.props.into_attrs()}
            class=with_default_class("leptonic-VirtualList", classes)
            style=styles
            tabindex=is_focusable.then_some(0)
            data-focused=flag(focus_ring.is_focused)
            data-focus-visible=flag(focus_ring.is_focus_visible)
        >
            {render_visible_items(root, render)}
        </div>
    }
}

/// Renders the visible items of a virtualized collection, each through `render` (absolutely
/// positioned wrappers in the content box).
///
/// The wrappers are in visual order in the DOM (a text selection and screen readers follow the
/// DOM order) and a rendered item's wrapper is never moved: moving an element collapses a text
/// selection in it. So the rows are mounted here, each before its successor, instead of through a
/// keyed `For` (whose diff also moves rows that keep their order, e.g. when the first one leaves).
pub(crate) fn render_visible_items(
    root: VirtualizedRoot,
    render: Arc<dyn Fn(Key) -> AnyView + Send + Sync>,
) -> impl IntoView {
    let content = NodeRef::<leptos::html::Div>::new();
    // The server renders an empty content box (nothing is visible before the view is measured).
    #[cfg(not(feature = "ssr"))]
    mount_visible_items(root, render, content);
    #[cfg(feature = "ssr")]
    let _ = render;
    view! { <div node_ref=content role="presentation" style=root.content_styles.get_value()></div> }
}

/// Mounts and unmounts the rows of the visible items into `content` (see
/// [`render_visible_items`]).
#[cfg(not(feature = "ssr"))]
fn mount_visible_items(
    root: VirtualizedRoot,
    render: Arc<dyn Fn(Key) -> AnyView + Send + Sync>,
    content: NodeRef<leptos::html::Div>,
) {
    use leptos::tachys::view::{Mountable, Render, any_view::AnyViewState};

    use crate::hooks::collections::NodeKind;

    struct Row {
        owner: Owner,
        /// Keeps the row's batch owner alive (see `batch`).
        _batch: Owner,
        state: AnyViewState,
        element: CapturedElement,
    }

    /// Rows per batch owner.
    const ROWS_PER_BATCH: usize = 64;

    let Some(parent_owner) = Owner::current() else {
        return;
    };
    // The rows' owners are children of batch owners (children of the parent), a new one every
    // `ROWS_PER_BATCH` rows: a dropped owner leaves a dead weak entry in its parent until the
    // parent is cleaned up, which would be one per row mounted while scrolling. A batch is freed
    // with its last row, its entries with it; the parent keeps one per batch.
    let batch = StoredValue::new_local((parent_owner.child(), 0_usize));
    // The rendered rows, and their keys in DOM order.
    let rows = StoredValue::new_local(HashMap::<Key, Row>::new());
    let order = StoredValue::new_local(Vec::<Key>::new());
    let visible = root.visible;

    Effect::new(move |_| {
        let Some(parent) = content.get() else {
            return;
        };
        let parent: &web_sys::Element = &parent;
        // The items in visual order: top to bottom, then left to right.
        let keys: Vec<Key> = visible.with(|infos| {
            let mut items = infos
                .iter()
                .filter(|info| info.kind == NodeKind::Item)
                .collect::<Vec<_>>();
            items.sort_by(|a, b| {
                a.rect
                    .y
                    .total_cmp(&b.rect.y)
                    .then(a.rect.x.total_cmp(&b.rect.x))
            });
            items.into_iter().map(|info| info.key.clone()).collect()
        });
        let wanted: HashSet<&Key> = keys.iter().collect();
        rows.update_value(|rows| {
            rows.retain(|key, row| {
                if wanted.contains(key) {
                    return true;
                }
                row.state.unmount();
                row.owner.cleanup();
                false
            });
            // Kept rows keep their relative order in a list. Where a layout reorders them,
            // they are re-appended in order (moving them, as nothing else can).
            let kept_before: Vec<Key> = order.with_value(|order| {
                order
                    .iter()
                    .filter(|key| rows.contains_key(*key))
                    .cloned()
                    .collect()
            });
            let kept_now: Vec<Key> = keys
                .iter()
                .filter(|key| rows.contains_key(*key))
                .cloned()
                .collect();
            if kept_before != kept_now {
                for key in &kept_now {
                    if let Some(element) = rows[key].element.get_untracked() {
                        let _ = parent.append_child(&element);
                    }
                }
            }
            // New rows go before their successor (iterating from the end).
            let mut next: Option<web_sys::Node> = None;
            for key in keys.iter().rev() {
                if let Some(row) = rows.get(key) {
                    next = row
                        .element
                        .get_untracked()
                        .map(|element| (*element).clone().into());
                    continue;
                }
                // Owned by the row (the effect's own owner is disposed on each run).
                let batch_owner = batch
                    .try_update_value(|(owner, count)| {
                        if *count == ROWS_PER_BATCH {
                            *owner = parent_owner.child();
                            *count = 0;
                        }
                        *count += 1;
                        owner.clone()
                    })
                    .unwrap_or_else(|| parent_owner.clone());
                let owner = batch_owner.child();
                let element = owner.with(CapturedElement::new);
                let render = Arc::clone(&render);
                let item_key = key.clone();
                let mut state = owner.with(|| {
                    view! {
                        <VirtualizedItem root=root item_key=item_key.clone() element=element>
                            {render(item_key)}
                        </VirtualizedItem>
                    }
                    .into_any()
                    .build()
                });
                state.mount(parent, next.as_ref());
                next = element
                    .get_untracked()
                    .map(|element| (*element).clone().into());
                rows.insert(
                    key.clone(),
                    Row {
                        owner,
                        _batch: batch_owner,
                        state,
                        element,
                    },
                );
            }
        });
        order.set_value(keys);
    });

    on_cleanup(move || {
        rows.try_update_value(|rows| {
            for (_, mut row) in rows.drain() {
                row.state.unmount();
                row.owner.cleanup();
            }
        });
    });
}

/// The wrapper of a visible item: positioned at its layout info, measured if estimated.
#[cfg(not(feature = "ssr"))]
#[component]
fn VirtualizedItem(
    root: VirtualizedRoot,
    item_key: Key,
    /// Captures the wrapper.
    element: CapturedElement,
    children: Children,
) -> impl IntoView {
    use crate::hooks::{
        collections::NodeKind,
        virtualizer::{UseVirtualizerItemInput, use_virtualizer_item},
    };

    let token = Arc::new(());
    root.rendered.update_value(|rendered| {
        rendered.insert(item_key.clone(), (Arc::clone(&token), element));
    });
    let registered = item_key.clone();
    on_cleanup(move || {
        root.rendered.try_update_value(|rendered| {
            // A re-rendered item may have registered its new wrapper already.
            if rendered
                .get(&registered)
                .is_some_and(|(registered_token, _)| Arc::ptr_eq(registered_token, &token))
            {
                rendered.remove(&registered);
            }
        });
    });
    let key = item_key.clone();
    let layout_info = Memo::new(move |_| {
        root.layout_infos
            .with(|infos| infos.get(&key).cloned())
            .unwrap_or_else(|| LayoutInfo::new(NodeKind::Item, key.clone(), Rect::default()))
    });
    let content = CapturedElement::new();
    let item = use_virtualizer_item(UseVirtualizerItemInput {
        element,
        content,
        layout_info: layout_info.into(),
        parent: Signal::stored(None),
        measurer: root.measurer,
    });
    let styles = item.styles;
    view! {
        <div role="presentation" {..element.attr()} style=move || styles.get()>
            <div role="presentation" {..content.attr()} style=item.content_styles>
                {children()}
            </div>
        </div>
    }
}
