use std::collections::HashSet;

use leptonic::{
    hooks::{
        collections::{CollectionMemo, Key, use_list_collection},
        virtualizer::{
            LayoutInfo, ListLayout, ListLayoutOptions, ScrollDirection, UseScrollViewInput,
            UseVirtualizerItemInput, UseVirtualizerStateInput, VirtualizerState, use_scroll_view,
            use_virtualizer_item, use_virtualizer_state,
        },
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

/// How many rows the list holds.
const ROWS: usize = 100_000;

#[component]
pub fn VirtualizerHooksDemo() -> impl IntoView {
    let rows = use_list_collection(
        Signal::stored((1..=ROWS).collect::<Vec<usize>>()),
        |number| Key::from(*number),
        |number| format!("Row {number}"),
    );

    // Lays out the collection: every row is 32px high.
    let state = use_virtualizer_state(UseVirtualizerStateInput {
        layout: ListLayout::new(ListLayoutOptions {
            row_size: Some(32.0),
            ..ListLayoutOptions::default()
        }),
        collection: rows.into(),
        persisted_keys: Signal::stored(HashSet::new()),
        layout_options: Signal::stored(None),
        // Called when the virtualizer moves the viewport, which only anchored layouts do.
        on_visible_rect_change: Callback::new(|_| {}),
    });

    // The scrolling element reports what is visible to the state.
    let element = CapturedElement::new();
    let scroll_view = use_scroll_view(
        UseScrollViewInput {
            content_size: state.content_size.into(),
            on_visible_rect_change: Callback::new(move |rect| state.set_visible_rect(rect)),
            on_size_change: Some(Callback::new(move |size| state.set_size(size))),
            on_scroll_start: Some(Callback::new(move |()| state.start_scrolling())),
            on_scroll_end: Some(Callback::new(move |()| state.end_scrolling())),
            scroll_direction: ScrollDirection::Vertical,
            allows_window_scrolling: false,
        },
        element,
    );

    view! {
        <div
            {..element.attr()}
            class="demo-virt-scroll"
            style=scroll_view.scroll_view_styles
            tabindex="0"
            role="region"
            aria-label="Rows"
        >
            <div style=scroll_view.content_styles>
                // Only the visible rows (and a few more in the scroll direction).
                <For each=move || state.visible.get() key=|info| info.key.clone() let:info>
                    <VirtualRow info=info state=state rows=rows/>
                </For>
            </div>
        </div>
        <p class="demo-status">
            {move || format!("{} of {ROWS} rows rendered.", state.visible.with(Vec::len))}
        </p>
    }
}

/// A rendered row, positioned where the layout put it.
#[component]
fn VirtualRow(
    info: LayoutInfo,
    state: VirtualizerState<ListLayout>,
    rows: CollectionMemo,
) -> impl IntoView {
    let key = info.key.clone();
    let text = rows.with_untracked(|rows| {
        rows.get(&key)
            .map(|row| row.text_value.to_string())
            .unwrap_or_default()
    });
    let layout_info = Signal::derive(move || {
        state
            .visible
            .with(|visible| visible.iter().find(|visible| visible.key == key).cloned())
            .unwrap_or_else(|| info.clone())
    });
    let element = CapturedElement::new();
    let styles = use_virtualizer_item(
        UseVirtualizerItemInput {
            layout_info,
            parent: Signal::stored(None),
            update_item_size: Callback::new(move |(key, size)| state.update_item_size(&key, size)),
            should_observe_item_size: false,
        },
        element,
    )
    .styles;

    view! {
        <div {..element.attr()} class="demo-virt-row" style=move || styles.get()>
            {text}
        </div>
    }
}
