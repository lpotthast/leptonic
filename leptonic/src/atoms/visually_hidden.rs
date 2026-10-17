// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
// Upstream: react-aria/test/visually-hidden/VisuallyHidden.test.tsx @ 99e6102368
use leptos::{either::Either, prelude::*};
use leptos_classes::Classes;

use crate::{
    atoms::field::TextElement,
    hooks::visually_hidden::{UseVisuallyHiddenInput, use_visually_hidden},
    utils::default_class::with_default_class,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The element is a `TextElement` (`Div`, `Span`) instead of any `elementType`. Reason: a typed
//   choice of the elements that fit instead of a string or component.
// - No `styles` prop, as `use_visually_hidden` takes no `style` to merge: the hiding styles are
//   the element's whole `style` (react-aria merges a `style` prop into them).
//
// =============================================================================

/// Hides its children visually while keeping them available to assistive technology. With
/// `is_focusable`, they show while focus is within them (e.g. a "skip to content" link).
///
/// Default class: `leptonic-VisuallyHidden`.
#[component]
pub fn VisuallyHidden(
    #[prop(default = TextElement::Div)] element: TextElement,
    /// Show the content while focus is within it (e.g. a "skip to content" link).
    #[prop(into, optional)]
    is_focusable: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-VisuallyHidden", classes);
    let hidden = use_visually_hidden(UseVisuallyHiddenInput { is_focusable }).target;
    match element {
        TextElement::Div => Either::Left(view! {
            <div {..hidden.attrs()} class=classes>{children()}</div>
        }),
        TextElement::Span => Either::Right(view! {
            <span {..hidden.attrs()} class=classes>{children()}</span>
        }),
    }
}
