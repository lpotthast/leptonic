// Upstream: react-aria/src/utils/useDescription.ts @ 99e6102368
//! Hidden description elements referenced by `aria-describedby` (react-aria: `useDescription`).
//!
//! The registry of description elements is client-only: on the server, many requests interleave
//! on a thread and a request's render can move between threads, so a thread-local registry would
//! mix requests. The server renders no description (as react-aria, which creates the element in a
//! layout effect), and hydration agrees.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the description as a signal and returns the id for `aria-describedby` as a signal
//   (react-aria: a string, returning `{'aria-describedby'}` props).
//
// ## ADDITIONS
// - The description elements are marked `data-client-only` (they exist after hydration only).
//
// =============================================================================

use leptos::prelude::*;

use crate::Id;

/// The id of a visually hidden element containing `description`, for `aria-describedby`: `None`
/// without a description, during server-side rendering and until mounted.
///
/// Identical texts share one element (reference counted); it is removed when its last user is
/// disposed or its description changes.
pub fn use_description(description: Signal<Option<String>>) -> Signal<Option<Id>> {
    #[cfg(feature = "ssr")]
    {
        let _ = description;
        Signal::stored(None)
    }

    #[cfg(not(feature = "ssr"))]
    {
        use std::sync::Arc;

        use crate::utils::probe::probe_sources;

        // A description that is absent (or empty) and reads no reactive source can never be
        // there: no signal and no effect for it (most callers' descriptions are optional).
        if let (false, false) =
            probe_sources(|| description.with(|d| d.as_deref().is_some_and(|d| !d.is_empty())))
        {
            return Signal::stored(None);
        }
        let id = RwSignal::new(None::<Id>);
        // The description shown now (shared: the registry's key) and its element's id.
        let current: StoredValue<Option<(Arc<str>, Id)>> = StoredValue::new(None);
        Effect::new(move || {
            let changed = description.with(|next| {
                let next = next
                    .as_deref()
                    .filter(|description| !description.is_empty());
                let unchanged =
                    current.with_value(|current| current.as_ref().map(|(text, _)| &**text) == next);
                (!unchanged).then(|| next.map(Arc::<str>::from))
            });
            let Some(next) = changed else {
                return;
            };
            if let Some((text, elem_id)) = current.try_update_value(Option::take).flatten() {
                registry::release(&text, &elem_id);
            }
            let acquired = next.map(|text| {
                let elem_id = registry::acquire(&text);
                (text, elem_id)
            });
            id.set(acquired.as_ref().map(|(_, elem_id)| elem_id.clone()));
            current.set_value(acquired);
        });
        on_cleanup(move || {
            if let Some((text, elem_id)) = current.try_update_value(Option::take).flatten() {
                registry::release(&text, &elem_id);
            }
        });
        id.into()
    }
}

#[cfg(not(feature = "ssr"))]
mod registry {
    use std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        sync::Arc,
    };

    use leptos_use::use_document;

    use crate::Id;

    struct DescriptionEntry {
        elem_id: Id,
        ref_count: usize,
    }

    thread_local! {
        static DESCRIPTION_NODES: RefCell<HashMap<Arc<str>, DescriptionEntry>> =
            RefCell::new(HashMap::new());
        static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    }

    pub(super) fn acquire(description: &Arc<str>) -> Id {
        DESCRIPTION_NODES.with_borrow_mut(|map| {
            map.entry(Arc::clone(description))
                .and_modify(|e| e.ref_count += 1)
                .or_insert_with(|| {
                    let elem_id = next_id();
                    insert_dom_node(&elem_id, description);
                    DescriptionEntry {
                        elem_id,
                        ref_count: 1,
                    }
                })
                .elem_id
                .clone()
        })
    }

    pub(super) fn release(description: &Arc<str>, id: &str) {
        let removed = DESCRIPTION_NODES.with_borrow_mut(|map| {
            let Some(entry) = map.get_mut(description) else {
                return false;
            };
            entry.ref_count -= 1;
            if entry.ref_count == 0 {
                map.remove(description);
                return true;
            }
            false
        });
        if removed
            && let Some(el) = use_document()
                .as_ref()
                .and_then(|document| document.get_element_by_id(id))
        {
            el.remove();
        }
    }

    fn next_id() -> Id {
        let id = NEXT_ID.get();
        NEXT_ID.set(id.wrapping_add(1));
        Id::derived(format_args!("leptonic-desc-{id}"))
    }

    fn insert_dom_node(id: &str, description: &str) {
        if let Some(document) = use_document().as_ref()
            && let Ok(div) = document.create_element("div")
        {
            let _ = div.set_attribute("id", id);
            let _ = div.set_attribute("style", "display: none;");
            // Created after hydration only; tells tooling it has no server-rendered counterpart.
            let _ = div.set_attribute("data-client-only", "");
            div.set_text_content(Some(description));
            if let Some(body) = document.body() {
                let _ = body.append_child(&div);
            }
        }
    }
}

#[cfg(all(test, not(feature = "ssr")))]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{testing::with_owner, utils::probe::is_constant};

    #[test]
    fn a_description_that_can_never_be_there_has_a_constant_id() {
        with_owner(|| {
            let absent = use_description(Signal::stored(None));
            let empty = use_description(Signal::stored(Some(String::new())));
            assert_that!(is_constant(absent)).is_true();
            assert_that!(is_constant(empty)).is_true();
        });
    }
}
