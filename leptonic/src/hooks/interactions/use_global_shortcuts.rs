// No upstream: document-wide keyboard shortcuts (react-aria has no equivalent).
//! Keyboard shortcuts for the whole document.
use crate::utils::keyboard_shortcut::KeyboardShortcuts;

/// Input of [`use_global_shortcuts`].
#[derive(Debug, Default)]
pub struct UseGlobalShortcutsInput {
    /// Shortcuts that work anywhere, also while the user types in a text field, e.g. `Mod+K`
    /// for a command palette.
    pub anywhere: KeyboardShortcuts,
    /// Shortcuts that work only while the user isn't typing in a text field, `<select>` or
    /// editable content (see [`is_typing_target`](crate::utils::focusability::is_typing_target)),
    /// e.g. a bare `/` focusing a search field.
    pub outside_text_fields: KeyboardShortcuts,
}

/// Binds keyboard shortcuts document-wide while the calling owner lives (removed on cleanup).
///
/// The shortcuts see every key press first (a capture listener on the document): leptonic's
/// components stop their events from bubbling, so a listener at the app's root would miss them.
/// A handled shortcut prevents the browser default and stops the event (see
/// [`ShortcutOutcome`](crate::utils::keyboard_shortcut::ShortcutOutcome)). Key presses that
/// compose text (IME) are ignored. Server-side rendering binds nothing.
///
/// When several calls bind the same shortcut, the most recent one wins (e.g. a dialog's
/// shortcuts over the app's); its handler can pass the key press on to the earlier ones with
/// `continue_propagation`.
pub fn use_global_shortcuts(input: UseGlobalShortcutsInput) {
    #[cfg(feature = "ssr")]
    let _ = input;

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::*;

        let id = registry::register(input);
        on_cleanup(move || registry::unregister(id));
    }
}

/// The bound shortcut sets, newest last, and the one document listener checking them.
#[cfg(not(feature = "ssr"))]
mod registry {
    use std::{cell::RefCell, rc::Rc};

    use leptos::ev;
    use wasm_bindgen::JsCast;

    use super::UseGlobalShortcutsInput;
    use crate::utils::{
        event_listeners::listen_to, focusability::is_typing_target,
        keyboard_shortcut::ShortcutOutcome,
    };

    #[derive(Default)]
    struct Registry {
        next_id: u64,
        entries: Vec<(u64, Rc<UseGlobalShortcutsInput>)>,
        listener: Option<Box<dyn std::any::Any>>,
    }

    thread_local! {
        static REGISTRY: RefCell<Registry> = RefCell::default();
    }

    pub(super) fn register(input: UseGlobalShortcutsInput) -> u64 {
        REGISTRY.with_borrow_mut(|registry| {
            let id = registry.next_id;
            registry.next_id += 1;
            registry.entries.push((id, Rc::new(input)));
            if registry.listener.is_none()
                && let Some(document) = leptos_use::use_document().as_ref().cloned()
            {
                registry.listener = Some(Box::new(listen_to(
                    &document,
                    ev::keydown,
                    true,
                    on_keydown,
                )));
            }
            id
        })
    }

    pub(super) fn unregister(id: u64) {
        // The listener is dropped outside the borrow (dropping it removes it from the document).
        let _listener = REGISTRY.with_borrow_mut(|registry| {
            registry.entries.retain(|(entry_id, _)| *entry_id != id);
            if registry.entries.is_empty() {
                registry.listener.take()
            } else {
                None
            }
        });
    }

    fn on_keydown(e: web_sys::KeyboardEvent) {
        if e.is_composing() {
            return;
        }
        let typing = crate::utils::shadow_dom::get_event_target(&e)
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|target| is_typing_target(&target));
        // A snapshot: handlers may bind or unbind shortcuts.
        let entries: Vec<_> = REGISTRY.with_borrow(|registry| {
            registry
                .entries
                .iter()
                .rev()
                .map(|(id, entry)| (*id, Rc::clone(entry)))
                .collect()
        });
        // An ignored shortcut counts as none.
        let acted = |outcome: Option<ShortcutOutcome>| {
            outcome.filter(|outcome| !matches!(outcome, ShortcutOutcome::Ignored))
        };
        for (id, entry) in entries {
            // A handler before may have unbound it (e.g. by navigating away).
            let is_bound = REGISTRY.with_borrow(|registry| {
                registry.entries.iter().any(|(entry_id, _)| *entry_id == id)
            });
            if !is_bound {
                continue;
            }
            let outcome = (!typing)
                .then(|| acted(entry.outside_text_fields.handle(&e)))
                .flatten()
                .or_else(|| acted(entry.anywhere.handle(&e)));
            let Some(outcome) = outcome else {
                continue;
            };
            if outcome.prevent_default() {
                e.prevent_default();
            }
            if !outcome.continue_propagation() {
                e.stop_propagation();
                return;
            }
        }
    }
}
