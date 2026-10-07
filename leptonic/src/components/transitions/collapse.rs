use leptos::prelude::*;

use super::transition;
use crate::utils::{classes::Classes, styles::Styles};

/// The direction a [`Collapse`] collapses in.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum CollapseAxis {
    /// Its width.
    X,
    /// Its height.
    #[default]
    Y,
}

/// Expands its children to their full height (or width) and collapses them to nothing. Follows
/// changes of their size while shown.
#[component]
pub fn Collapse(
    /// Whether the children are shown: a value or any signal.
    #[prop(into)]
    is_shown: Signal<bool>,
    #[prop(optional)] axis: CollapseAxis,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = classes.add(match axis {
        CollapseAxis::X => "leptonic-collapse-x",
        CollapseAxis::Y => "leptonic-collapse-y",
    });
    // The grid track animates between nothing and the content's size; the content clips.
    transition(
        "leptonic-collapse",
        is_shown,
        classes,
        styles,
        Box::new(move || {
            view! { <div class="leptonic-collapse-content">{children()}</div> }.into_any()
        }),
    )
}
