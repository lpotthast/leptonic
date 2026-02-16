use leptos::prelude::*;

// Based on: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/DismissButton.tsx

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## STRUCTURAL
//
// - react-aria wraps the button in a `<VisuallyHidden>` component. We apply
//   visually-hidden styles inline, matching the pattern in
//   `utils/live_announcer.rs`.
//
// ## NOT IMPLEMENTED
//
// - `aria-labelledby`: react-aria supports both `aria-label` and
//   `aria-labelledby`. We only support `aria-label` since `aria-labelledby` is
//   rarely needed for dismiss buttons.
// - Localized default label: react-aria uses i18n for the default "Dismiss"
//   label. We use a static English default with an `aria_label` prop override.
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

    /// Custom accessible label. Defaults to "Dismiss".
    #[prop(into, optional)]
    aria_label: Option<&'static str>,
) -> impl IntoView {
    let label = aria_label.unwrap_or("Dismiss");

    let on_click = move |_: web_sys::MouseEvent| {
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.run(());
        }
    };

    view! {
        <button
            aria-label=label
            tabindex="-1"
            on:click=on_click
            style="position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;"
        />
    }
}
