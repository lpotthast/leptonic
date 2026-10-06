use leptonic::{
    components::prelude::Button,
    utils::live_announcer::{announce_assertive, announce_polite, clear_announcer},
};
use leptos::prelude::*;

#[component]
pub fn LiveAnnouncerDemo() -> impl IntoView {
    let count = RwSignal::new(0u32);
    let last = RwSignal::new(String::from("nothing yet"));

    let polite = move |_| {
        count.update(|c| *c += 1);
        let message = format!("{} items in your cart.", count.get_untracked());
        announce_polite(message.as_str());
        last.set(format!("\u{201c}{message}\u{201d} (polite)"));
    };
    let assertive = move |_| {
        announce_assertive("Connection lost. Your changes are not saved.");
        last.set(
            "\u{201c}Connection lost. Your changes are not saved.\u{201d} (assertive)".to_owned(),
        );
    };

    view! {
        <div class="demo-flex-center-row">
            <Button on_press=polite>"Add to cart"</Button>
            <Button on_press=assertive>"Simulate connection loss"</Button>
            <Button on_press=move |_| clear_announcer(None)>"Clear"</Button>
        </div>
        <p class="demo-caption">"Last announcement: "{last}</p>
    }
}
