//! The hook `use_item_measurer` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_item_measurer.rs`): the measurer of [`SMALL`] or
//! [`LARGE`] items of estimated size (`#perf-items > div`, 25px rows, "Item 0", ...), each
//! registered with it by `use_virtualizer_item` (the only way to register an item). It reports
//! their sizes to a counter. Mounted at first, with [`SMALL`] items, not observing resizes.
//!
//! Its controls ([`Control`]) mount and unmount it, switch the item count (`small`, `large`) and
//! `should_observe_item_size` (`observe`, `dont-observe`) for the next mount, resize the first
//! item's content (`grow-first`: 40px, `shrink-first`: 25px), hide and show the items
//! (`hide`, `show`: `display: none`), and read how many sizes it reported (`measurements`).

use leptonic::{
    CapturedElement,
    hooks::{
        collections::{Key, NodeKind, Rect},
        virtualizer::{
            ItemMeasurer, LayoutInfo, UseItemMeasurerInput, UseVirtualizerItemInput,
            UseVirtualizerItemReturn, use_item_measurer, use_virtualizer_item,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small fixture.
pub const SMALL: u32 = 10;
/// The items of the large fixture.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let observe = RwSignal::new(false);
    let first_height = RwSignal::new(25);
    let hidden = RwSignal::new(false);
    let measurements = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("observe", move || observe.set(true)),
        Control::action("dont-observe", move || observe.set(false)),
        Control::action("grow-first", move || first_height.set(40)),
        Control::action("shrink-first", move || first_height.set(25)),
        Control::action("hide", move || hidden.set(true)),
        Control::action("show", move || hidden.set(false)),
        Control::read("measurements", move || f64::from(measurements.get_value())),
    ];
    view! {
        <h1>"use_item_measurer performance"</h1>
        <PerfControls controls />
        <div id="perf-container" style:display=move || if hidden.get() { "none" } else { "block" }>
            <Show when=move || mounted.get()>
                <Items
                    count=count.get_untracked()
                    should_observe_item_size=observe.get_untracked()
                    first_height
                    measurements
                />
            </Show>
        </div>
    }
}

/// The measurer and its items, in a 100 × 200 px scrolling container.
#[component]
fn Items(
    count: u32,
    should_observe_item_size: bool,
    first_height: RwSignal<i32>,
    measurements: StoredValue<u32>,
) -> impl IntoView {
    let measurer = use_item_measurer(UseItemMeasurerInput {
        update_item_size: Callback::new(move |_| {
            measurements.update_value(|measurements| *measurements += 1);
        }),
        should_observe_item_size,
    });
    view! {
        <div
            id="perf-items"
            style="position: relative; width: 100px; height: 200px; overflow: auto"
        >
            {(0..count).map(|index| view! { <Item index first_height measurer /> }).collect_view()}
        </div>
    }
}

/// An item of estimated size; the first one's content is `first_height` px high.
#[component]
fn Item(index: u32, first_height: RwSignal<i32>, measurer: ItemMeasurer) -> impl IntoView {
    let element = CapturedElement::new();
    let content = CapturedElement::new();
    let rect = Rect::new(0.0, f64::from(index) * 25.0, 100.0, 25.0);
    let layout_info = LayoutInfo {
        estimated_size: true,
        ..LayoutInfo::new(NodeKind::Item, Key::from(format!("item-{index}")), rect)
    };
    let UseVirtualizerItemReturn {
        styles,
        content_styles,
    } = use_virtualizer_item(UseVirtualizerItemInput {
        element,
        content,
        layout_info: Signal::stored(layout_info),
        parent: Signal::stored(None),
        measurer,
    });
    let height = move || {
        if index == 0 {
            format!("{}px", first_height.get())
        } else {
            "25px".to_owned()
        }
    };
    view! {
        <div {..element.attr()} style=move || styles.get()>
            <div {..content.attr()} style=content_styles>
                <div style:height=height>{format!("Item {index}")}</div>
            </div>
        </div>
    }
}
