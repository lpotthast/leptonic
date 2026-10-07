use leptonic::{atoms::button::Button, utils::clipboard::write_text};
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn ClipboardWriteTextDemo() -> impl IntoView {
    let status = RwSignal::new(String::from("Nothing copied yet."));

    // Call `write_text` right in the press handler: browsers only allow writing during a user gesture.
    let copy = move |_| {
        spawn_local(async move {
            match write_text("cargo add leptonic").await {
                Ok(()) => status.set("Copied \u{201c}cargo add leptonic\u{201d}.".to_owned()),
                Err(err) => status.set(format!("Not copied: {err}.")),
            }
        });
    };

    view! {
        <Button on_press=copy classes="demo-btn">"Copy install command"</Button>
        <p class="demo-status">{status}</p>
    }
}
