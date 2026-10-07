use leptos::prelude::*;

use super::transition;
use crate::utils::{classes::Classes, styles::Styles};

/// Fades its children in and out.
#[component]
pub fn Fade(
    /// Whether the children are shown: a value or any signal.
    #[prop(into)]
    is_shown: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    transition("leptonic-fade", is_shown, classes, styles, children)
}
