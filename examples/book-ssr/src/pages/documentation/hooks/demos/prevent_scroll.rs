use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn PreventScrollDemo() -> impl IntoView {
    let (prevent_scroll, set_prevent_scroll) = signal(false);

    let _scroll_prevention = use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || !prevent_scroll.get()),
    });

    view! {
        <FormControl classes="demo-form-row">
            <Checkbox checked=prevent_scroll set_checked=set_prevent_scroll />
            <Label>"Prevent scroll"</Label>
        </FormControl>

        <Show
            when=move || prevent_scroll.get()
            fallback=|| view! {
                <p style="margin: 0.5em 0 0;">"Scroll prevention is " <strong>"disabled"</strong> ". You can scroll the page normally."</p>
            }
        >
            <p style="margin: 0.5em 0 0;">"Scroll prevention is " <strong>"enabled"</strong> ". Try scrolling \u{2014} the page won\u{2019}t scroll!"</p>
        </Show>
    }
}
