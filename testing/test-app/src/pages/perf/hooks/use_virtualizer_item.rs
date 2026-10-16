//! The hook `use_virtualizer_item` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_virtualizer_item.rs`): [`SMALL`] or [`LARGE`] items
//! (`#perf-items > div`, "Item 0", ...), each positioned by `use_virtualizer_item` at its layout
//! info (25px rows, offset by `offset`), measured by one `use_item_measurer` (not observing
//! resizes) while their sizes are estimated. Mounted at first, with [`SMALL`] fixed-size items.
//!
//! Its controls ([`Control`]) mount and unmount the items, switch their count (`small`, `large`;
//! while unmounted), move every item down by 10px and back (`move`, `move-back`: new layout
//! infos), make their sizes estimated and fixed again (`estimate`, `fix`), and read how many
//! sizes the measurer reported (`measurements`).

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
    let offset = RwSignal::new(0.0_f64);
    let estimated = RwSignal::new(false);
    let measurements = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("move", move || offset.set(10.0)),
        Control::action("move-back", move || offset.set(0.0)),
        Control::action("estimate", move || estimated.set(true)),
        Control::action("fix", move || estimated.set(false)),
        Control::read("measurements", move || f64::from(measurements.get_value())),
    ];
    view! {
        <h1>"use_virtualizer_item performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Items count=count.get_untracked() offset estimated measurements />
        </Show>
    }
}

/// The items in a 100 × 200 px scrolling container.
#[component]
fn Items(
    count: u32,
    offset: RwSignal<f64>,
    estimated: RwSignal<bool>,
    measurements: StoredValue<u32>,
) -> impl IntoView {
    let measurer = use_item_measurer(UseItemMeasurerInput {
        update_item_size: Callback::new(move |_| {
            measurements.update_value(|measurements| *measurements += 1);
        }),
        should_observe_item_size: false,
    });
    view! {
        <div
            id="perf-items"
            style="position: relative; width: 100px; height: 200px; overflow: auto"
        >
            {(0..count)
                .map(|index| view! { <Item index offset estimated measurer /> })
                .collect_view()}
        </div>
    }
}

/// An item: its wrapper positioned at its layout info, its content wrapper measured.
#[component]
fn Item(
    index: u32,
    offset: RwSignal<f64>,
    estimated: RwSignal<bool>,
    measurer: ItemMeasurer,
) -> impl IntoView {
    let element = CapturedElement::new();
    let content = CapturedElement::new();
    let layout_info = Memo::new(move |_| {
        let rect = Rect::new(0.0, f64::from(index) * 25.0 + offset.get(), 100.0, 25.0);
        LayoutInfo {
            estimated_size: estimated.get(),
            ..LayoutInfo::new(NodeKind::Item, Key::from(format!("item-{index}")), rect)
        }
    });
    let UseVirtualizerItemReturn {
        styles,
        content_styles,
    } = use_virtualizer_item(UseVirtualizerItemInput {
        element,
        content,
        layout_info: layout_info.into(),
        parent: Signal::stored(None),
        measurer,
    });
    view! {
        <div {..element.attr()} style=move || styles.get()>
            <div {..content.attr()} style=content_styles>
                {format!("Item {index}")}
            </div>
        </div>
    }
}
