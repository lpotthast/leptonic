//! Drop indicator hook for visual drop target feedback within collections.
//!
//! Provides ARIA labels and visibility state for drop indicator elements
//! that show users where items will be inserted during drag-and-drop.
//!
//! Based on react-aria's `useDropIndicator` from
//! `@react-aria/dnd/src/useDropIndicator.ts`.

use leptos::{attr, attr::Attr, prelude::*};

use super::{
    drag_manager,
    droppable_collection_state::DroppableCollectionState,
    types::{DropPosition, DropTarget},
};
use crate::hooks::IntoAttrs;

//
// ## RUST-NATIVE DESIGN
//
// - Aria-label is a derived `Signal<String>` computed from `DropTarget` and
//   `get_text_value`, instead of react-aria's imperative string building.
// - `is_hidden` is a derived `Signal<bool>` instead of an imperative check.
//
// ## API DIFFERENCES
//
// - `get_text_value` is a `Callback` returning a human-readable label for a
//   key, used in aria-label computation.
// - Uses `DroppableCollectionState` directly instead of react-aria's
//   `DropIndicatorAria` return type.
//

/// Input for the [`use_drop_indicator`] hook.
pub struct UseDropIndicatorInput {
    /// The drop target this indicator represents.
    pub target: DropTarget,
    /// The droppable collection state.
    pub state: DroppableCollectionState,
    /// Ordered keys in the collection (for adjacent label computation).
    pub collection_keys: Signal<Vec<String>>,
    /// Returns human-readable text for a key (used in aria-label).
    pub get_text_value: Callback<String, String>,
}

/// Return value of the [`use_drop_indicator`] hook.
pub struct UseDropIndicatorReturn {
    /// Props for the drop indicator element.
    pub drop_indicator_props: UseDropIndicatorProps,
    /// Whether this indicator is the current drop target.
    pub is_drop_target: Signal<bool>,
    /// Whether this indicator should be hidden (no active drag session
    /// and this indicator is not the current drop target).
    pub is_hidden: Signal<bool>,
}

/// Props for a drop indicator element.
#[derive(Debug)]
pub struct UseDropIndicatorProps {
    /// The `role` attribute.
    pub role: &'static str,
    /// The `aria-roledescription` attribute.
    pub aria_roledescription: &'static str,
    /// The computed `aria-label` for the indicator.
    pub aria_label: Signal<String>,
    /// The `aria-hidden` attribute — hides the indicator from screen readers
    /// when no drag session is active.
    pub aria_hidden: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseDropIndicatorProps {
    type Attrs = UseDropIndicatorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaRoledescription, self.aria_roledescription),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for a drop indicator element.
pub type UseDropIndicatorAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRoledescription, &'static str>,
    Attr<attr::AriaLabel, Signal<String>>,
    Attr<attr::AriaHidden, Signal<Option<&'static str>>>,
);

/// Provides accessibility props and visibility state for a drop indicator
/// within a droppable collection.
///
/// Drop indicators are visual elements (typically lines or highlights) that
/// show where dragged items will be inserted. Each indicator corresponds to
/// a specific [`DropTarget`] position.
///
/// # Example
///
/// ```ignore
/// let indicator = use_drop_indicator(UseDropIndicatorInput {
///     target: DropTarget::Item { key: "item-1".into(), position: DropPosition::Before },
///     state: collection_state.clone(),
///     collection_keys: keys_signal,
///     get_text_value: Callback::new(|key: String| format!("Item {key}")),
/// });
///
/// view! {
///     <div
///         {..indicator.drop_indicator_props.into_attrs()}
///         class:hidden=move || indicator.is_hidden.get()
///         class:active=move || indicator.is_drop_target.get()
///     />
/// }
/// ```
pub fn use_drop_indicator(input: UseDropIndicatorInput) -> UseDropIndicatorReturn {
    let UseDropIndicatorInput {
        target,
        state,
        collection_keys,
        get_text_value,
    } = input;

    let target_for_active = target.clone();
    let state_for_active = state.clone();

    // Whether this indicator is the current drop target.
    let is_drop_target =
        Signal::derive(move || state_for_active.is_drop_target(&target_for_active));

    // Subscribe to keyboard drag session state.
    let session_active = drag_manager::use_drag_session_active();

    // Whether to hide the indicator: visible when actively targeted OR
    // when a keyboard drag session is active (so indicators are navigable).
    let is_hidden = Signal::derive(move || !is_drop_target.get() && !session_active.get());

    // aria-hidden: hide from screen readers when no session is active.
    let aria_hidden = Signal::derive(move || {
        if session_active.get() {
            None
        } else {
            Some("true")
        }
    });

    // Compute the aria-label based on the drop target position.
    let aria_label =
        Signal::derive(move || compute_aria_label(&target, &collection_keys, &get_text_value));

    UseDropIndicatorReturn {
        drop_indicator_props: UseDropIndicatorProps {
            role: "option",
            aria_roledescription: "drop indicator",
            aria_label,
            aria_hidden,
        },
        is_drop_target,
        is_hidden,
    }
}

/// Computes the aria-label for a drop indicator based on its position.
fn compute_aria_label(
    target: &DropTarget,
    collection_keys: &Signal<Vec<String>>,
    get_text_value: &Callback<String, String>,
) -> String {
    match target {
        DropTarget::Root => "Drop on collection".to_owned(),
        DropTarget::Item { key, position } => {
            let text = get_text_value.run(key.clone());
            match position {
                DropPosition::On => format!("Drop on {text}"),
                DropPosition::Before => {
                    let prev_text = collection_keys.with(|keys| {
                        let idx = keys.iter().position(|k| k == key);
                        idx.and_then(|i| {
                            if i > 0 {
                                Some(get_text_value.run(keys[i - 1].clone()))
                            } else {
                                None
                            }
                        })
                    });

                    if let Some(prev) = prev_text {
                        format!("Insert between {prev} and {text}")
                    } else {
                        format!("Insert before {text}")
                    }
                }
                DropPosition::After => {
                    let next_text = collection_keys.with(|keys| {
                        let idx = keys.iter().position(|k| k == key);
                        idx.and_then(|i| keys.get(i + 1).map(|k| get_text_value.run(k.clone())))
                    });

                    if let Some(next) = next_text {
                        format!("Insert between {text} and {next}")
                    } else {
                        format!("Insert after {text}")
                    }
                }
            }
        }
    }
}
