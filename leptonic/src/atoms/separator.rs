// Upstream: react-aria-components/src/Separator.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{IntoAttrs, SeparatorElementType, UseSeparatorInput, use_separator},
    utils::{classes::Classes, orientation::Orientation, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The element follows from the orientation and context (an `<hr>`, or a `<div>` when vertical
//   or inside a `Menu`); no `elementType` prop.
// - A separator is not a collection node: inside a `Menu` it is rendered between the items as
//   any other child (react-aria-components: a `SeparatorNode` in the collection).
//
// =============================================================================

/// Provided by components whose separators can't be `<hr>`s (a menu: `role="separator"` on a
/// `<div>`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct SeparatorContext {
    pub element_type: SeparatorElementType,
}

/// A separator dividing content: an `<hr>` (horizontal), or a `<div>` with `role="separator"` when
/// vertical or inside a [`Menu`](super::menu::Menu).
#[component]
pub fn Separator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let element_type = use_context::<SeparatorContext>().map_or(
        match orientation {
            Orientation::Horizontal => SeparatorElementType::Hr,
            Orientation::Vertical => SeparatorElementType::Div,
        },
        |ctx| ctx.element_type,
    );
    // An `<hr>` can't be vertical.
    let element_type = match (element_type, orientation) {
        (SeparatorElementType::Hr, Orientation::Vertical) => SeparatorElementType::Div,
        (element_type, _) => element_type,
    };
    let props = use_separator(UseSeparatorInput {
        orientation,
        element_type,
        aria_label,
        ..UseSeparatorInput::default()
    })
    .props
    .into_attrs();

    match element_type {
        SeparatorElementType::Hr => {
            view! { <hr {..props} class=classes style=styles /> }.into_any()
        }
        SeparatorElementType::Div => {
            view! { <div {..props} class=classes style=styles></div> }.into_any()
        }
        SeparatorElementType::Span => {
            view! { <span {..props} class=classes style=styles></span> }.into_any()
        }
    }
}
