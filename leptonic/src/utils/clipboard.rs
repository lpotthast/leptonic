// No upstream: writing text to the system clipboard (react-aria has no clipboard writer; its
// `useClipboard` handles clipboard events, `hooks::clipboard::use_clipboard`).
//! Writing to the system clipboard (the async Clipboard API; requires the `clipboard` feature).

/// Why text couldn't be written to the clipboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardError {
    /// No clipboard: on the server, without a browser window, or outside a secure context
    /// (browsers offer no `navigator.clipboard` to pages served over plain HTTP, except from
    /// `localhost`, e.g. a development server opened from a phone in the LAN).
    Unavailable,
    /// The browser refused (no permission, the page isn't focused, ...).
    Denied,
    /// The text to write didn't come ([`write_text_deferred`]'s future gave `None`).
    NoText,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unavailable => "no clipboard available",
            Self::Denied => "the browser denied writing to the clipboard",
            Self::NoText => "no text to write to the clipboard",
        })
    }
}

impl std::error::Error for ClipboardError {}

/// The browser's clipboard. `navigator.clipboard` is `undefined` outside secure contexts, where
/// calling it would throw.
fn clipboard() -> Result<web_sys::Clipboard, ClipboardError> {
    use wasm_bindgen::JsCast;

    let navigator = leptos_use::use_window()
        .navigator()
        .ok_or(ClipboardError::Unavailable)?;
    let clipboard = js_sys::Reflect::get(&navigator, &"clipboard".into())
        .map_err(|_| ClipboardError::Unavailable)?;
    if clipboard.is_undefined() || clipboard.is_null() {
        return Err(ClipboardError::Unavailable);
    }
    Ok(clipboard.unchecked_into())
}

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
/// [`ClipboardError::Unavailable`] without a clipboard (see there), [`ClipboardError::Denied`]
/// when the browser refuses.
pub async fn write_text(text: &str) -> Result<(), ClipboardError> {
    let promise = clipboard()?.write_text(text);
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|_| ClipboardError::Denied)
}

/// Writes text that is still loading to the clipboard, e.g. a "Copy" button for content fetched on
/// press.
///
/// Call it directly in the event handler (the press), not in a task it spawns: browsers only allow
/// clipboard writes during the user's interaction, and Safari doesn't keep that permission over an
/// `await` of a download. The write starts right away with a pending clipboard item (`navigator.
/// clipboard.write([new ClipboardItem({"text/plain": promise})])`), which `text` completes. The
/// returned future resolves once the text is written.
///
/// ```ignore
/// let on_press = move |_| {
///     let written = write_text_deferred(async move { fetch_markdown().await.ok() });
///     leptos::task::spawn_local(async move {
///         if let Err(err) = written.await {
///             leptos::logging::warn!("{err}");
///         }
///     });
/// };
/// ```
///
/// # Errors
///
/// [`ClipboardError::Unavailable`] without a clipboard (see there), [`ClipboardError::NoText`] when
/// `text` gives `None`, [`ClipboardError::Denied`] when the browser refuses.
pub fn write_text_deferred(
    text: impl Future<Output = Option<String>> + 'static,
) -> impl Future<Output = Result<(), ClipboardError>> {
    let started = start_deferred_write(text);
    async move {
        let (promise, gave_text) = started?;
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;
        if !gave_text.get() {
            return Err(ClipboardError::NoText);
        }
        result.map(|_| ()).map_err(|_| ClipboardError::Denied)
    }
}

/// Issues the clipboard write synchronously (within the user's interaction); the promise settles
/// once the text arrived and was written. The flag tells whether `text` gave any.
fn start_deferred_write(
    text: impl Future<Output = Option<String>> + 'static,
) -> Result<(js_sys::Promise, std::rc::Rc<std::cell::Cell<bool>>), ClipboardError> {
    use wasm_bindgen::JsValue;

    let clipboard = clipboard()?;
    let gave_text = std::rc::Rc::new(std::cell::Cell::new(false));
    // A promise of the text as a `text/plain` blob (Safari takes blobs only).
    let blob = wasm_bindgen_futures::future_to_promise({
        let gave_text = std::rc::Rc::clone(&gave_text);
        async move {
            let text = text.await.ok_or(JsValue::NULL)?;
            gave_text.set(true);
            let parts = js_sys::Array::of1(&JsValue::from_str(&text));
            let options = web_sys::BlobPropertyBag::new();
            options.set_type("text/plain");
            web_sys::Blob::new_with_str_sequence_and_options(&parts, &options).map(JsValue::from)
        }
    });
    let record = js_sys::Object::new();
    js_sys::Reflect::set(&record, &JsValue::from_str("text/plain"), &blob)
        .map_err(|_| ClipboardError::Unavailable)?;
    let item = web_sys::ClipboardItem::new_with_record_from_str_to_blob_promise(&record)
        .map_err(|_| ClipboardError::Unavailable)?;
    let promise = clipboard.write(&js_sys::Array::of1(&item));
    Ok((promise, gave_text))
}
