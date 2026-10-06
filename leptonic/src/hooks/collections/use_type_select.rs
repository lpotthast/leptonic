// Upstream: react-aria/src/selection/useTypeSelect.ts @ 99e6102368
use std::{sync::Arc, time::Duration};

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use super::{Key, KeyboardDelegate, SelectionManager};
use crate::utils::{EventAccessors, EventHandler, node_contains};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The typed text is forgotten after this long without typing.
const TYPEAHEAD_DEBOUNCE_WAIT: Duration = Duration::from_secs(1);

/// Input of [`use_type_select`].
#[derive(Clone)]
pub struct UseTypeSelectInput {
    /// Finds items by their text (`KeyboardDelegate::key_for_search`).
    pub delegate: Signal<Arc<dyn KeyboardDelegate>>,
    pub selection: SelectionManager,
    /// Called with the item type-ahead moved focus to.
    pub on_type_select: Option<Callback<Key>>,
}

/// Return value of [`use_type_select`].
#[derive(Debug)]
pub struct UseTypeSelectReturn {
    pub props: UseTypeSelectProps,
}

/// Handlers for the collection element. Attach `on_keydown_capture` in the capture phase
/// (`handler.into_on(ev::capture(ev::keydown))`).
#[derive(Debug)]
pub struct UseTypeSelectProps {
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

/// Type-ahead: typing characters moves focus to the next item whose text starts with what was
/// typed. Typing within a second continues the search ("ap" finds "Apple" after "Avocado"), and
/// a space then belongs to the search instead of selecting the focused item.
pub fn use_type_select(input: UseTypeSelectInput) -> UseTypeSelectReturn {
    let UseTypeSelectInput {
        delegate,
        selection,
        on_type_select,
    } = input;

    let search = StoredValue::new(String::new());
    let timeout: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    let restart_timeout = move || {
        if let Some(handle) = timeout.get_value() {
            handle.clear();
        }
        let handle = set_timeout_with_handle(
            move || search.update_value(String::clear),
            TYPEAHEAD_DEBOUNCE_WAIT,
        )
        .ok();
        timeout.set_value(handle);
    };
    on_cleanup(move || {
        if let Some(handle) = timeout.get_value() {
            handle.clear();
        }
    });

    // Prefer items after the focused one, then search from the top.
    let find = move |text: &str| {
        let delegate = delegate.get_untracked();
        let focused = untrack(|| selection.focused_key());
        delegate
            .key_for_search(text, focused.as_ref())
            .or_else(|| delegate.key_for_search(text, None))
    };
    let focus = move |key: Key| {
        selection.set_focused_key(Some(key.clone()), None);
        if let Some(on_type_select) = on_type_select {
            on_type_select.run(key);
        }
    };

    let on_keydown_capture = move |e: KeyboardEvent| {
        // During a search, a space belongs to the search (instead of e.g. selecting the focused
        // item). Handled in the capture phase, before the item sees the key.
        if e.key() != " " || search.with_value(String::is_empty) {
            return;
        }
        e.prevent_default();
        e.stop_propagation();
        search.update_value(|s| s.push(' '));
        if let Some(key) = find(&search.get_value()) {
            focus(key);
        }
        restart_timeout();
    };

    let on_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        let Some(character) = character_for_key(&key) else {
            return;
        };
        let inside = node_contains(
            e.expect_current_target().dyn_ref::<web_sys::Node>(),
            e.target()
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>()),
        )
        .unwrap_or(false);
        if e.ctrl_key()
            || e.meta_key()
            || e.alt_key()
            || !inside
            || (search.with_value(String::is_empty) && character == " ")
        {
            return;
        }

        search.update_value(|s| s.push_str(character));
        if let Some(key) = find(&search.get_value()) {
            focus(key);
            e.prevent_default();
            e.stop_propagation();
        } else {
            // Nothing matches: the search is over.
            search.update_value(String::clear);
            if let Some(handle) = timeout.get_value() {
                handle.clear();
            }
            timeout.set_value(None);
            return;
        }
        restart_timeout();
    };

    UseTypeSelectReturn {
        props: UseTypeSelectProps {
            on_keydown_capture: EventHandler::new(on_keydown_capture),
            on_keydown: EventHandler::new(on_keydown),
        },
    }
}

/// The text a key press types, if any. Named keys ("Enter", "ArrowDown", ...) type nothing.
/// Single characters and non-ASCII key values (e.g. from input methods) do.
fn character_for_key(key: &str) -> Option<&str> {
    let is_named_key =
        key.chars().count() > 1 && key.starts_with(|c: char| c.is_ascii_alphabetic());
    (!key.is_empty() && !is_named_key).then_some(key)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn named_keys_type_nothing() {
        assert_that!(character_for_key("a")).is_equal_to(Some("a"));
        assert_that!(character_for_key(" ")).is_equal_to(Some(" "));
        assert_that!(character_for_key("ß")).is_equal_to(Some("ß"));
        assert_that!(character_for_key("ä")).is_equal_to(Some("ä"));
        assert_that!(character_for_key("Enter")).is_none();
        assert_that!(character_for_key("ArrowDown")).is_none();
        assert_that!(character_for_key("")).is_none();
    }
}
