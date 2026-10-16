//! `use_link` on a plain `<a>` (`#perf-link`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_link.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, its pressed and hovered state as
//! `data-pressed` and `data-hovered`. A surrounding element prevents the default of its clicks,
//! so that a press measures the link and not a navigation. Its controls ([`Control`]) mount and
//! unmount it, change its reactive inputs, and read how often it was pressed (`presses`).

use leptonic::{
    AriaCurrent, flag,
    hooks::link::{LinkTarget, UseLinkInput, use_link},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The link's reactive inputs.
#[derive(Clone, Copy)]
struct Inputs {
    href: RwSignal<Option<String>>,
    disabled: RwSignal<bool>,
    target: RwSignal<LinkTarget>,
    current: RwSignal<Option<AriaCurrent>>,
    excluded: RwSignal<bool>,
    press_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        href: RwSignal::new(Some("/perf/hooks/use_link?one".to_owned())),
        disabled: RwSignal::new(false),
        target: RwSignal::new(LinkTarget::Same),
        current: RwSignal::new(None),
        excluded: RwSignal::new(false),
        press_count: StoredValue::new(0),
    };
    let Inputs {
        href,
        disabled,
        target,
        current,
        excluded,
        press_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("change-href", move || {
            href.set(Some("/perf/hooks/use_link?two".to_owned()));
        }),
        Control::action("restore-href", move || {
            href.set(Some("/perf/hooks/use_link?one".to_owned()));
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("open-in-new-tab", move || target.set(LinkTarget::Blank)),
        Control::action("open-here", move || target.set(LinkTarget::Same)),
        Control::action("make-current", move || current.set(Some(AriaCurrent::Page))),
        Control::action("make-not-current", move || current.set(None)),
        Control::action("exclude-from-tab-order", move || excluded.set(true)),
        Control::action("include-in-tab-order", move || excluded.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"use_link performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div on:click=|e| e.prevent_default()>
            <Show when=move || mounted.get()>
                <Link inputs />
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// The anchor with `use_link`.
#[component]
fn Link(inputs: Inputs) -> impl IntoView {
    let link = use_link(UseLinkInput {
        href: inputs.href.into(),
        is_disabled: inputs.disabled.into(),
        target: inputs.target.into(),
        aria_current: inputs.current.into(),
        exclude_from_tab_order: inputs.excluded.into(),
        on_press: Some(Callback::new(move |_| {
            inputs.press_count.update_value(|count| *count += 1);
        })),
        ..UseLinkInput::default()
    });
    let (attrs, styles) = link.props.into_parts();
    view! {
        <a
            id="perf-link"
            {..attrs}
            style=styles
            data-pressed=flag(link.is_pressed)
            data-hovered=flag(link.is_hovered)
        >
            "Follow me"
        </a>
    }
}
