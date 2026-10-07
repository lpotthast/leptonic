use std::collections::VecDeque;

use leptonic::{
    atoms::{button::Button, checkbox::Checkbox},
    hooks::*,
};
use leptos::prelude::*;

#[component]
pub fn FocusableDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let exclude_from_tab_order = RwSignal::new(false);
    let (is_focused, set_is_focused) = signal(false);
    let (keys, set_keys) = signal(VecDeque::<String>::new());

    // A scrollable region must be focusable, so keyboard users can scroll it with the arrow keys.
    let UseFocusableReturn {
        props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        is_disabled: disabled.into(),
        exclude_from_tab_order: exclude_from_tab_order.into(),
        on_focus_change: Some(Callback::new(move |focused| set_is_focused.set(focused))),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            // The hook doesn't prevent the default: the arrow keys still scroll the region.
            set_keys.update(|keys| {
                keys.push_front(e.key_value());
                keys.truncate(50);
            });
        })),
        ..Default::default()
    });

    view! {
        <div role="region" aria-label="Release notes" class="demo-focusable-region" {..props.into_attrs()}>
            <p><strong>"Release notes"</strong></p>
            <p>"Version 3 adds keyboard support to every collection: arrow keys move between items, Home and End jump to the ends."</p>
            <p>"Dialogs now restore focus to the element that opened them, and tooltips open on keyboard focus."</p>
            <p>"Focus rings show only for keyboard users, so pointer users no longer see outlines after a click."</p>
            <p>"Screen reader users hear a description of long-press actions."</p>
        </div>

        <div class="demo-controls">
            <Button on_press=move |_| focus_handle.focus() classes="demo-btn">"Focus the notes"</Button>
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
            <Checkbox is_selected=exclude_from_tab_order set_selected=exclude_from_tab_order classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Exclude from tab order"
            </Checkbox>
        </div>

        <p class="demo-status">
            {move || if is_focused.get() { "The notes have focus." } else { "The notes don\u{2019}t have focus." }}
        </p>

        <p>"Last " {move || keys.with(VecDeque::len)} " keys:"</p>
        <pre class="demo-event-log">
            {move || keys.with(|keys| keys.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
