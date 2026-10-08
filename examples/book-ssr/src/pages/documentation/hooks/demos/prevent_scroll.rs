use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*};
use leptos::prelude::*;

#[component]
pub fn PreventScrollDemo() -> impl IntoView {
    let (prevent_scroll, set_prevent_scroll) = signal(false);

    // Scrolling is prevented while `is_disabled` is `false`.
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !prevent_scroll.get()),
    });

    view! {
        <CheckboxField is_selected=prevent_scroll set_selected=set_prevent_scroll>
            <CheckboxButton classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Prevent scroll"
            </CheckboxButton>
        </CheckboxField>

        <p class="demo-status">
            {move || {
                if prevent_scroll.get() {
                    "Scroll prevention is on. Try scrolling: the page stays where it is."
                } else {
                    "Scroll prevention is off. The page scrolls normally."
                }
            }}
        </p>
    }
}
