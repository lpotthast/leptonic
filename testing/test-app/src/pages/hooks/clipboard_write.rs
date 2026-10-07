use std::time::Duration;

use leptonic::utils::clipboard::{write_text, write_text_deferred};
use leptos::prelude::*;

/// `utils::clipboard`: "Write" writes "Written now"; "Write later" writes "Loaded later" once it
/// "loaded" (300ms after the press); "Write nothing" loads nothing. `#test-clipboard-write-status`
/// shows the outcome ("written" or the error).
#[component]
pub fn PageHookClipboardWrite() -> impl IntoView {
    let status = RwSignal::new(String::new());
    let report = move |result: Result<(), leptonic::utils::clipboard::ClipboardError>| {
        status.set(match result {
            Ok(()) => "written".to_owned(),
            Err(err) => err.to_string(),
        });
    };
    // Something that loads for a while, e.g. a download.
    let load = |text: Option<&'static str>| async move {
        gloo_timers::future::sleep(Duration::from_millis(300)).await;
        text.map(str::to_owned)
    };

    view! {
        <h1>"Clipboard writes"</h1>
        <button
            id="test-clipboard-write"
            on:click=move |_| {
                status.set("writing".to_owned());
                leptos::task::spawn_local(async move { report(write_text("Written now").await) });
            }
        >
            "Write"
        </button>
        <button
            id="test-clipboard-write-later"
            on:click=move |_| {
                status.set("writing".to_owned());
                // Issued in the handler, completed when the text arrives.
                let written = write_text_deferred(load(Some("Loaded later")));
                leptos::task::spawn_local(async move { report(written.await) });
            }
        >
            "Write later"
        </button>
        <button
            id="test-clipboard-write-nothing"
            on:click=move |_| {
                status.set("writing".to_owned());
                let written = write_text_deferred(load(None));
                leptos::task::spawn_local(async move { report(written.await) });
            }
        >
            "Write nothing"
        </button>
        <p>"Status: " <span id="test-clipboard-write-status">{status}</span></p>
    }
}
