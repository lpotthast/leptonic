use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use leptos::{oco::Oco, prelude::*};
use leptos_use::use_document;

use crate::utils::aria::AriaDescribedby;

// TODO: Should this really be in utils? Move to hooks?

struct DescriptionEntry {
    /// ID of the descriptive <span> (e.g. "leptos-desc-42").
    elem_id: Oco<'static, str>,

    /// The number of active requests for this description.
    ref_count: usize,
}

thread_local! {
    /// Stores all descriptive texts currently in use, together with the identifier of the DOM
    /// element that was created for it and the number of active requests.
    static DESCRIPTION_NODES: RefCell<HashMap<Oco<'static, str>, DescriptionEntry>> =
        RefCell::new(HashMap::new());

    /// Counter used to generate unique element IDs.
    static NEXT_ID: AtomicU64 = const { AtomicU64::new(0) };
}

/// Returns true when the entry was removed.
fn decrement(description: &Oco<'static, str>) -> bool {
    DESCRIPTION_NODES.with(|nodes| {
        let mut map = nodes.borrow_mut();
        if let Some(entry) = map.get_mut(description) {
            entry.ref_count -= 1;
            if entry.ref_count == 0 {
                map.remove(description);
                return true;
            }
        }
        false
    })
}

/// Create a new, unique ID for the next DOM element to insert.
fn next_id() -> Oco<'static, str> {
    NEXT_ID.with(|id| {
        // Note: We do want the wrap-around-on-overflow behavior.
        let next_id = id.fetch_add(1, Ordering::SeqCst);
        Oco::Owned(format!("leptonic-desc-{next_id}"))
    })
}

/// Creates a visually-hidden `<div>` element containing the given description text,
/// appends it to `<body>`, and returns the element's unique ID.
///
/// This is the Leptonic equivalent of react-aria's `useDescription` hook.
/// The hidden element is referenced by `aria-describedby` so that assistive technologies
/// can read the description text.
///
/// Identical description texts are deduplicated: multiple callers sharing the same text
/// share a single hidden DOM element, tracked via a global reference-counted registry.
///
/// The element is removed from the DOM when the last consumer's reactive scope is dropped.
pub fn use_description(description: Oco<'static, str>) -> AriaDescribedby {
    let id = DESCRIPTION_NODES.with(|nodes| {
        let mut map = nodes.borrow_mut();

        let entry = map
            .entry(description.clone())
            .and_modify(|e| e.ref_count += 1)
            .or_insert_with(|| {
                let elem_id = next_id();
                create_dom_node(elem_id.clone(), description.clone());
                DescriptionEntry {
                    elem_id,
                    ref_count: 1,
                }
            });

        entry.elem_id.clone()
    });

    let cleanup_id = id.clone();
    on_cleanup(move || {
        let should_remove = decrement(&description);
        if should_remove {
            remove_dom_node(&cleanup_id);
        }
    });

    AriaDescribedby::element_with_id(id)
}

fn create_dom_node(id: Oco<'static, str>, description: Oco<'static, str>) {
    Effect::new(move || insert_dom_node(&id, &description));
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

/// Take a reference on the description element for `description` (creating it), returning its id.
fn acquire(description: &Oco<'static, str>) -> Oco<'static, str> {
    DESCRIPTION_NODES.with(|nodes| {
        let mut map = nodes.borrow_mut();
        let entry = map
            .entry(description.clone())
            .and_modify(|e| e.ref_count += 1)
            .or_insert_with(|| {
                let elem_id = next_id();
                insert_dom_node(&elem_id, description);
                DescriptionEntry {
                    elem_id,
                    ref_count: 1,
                }
            });
        entry.elem_id.clone()
    })
}

/// Release a reference taken with `acquire`.
fn release(description: &Oco<'static, str>, id: &str) {
    if decrement(description) {
        remove_dom_node(&Oco::Owned(id.to_owned()));
    }
}

/// [`use_description`] for a description that changes: the id of a hidden element containing
/// the current description (`None` without one, and during server-side rendering).
pub fn use_reactive_description(description: Signal<Option<String>>) -> Signal<Option<String>> {
    let id = RwSignal::new(None::<String>);
    let current: StoredValue<Option<(Oco<'static, str>, Oco<'static, str>)>> =
        StoredValue::new(None);
    Effect::new(move || {
        let next = description.get().map(Oco::from);
        let unchanged = current.with_value(|c| c.as_ref().map(|(text, _)| text) == next.as_ref());
        if unchanged {
            return;
        }
        if let Some((text, elem_id)) = current.get_value() {
            release(&text, &elem_id);
        }
        let acquired = next.map(|text| {
            let elem_id = acquire(&text);
            (text, elem_id)
        });
        id.set(acquired.as_ref().map(|(_, elem_id)| elem_id.to_string()));
        current.set_value(acquired);
    });
    on_cleanup(move || {
        if let Some((text, elem_id)) = current.try_get_value().flatten() {
            release(&text, &elem_id);
        }
    });
    id.into()
}

fn remove_dom_node(id: &Oco<'static, str>) {
    if let Some(document) = use_document().as_ref()
        && let Some(el) = document.get_element_by_id(id)
    {
        el.remove();
    }
}
