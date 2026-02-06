//! Element capture attribute for hooks that need direct DOM element access.
//!
//! This module provides an [`ElementCaptureAttr`] that implements the tachys `Attribute` trait
//! and captures the DOM element when the view is built. This enables "spread-only" usage patterns
//! where hooks can automatically capture their element without requiring users to manually create
//! and pass `NodeRef` instances.
//!
//! # Example
//!
//! ```ignore
//! // Hook creates internal storage
//! let element_storage: StoredValue<Option<SendWrapper<web_sys::Element>>> =
//!     StoredValue::new(None);
//!
//! // Return attrs tuple with element capture at the end
//! UseHookReturn {
//!     attrs: (
//!         Attr(attr::Role, "menuitem"),
//!         // ... other attrs ...
//!         element_capture(move |el| {
//!             element_storage.set_value(Some(SendWrapper::new(el)));
//!         }),
//!     ),
//! }
//! ```
//!
//! # Timing Guarantee
//!
//! The `Attribute::build()` method is called synchronously during view construction,
//! so the element is captured before any Effects run. This allows Effects to safely
//! access the captured element.

use leptos::tachys::html::attribute::{Attribute, NamedAttributeKey, NextAttribute};
use send_wrapper::SendWrapper;
use std::future::Future;
use std::sync::Arc;

/// Callback type for element capture.
pub type ElementCaptureCallback = Arc<dyn Fn(web_sys::Element) + Send + Sync>;

/// An attribute that captures the DOM element when the view is built.
///
/// This attribute produces no HTML output and has no visible effect on the element.
/// It simply invokes the callback with the element when `build()` or `hydrate()` is called.
#[derive(Clone)]
pub struct ElementCaptureAttr {
    callback: Option<SendWrapper<ElementCaptureCallback>>,
}

impl std::fmt::Debug for ElementCaptureAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementCaptureAttr")
            .field("callback", &self.callback.as_ref().map(|_| "..."))
            .finish()
    }
}

impl ElementCaptureAttr {
    /// Creates a new element capture attribute.
    ///
    /// On the server (SSR), this creates a no-op attribute since there's no DOM.
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(web_sys::Element) + Send + Sync + 'static,
    {
        #[cfg(feature = "ssr")]
        {
            // During SSR, there is no DOM element to capture.
            let _ = callback;
            Self { callback: None }
        }
        #[cfg(not(feature = "ssr"))]
        {
            Self {
                callback: Some(SendWrapper::new(Arc::new(callback))),
            }
        }
    }

    /// Invokes the callback with the element if present.
    fn invoke(&self, el: &web_sys::Element) {
        if let Some(callback) = &self.callback {
            callback(el.clone());
        }
    }
}

impl Attribute for ElementCaptureAttr {
    const MIN_LENGTH: usize = 0;

    type State = ();
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
        // No HTML output for element capture.
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &web_sys::Element) -> Self::State {
        self.invoke(el);
    }

    fn build(self, el: &web_sys::Element) -> Self::State {
        self.invoke(el);
    }

    fn rebuild(self, _state: &mut Self::State) {
        // No state to rebuild.
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        // No async data to resolve.
    }

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }

    fn keys(&self) -> Vec<NamedAttributeKey> {
        // No attribute keys - this doesn't create any DOM attributes.
        vec![]
    }
}

impl NextAttribute for ElementCaptureAttr {
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

/// Creates an element capture attribute with the given callback.
///
/// The callback will be invoked with the DOM element when the view is built.
/// This is useful for hooks that need direct DOM access (e.g., for focus management).
///
/// # Example
///
/// ```ignore
/// use leptos::prelude::*;
/// use leptonic::utils::element_capture;
/// use send_wrapper::SendWrapper;
///
/// let element_storage: StoredValue<Option<SendWrapper<web_sys::Element>>> =
///     StoredValue::new(None);
///
/// let attrs = (
///     Attr(attr::Role, "button"),
///     element_capture(move |el| {
///         element_storage.set_value(Some(SendWrapper::new(el)));
///     }),
/// );
/// ```
pub fn element_capture<F>(callback: F) -> ElementCaptureAttr
where
    F: Fn(web_sys::Element) + Send + Sync + 'static,
{
    ElementCaptureAttr::new(callback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_capture_attr_has_zero_html_length() {
        let attr = element_capture(|_| {});
        assert_eq!(attr.html_len(), 0);
    }

    #[test]
    fn element_capture_attr_produces_no_html() {
        let attr = element_capture(|_| {});
        let mut buf = String::new();
        let mut class = String::new();
        let mut style = String::new();
        let mut inner_html = String::new();
        attr.to_html(&mut buf, &mut class, &mut style, &mut inner_html);
        assert!(buf.is_empty());
        assert!(class.is_empty());
        assert!(style.is_empty());
        assert!(inner_html.is_empty());
    }

    #[test]
    fn element_capture_attr_has_no_keys() {
        let attr = element_capture(|_| {});
        assert!(attr.keys().is_empty());
    }
}
