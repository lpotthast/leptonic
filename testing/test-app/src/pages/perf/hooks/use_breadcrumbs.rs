//! `use_breadcrumbs` on a plain `<ol id="perf-breadcrumbs">` in a `<nav>`, for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_breadcrumbs.rs`), mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and set and unset its label (`aria_label`; without it,
//! the localized "Breadcrumbs").

use leptonic::{
    IntoAttrs,
    hooks::breadcrumbs::{UseBreadcrumbsInput, use_breadcrumbs},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The trail: `use_breadcrumbs` on its `<ol>`.
#[component]
fn Trail(label: RwSignal<Option<String>>) -> impl IntoView {
    let breadcrumbs = use_breadcrumbs(UseBreadcrumbsInput {
        aria_label: label.into(),
    });
    view! {
        <ol {..breadcrumbs.props.into_attrs()} id="perf-breadcrumbs">
            <li>"Home"</li>
        </ol>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-label", move || label.set(Some("Trail".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
    ];
    view! {
        <h1>"use_breadcrumbs performance"</h1>
        <PerfControls controls />
        <nav>
            <Show when=move || mounted.get()>
                <Trail label />
            </Show>
        </nav>
    }
}
