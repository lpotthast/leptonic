//! Writing to the system clipboard (the async Clipboard API; requires the `clipboard` feature).

/// Why text couldn't be written to the clipboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardError {
    /// No clipboard: on the server, or without a browser window.
    Unavailable,
    /// The browser refused (no permission, the page isn't focused, an insecure context, ...).
    Denied,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unavailable => "no clipboard available",
            Self::Denied => "the browser denied writing to the clipboard",
        })
    }
}

impl std::error::Error for ClipboardError {}

/// Writes `text` to the clipboard.
///
/// ```ignore
/// leptos::task::spawn_local(async move {
///     if let Err(err) = write_text("Copied!").await {
///         leptos::logging::warn!("{err}");
///     }
/// });
/// ```
///
/// # Errors
///
/// [`ClipboardError::Unavailable`] without a browser window, [`ClipboardError::Denied`] when the
/// browser refuses.
pub async fn write_text(text: &str) -> Result<(), ClipboardError> {
    let promise = {
        let window = leptos_use::use_window();
        let navigator = window.navigator().ok_or(ClipboardError::Unavailable)?;
        navigator.clipboard().write_text(text)
    };
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|_| ClipboardError::Denied)
}
