//! Element capture attribute for hooks that need direct DOM element access.
//!
//! This module provides an [`ElementCaptureAttr`] that implements the tachys `Attribute` trait
//! and captures the DOM element when the view is built. This enables "spread-only" usage patterns
//! where hooks can automatically capture their element without requiring users to manually create
//! and pass `NodeRef` instances.
//!
//! # `CapturedElement`
//!
//! The preferred way to use element capture in hooks is through [`CapturedElement`], which bundles
//! a `StoredValue` for DOM element storage with a `Trigger` for reactive tracking. This ensures
//! that Effects reading the element via [`CapturedElement::get()`] automatically re-run when the
//! element is captured — which is critical when the element lives inside a reactive boundary
//! like `<Show>` or is rendered during client-side navigation.
//!
//! ```ignore
//! let element = CapturedElement::new();
//!
//! // In an Effect — reactive read causes re-run when the element is captured.
//! Effect::new(move |_| {
//!     let Some(el) = element.get() else { return };
//!     // ... use el ...
//! });
//!
//! // In the return value — spread the attr onto the target element.
//! UseHookReturn {
//!     attrs: (
//!         Attr(attr::Role, "menuitem"),
//!         element.attr(),
//!     ),
//! }
//! ```
//!
//! # Timing
//!
//! The `Attribute::build()` method is called synchronously during view construction.
//! During SSR hydration this means the element is captured before Effects run. However,
//! when the element is inside a reactive boundary (`<Show>`, `<Suspense>`, etc.) and
//! the page is rendered via client-side navigation, the element may be captured *after*
//! Effects have already run once. `CapturedElement` handles this case by notifying a
//! `Trigger`, causing dependent Effects to re-run.

use std::{future::Future, sync::Arc};

use leptos::{
    prelude::*,
    tachys::html::attribute::{Attribute, NamedAttributeKey, NextAttribute},
};
use send_wrapper::SendWrapper;
use web_sys::DomRect;

/// A reactive element reference populated by an [`ElementCaptureAttr`].
///
/// Bundles a `StoredValue` for the DOM element with a `Trigger` so that
/// Effects reading the element via [`get()`](CapturedElement::get) automatically
/// re-run when the element is captured (or re-captured after a `<Show>` toggle).
#[derive(Debug, Copy, Clone)]
pub struct CapturedElement {
    storage: StoredValue<Option<SendWrapper<web_sys::Element>>>,
    trigger: Trigger,
}

impl CapturedElement {
    /// Creates a new, empty `CapturedElement`.
    ///
    /// Call [`attr()`](CapturedElement::attr) to obtain the [`ElementCaptureAttr`]
    /// that should be spread onto the target element.
    pub fn new() -> Self {
        Self {
            storage: StoredValue::new(None),
            trigger: Trigger::new(),
        }
    }

    /// Reactively read (clone) the captured element.
    ///
    /// Tracks the internal `Trigger`, so calling this inside an `Effect` will
    /// cause the Effect to re-run when the element is captured or re-captured.
    pub fn get(&self) -> Option<SendWrapper<web_sys::Element>> {
        self.trigger.track();
        self.storage.get_value()
    }

    /// Read (clone) the captured element without reactive tracking.
    ///
    /// Use this inside event handlers where tracking is not needed.
    pub fn get_untracked(&self) -> Option<SendWrapper<web_sys::Element>> {
        self.storage.get_value()
    }

    /// Programmatically set the captured element and notify dependents.
    ///
    /// This is useful when a hook needs to forward an element to a parent's
    /// `CapturedElement` (e.g., via `FocusableContext`).
    pub fn set(&self, el: web_sys::Element) {
        self.storage.set_value(Some(SendWrapper::new(el)));
        self.trigger.notify();
    }

    /// Access the captured element without reactive tracking.
    ///
    /// Use this inside event handlers where tracking is not needed.
    pub fn with_untracked<U>(&self, accessor: impl Fn(Option<&web_sys::Element>) -> U) -> U {
        self.storage
            .with_value(move |e| accessor(e.as_deref()))
    }

    /// Create the [`ElementCaptureAttr`] to spread onto the target element.
    pub fn attr(&self) -> ElementCaptureAttr {
        let storage = self.storage;
        let trigger = self.trigger;
        ElementCaptureAttr::new(move |el| {
            storage.set_value(Some(SendWrapper::new(el)));
            trigger.notify();
        })
    }

    pub fn get_bounding_client_rect_untracked(&self) -> Option<DomRect> {
        self.with_untracked(|el| el.map(web_sys::Element::get_bounding_client_rect))
    }
}

impl Default for CapturedElement {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// Creates an element capture attribute with the given callback.
    ///
    /// The callback will be invoked with the DOM element when the view is built.
    /// This is useful for hooks that need direct DOM access (e.g., for focus management).
    ///
    /// On the server (SSR), this creates a no-op attribute since there's no DOM.
    ///
    /// **Prefer [`CapturedElement`]** over using this directly — it bundles
    /// reactive storage and a `Trigger` so that Effects automatically re-run
    /// when the element is captured.
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

    /// Combines two element capture attributes into one that invokes both callbacks.
    ///
    /// This is the `ElementCaptureAttr` equivalent of [`EventHandler::chain`].
    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        match (self.callback, other.callback) {
            (Some(a), Some(b)) => Self::new(move |el: web_sys::Element| {
                (*a)(el.clone());
                (*b)(el);
            }),
            (a @ Some(_), None) => Self { callback: a },
            (None, b @ Some(_)) => Self { callback: b },
            (None, None) => Self { callback: None },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_capture_attr_has_zero_html_length() {
        let attr = ElementCaptureAttr::new(|_| {});
        assert_eq!(attr.html_len(), 0);
    }

    #[test]
    fn element_capture_attr_produces_no_html() {
        let attr = ElementCaptureAttr::new(|_| {});
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
        let attr = ElementCaptureAttr::new(|_| {});
        assert!(attr.keys().is_empty());
    }
}
