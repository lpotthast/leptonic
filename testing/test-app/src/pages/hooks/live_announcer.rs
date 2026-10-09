use leptonic::{
    Assertiveness, announce_assertive, announce_polite, announce_with_timeout, clear_announcer,
};
use leptos::prelude::*;

#[component]
pub fn PageLiveAnnouncer() -> impl IntoView {
    view! {
        <div id="test-page-live-announcer">
            <h1>"live_announcer"</h1>
            <button id="test-la-polite" on:click=|_| announce_polite("Polite hello")>
                "Announce politely"
            </button>
            <button id="test-la-assertive" on:click=|_| announce_assertive("Urgent hello")>
                "Announce assertively"
            </button>
            <button
                id="test-la-short"
                on:click=|_| {
                    announce_with_timeout(
                        "Short-lived",
                        Assertiveness::Polite,
                        std::time::Duration::from_millis(300),
                    )
                }
            >
                "Announce briefly"
            </button>
            <button id="test-la-clear" on:click=|_| clear_announcer(None)>
                "Clear"
            </button>
        </div>
    }
}
