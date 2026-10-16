//! `use_popover` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_popover.rs`): a plain trigger button (`#perf-trigger`,
//! 150px from the top, with the hook's trigger props) and a plain popover (`.perf-popover`,
//! rendered while open, with the hook's props and styles, its placement in `data-placement`, a
//! button `#perf-popover-button` it focuses when it opens), in a scrolling container
//! (`#perf-scroller`), between plain buttons (`#perf-before`, `#perf-after`), mounted at first,
//! closed, modal, below the trigger. Its state (`use_overlay_trigger_state`) is bound to the
//! app's open state. Its controls ([`Control`]) mount it (modal, or non-modal:
//! `mount-non-modal`) and unmount it, open and close it, and move it above the trigger and back.

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::overlay::{
        OverlayPositionOptions, Placement, PlacementAxis, PopoverModality,
        UseOverlayTriggerStateInput, UsePopoverInput, use_overlay_trigger_state, use_popover,
    },
};
use leptos::{html, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let modality = RwSignal::new(PopoverModality::Modal);
    let open = RwSignal::new(false);
    let placement = RwSignal::new(Placement::Bottom);
    let controls = vec![
        Control::action("mount", move || {
            modality.set(PopoverModality::Modal);
            mounted.set(true);
        }),
        Control::action("mount-non-modal", move || {
            modality.set(PopoverModality::NonModal);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("place-above", move || placement.set(Placement::Top)),
        Control::action("place-below", move || placement.set(Placement::Bottom)),
    ];
    view! {
        <h1>"use_popover performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-scroller" style="overflow: auto; height: 400px;">
            <div style="height: 800px; padding: 150px 2em;">
                <Show when=move || mounted.get()>
                    <Subject open placement modality=modality.get_untracked() />
                </Show>
            </div>
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook on a plain trigger and popover.
#[component]
fn Subject(
    open: RwSignal<bool>,
    placement: RwSignal<Placement>,
    modality: PopoverModality,
) -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        value: Some(open.into()),
        ..UseOverlayTriggerStateInput::default()
    });
    let popover = use_popover(UsePopoverInput {
        state,
        trigger: CapturedElement::new(),
        position: OverlayPositionOptions {
            placement: placement.into(),
            offset: Signal::stored(8.0),
            ..OverlayPositionOptions::default()
        },
        target_rect: Signal::stored(None),
        scroll: None,
        modality,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });
    let resolved = popover.placement;
    let (attrs, styles) = popover.props.into_parts();
    let attrs = StoredValue::new(attrs);
    let styles = StoredValue::new(styles);
    let is_open = state.is_open;
    view! {
        <button id="perf-trigger" {..popover.trigger_props.into_attrs()}>
            "Settings"
        </button>
        <Show when=move || is_open.get()>
            {
                let button = NodeRef::<html::Button>::new();
                // Focuses the popover's button when it opens.
                Effect::new(move || {
                    if let Some(button) = button.get() {
                        let _ = button.focus();
                    }
                });
                view! {
                    <div
                        class="perf-popover"
                        {..attrs.get_value()}
                        style=styles.get_value()
                        data-placement=move || resolved.get().map(PlacementAxis::as_str)
                    >
                        <button id="perf-popover-button" node_ref=button>
                            "Apply"
                        </button>
                    </div>
                }
            }
        </Show>
    }
}
