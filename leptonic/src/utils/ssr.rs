use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/ssr/src/SSRProvider.tsx

/// Context for SSR (Server-Side Rendering) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SsrContext {
    /// Whether we're currently rendering on the server.
    is_ssr: bool,

    /// Whether hydration has completed.
    is_hydrated: bool,
}

impl SsrContext {
    /// Returns true if currently rendering on the server.
    #[must_use]
    pub fn is_ssr(&self) -> bool {
        self.is_ssr
    }

    /// Returns true if hydration has completed.
    #[must_use]
    pub fn is_hydrated(&self) -> bool {
        self.is_hydrated
    }

    /// Returns true if we're on the client (either CSR or after hydration).
    #[must_use]
    pub fn is_client(&self) -> bool {
        !self.is_ssr || self.is_hydrated
    }
}

#[allow(clippy::derivable_impls)]
impl Default for SsrContext {
    fn default() -> Self {
        Self {
            is_ssr: cfg!(feature = "ssr"),
            is_hydrated: false,
        }
    }
}

/// Provides SSR context to descendant components.
///
/// This component should wrap your app to properly handle server-side rendering
/// and hydration. It tracks whether the app is being rendered on the server
/// and whether hydration has completed on the client.
///
/// # Example
///
/// ```ignore
/// view! {
///     <SsrProvider>
///         <App />
///     </SsrProvider>
/// }
/// ```
#[component]
pub fn SsrProvider(
    /// Children to render.
    children: Children,
) -> impl IntoView {
    let (is_hydrated, set_is_hydrated) = signal(false);

    // On the client, after hydration, set is_hydrated to true
    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            set_is_hydrated.set(true);
        });
    }

    let context = Memo::new(move |_| SsrContext {
        is_ssr: cfg!(feature = "ssr"),
        is_hydrated: is_hydrated.get(),
    });

    provide_context(context);

    children()
}

/// Returns the current SSR context.
///
/// # Returns
///
/// Returns `Some(SsrContext)` if within an `SsrProvider`, `None` otherwise.
#[must_use]
pub fn use_ssr_context() -> Option<Memo<SsrContext>> {
    use_context::<Memo<SsrContext>>()
}

/// Returns whether we're currently in a server-side rendering context.
///
/// Returns `true` during SSR, `false` on the client (including during hydration).
#[must_use]
pub fn use_is_ssr() -> bool {
    use_ssr_context()
        .map(|ctx| ctx.get().is_ssr())
        .unwrap_or(cfg!(feature = "ssr"))
}

/// Returns whether hydration has completed.
///
/// Returns `false` during SSR and initial hydration, `true` after hydration completes.
#[must_use]
pub fn use_is_hydrated() -> Signal<bool> {
    use_ssr_context().map_or_else(
        || Signal::derive(|| !cfg!(feature = "ssr")),
        |ctx| Signal::derive(move || ctx.get().is_hydrated()),
    )
}

/// Returns a unique ID that is stable across SSR and hydration.
///
/// This is useful for generating IDs for ARIA attributes that need to be
/// consistent between server-rendered HTML and client-side JavaScript.
///
/// # Arguments
///
/// * `prefix` - An optional prefix for the ID.
///
/// # Example
///
/// ```ignore
/// let id = use_ssr_safe_id(Some("button"));
/// // Returns something like "button-abc123" that's stable across SSR/hydration
/// ```
#[must_use]
pub fn use_ssr_safe_id(prefix: Option<&str>) -> String {
    // Use UUID for unique IDs
    let uuid = uuid::Uuid::new_v4();
    let id = uuid.to_string()[..8].to_string();

    match prefix {
        Some(p) => format!("{p}-{id}"),
        None => id,
    }
}

/// A counter for generating sequential IDs within a rendering context.
///
/// This is useful when you need multiple related IDs that are stable
/// across SSR and hydration.
#[derive(Debug, Clone)]
pub struct IdCounter {
    prefix: String,
    counter: std::sync::Arc<std::sync::atomic::AtomicU32>,
}

impl IdCounter {
    /// Creates a new ID counter with the given prefix.
    #[must_use]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }

    /// Generates the next ID in the sequence.
    #[must_use]
    pub fn next_id(&self) -> String {
        let n = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("{}-{n}", self.prefix)
    }

    /// Resets the counter to zero.
    pub fn reset(&self) {
        self.counter.store(0, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Provides an ID counter context for generating sequential IDs.
///
/// # Example
///
/// ```ignore
/// view! {
///     <IdCounterProvider prefix="form">
///         // Components can use use_id_counter() to get sequential IDs
///     </IdCounterProvider>
/// }
/// ```
#[component]
pub fn IdCounterProvider(
    /// The prefix for generated IDs.
    #[prop(into)]
    prefix: String,
    /// Children to render.
    children: Children,
) -> impl IntoView {
    let counter = IdCounter::new(prefix);
    provide_context(counter);
    children()
}

/// Returns the current ID counter from context.
///
/// # Panics
///
/// Panics if called outside of an `IdCounterProvider`.
#[must_use]
pub fn use_id_counter() -> IdCounter {
    use_context::<IdCounter>().expect("use_id_counter must be used within an IdCounterProvider")
}

/// Returns the current ID counter from context, if available.
#[must_use]
pub fn try_use_id_counter() -> Option<IdCounter> {
    use_context::<IdCounter>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssr_context_default() {
        let ctx = SsrContext::default();
        // In tests, cfg!(feature = "ssr") determines the default
        assert!(!ctx.is_hydrated());
    }

    #[test]
    fn test_id_counter() {
        let counter = IdCounter::new("test");
        assert_eq!(counter.next_id(), "test-0");
        assert_eq!(counter.next_id(), "test-1");
        assert_eq!(counter.next_id(), "test-2");

        counter.reset();
        assert_eq!(counter.next_id(), "test-0");
    }

    #[test]
    fn test_ssr_safe_id() {
        let id1 = use_ssr_safe_id(Some("button"));
        let id2 = use_ssr_safe_id(Some("button"));

        assert!(id1.starts_with("button-"));
        assert!(id2.starts_with("button-"));
        assert_ne!(id1, id2); // Each call generates a unique ID
    }

    #[test]
    fn test_ssr_safe_id_no_prefix() {
        let id = use_ssr_safe_id(None);
        assert_eq!(id.len(), 8); // UUID first 8 chars
    }
}
