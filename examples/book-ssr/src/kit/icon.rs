use leptos::prelude::*;
use leptos_classes::Classes;

/// An `icondata` icon (`leptos_icons`), `1rem` square unless the class sizes it (`.doc-icon` in `_shell.scss`).
///
/// Decorative (hidden from assistive technology) unless it gets an `aria_label`, e.g. next to text or inside a
/// labelled button. With one, it is an image of that name.
#[component]
pub fn Icon(
    #[prop(into)] icon: Signal<icondata::Icon>,
    /// Names the icon as an image. Without it, the icon is decorative.
    #[prop(optional)]
    aria_label: Option<&'static str>,
    #[prop(into, optional)] classes: Classes,
) -> impl IntoView {
    view! {
        <span
            class=classes.add("doc-icon")
            role=aria_label.map(|_| "img")
            aria-label=aria_label
            aria-hidden=aria_label.is_none().then_some("true")
        >
            <leptos_icons::Icon icon width="100%" height="100%"/>
        </span>
    }
}
