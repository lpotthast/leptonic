//! `use_focus` on a plain `<button>` (`#perf-focus`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_focus.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, its focus (from `on_focus_change`) as
//! `data-focused`. Its controls ([`Control`]) mount and unmount it, disable and enable it, and
//! read how often it got focus (`focuses`).

use leptonic::hooks::focus::{UseFocusInput, use_focus};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let focus_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("focuses", move || f64::from(focus_count.get_value())),
    ];
    view! {
        <h1>"use_focus performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Focused disabled focus_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The button with `use_focus`.
#[component]
fn Focused(disabled: RwSignal<bool>, focus_count: StoredValue<u32>) -> impl IntoView {
    let focused = RwSignal::new(false);
    let focus = use_focus(UseFocusInput {
        is_disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| {
            focus_count.update_value(|count| *count += 1);
        })),
        on_focus_change: Some(Callback::new(move |is_focused| focused.set(is_focused))),
        ..UseFocusInput::default()
    });
    view! {
        <button
            id="perf-focus"
            {..focus.target.attrs()}
            data-focused=move || focused.get().then_some("true")
        >
            "Focus me"
        </button>
    }
}
