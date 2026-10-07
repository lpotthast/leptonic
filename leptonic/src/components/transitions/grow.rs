use leptos::prelude::*;

use super::transition;
use crate::utils::{classes::Classes, styles::Styles};

/// Grows its children from their center while fading them in.
#[component]
pub fn Grow(
    /// Whether the children are shown: a value or any signal.
    #[prop(into)]
    is_shown: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    transition("leptonic-grow", is_shown, classes, styles, children)
}
