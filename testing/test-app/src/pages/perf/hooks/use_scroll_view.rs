//! The hook `use_scroll_view` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_scroll_view.rs`): a 100 × 100 px scroll view
//! (`#perf-scroll-view`) around a content box (`#perf-scroll-content`) of 100 × 2,500 px (the
//! content size it is given), mounted at first, between two plain buttons (`#perf-before`,
//! `#perf-after`), with a 2,000px spacer after it (`#perf-spacer`) so that the window scrolls.
//! The view shows `data-scrolling` and `data-user-scrolling`.
//!
//! Its controls ([`Control`]) mount and unmount it, scroll it programmatically (`scroll-to`:
//! to 500px, `scroll-to-the-end`), change the content size (`grow-content`: 5,000px,
//! `shrink-content`: 2,500px), the view's height (`taller-view`: 200px, `default-view`),
//! `allows_window_scrolling` (`window-scrolling-off`, `window-scrolling-on`) and
//! `scroll_direction` (`vertical`, `both`), and read the top of the last visible rectangle it
//! reported (`visible-top`) and how often it reported a size (`size-changes`).

use leptonic::{
    CapturedElement, flag,
    hooks::{
        collections::{Rect, Size},
        virtualizer::{
            ScrollDirection, ScrollViewScroller, UseScrollViewInput, UseScrollViewReturn,
            use_scroll_view,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The content's height (px), and the grown one.
const CONTENT: f64 = 2_500.0;
const GROWN_CONTENT: f64 = 5_000.0;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let content_size = RwSignal::new(Size::new(100.0, CONTENT));
    let height = RwSignal::new(100);
    let window_scrolling = RwSignal::new(true);
    let direction = RwSignal::new(ScrollDirection::Both);
    let visible_top = StoredValue::new(0.0_f64);
    let size_changes = StoredValue::new(0_u32);
    let scroller = StoredValue::new(None::<ScrollViewScroller>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("scroll-to", move || {
            if let Some(scroller) = scroller.get_value() {
                scroller.scroll_to(Rect::new(0.0, 500.0, 100.0, 100.0));
            }
        }),
        Control::action("scroll-to-the-end", move || {
            if let Some(scroller) = scroller.get_value() {
                scroller.scroll_to_end();
            }
        }),
        Control::action("grow-content", move || {
            content_size.set(Size::new(100.0, GROWN_CONTENT));
        }),
        Control::action("shrink-content", move || {
            content_size.set(Size::new(100.0, CONTENT));
        }),
        Control::action("taller-view", move || height.set(200)),
        Control::action("default-view", move || height.set(100)),
        Control::action("window-scrolling-off", move || window_scrolling.set(false)),
        Control::action("window-scrolling-on", move || window_scrolling.set(true)),
        Control::action("vertical", move || direction.set(ScrollDirection::Vertical)),
        Control::action("both", move || direction.set(ScrollDirection::Both)),
        Control::read("visible-top", move || visible_top.get_value()),
        Control::read("size-changes", move || f64::from(size_changes.get_value())),
    ];
    view! {
        <h1>"use_scroll_view performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ScrollView
                content_size
                height
                window_scrolling
                direction
                visible_top
                size_changes
                scroller
            />
        </Show>
        <button id="perf-after">"After"</button>
        <div id="perf-spacer" style="height: 2000px"></div>
    }
}

/// The scroll view and its content box.
#[component]
fn ScrollView(
    content_size: RwSignal<Size>,
    height: RwSignal<i32>,
    window_scrolling: RwSignal<bool>,
    direction: RwSignal<ScrollDirection>,
    visible_top: StoredValue<f64>,
    size_changes: StoredValue<u32>,
    scroller: StoredValue<Option<ScrollViewScroller>>,
) -> impl IntoView {
    let element = CapturedElement::new();
    let UseScrollViewReturn {
        scroll_view_styles,
        content_styles,
        is_scrolling,
        is_user_scrolling,
        scroller: created,
    } = use_scroll_view(UseScrollViewInput {
        element,
        content_size: content_size.into(),
        on_visible_rect_change: Callback::new(move |rect: Rect| visible_top.set_value(rect.y)),
        on_size_change: Some(Callback::new(move |_: Size| {
            size_changes.update_value(|changes| *changes += 1);
        })),
        on_scroll_start: None,
        on_scroll_end: None,
        scroll_direction: direction.into(),
        allows_window_scrolling: window_scrolling.into(),
    });
    scroller.set_value(Some(created));
    on_cleanup(move || scroller.set_value(None));
    let styles = scroll_view_styles
        .add_unchecked("width", "100px")
        .add_optional_unchecked("height", move || Some(format!("{}px", height.get())));
    view! {
        <div
            id="perf-scroll-view"
            {..element.attr()}
            style=styles
            data-scrolling=flag(is_scrolling)
            data-user-scrolling=flag(is_user_scrolling)
        >
            <div id="perf-scroll-content" style=content_styles></div>
        </div>
    }
}
