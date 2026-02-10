use std::sync::atomic::{AtomicIsize, Ordering::SeqCst};

use leptos::prelude::*;
use leptos_use::use_window;

static PREVENT_SCROLL_COUNT: AtomicIsize = AtomicIsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct UsePreventScrollInput {
    pub disabled: Signal<bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct UsePreventScrollReturn {
    /// Props for the element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UsePreventScrollProps,
}

/// Props from `use_prevent_scroll` that can be converted to spreadable attributes.
#[derive(Debug, Clone, Copy)]
pub struct UsePreventScrollProps;

impl UsePreventScrollProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UsePreventScrollAttrs {}

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UsePreventScrollAttrs {}
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UsePreventScrollAttrs = ();

/// # Panics
///
/// Panics if setting the style attribute on the document root element fails.
pub fn use_prevent_scroll(input: UsePreventScrollInput) -> UsePreventScrollReturn {
    let UsePreventScrollInput { disabled } = input;

    let style = move |window: &web_sys::Window, root: &web_sys::Element| {
        format!(
            "overflow: hidden; padding-right: {}px;",
            window
                .inner_width()
                .map(|it| it.as_f64().unwrap_or(0.0))
                .unwrap_or(0.0)
                - f64::from(root.client_width())
        )
    };

    let register = move || {
        PREVENT_SCROLL_COUNT.fetch_add(1, SeqCst);

        if let Some(window) = use_window().as_ref() {
            if let Some(doc) = window.document() {
                if let Some(root) = doc.document_element() {
                    root.set_attribute("style", style(window, &root).as_str())
                        .expect("Being able to set style attribute.");
                }
            }
        }
    };

    let cleanup = || {
        let _prev = PREVENT_SCROLL_COUNT.fetch_sub(1, SeqCst);
        let remaining = PREVENT_SCROLL_COUNT.load(SeqCst);
        if remaining == 0 {
            if let Some(window) = use_window().as_ref() {
                if let Some(doc) = window.document() {
                    if let Some(root) = doc.document_element() {
                        root.set_attribute("style", "")
                            .expect("Being able to set style attribute.");
                    }
                }
            }
        }
    };

    let _effect = Effect::new(move |last| {
        if let Some(Some(())) = last {
            cleanup();
        }

        if disabled.get() {
            None
        } else {
            register();
            Some(())
        }
    });

    on_cleanup(move || {
        cleanup();
    });

    UsePreventScrollReturn {
        props: UsePreventScrollProps,
    }
}
