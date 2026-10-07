// Upstream: react-aria-components/src/Toolbar.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{IntoAttrs, UseToolbarInput, use_toolbar},
    utils::{
        classes::Classes, default_class::with_default_class, orientation::Orientation,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Render props become the `data-orientation` attribute plus plain children.
//
// =============================================================================

/// A toolbar: a group of controls that is one tab stop, between whose focusable children the
/// arrow keys move focus. A toolbar inside another toolbar becomes a `group` of it.
///
/// Data attributes: `data-orientation` (`horizontal`, `vertical`).
///
/// Default class: `leptonic-Toolbar`.
#[component]
pub fn Toolbar(
    /// The axis of the arrow keys.
    #[prop(default = Orientation::Horizontal)]
    orientation: Orientation,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// The ids of the elements naming the toolbar. Ignored when `aria_label` is set.
    #[prop(into, optional)]
    aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Toolbar", classes);
    let toolbar = use_toolbar(UseToolbarInput {
        orientation,
        aria_label,
        aria_labelledby,
    });
    view! {
        <div
            {..toolbar.props.into_attrs()}
            class=classes
            style=styles
            data-orientation=orientation.as_str()
        >
            {children()}
        </div>
    }
}
