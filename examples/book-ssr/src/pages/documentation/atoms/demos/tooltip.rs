use std::time::Duration;

use leptonic::{
    atoms::prelude::{Button, CheckboxButton, CheckboxField, Focusable, OverlayArrow, Tooltip, TooltipTrigger},
    hooks::Placement,
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn TooltipDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let publish_open = RwSignal::new(false);
    let shipping_open = RwSignal::new(false);
    // Hovering opens the first tooltip after half a second (default: 1.5 s).
    let delay = Duration::from_millis(500);

    view! {
        <div class="demo-tooltip-stage">
            // A focusable atom inside a `TooltipTrigger` is its trigger, here a `Button`.
            <TooltipTrigger delay=delay is_disabled=disabled is_open=publish_open set_open=publish_open>
                <Button classes="demo-btn">"Publish"</Button>
                <Tooltip offset=8.0 classes=["demo-tooltip", "demo-tooltip-fade"]>
                    "Make the post visible to everyone"
                    <OverlayArrow classes="demo-tooltip-arrow">
                        <span class="demo-tooltip-arrow-shape"></span>
                    </OverlayArrow>
                </Tooltip>
            </TooltipTrigger>
            // `Focusable` makes an element that can't take focus on its own a trigger. It needs a role
            // and a name.
            <TooltipTrigger delay=delay is_disabled=disabled is_open=shipping_open set_open=shipping_open>
                <Focusable>
                    <span role="img" aria-label="Shipping" class="demo-tooltip-icon">
                        <Icon icon=icondata::BsInfoCircle/>
                    </span>
                </Focusable>
                <Tooltip placement=Placement::Bottom offset=8.0 classes=["demo-tooltip", "demo-tooltip-fade"]>
                    "Ships within two days"
                    <OverlayArrow classes="demo-tooltip-arrow">
                        <span class="demo-tooltip-arrow-shape"></span>
                    </OverlayArrow>
                </Tooltip>
            </TooltipTrigger>
        </div>
        <p class="demo-status">
            {move || match (publish_open.get(), shipping_open.get()) {
                (true, _) => "The tooltip of \u{201c}Publish\u{201d} is open.",
                (_, true) => "The tooltip of \u{201c}Shipping\u{201d} is open.",
                _ => "No tooltip is open.",
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
