// Upstream: react-aria/src/utils/useDescription.ts @ 99e6102368
//! Hidden description elements referenced by `aria-describedby` (react-aria: `useDescription`).
//!
//! The registry of description elements is client-only: on the server, many requests interleave
//! on a thread and a request's render can move between threads, so a thread-local registry would
//! mix requests. The server renders no description (as react-aria, which creates the element in a
//! layout effect), and hydration agrees.

use leptos::prelude::*;

/// The id of a visually hidden element containing `description`, for `aria-describedby`: `None`
/// without a description, during server-side rendering and until mounted.
///
/// Identical texts share one element (reference counted); it is removed when its last user is
/// disposed or its description changes.
pub fn use_description(description: Signal<Option<String>>) -> Signal<Option<String>> {
    #[cfg(feature = "ssr")]
    {
        let _ = description;
        Signal::stored(None)
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::oco::Oco;

        let id = RwSignal::new(None::<String>);
        let current: StoredValue<Option<(Oco<'static, str>, Oco<'static, str>)>> =
            StoredValue::new(None);
        Effect::new(move || {
            // No element for an empty description (react-aria: `if (!description) return`).
            let next = description
                .get()
                .filter(|description| !description.is_empty())
                .map(Oco::from);
            let unchanged =
                current.with_value(|c| c.as_ref().map(|(text, _)| text) == next.as_ref());
            if unchanged {
                return;
            }
            if let Some((text, elem_id)) = current.get_value() {
                registry::release(&text, &elem_id);
            }
            let acquired = next.map(|text| {
                let elem_id = registry::acquire(&text);
                (text, elem_id)
            });
            id.set(acquired.as_ref().map(|(_, elem_id)| elem_id.to_string()));
            current.set_value(acquired);
        });
        on_cleanup(move || {
            if let Some((text, elem_id)) = current.try_get_value().flatten() {
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
    };

    use leptos::oco::Oco;
    use leptos_use::use_document;

    struct DescriptionEntry {
        /// Id of the hidden element (e.g. "leptonic-desc-42").
        elem_id: Oco<'static, str>,
        /// The number of active users.
        ref_count: usize,
    }

    thread_local! {
        /// The descriptions in use, with the id of their element and the number of users. The
        /// browser runs one page per thread.
        static DESCRIPTION_NODES: RefCell<HashMap<Oco<'static, str>, DescriptionEntry>> =
            RefCell::new(HashMap::new());
        static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    }

    /// Takes a reference on the element for `description` (creating it), returning its id.
    pub(super) fn acquire(description: &Oco<'static, str>) -> Oco<'static, str> {
        DESCRIPTION_NODES.with_borrow_mut(|map| {
            map.entry(description.clone())
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

    /// Releases a reference taken with [`acquire`].
    pub(super) fn release(description: &Oco<'static, str>, id: &str) {
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

    fn next_id() -> Oco<'static, str> {
        let id = NEXT_ID.get();
        NEXT_ID.set(id.wrapping_add(1));
        Oco::Owned(format!("leptonic-desc-{id}"))
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
