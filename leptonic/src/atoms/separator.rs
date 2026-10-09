// Upstream: react-aria-components/src/Separator.tsx @ 99e6102368
use leptos::prelude::*;
use leptos_classes::Classes;

use crate::{
    IntoAttrs, Orientation,
    hooks::separator::{SeparatorElementType, UseSeparatorInput, use_separator},
    utils::{default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The element follows from the orientation and context (an `<hr>`, or a `<div>` when vertical
//   or inside a `Menu`; it changes with the orientation); no `elementType` prop.
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
///
/// Default class: `leptonic-Separator`.
#[component]
pub fn Separator(
    /// Default: horizontal.
    #[prop(into, default = Orientation::Horizontal.into())]
    orientation: Signal<Orientation>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Separator", classes);
    let context = use_context::<SeparatorContext>();
    // The element follows the orientation: an `<hr>` can't be vertical.
    let element_type = move || match (
        context.map_or(SeparatorElementType::Hr, |ctx| ctx.element_type),
        orientation.get(),
    ) {
        (SeparatorElementType::Hr, Orientation::Vertical) => SeparatorElementType::Div,
        (element_type, _) => element_type,
    };
    let element_type = Memo::new(move |_| element_type());
    move || {
        let element_type = element_type.get();
        let props = use_separator(UseSeparatorInput {
            orientation,
            element_type,
            id: id.clone(),
            aria_label,
            aria_labelledby: aria_labelledby.clone(),
        })
        .props
        .into_attrs();
        let (classes, styles) = (classes.clone(), styles.clone());
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
}
