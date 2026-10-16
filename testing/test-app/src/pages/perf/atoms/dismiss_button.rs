//! The `DismissButton` alone (`#perf-dismiss`, its id), for its performance test
//! (`leptonic/tests/perf_tests/atoms/dismiss_button.rs`): mounted at first, named "Dismiss" (its
//! default). Its controls ([`Control`]) mount and unmount it, change its label, and read how
//! often it dismissed (`dismissals`).

use leptonic::atoms::dismiss_button::DismissButton;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let dismissal_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("label", move || label.set(Some("Close".to_owned()))),
        Control::action("unlabel", move || label.set(None)),
        Control::read("dismissals", move || f64::from(dismissal_count.get_value())),
    ];
    view! {
        <h1>"DismissButton performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <DismissButton
                id="perf-dismiss"
                aria_label=label
                on_dismiss=move |()| dismissal_count.update_value(|count| *count += 1)
            />
        </Show>
    }
}
