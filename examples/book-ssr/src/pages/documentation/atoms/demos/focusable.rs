use std::time::Duration;

use leptonic::atoms;
use leptos::prelude::*;

#[component]
pub fn FocusableDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let focused = RwSignal::new(false);

    view! {
        <p class="demo-tooltip-stage">
            "Storage used: 41.5 GB "
            // The icon isn't a button: `Focusable` makes it a tab stop, so keyboard users get the tooltip too.
            <atoms::tooltip::TooltipTrigger delay=Duration::from_millis(300)>
                <atoms::focusable::Focusable is_disabled=disabled on_focus_change=move |is_focused| focused.set(is_focused)>
                    <span role="img" aria-label="About storage" class="demo-info-icon">"\u{24d8}"</span>
                </atoms::focusable::Focusable>
                <atoms::tooltip::Tooltip offset=6.0 classes="demo-tooltip">"Includes photos and backups"</atoms::tooltip::Tooltip>
            </atoms::tooltip::TooltipTrigger>
        </p>
        <p class="demo-status">{move || if focused.get() { "The icon has focus." } else { "The icon has no focus." }}</p>
        <div class="demo-controls">
            <atoms::checkbox::CheckboxField is_selected=disabled set_selected=disabled>
                <atoms::checkbox::CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </atoms::checkbox::CheckboxButton>
            </atoms::checkbox::CheckboxField>
        </div>
    }
}
