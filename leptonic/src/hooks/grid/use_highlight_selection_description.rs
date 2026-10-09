// Upstream: react-aria/src/grid/useHighlightSelectionDescription.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{
        collections::{SelectionBehavior, SelectionManager, SelectionMode},
        focus::{Modality, use_interaction_modality},
    },
    utils::{
        intl_strings::{GridStrings, use_localized_strings},
        platform::device::has_touch_events,
        use_description::use_description,
    },
};

// No deviations from react-aria beyond the project-wide API conventions.

/// Input of [`use_highlight_selection_description`].
#[derive(Debug, Clone, Copy)]
pub struct UseHighlightSelectionDescriptionInput {
    pub selection: SelectionManager,
    /// Whether the items have actions (row or cell actions).
    pub has_item_actions: bool,
}

/// How to select items of a collection whose press performs their action ("highlight
/// selection": replace behavior with actions): on touch devices, "Long press to enter selection
/// mode." Returns the id for the collection's `aria-describedby` while there is a description.
pub fn use_highlight_selection_description(
    input: UseHighlightSelectionDescriptionInput,
) -> Signal<Option<String>> {
    let UseHighlightSelectionDescriptionInput {
        selection,
        has_item_actions,
    } = input;
    let strings = use_localized_strings::<GridStrings>();
    let modality = use_interaction_modality();
    use_description(Signal::derive(move || {
        // Before any interaction too (react-aria: modality `null`).
        let should_long_press = matches!(
            modality.get(),
            None | Some(Modality::Pointer | Modality::Virtual)
        ) && has_touch_events();
        let highlights = selection.selection_behavior() == SelectionBehavior::Replace
            && selection.selection_mode() != SelectionMode::None
            && has_item_actions;
        (highlights && should_long_press).then(|| strings.read().long_press_to_select())
    }))
}
