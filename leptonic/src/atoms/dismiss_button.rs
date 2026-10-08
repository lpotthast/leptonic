// Upstream: react-aria/src/overlays/DismissButton.tsx @ 99e6102368
use leptos::prelude::*;

use super::visually_hidden::VisuallyHidden;
use crate::utils::id::use_id;
use crate::utils::intl_strings::{OverlayStrings, use_localized_strings};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `on_dismiss` is required: a dismiss button that dismisses nothing is a trap for screen reader
//   users.
//
// =============================================================================

/// A visually hidden button that allows screen reader users to dismiss an overlay.
///
/// Place at the start and/or end of overlay content (popovers, modals, trays)
/// to provide an accessible dismiss mechanism for users who cannot press Escape,
/// such as mobile VoiceOver users. `Popover` and a dismissable `ModalContent` render theirs.
///
/// Named "Dismiss" unless `aria_label` or `aria_labelledby` name it; with both, it is labelled by
/// itself (its `aria_label`) and the referenced elements (react-aria's `useLabels`).
#[component]
pub fn DismissButton(
    /// Called when the dismiss button is activated.
    #[prop(into)]
    on_dismiss: Callback<()>,
    /// The button's name. Default: "Dismiss" (unless `aria_labelledby` names it).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// The ids of the elements naming the button.
    #[prop(into, optional)]
    aria_labelledby: Option<String>,
    /// The button's id. Default: a generated one.
    #[prop(into, optional)]
    id: Option<String>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| use_id("dismiss-button"));
    let labelledby = aria_labelledby.map(|ids| {
        ids.split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    });
    let has_labelledby = labelledby.as_ref().is_some_and(|ids| !ids.is_empty());
    let strings = use_localized_strings::<OverlayStrings>();
    let label = move || {
        aria_label
            .get()
            .or_else(|| (!has_labelledby).then(|| strings.read().dismiss()))
    };
    // Labelled by other elements and its own label: its own id comes first.
    let own_id = id.clone();
    let labelledby = move || {
        let ids = labelledby.clone().filter(|ids| !ids.is_empty())?;
        let mut all = Vec::with_capacity(ids.len() + 1);
        if aria_label.read().is_some() {
            all.push(own_id.clone());
        }
        for id in ids {
            if !all.contains(&id) {
                all.push(id);
            }
        }
        Some(all.join(" "))
    };

    view! {
        <VisuallyHidden>
            <button
                id=id
                aria-label=label
                aria-labelledby=labelledby
                tabindex="-1"
                on:click=move |_| on_dismiss.run(())
                style="width: 1px; height: 1px;"
            />
        </VisuallyHidden>
    }
}
