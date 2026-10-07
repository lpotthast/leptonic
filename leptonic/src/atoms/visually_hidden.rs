// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
use leptos::{either::Either, prelude::*};

use crate::{
    atoms::field::TextElement,
    hooks::{IntoAttrs, UseVisuallyHiddenInput, use_visually_hidden},
    utils::{classes::Classes, default_class::with_default_class},
};

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
    let classes = with_default_class("leptonic-VisuallyHidden", classes);
    let props = use_visually_hidden(UseVisuallyHiddenInput { is_focusable }).props;
    match element {
        TextElement::Div => Either::Left(view! {
            <div {..props.into_attrs()} class=classes>{children()}</div>
        }),
        TextElement::Span => Either::Right(view! {
            <span {..props.into_attrs()} class=classes>{children()}</span>
        }),
    }
}
