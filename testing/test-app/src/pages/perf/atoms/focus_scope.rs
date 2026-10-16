//! The `FocusScope` alone (`.perf-scope`), containing focus, around 10 buttons (or 1,000:
//! controls `use-10`, `use-1000`; `#perf-scope-0` to `#perf-scope-<n - 1>`), for its performance
//! test (`leptonic/tests/perf_tests/atoms/focus_scope.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and
//! unmount it, and switch its containment and focus restoration.

use leptonic::atoms::focus_scope::FocusScope;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let buttons = RwSignal::new(10_usize);
    let contain = RwSignal::new(true);
    let restore = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-10", move || buttons.set(10)),
        Control::action("use-1000", move || buttons.set(1000)),
        Control::action("contain", move || contain.set(true)),
        Control::action("release", move || contain.set(false)),
        Control::action("restore-focus", move || restore.set(true)),
        Control::action("keep-focus", move || restore.set(false)),
    ];
    view! {
        <h1>"FocusScope performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <FocusScope contain=contain restore_focus=restore classes="perf-scope">
                {move || {
                    (0..buttons.get())
                        .map(|i| {
                            view! { <button id=format!("perf-scope-{i}")>{i}</button> }
                        })
                        .collect_view()
                }}
            </FocusScope>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
