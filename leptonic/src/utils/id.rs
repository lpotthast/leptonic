// Upstream: react-aria/src/utils/useId.ts @ 99e6102368
// Upstream: react-aria/test/utils/useId.test.jsx @ 99e6102368
//! Element ids that are identical on the server and on the client.
//!
//! Hooks link elements through ids: `aria-labelledby`, `aria-controls`, `aria-activedescendant`,
//! `for`, ... With random ids, the server renders one set of ids and the hydrating client
//! computes another. Attributes the client updates later then point at ids that don't exist.
//!
//! [`use_id`] draws from the same counter Leptos uses for resources: server and client create
//! components in the same order, so they produce the same ids. On the client, after hydration
//! (and in client-side rendered apps), ids come from a separate counter with a `c` marker, so they
//! never collide with server-generated ones.

use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::Owner;

static CLIENT_ID: AtomicUsize = AtomicUsize::new(0);

/// A document-unique id like `"listbox-17"`, stable between server-side rendering and hydration.
///
/// Call it while a component or hook is being created, unconditionally and in the same order on
/// the server and the client: never inside an effect, an event handler, or a code path that only
/// exists on one side (e.g. behind `#[cfg(feature = "ssr")]`). Otherwise the server and the client
/// disagree on all ids created afterwards.
pub fn use_id(prefix: &str) -> String {
    let shared = Owner::current_shared_context()
        .filter(|context| !context.is_browser() || context.during_hydration());
    match shared {
        Some(context) => format!("{prefix}-{}", context.next_id().into_inner()),
        None => format!("{prefix}-c{}", CLIENT_ID.fetch_add(1, Ordering::Relaxed)),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn ids_are_unique_without_a_shared_context() {
        with_owner(|| {
            let a = use_id("listbox");
            let b = use_id("listbox");
            assert_that!(a.as_str()).starts_with("listbox-c");
            assert_that!(a).is_not_equal_to(b);
        });
    }
}
