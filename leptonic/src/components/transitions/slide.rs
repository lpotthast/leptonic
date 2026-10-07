use leptos::prelude::*;

use super::transition;
use crate::utils::{classes::Classes, styles::Styles};

/// Slides its children in from below.
#[component]
pub fn Slide(
    /// Whether the children are shown: a value or any signal.
    #[prop(into)]
    is_shown: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    transition("leptonic-slide", is_shown, classes, styles, children)
}
