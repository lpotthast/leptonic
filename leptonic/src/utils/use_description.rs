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
fn decrement(description: Oco<'static, str>) -> bool {
    DESCRIPTION_NODES.with(|nodes| {
        let mut map = nodes.borrow_mut();
        if let Some(entry) = map.get_mut(&description) {
            entry.ref_count -= 1;
            if entry.ref_count == 0 {
                map.remove(&description);
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
        let should_remove = decrement(description.clone());
        if should_remove {
            remove_dom_node(cleanup_id);
        }
    });

    AriaDescribedby::element_with_id(id)
}

fn create_dom_node(id: Oco<'static, str>, description: Oco<'static, str>) {
    Effect::new(move || {
        if let Some(document) = use_document().as_ref() {
            if let Ok(div) = document.create_element("div") {
                let _ = div.set_attribute("id", &id);
                let _ = div.set_attribute("style", "display: none;");
                div.set_text_content(Some(&description));

                if let Some(body) = document.body() {
                    let _ = body.append_child(&div);
                }
            }
        }
    });
}

fn remove_dom_node(id: Oco<'static, str>) {
    if let Some(document) = use_document().as_ref() {
        if let Some(el) = document.get_element_by_id(&id) {
            el.remove();
        }
    }
}
