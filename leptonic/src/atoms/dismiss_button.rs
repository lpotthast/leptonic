// Upstream: react-aria/src/overlays/DismissButton.tsx @ 99e6102368
use leptos::prelude::*;

use super::visually_hidden::VisuallyHidden;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Localized default label: "Dismiss" is English until leptonic has a localized string
//   formatter.
//
// =============================================================================

/// A visually hidden button that allows screen reader users to dismiss an overlay.
///
/// Place at the start and/or end of overlay content (popovers, modals, trays)
/// to provide an accessible dismiss mechanism for users who cannot press Escape,
/// such as mobile VoiceOver users.
#[component]
pub fn DismissButton(
    /// Callback invoked when the dismiss button is activated.
    #[prop(into, optional)]
    on_dismiss: Option<Callback<()>>,
    /// The button's name. Default: "Dismiss" (unless `aria_labelledby` names it).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
) -> impl IntoView {
    let has_labelledby = aria_labelledby.is_some();
    let label = move || {
        aria_label
            .get()
            .or_else(|| (!has_labelledby).then(|| "Dismiss".to_owned()))
    };
    let on_click = move |_: web_sys::MouseEvent| {
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.run(());
        }
    };

    view! {
        <VisuallyHidden>
            <button
                aria-label=label
                aria-labelledby=aria_labelledby
                tabindex="-1"
                on:click=on_click
                style="width: 1px; height: 1px;"
            />
        </VisuallyHidden>
    }
}
