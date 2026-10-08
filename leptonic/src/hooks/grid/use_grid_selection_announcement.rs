// Upstream: react-aria/src/grid/useGridSelectionAnnouncement.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::prelude::*;

use crate::{
    hooks::collections::{
        CollectionMemo, Key, Selection, SelectionBehavior, SelectionManager, SelectionMode,
    },
    utils::intl_strings::{GridStrings, use_localized_strings},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `get_row_text` returns `Option<String>` (react-aria: a string, empty for none).
//
// =============================================================================

/// The text announced for a row (`None`: nothing to announce).
pub type GetRowText = Arc<dyn Fn(&Key) -> Option<String> + Send + Sync>;

/// Input of [`use_grid_selection_announcement`].
#[derive(Clone)]
pub struct UseGridSelectionAnnouncementInput {
    pub selection: SelectionManager,
    pub collection: CollectionMemo,
    /// The text announced for a row. Default: its text value.
    pub get_row_text: Option<GetRowText>,
}

/// Announces selection changes of a grid (or grid list, or table) through the live announcer:
/// "Apple selected.", "Apple not selected.", "3 items selected.", "All items selected." Many
/// screen readers don't announce them themselves.
pub fn use_grid_selection_announcement(input: UseGridSelectionAnnouncementInput) {
    let UseGridSelectionAnnouncementInput {
        selection,
        collection,
        get_row_text,
    } = input;
    let strings = use_localized_strings::<GridStrings>();
    let row_text = move |key: &Key| match &get_row_text {
        Some(get_row_text) => get_row_text(key),
        None => collection.with_untracked(|c| {
            c.get(key)
                .map(|node| node.text_value.to_string())
                .filter(|text| !text.is_empty())
        }),
    };
    let last = StoredValue::new(untrack(|| selection.raw_selection()));
    let announce_change = Arc::new(move || {
        let current = untrack(|| selection.raw_selection());
        let previous = last.get_value();
        last.set_value(current.clone());
        if !untrack(|| selection.is_focused()) || current == previous {
            return;
        }
        let added = diff(&current, &previous);
        let removed = diff(&previous, &current);
        let strings = strings.get_untracked();
        let has_item = |key: &Key| collection.with_untracked(|c| c.get(key).is_some());
        let mut messages = Vec::new();
        let selected_keys = untrack(|| selection.selected_keys());
        let is_replace = untrack(|| selection.selection_behavior()) == SelectionBehavior::Replace;
        if selected_keys.len() == 1 && is_replace {
            if let Some(key) = selected_keys.iter().next().filter(|key| has_item(key))
                && let Some(text) = row_text(key)
            {
                messages.push(strings.selected_item(&text));
            }
        } else if added.len() == 1 && removed.is_empty() {
            if let Some(text) = added.iter().next().and_then(&row_text) {
                messages.push(strings.selected_item(&text));
            }
        } else if removed.len() == 1
            && added.is_empty()
            && let Some(key) = removed.iter().next().filter(|key| has_item(key))
            && let Some(text) = row_text(key)
        {
            messages.push(strings.deselected_item(&text));
        }
        // How many items are selected, except when selecting the first one.
        let size = |selection: &Selection| match selection {
            Selection::All => None,
            Selection::Keys(keys) => Some(keys.len()),
        };
        if untrack(|| selection.selection_mode()) == SelectionMode::Multiple
            && (messages.is_empty()
                || size(&current).is_none_or(|size| size > 1)
                || size(&previous).is_none_or(|size| size > 1))
        {
            messages.push(match size(&current) {
                None => strings.selected_all(),
                Some(count) => strings.selected_count(count),
            });
        }
        if !messages.is_empty() {
            crate::utils::live_announcer::announce(
                messages.join(" "),
                crate::utils::live_announcer::Assertiveness::Polite,
            );
        }
    });

    // On changes after the first render (react-aria's `useUpdateEffect`).
    Effect::new(move |previous: Option<()>| {
        let _ = selection.raw_selection();
        let is_focused = selection.is_focused();
        if previous.is_none() {
            return;
        }
        if is_focused {
            announce_change();
        } else {
            // A frame later: the collection may be about to get focus (e.g. on mouse down). The
            // cleanup cancels the frame when the grid goes away.
            let announce_change = announce_change.clone();
            let handle = request_animation_frame_with_handle(move || announce_change()).ok();
            on_cleanup(move || {
                if let Some(handle) = handle {
                    handle.cancel();
                }
            });
        }
    });
}

/// The keys in `a` but not in `b` (none if either is "all").
fn diff(a: &Selection, b: &Selection) -> HashSet<Key> {
    match (a, b) {
        (Selection::Keys(a), Selection::Keys(b)) => {
            a.iter().filter(|key| !b.contains(key)).cloned().collect()
        }
        _ => HashSet::new(),
    }
}
