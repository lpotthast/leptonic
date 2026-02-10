use leptos::prelude::*;

/// State for managing disclosure visibility.
#[derive(Clone, Copy)]
pub struct UseDisclosureStateReturn {
    /// Whether the disclosure is expanded.
    pub is_expanded: Signal<bool>,

    /// Expand the disclosure.
    pub expand: Callback<()>,

    /// Collapse the disclosure.
    pub collapse: Callback<()>,

    /// Toggle the disclosure.
    pub toggle: Callback<()>,
}

/// Creates internal state for a disclosure.
pub fn use_disclosure_state(default_expanded: bool) -> UseDisclosureStateReturn {
    let (is_expanded, set_is_expanded) = signal(default_expanded);

    UseDisclosureStateReturn {
        is_expanded: is_expanded.into(),
        expand: Callback::new(move |_| set_is_expanded.set(true)),
        collapse: Callback::new(move |_| set_is_expanded.set(false)),
        toggle: Callback::new(move |_| set_is_expanded.update(|v| *v = !*v)),
    }
}
