//! `use_hover` on a plain `<div>` (`#perf-hover`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_hover.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to move the pointer to, mounted at first, its hovered state as
//! `data-hovered`. Its controls ([`Control`]) mount and unmount it, disable and enable it, and
//! read how often a hover started (`hovers`).

use leptonic::{
    IntoAttrs, flag,
    hooks::interactions::{UseHoverInput, use_hover},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let hover_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("hovers", move || f64::from(hover_count.get_value())),
    ];
    view! {
        <h1>"use_hover performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Hovered disabled hover_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The element with `use_hover`.
#[component]
fn Hovered(disabled: RwSignal<bool>, hover_count: StoredValue<u32>) -> impl IntoView {
    let hover = use_hover(UseHoverInput {
        is_disabled: disabled.into(),
        on_hover_start: Some(Callback::new(move |_| {
            hover_count.update_value(|count| *count += 1);
        })),
        ..UseHoverInput::default()
    });
    view! {
        <div id="perf-hover" {..hover.props.into_attrs()} data-hovered=flag(hover.is_hovered)>
            "Hover me"
        </div>
    }
}
