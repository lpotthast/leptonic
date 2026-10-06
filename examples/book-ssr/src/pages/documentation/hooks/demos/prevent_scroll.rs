use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn PreventScrollDemo() -> impl IntoView {
    let (prevent_scroll, set_prevent_scroll) = signal(false);

    // Scrolling is prevented while `disabled` is `false`.
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !prevent_scroll.get()),
    });

    view! {
        <Checkbox state=(prevent_scroll, set_prevent_scroll) classes="demo-form-row">"Prevent scroll"</Checkbox>

        <p class="demo-mt-half">
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
