use leptos::{ev, prelude::*};
use leptos_use::{use_document, use_event_listener, use_window};
use wasm_bindgen::JsCast;

use crate::{
    components::{
        prelude::ToastRoot,
        theme::{Theme, ThemeProvider},
    },
    signal_ls,
};

/// Leptonic's root context. Always available in components under <Root>.
#[derive(Debug, Clone, Copy)]
pub struct Leptonic {
    /// Whether the users device should be considered 'mobile'.
    /// Please read: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent>
    /// and prefer other detection methods for selective functionality or styling.
    pub is_mobile_device: Signal<bool>,

    /// Always provides the inverse of `is_mobile_device`.
    pub is_desktop_device: Signal<bool>,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Root<T>(
    /// Root directory of JS files used for dynamic script imports. Defaults to "js", as this is commonly used.
    /// Change this if you chose a non-standard location for `[package.metadata.leptonic] > js-dir`.
    #[allow(unused_variables)]
    #[prop(into, default = Oco::Borrowed("js"))]
    runtime_js_dir: Oco<'static, str>,

    default_theme: T,

    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    if let Some(_root_context) = use_context::<Leptonic>() {
        tracing::warn!(
            "The <Root> component must only be used once! Detected that <Root> was rendered when it was already rendered higher up the stack. Remove this usage."
        );
    }

    let win = use_window();

    let update_vh = move || {
        #[derive(Debug)]
        enum Error {
            InnerHeightIndeterminable,
            InnerHeightNotNumber,
            DocumentIndeterminable,
            SetPropertyFailed,
        }
        if let Some(window) = &*use_window() {
            let inner_height = window
                .inner_height()
                .map_err(|_err| Error::InnerHeightIndeterminable)?;
            let inner_height = inner_height.as_f64().ok_or(Error::InnerHeightNotNumber)?;
            if let Some(document) = &*use_document() {
                document
                    .document_element()
                    .ok_or(Error::DocumentIndeterminable)?
                    .unchecked_into::<web_sys::HtmlElement>()
                    .style()
                    .set_property("--leptonic-vh", format!("{inner_height}px").as_str())
                    .map_err(|_err| Error::SetPropertyFailed)?;
            }
        }
        Result::<(), Error>::Ok(())
    };

    if let Err(err) = update_vh() {
        tracing::warn!(?err, "Could not calculate real viewport height");
    }

    if let Some(win) = &*win {
        let _cleanup = use_event_listener(win.clone(), ev::resize, move |_e| {
            if let Err(err) = update_vh() {
                tracing::warn!(?err, "Could not calculate real viewport height");
            }
        });
    }

    // Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent
    let is_mobile_device = Signal::derive(move || {
        use_window().as_ref().is_some_and(|window| {
            window
                .navigator()
                .user_agent()
                .map(|agent| agent.to_lowercase().contains("mobi"))
                .unwrap_or(false)
        })
    });

    provide_context(Leptonic {
        is_mobile_device,
        is_desktop_device: Signal::derive(move || !is_mobile_device.get()),
    });

    cfg_if::cfg_if! { if #[cfg(feature="tiptap")] {
        use leptos_meta::Script;
        let tiptap_js_module_includes = view! {
            <Script type_="module" src=format!("/{}/tiptap-bundle.min.js", runtime_js_dir)/>
            <Script type_="module" src=format!("/{}/tiptap.js", runtime_js_dir)/>
        };
    } else {
        let tiptap_js_module_includes = ();
    }}

    view! {
        {tiptap_js_module_includes}

        <ThemeProvider theme=signal_ls("theme", default_theme)>
            <ToastRoot>
                {children()}
            </ToastRoot>
        </ThemeProvider>
    }
}
