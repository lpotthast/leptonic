//! `use_breadcrumb_item` on a plain `<a id="perf-item">` (its link, to this page with a query:
//! pressing it navigates without leaving the page) in a `<nav>`, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_breadcrumb_item.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, its pressed and hovered
//! state shown (`data-pressed`, `data-hovered`). Its controls ([`Control`]) mount and unmount it,
//! make it the current item and not (`is_current`), disable and enable it, and read how often it
//! was pressed (`presses`).

use leptonic::{
    flag,
    hooks::{
        breadcrumbs::{UseBreadcrumbItemInput, use_breadcrumb_item},
        link::{LinkElementType, UseLinkInput},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The item: `use_breadcrumb_item` on its `<a>`.
#[component]
fn Item(
    is_current: RwSignal<bool>,
    is_disabled: RwSignal<bool>,
    press_count: StoredValue<u32>,
) -> impl IntoView {
    let item = use_breadcrumb_item(UseBreadcrumbItemInput {
        link: UseLinkInput {
            href: Signal::stored(Some("/perf/hooks/use_breadcrumb_item?crumb=1".to_owned())),
            element_type: Signal::stored(LinkElementType::Anchor),
            is_disabled: is_disabled.into(),
            on_press: Some(Callback::new(move |_| {
                press_count.update_value(|count| *count += 1);
            })),
            ..UseLinkInput::default()
        },
        is_current: is_current.into(),
        ..UseBreadcrumbItemInput::default()
    });
    let (attrs, styles) = item.link.attrs();
    view! {
        <a
            {..attrs}
            id="perf-item"
            style=styles
            data-pressed=flag(item.is_pressed)
            data-hovered=flag(item.is_hovered)
        >
            "Item"
        </a>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_current = RwSignal::new(false);
    let is_disabled = RwSignal::new(false);
    let press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("make-current", move || is_current.set(true)),
        Control::action("make-not-current", move || is_current.set(false)),
        Control::action("disable", move || is_disabled.set(true)),
        Control::action("enable", move || is_disabled.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"use_breadcrumb_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <nav>
            <Show when=move || mounted.get()>
                <Item is_current is_disabled press_count />
            </Show>
        </nav>
        <button id="perf-after">"After"</button>
    }
}
