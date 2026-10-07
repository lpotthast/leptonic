use leptonic::{
    atoms::prelude::{Button, DialogTrigger, OverlayArrow, Popover},
    hooks::{Placement, PopoverModality},
};
use leptos::prelude::*;

/// Popovers positioned by `use_overlay_position` (react-aria `useOverlayPosition.test.tsx` setups):
/// - "Flip" (`#test-op-flip-trigger`) at the top of the page: a 300px high popover placed above,
///   which has no room there and flips below.
/// - "Above" (`#test-op-above-trigger`) below a 400px spacer: a 120x60 popover placed above with a
///   10px offset and an `OverlayArrow` (`.test-op-arrow`, 12x6).
///
/// `#test-op-shift` puts a 400px spacer above "Flip", so that its popover fits above it.
#[component]
pub fn PageAtomOverlayPosition() -> impl IntoView {
    let shifted = RwSignal::new(false);
    view! {
        <div id="test-page-atom-overlay-position">
            <button id="test-op-shift" on:click=move |_| shifted.update(|shifted| *shifted = !*shifted)>
                "Shift"
            </button>
            <div style:height=move || if shifted.get() { "400px" } else { "0px" }></div>
            <DialogTrigger>
                <Button attr:id="test-op-flip-trigger">"Flip"</Button>
                <Popover
                    placement=Placement::Top
                    modality=PopoverModality::NonModal
                    aria_label="Flipped"
                    classes="test-op-flip-popover"
                >
                    <div style="width: 150px; height: 300px;">"Tall content"</div>
                </Popover>
            </DialogTrigger>

            <div style="height: 400px;"></div>
            <div style="margin-left: 200px;">
            <DialogTrigger>
                <Button attr:id="test-op-above-trigger" classes="test-op-above-button">"Above"</Button>
                <Popover
                    placement=Placement::Top
                    offset=10.0
                    modality=PopoverModality::NonModal
                    aria_label="Above"
                    classes="test-op-above-popover"
                >
                    <div style="width: 120px; height: 60px;">"Short content"</div>
                    <OverlayArrow classes="test-op-arrow">
                        <div style="width: 12px; height: 6px;"></div>
                    </OverlayArrow>
                </Popover>
            </DialogTrigger>
            </div>
        </div>
    }
}
