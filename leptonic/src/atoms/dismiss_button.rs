// Upstream: react-aria/src/overlays/DismissButton.tsx @ 99e6102368
// Upstream: react-aria/test/overlays/DismissButton.test.tsx @ 99e6102368
use leptos::prelude::*;

use super::visually_hidden::VisuallyHidden;
use crate::{
    labels,
    utils::{
        id::use_id,
        intl_strings::{OverlayStrings, use_localized_strings},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `on_dismiss` is required: a dismiss button that dismisses nothing is a trap for screen reader
//   users.
//
// ## ADDITIONS
// - `type="button"`: upstream's dismiss button submits an enclosing form (and Enter in a field
//   "dismisses" through implicit submission).
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
    let strings = use_localized_strings::<OverlayStrings>();
    // With both a label and labelling ids, labelled by itself (first) and the referenced
    // elements; without either, "Dismiss" (react-aria's `useLabels` with a default label).
    let labelling = {
        let id = id.clone();
        Signal::derive(move || labels(&id, aria_label.get(), aria_labelledby.as_deref()))
    };
    let label = move || {
        labelling.with(|labelling| {
            labelling.aria_label.clone().or_else(|| {
                labelling
                    .aria_labelledby
                    .is_none()
                    .then(|| strings.read().dismiss())
            })
        })
    };
    let labelledby = move || labelling.with(|labelling| labelling.aria_labelledby.clone());

    view! {
        <VisuallyHidden>
            <button
                type="button"
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
