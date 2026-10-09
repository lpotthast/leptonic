// No upstream: chainable DOM event handlers and their listener attributes (react-aria chains
// handlers with `chain`/`mergeProps`).
use std::sync::Arc;

use leptos::{
    attr::{Attribute, NextAttribute},
    ev::EventDescriptor,
    prelude::Owner,
};
use wasm_bindgen::JsValue;

use super::event_listeners::{Listener, listen_with_options};

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
/// use leptonic::EventHandler;
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
/// // The listener attribute, for view spreading
/// let on_keydown = combined.into_on(ev::keydown);
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

/// The listener attribute of an [`EventHandler`] for the event `D` ([`EventHandler::into_on`]):
/// a DOM listener for a handler, nothing for an empty one (no listener, no wasm closure).
///
/// These listeners use `Fn`, so synchronously dispatched events may re-enter them. Leptos's
/// standard `on` attribute uses an `FnMut` closure, whose Wasm boundary rejects re-entry before
/// a hook can apply its own event guard (e.g. a press callback clicking the pressed element).
#[derive(Clone)]
pub struct OnEvent<D: EventDescriptor>
where
    D::EventType: 'static,
{
    event: D,
    handler: EventHandler<D::EventType>,
    owner: Owner,
}

impl<D: EventDescriptor> std::fmt::Debug for OnEvent<D>
where
    D::EventType: 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnEvent")
            .field("event", &self.event.name())
            .field("handler", &self.handler)
            .finish_non_exhaustive()
    }
}

/// Mounted listener state; dropping it removes the listener from its element.
#[derive(Debug)]
pub struct EventListenerState {
    element: web_sys::Element,
    listener: Option<Listener>,
}

impl<D> OnEvent<D>
where
    D: EventDescriptor,
    D::EventType: Clone + From<JsValue> + 'static,
{
    fn attach(self, element: &web_sys::Element) -> Option<Listener> {
        if self.handler.is_empty() {
            return None;
        }
        let Self {
            event,
            handler,
            owner,
        } = self;
        Some(listen_with_options(
            element.as_ref(),
            event.name(),
            D::CAPTURE,
            event.options(),
            move |event| {
                #[cfg(debug_assertions)]
                let _zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();
                owner.with(|| handler.call(D::EventType::from(JsValue::from(event))));
            },
        ))
    }
}

impl<D> Attribute for OnEvent<D>
where
    D: EventDescriptor + Send + 'static,
    D::EventType: Clone + From<JsValue> + 'static,
{
    const MIN_LENGTH: usize = 0;
    type State = EventListenerState;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, _: &mut String, _: &mut String, _: &mut String, _: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, element: &web_sys::Element) -> Self::State {
        self.build(element)
    }

    fn build(self, element: &web_sys::Element) -> Self::State {
        EventListenerState {
            element: element.clone(),
            listener: self.attach(element),
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        state.listener.take();
        state.listener = self.attach(&state.element);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }
}

impl<D> NextAttribute for OnEvent<D>
where
    D: EventDescriptor + Send + 'static,
    D::EventType: Clone + From<JsValue> + 'static,
{
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

impl<E: Clone + 'static> EventHandler<E> {
    /// The listener attribute for the event `event` (`ev::keydown`, `ev::click`, ...), spread onto
    /// the element: a listener calling every handler in order, or nothing when the handler is
    /// [`empty`](Self::empty), so absent handlers attach no DOM listener.
    ///
    /// The event descriptor names the event: an `EventHandler<KeyboardEvent>` knows the event's
    /// type, not its name.
    ///
    /// ```ignore
    /// let handler = EventHandler::new(|e: KeyboardEvent| { /* ... */ });
    /// let on_keydown: OnEvent<ev::keydown> = handler.into_on(ev::keydown);
    /// ```
    pub fn into_on<D>(self, event: D) -> OnEvent<D>
    where
        D: EventDescriptor<EventType = E> + Send + Clone + 'static,
        E: From<JsValue>,
    {
        OnEvent {
            event,
            handler: self,
            owner: Owner::current().unwrap_or_default(),
        }
    }
}
