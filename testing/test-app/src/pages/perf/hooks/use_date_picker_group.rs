//! `use_date_picker_group` on a plain element, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_picker_group.rs`): a group (`#perf-group`, 400px
//! wide: room to press it beside its parts) of three focusable plain parts standing in for
//! segments (`#perf-month`, `#perf-day`, `#perf-year`), with an overlay state
//! (`use_overlay_trigger_state`) its Alt+ArrowDown opens (`#perf-open` shows it), between two
//! plain buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`])
//! mount and unmount it and close the overlay.

use leptonic::{
    CapturedElement,
    hooks::{
        datepicker::{GroupArrowKeys, UseDatePickerGroupInput, use_date_picker_group},
        overlay::{OverlayTriggerState, UseOverlayTriggerStateInput, use_overlay_trigger_state},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The group and its parts.
#[component]
fn Group(overlay: StoredValue<Option<OverlayTriggerState>>) -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    overlay.set_value(Some(state));
    on_cleanup(move || overlay.set_value(None));
    let element = CapturedElement::new();
    let (attrs, styles) = use_date_picker_group(UseDatePickerGroupInput {
        element,
        arrow_keys: GroupArrowKeys::MoveBetweenSegments,
        overlay: Some(state),
    })
    .into_parts();
    view! {
        <div
            {..attrs}
            {..element.attr()}
            id="perf-group"
            role="group"
            style=styles.add_unchecked("display", "flex").add_unchecked("width", "400px")
        >
            <span id="perf-month" tabindex="0">
                "6"
            </span>
            "/"
            <span id="perf-day" tabindex="0">
                "5"
            </span>
            "/"
            <span id="perf-year" tabindex="0">
                "2024"
            </span>
        </div>
        <p>"Open: " <span id="perf-open">{move || state.is_open.get().to_string()}</span></p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let overlay = StoredValue::new(None::<OverlayTriggerState>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("close", move || {
            if let Some(overlay) = overlay.get_value() {
                overlay.set_open(false);
            }
        }),
    ];
    view! {
        <h1>"use_date_picker_group performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Group overlay=overlay />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
