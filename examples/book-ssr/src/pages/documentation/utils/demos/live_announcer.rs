use leptonic::{
    atoms::button::Button,
    utils::live_announcer::{announce_assertive, announce_polite, clear_announcer},
};
use leptos::prelude::*;

const CONNECTION_LOST: &str = "Connection lost. Your changes are not saved.";

#[component]
pub fn LiveAnnouncerDemo() -> impl IntoView {
    let items = RwSignal::new(0u32);
    let last = RwSignal::new(String::from("none yet"));

    let add_to_cart = move |_| {
        items.update(|items| *items += 1);
        let count = items.get_untracked();
        let message = if count == 1 {
            String::from("1 item in your cart.")
        } else {
            format!("{count} items in your cart.")
        };
        announce_polite(message.as_str());
        last.set(format!("announced \u{201c}{message}\u{201d} politely."));
    };
    let lose_connection = move |_| {
        announce_assertive(CONNECTION_LOST);
        last.set(format!(
            "announced \u{201c}{CONNECTION_LOST}\u{201d} assertively."
        ));
    };
    let clear = move |_| {
        clear_announcer(None);
        last.set(String::from("cleared the pending announcements."));
    };

    view! {
        <div class="demo-flex-center-row">
            <Button on_press=add_to_cart classes="demo-btn">"Add to cart"</Button>
            <Button on_press=lose_connection classes="demo-btn">"Simulate connection loss"</Button>
            <Button on_press=clear classes="demo-btn">"Clear"</Button>
        </div>
        <p class="demo-status">"Last action: "{last}</p>
    }
}
