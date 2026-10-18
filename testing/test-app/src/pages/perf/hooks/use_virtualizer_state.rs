//! The hook `use_virtualizer_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_virtualizer_state.rs`): the state of a collection of
//! [`SMALL`] or [`LARGE`] items (integer keys `0`, `1`, ... and no texts, which allocate nothing:
//! the budget measures the library, not the fixture) in a `ListLayout` of rows estimated at 25px,
//! without a scroll view: the controls ([`Control`]) call its methods. `#perf-state` shows what
//! it lays out: `data-visible` (how many layout infos it renders) and `data-height` (the content
//! height). Mounted at first, with [`SMALL`] items and no size (nothing visible).
//!
//! Controls: `mount`, `unmount`; `small`, `large` (the size, while unmounted: `#perf-count`
//! builds the collection outside the measurements); `set-size` (a 100 × 100 px view at the top);
//! `scroll-by-a-row`, `scroll-by-a-page`, `scroll-to-the-middle`, `scroll-to-the-end` (the
//! visible rectangle, downwards); `start-scrolling`, `end-scrolling`; `measure-first-item` (the
//! first item measured at 40px), `measure-first-item-again` (at 25px); `persist-last`,
//! `persist-none` (the persisted keys); `taller-rows`, `default-rows` (the layout options);
//! `append`, `remove` (10 items). Reads: `layout-info-of-the-last` (its top, laying out up to
//! it), `key-at-the-middle` (the index of the item at the content's middle), `top` (of the
//! visible rectangle).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::{Key, Rect, Size, UseListCollectionInput, use_list_collection},
    virtualizer::{
        ItemSize, ListLayout, ListLayoutOptions, UseVirtualizerStateInput, VirtualizerState,
        use_virtualizer_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small collection.
pub const SMALL: usize = 10;
/// The items of the large collection.
pub const LARGE: usize = 10_000;
/// Items per `append`.
pub const APPENDED: usize = 10;
/// The view's size (px).
const VIEW: f64 = 100.0;

/// The layout options with rows estimated at `row_size` px.
fn rows_of(row_size: f64) -> ListLayoutOptions {
    ListLayoutOptions {
        row_size: ItemSize::Estimated(row_size),
        ..ListLayoutOptions::default()
    }
}

/// The index of the item `key` (its key).
fn index_of(key: &Key) -> f64 {
    key.as_i64()
        .and_then(|index| u32::try_from(index).ok())
        .map_or(-1.0, f64::from)
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let persisted = RwSignal::new(HashSet::<Key>::new());
    let options = RwSignal::new(Some(rows_of(25.0)));
    let collection = use_list_collection(UseListCollectionInput {
        items: Signal::derive(move || (0..count.get()).collect::<Vec<usize>>()),
        key: |i| Key::from(*i),
        text_value: |_| "",
    });
    // The mounted state, for the controls.
    let state = StoredValue::new(None::<VirtualizerState<ListLayout>>);
    let with_state = move |f: &dyn Fn(VirtualizerState<ListLayout>)| {
        if let Some(state) = state.get_value() {
            f(state);
        }
    };
    let top = move || {
        state
            .get_value()
            .map_or(0.0, |state| state.visible_rect().get_untracked().y)
    };
    let height = move || {
        state
            .get_value()
            .map_or(0.0, |state| state.content_size().get_untracked().height)
    };
    let scroll_to =
        move |y: f64| with_state(&|state| state.set_visible_rect(Rect::new(0.0, y, VIEW, VIEW)));
    let last_key = move || collection.with_untracked(|c| c.last_key().cloned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("set-size", move || {
            with_state(&|state| {
                state.set_size(Size::new(VIEW, VIEW));
                state.set_visible_rect(Rect::new(0.0, 0.0, VIEW, VIEW));
            });
        }),
        Control::action("scroll-by-a-row", move || scroll_to(top() + 25.0)),
        Control::action("scroll-by-a-page", move || scroll_to(top() + VIEW)),
        Control::action("scroll-to-the-middle", move || {
            scroll_to((height() / 2.0).floor());
        }),
        Control::action("scroll-to-the-end", move || scroll_to(height() - VIEW)),
        Control::action("start-scrolling", move || {
            with_state(&|state| state.start_scrolling());
        }),
        Control::action("end-scrolling", move || {
            with_state(&|state| state.end_scrolling());
        }),
        Control::action("measure-first-item", move || {
            with_state(&|state| {
                state.update_item_size(&Key::from(0), Size::new(VIEW, 40.0));
            });
        }),
        Control::action("measure-first-item-again", move || {
            with_state(&|state| {
                state.update_item_size(&Key::from(0), Size::new(VIEW, 25.0));
            });
        }),
        Control::action("persist-last", move || {
            persisted.set(last_key().into_iter().collect());
        }),
        Control::action("persist-none", move || persisted.set(HashSet::new())),
        Control::action("taller-rows", move || options.set(Some(rows_of(40.0)))),
        Control::action("default-rows", move || options.set(Some(rows_of(25.0)))),
        Control::action("append", move || count.update(|count| *count += APPENDED)),
        Control::action("remove", move || count.update(|count| *count -= APPENDED)),
        Control::read("layout-info-of-the-last", move || {
            let (Some(state), Some(key)) = (state.get_value(), last_key()) else {
                return -1.0;
            };
            state.layout_info(&key).map_or(-1.0, |info| info.rect.y)
        }),
        Control::read("key-at-the-middle", move || {
            let Some(state) = state.get_value() else {
                return -1.0;
            };
            // The middle of the row there.
            let middle = leptonic::Point::new(1.0, (height() / 2.0).floor() + 12.0);
            state.key_at_point(middle).as_ref().map_or(-1.0, index_of)
        }),
        Control::read("top", top),
    ];
    view! {
        <h1>"use_virtualizer_state performance"</h1>
        <PerfControls controls />
        // Reads the collection: it is built when the size changes, not when the state mounts.
        <p id="perf-count" data-count=move || collection.with(|c| c.size())></p>
        <Show when=move || mounted.get()>
            {untrack(|| state_view(collection, persisted, options, state))}
        </Show>
    }
}

/// The state, shown in `#perf-state`.
fn state_view(
    collection: leptonic::hooks::collections::CollectionMemo,
    persisted: RwSignal<HashSet<Key>>,
    options: RwSignal<Option<ListLayoutOptions>>,
    state: StoredValue<Option<VirtualizerState<ListLayout>>>,
) -> impl IntoView {
    let created = use_virtualizer_state(UseVirtualizerStateInput {
        layout: ListLayout::new(rows_of(25.0)),
        collection: collection.into(),
        persisted_keys: persisted.into(),
        layout_options: options.into(),
        on_visible_rect_change: Callback::new(|_| {}),
    });
    state.set_value(Some(created));
    on_cleanup(move || state.set_value(None));
    let visible = created.visible();
    let content_size = created.content_size();
    view! {
        <div
            id="perf-state"
            data-visible=move || visible.with(Vec::len)
            data-height=move || content_size.get().height
        ></div>
    }
}
