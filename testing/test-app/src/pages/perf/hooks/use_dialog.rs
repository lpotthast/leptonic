//! `use_dialog` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_dialog.rs`): its props on a plain element
//! (`role="dialog"`, not in a modal) with a title (an `h2` with the title slot's props, "Settings")
//! and a button (`#perf-dialog-button`), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first (it focuses itself). Its controls ([`Control`]) mount and
//! unmount it, show and hide its title, and set and unset its `aria_label`.

use leptonic::hooks::dialog::{DialogRole, UseDialogInput, use_dialog};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let titled = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("show-title", move || titled.set(true)),
        Control::action("hide-title", move || titled.set(false)),
        Control::action("label", move || {
            label.set(Some("Settings dialog".to_owned()));
        }),
        Control::action("unlabel", move || label.set(None)),
    ];
    view! {
        <h1>"use_dialog performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject titled=titled.into() label />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain element, with a title slot.
#[component]
fn Subject(titled: Signal<bool>, label: RwSignal<Option<String>>) -> impl IntoView {
    let dialog = use_dialog(UseDialogInput {
        role: DialogRole::Dialog,
        aria_label: label.into(),
        ..UseDialogInput::default()
    });
    let title = dialog.title;
    view! {
        <div {..dialog.dialog.attrs()}>
            <Show when=move || titled.get()>
                <h2 class="perf-title" {..title.attrs()}>
                    "Settings"
                </h2>
            </Show>
            <button id="perf-dialog-button">"Apply"</button>
        </div>
    }
}
