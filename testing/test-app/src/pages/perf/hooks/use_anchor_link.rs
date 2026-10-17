//! `use_anchor_link` on a plain `<a>` (`#perf-anchor-link`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_anchor_link.rs`): linking to `#perf-anchor-target` far
//! below (scrolling instantly), between two plain buttons (`#perf-before`, `#perf-after`) to tab
//! from and to, mounted at first, its pressed and hovered state as `data-pressed` and
//! `data-hovered`. Its controls ([`Control`]) mount and unmount it and read how often it was
//! pressed (`presses`).

use leptonic::{
    ScrollBehavior, flag,
    hooks::link::{UseAnchorLinkInput, UseLinkInput, use_anchor_link},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"use_anchor_link performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <AnchorLink press_count />
        </Show>
        <button id="perf-after">"After"</button>
        <div style="height: 3000px"></div>
        <div id="perf-anchor-target">"Target"</div>
    }
}

/// The anchor with `use_anchor_link`.
#[component]
fn AnchorLink(press_count: StoredValue<u32>) -> impl IntoView {
    let link = use_anchor_link(UseAnchorLinkInput {
        href: "#perf-anchor-target".into(),
        scroll_behavior: Some(ScrollBehavior::Instant),
        link: UseLinkInput {
            on_press: Some(Callback::new(move |_| {
                press_count.update_value(|count| *count += 1);
            })),
            ..UseLinkInput::default()
        },
    });
    let (attrs, styles) = link.link.attrs();
    view! {
        <a
            id="perf-anchor-link"
            {..attrs}
            style=styles
            data-pressed=flag(link.is_pressed)
            data-hovered=flag(link.is_hovered)
        >
            "To the target"
        </a>
    }
}
