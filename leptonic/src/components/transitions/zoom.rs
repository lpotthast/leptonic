use leptos::prelude::*;

use super::transition;
use crate::utils::{classes::Classes, styles::Styles};

/// Zooms its children in from nothing.
#[component]
pub fn Zoom(
    /// Whether the children are shown: a value or any signal.
    #[prop(into)]
    is_shown: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    transition("leptonic-zoom", is_shown, classes, styles, children)
}
