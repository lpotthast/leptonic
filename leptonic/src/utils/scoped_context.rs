//! Contexts scoped to a component's view.

use leptos::{prelude::*, tachys::reactive_graph::OwnedView};

/// Builds `view` in a child owner in which `provide` provides contexts, so that they reach only
/// the view's descendants (as Leptos' `<Provider>`, which wraps its children instead).
///
/// A component body has no owner of its own: `provide_context` there also reaches the
/// component's siblings, so of two sibling fields, the second one's context would win for both.
/// The returned view forwards attributes set on the component to its root element.
pub(crate) fn scoped_view<V>(provide: impl FnOnce(), view: impl FnOnce() -> V) -> OwnedView<V> {
    let owner = Owner::current()
        .expect("a component renders inside a reactive owner")
        .child();
    let view = owner.with(|| {
        provide();
        view()
    });
    OwnedView::new_with_owner(view, owner)
}
