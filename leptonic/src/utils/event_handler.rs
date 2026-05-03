use std::sync::Arc;

use leptos::{
    attr::Attribute,
    ev::{EventDescriptor, On, SharedEventCallback, on},
};
use wasm_bindgen::JsValue;

/// Internal storage for event handlers - optimized for the common single-handler case.
/// Uses `Arc<dyn Fn(E) + Send + Sync>` for thread-safety (required for SSR).
/// Handlers must be `Fn` (not `FnMut`).
/// This works for most Leptos handlers because signal setters take `&self`.
#[derive(Clone)]
enum EventHandlerInner<E: 'static> {
    /// Most common: single handler, no Vec allocation
    Single(Arc<dyn Fn(E) + Send + Sync + 'static>),
    /// Multiple chained handlers
    Multiple(Vec<Arc<dyn Fn(E) + Send + Sync + 'static>>),
    /// Empty handler (no-op)
    Empty,
}

/// A chainable event handler that wraps `Fn` closures.
/// Optimized to avoid `Vec` allocation for the common single-handler case.
///
/// **Note:** Handlers must be `Fn + Send + Sync`, not `FnMut`. This works for most Leptos
/// handlers because signal setters (`set_signal.set(value)`) take `&self`.
/// If you need mutable state in a handler, use a `StoredValue` or signal.
///
/// The `Send + Sync` bounds are required for SSR (server-side rendering) support
/// where views may be rendered across thread boundaries.
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::event_handler::EventHandler;
/// use leptos::ev;
/// use web_sys::KeyboardEvent;
///
/// // Create a handler
/// let handler = EventHandler::new(|e: KeyboardEvent| {
///     tracing::debug!("Key pressed: {}", e.key());
/// });
///
/// // Chain multiple handlers
/// let combined = handler.chain(EventHandler::new(|e: KeyboardEvent| {
///     tracing::debug!("Another handler");
/// }));
///
/// // Convert to On<> attribute for view spreading
/// let on_keydown = combined.to_on(ev::keydown);
/// ```
#[derive(Clone)]
pub struct EventHandler<E: 'static> {
    inner: EventHandlerInner<E>,
}

impl<E: 'static> std::fmt::Debug for EventHandler<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<E>();
        match &self.inner {
            EventHandlerInner::Empty => write!(f, "EventHandler<{type_name}>(Empty)"),
            EventHandlerInner::Single(_) => write!(f, "EventHandler<{type_name}>(Single)"),
            EventHandlerInner::Multiple(handlers) => {
                write!(f, "EventHandler<{type_name}>(Multiple({}))", handlers.len())
            }
        }
    }
}

impl<E: 'static> Default for EventHandler<E> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<E, F: Fn(E) + Send + Sync + 'static> From<F> for EventHandler<E> {
    fn from(f: F) -> Self {
        EventHandler::new(f)
    }
}

impl<E: 'static> EventHandler<E> {
    /// Create from a single closure (no Vec allocation).
    /// The closure must be `Fn + Send + Sync` (use signals/`StoredValue` for mutable state).
    pub fn new<F: Fn(E) + Send + Sync + 'static>(f: F) -> Self {
        Self {
            inner: EventHandlerInner::Single(Arc::new(f)),
        }
    }

    /// Create an empty handler (no-op).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            inner: EventHandlerInner::Empty,
        }
    }

    /// Returns true if this handler is empty (no-op).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.inner, EventHandlerInner::Empty)
    }

    /// Chain another handler to run after this one.
    #[must_use]
    pub fn chain(self, other: impl Into<Self>) -> Self {
        use EventHandlerInner::{Empty, Multiple, Single};
        let inner = match (self.inner, other.into().inner) {
            (Empty, other) => other,
            (this, Empty) => this,
            (Single(a), Single(b)) => Multiple(vec![a, b]),
            (Single(a), Multiple(mut others)) => {
                others.insert(0, a);
                Multiple(others)
            }
            (Multiple(mut these), Single(b)) => {
                these.push(b);
                Multiple(these)
            }
            (Multiple(mut these), Multiple(others)) => {
                these.extend(others);
                Multiple(these)
            }
        };
        Self { inner }
    }

    /// Add a single handler to the chain.
    ///
    /// Convenience method equivalent to `self.chain(EventHandler::new(f))`.
    #[must_use]
    pub fn then<F: Fn(E) + Send + Sync + 'static>(self, f: F) -> Self {
        self.chain(Self::new(f))
    }
}

impl<E: Clone + 'static> EventHandler<E> {
    /// Invoke all handlers in this chain with the given event.
    ///
    /// Useful for calling a handler programmatically (e.g., from within a
    /// wrapper closure that adds a guard condition).
    pub fn call(&self, e: E) {
        match &self.inner {
            EventHandlerInner::Empty => {}
            EventHandlerInner::Single(h) => h(e),
            EventHandlerInner::Multiple(handlers) => {
                let last_idx = handlers.len() - 1;
                for (i, h) in handlers.iter().enumerate() {
                    if i == last_idx {
                        h(e);
                        break;
                    }
                    h(e.clone());
                }
            }
        }
    }
}

impl<E: Clone + 'static> EventHandler<E> {
    /// Convert to an `On<>` attribute for the given event descriptor, cloning self.
    /// Creates a `FnMut` closure that calls all `Fn` handlers in sequence.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handler = EventHandler::new(|e: KeyboardEvent| { /* ... */ });
    /// let on_keydown: On<ev::keydown, _> = handler.to_on(ev::keydown);
    /// ```
    ///
    /// # Why `to_on` requires an event descriptor
    ///
    /// The `On<E, F>` type requires an event descriptor (`ev::keydown`, `ev::click`, etc.)
    /// to know the event name for attaching the DOM listener. `EventHandler<KeyboardEvent>`
    /// only knows the event *type*, not the event *name*. This method bridges that gap.
    pub fn to_on<D>(&self, event: D) -> On<D, SharedEventCallback<E>>
    where
        D: EventDescriptor<EventType = E> + Send + Clone + 'static,
        E: From<JsValue>,
    {
        // Create a FnMut closure that calls all the Fn handlers
        self.clone().into_on(event)
    }

    /// Convert into an `On<>` attribute for the given event descriptor, consuming self.
    /// Creates a `FnMut` closure that calls all `Fn` handlers in sequence.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handler = EventHandler::new(|e: KeyboardEvent| { /* ... */ });
    /// let on_keydown: On<ev::keydown, _> = handler.into_on(ev::keydown);
    /// ```
    ///
    /// # Why `to_on` requires an event descriptor
    ///
    /// The `On<E, F>` type requires an event descriptor (`ev::keydown`, `ev::click`, etc.)
    /// to know the event name for attaching the DOM listener. `EventHandler<KeyboardEvent>`
    /// only knows the event *type*, not the event *name*. This method bridges that gap.
    pub fn into_on<D>(self, event: D) -> On<D, SharedEventCallback<E>>
    where
        D: EventDescriptor<EventType = E> + Send + Clone + 'static,
        E: From<JsValue>,
    {
        // Create a FnMut closure that calls all the Fn handlers
        let handler = move |e: E| match &self.inner {
            EventHandlerInner::Empty => {}
            EventHandlerInner::Single(h) => h(e),
            EventHandlerInner::Multiple(handlers) => {
                let last_idx = handlers.len() - 1;
                for (i, h) in handlers.iter().enumerate() {
                    let is_last = i == last_idx;
                    if is_last {
                        h(e);
                        break;
                    }
                    h(e.clone());
                }
            }
        };
        on(event, handler).into_cloneable()
    }
}
